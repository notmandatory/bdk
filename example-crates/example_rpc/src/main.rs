use std::{
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant, SystemTime},
};

use bdk_bitcoind_rpc::{
    bitcoincore_rpc::{Auth, Client, RpcApi},
    Emitter,
};
use bdk_chain::{
    bitcoin::{hashes::Hash, BlockHash},
    indexed_tx_graph, keychain,
    local_chain::{self, CheckPoint, LocalChain},
    BlockId, ConfirmationTimeAnchor, IndexedTxGraph,
};
use example_cli::{
    anyhow,
    clap::{self, Args, Subcommand},
    Keychain,
};

const DB_MAGIC: &[u8] = b"bdk_example_rpc";
const DB_PATH: &str = ".bdk_example_rpc.db";
// const CHANNEL_BOUND: usize = 10;
// const LIVE_POLL_DUR_SECS: Duration = Duration::from_secs(15);

/// The block depth which we assume no reorgs can happen at.
const ASSUME_FINAL_DEPTH: u32 = 6;
const DB_COMMIT_DELAY_SEC: u64 = 30;

type ChangeSet = (
    local_chain::ChangeSet,
    indexed_tx_graph::ChangeSet<ConfirmationTimeAnchor, keychain::ChangeSet<Keychain>>,
);

#[derive(Args, Debug, Clone)]
struct RpcArgs {
    /// RPC URL
    #[clap(env = "RPC_URL", long, default_value = "127.0.0.1:8332")]
    url: String,
    /// RPC auth cookie file
    #[clap(env = "RPC_COOKIE", long)]
    rpc_cookie: Option<PathBuf>,
    /// RPC auth username
    #[clap(env = "RPC_USER", long)]
    rpc_user: Option<String>,
    /// RPC auth password
    #[clap(env = "RPC_PASS", long)]
    rpc_password: Option<String>,
}

impl From<RpcArgs> for Auth {
    fn from(args: RpcArgs) -> Self {
        match (args.rpc_cookie, args.rpc_user, args.rpc_password) {
            (None, None, None) => Self::None,
            (Some(path), _, _) => Self::CookieFile(path),
            (_, Some(user), Some(pass)) => Self::UserPass(user, pass),
            (_, Some(_), None) => panic!("rpc auth: missing rpc_pass"),
            (_, None, Some(_)) => panic!("rpc auth: missing rpc_user"),
        }
    }
}

impl RpcArgs {
    fn new_client(&self) -> anyhow::Result<Client> {
        Ok(Client::new(
            &self.url,
            match (&self.rpc_cookie, &self.rpc_user, &self.rpc_password) {
                (None, None, None) => Auth::None,
                (Some(path), _, _) => Auth::CookieFile(path.clone()),
                (_, Some(user), Some(pass)) => Auth::UserPass(user.clone(), pass.clone()),
                (_, Some(_), None) => panic!("rpc auth: missing rpc_pass"),
                (_, None, Some(_)) => panic!("rpc auth: missing rpc_user"),
            },
        )?)
    }
}

#[derive(Subcommand, Debug, Clone)]
enum RpcCommands {
    /// Syncs local state with remote state via RPC (starting from last point of agreement) and
    /// stores/indexes relevant transactions
    Sync {
        /// Starting block height to fallback to if no point of agreement if found
        #[clap(env = "FALLBACK_HEIGHT", long, default_value = "0")]
        fallback_height: u32,
        /// The unused-scripts lookahead will be kept at this size
        #[clap(long, default_value = "10")]
        lookahead: u32,
        #[clap(flatten)]
        rpc_args: RpcArgs,
    },
}

fn main() -> anyhow::Result<()> {
    // let sigterm_flag = start_ctrlc_handler();

    let (args, keymap, index, db, init_changeset) =
        example_cli::init::<RpcCommands, RpcArgs, ChangeSet>(DB_MAGIC, DB_PATH)?;

    let graph = Mutex::new({
        let mut graph = IndexedTxGraph::new(index);
        graph.apply_changeset(init_changeset.1);
        graph
    });

    let chain = Mutex::new(LocalChain::from_changeset(init_changeset.0));

    let rpc_cmd = match args.command {
        example_cli::Commands::ChainSpecific(rpc_cmd) => rpc_cmd,
        general_cmd => {
            let res = example_cli::handle_commands(
                &graph,
                &db,
                &chain,
                &keymap,
                args.network,
                |rpc_args, tx| {
                    let client = rpc_args.new_client()?;
                    client.send_raw_transaction(tx)?;
                    Ok(())
                },
                general_cmd,
            );
            db.lock().unwrap().commit()?;
            return res;
        }
    };

    match rpc_cmd {
        RpcCommands::Sync {
            fallback_height,
            lookahead,
            rpc_args,
        } => {
            let mut chain = chain.lock().unwrap();
            let mut graph = graph.lock().unwrap();
            let mut db = db.lock().unwrap();

            graph.index.set_lookahead_for_all(lookahead);

            // we start at a height lower than last-seen tip in case of reorgs
            let prev_cp = chain.tip();
            let start_height = prev_cp.as_ref().map_or(fallback_height, |cp| {
                cp.height().saturating_sub(ASSUME_FINAL_DEPTH)
            });

            let rpc_client = rpc_args.new_client()?;
            let mut emitter = Emitter::new(&rpc_client, start_height);

            let mut last_db_commit = Instant::now();

            while let Some(block) = emitter.next_block()? {
                let this_id = BlockId {
                    height: block.height,
                    hash: block.block.block_hash(),
                };
                let update_cp = if block.block.header.prev_blockhash == BlockHash::all_zeros() {
                    CheckPoint::new(this_id)
                } else {
                    CheckPoint::new(BlockId {
                        height: block.height - 1,
                        hash: block.block.header.prev_blockhash,
                    })
                    .extend(core::iter::once(this_id))
                    .expect("must construct checkpoint")
                };

                let chain_changeset = chain.apply_update(local_chain::Update {
                    tip: update_cp,
                    introduce_older_blocks: false,
                })?;
                let graph_changeset = graph.apply_block(block.block, block.height);

                db.stage((chain_changeset, graph_changeset));
                if last_db_commit.elapsed() >= Duration::from_secs(DB_COMMIT_DELAY_SEC) {
                    db.commit()?;
                    last_db_commit = Instant::now();
                }
            }

            // mempool
            let mempool_txs = emitter.mempool()?;
            let graph_changeset = graph.batch_insert_unconfirmed(
                mempool_txs.iter().map(|m_tx| (&m_tx.tx, Some(m_tx.time))),
            );
            db.stage((local_chain::ChangeSet::default(), graph_changeset));

            // commit one last time!
            db.commit()?;
        }
    }

    Ok(())
}

#[allow(dead_code)]
fn start_ctrlc_handler() -> Arc<AtomicBool> {
    let flag = Arc::new(AtomicBool::new(false));
    let cloned_flag = flag.clone();

    ctrlc::set_handler(move || cloned_flag.store(true, Ordering::Release));

    flag
}

#[allow(dead_code)]
fn await_flag(flag: &AtomicBool, duration: Duration) -> bool {
    let start = SystemTime::now();
    loop {
        if flag.load(Ordering::Acquire) {
            return true;
        }
        if SystemTime::now()
            .duration_since(start)
            .expect("should succeed")
            >= duration
        {
            return false;
        }
        std::thread::sleep(Duration::from_secs(1));
    }
}
