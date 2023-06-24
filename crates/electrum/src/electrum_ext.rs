use bdk_chain::{
    bitcoin::{hashes::hex::FromHex, BlockHash, OutPoint, Script, Transaction, Txid},
    keychain::LocalUpdate,
    local_chain::CheckPoint,
    tx_graph::{self, TxGraph},
    Anchor, BlockId, ConfirmationHeightAnchor, ConfirmationTimeAnchor,
};
use electrum_client::{Client, ElectrumApi, Error, HeaderNotification};
use std::{
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
};

#[derive(Debug, Clone)]
pub struct ElectrumUpdate<K, A> {
    pub graph_update: HashMap<Txid, BTreeSet<A>>,
    pub chain_update: CheckPoint,
    pub keychain_update: BTreeMap<K, u32>,
}

impl<K, A: Anchor> ElectrumUpdate<K, A> {
    pub fn new(cp: CheckPoint) -> Self {
        Self {
            graph_update: HashMap::new(),
            chain_update: cp,
            keychain_update: BTreeMap::new(),
        }
    }

    pub fn missing_full_txs<A2>(&self, graph: &TxGraph<A2>) -> Vec<Txid> {
        self.graph_update
            .keys()
            .filter(move |&&txid| graph.as_ref().get_tx(txid).is_none())
            .cloned()
            .collect()
    }

    pub fn finalize(
        self,
        client: &Client,
        seen_at: Option<u64>,
        missing: Vec<Txid>,
    ) -> Result<LocalUpdate<K, A>, Error> {
        let new_txs = client.batch_transaction_get(&missing)?;
        let mut graph_update = TxGraph::<A>::new(new_txs);
        for (txid, anchors) in self.graph_update {
            if let Some(seen_at) = seen_at {
                let _ = graph_update.insert_seen_at(txid, seen_at);
            }
            for anchor in anchors {
                let _ = graph_update.insert_anchor(txid, anchor);
            }
        }
        Ok(LocalUpdate {
            keychain: self.keychain_update,
            graph: graph_update,
            tip: self.chain_update,
        })
    }
}

impl<K> ElectrumUpdate<K, ConfirmationHeightAnchor> {
    /// Finalizes the [`ElectrumUpdate`] with `new_txs` and anchors of type
    /// [`ConfirmationTimeAnchor`].
    ///
    /// **Note:** The confirmation time might not be precisely correct if there has been a reorg.
    /// Electrum's API intends that we use the merkle proof API, we should change `bdk_electrum` to
    /// use it.
    pub fn finalize_as_confirmation_time(
        self,
        client: &Client,
        seen_at: Option<u64>,
        missing: Vec<Txid>,
    ) -> Result<LocalUpdate<K, ConfirmationTimeAnchor>, Error> {
        let update = self.finalize(client, seen_at, missing)?;

        let relevant_heights = {
            let mut visited_heights = HashSet::new();
            update
                .graph
                .all_anchors()
                .iter()
                .map(|(a, _)| a.confirmation_height_upper_bound())
                .filter(move |&h| visited_heights.insert(h))
                .collect::<Vec<_>>()
        };

        let height_to_time = relevant_heights
            .clone()
            .into_iter()
            .zip(
                client
                    .batch_block_header(relevant_heights)?
                    .into_iter()
                    .map(|bh| bh.time as u64),
            )
            .collect::<HashMap<u32, u64>>();

        let graph_additions = {
            let old_additions = TxGraph::default().determine_additions(&update.graph);
            tx_graph::Additions {
                txs: old_additions.txs,
                txouts: old_additions.txouts,
                last_seen: old_additions.last_seen,
                anchors: old_additions
                    .anchors
                    .into_iter()
                    .map(|(height_anchor, txid)| {
                        let confirmation_height = height_anchor.confirmation_height;
                        let confirmation_time = height_to_time[&confirmation_height];
                        let time_anchor = ConfirmationTimeAnchor {
                            anchor_block: height_anchor.anchor_block,
                            confirmation_height,
                            confirmation_time,
                        };
                        (time_anchor, txid)
                    })
                    .collect(),
            }
        };

        Ok(LocalUpdate {
            keychain: update.keychain,
            graph: {
                let mut graph = TxGraph::default();
                graph.apply_additions(graph_additions);
                graph
            },
            tip: update.tip,
        })
    }
}

pub trait ElectrumExt<A> {
    fn get_tip(&self) -> Result<(u32, BlockHash), Error>;

    fn scan<K: Ord + Clone>(
        &self,
        prev_tip: Option<CheckPoint>,
        keychain_spks: BTreeMap<K, impl IntoIterator<Item = (u32, Script)>>,
        txids: impl IntoIterator<Item = Txid>,
        outpoints: impl IntoIterator<Item = OutPoint>,
        stop_gap: usize,
        batch_size: usize,
    ) -> Result<ElectrumUpdate<K, A>, Error>;

    fn scan_without_keychain(
        &self,
        prev_tip: Option<CheckPoint>,
        misc_spks: impl IntoIterator<Item = Script>,
        txids: impl IntoIterator<Item = Txid>,
        outpoints: impl IntoIterator<Item = OutPoint>,
        batch_size: usize,
    ) -> Result<ElectrumUpdate<(), A>, Error> {
        let spk_iter = misc_spks
            .into_iter()
            .enumerate()
            .map(|(i, spk)| (i as u32, spk));

        self.scan(
            prev_tip,
            [((), spk_iter)].into(),
            txids,
            outpoints,
            usize::MAX,
            batch_size,
        )
    }
}

impl ElectrumExt<ConfirmationHeightAnchor> for Client {
    fn get_tip(&self) -> Result<(u32, BlockHash), Error> {
        // TODO: unsubscribe when added to the client, or is there a better call to use here?
        self.block_headers_subscribe()
            .map(|data| (data.height as u32, data.header.block_hash()))
    }

    fn scan<K: Ord + Clone>(
        &self,
        prev_tip: Option<CheckPoint>,
        keychain_spks: BTreeMap<K, impl IntoIterator<Item = (u32, Script)>>,
        txids: impl IntoIterator<Item = Txid>,
        outpoints: impl IntoIterator<Item = OutPoint>,
        stop_gap: usize,
        batch_size: usize,
    ) -> Result<ElectrumUpdate<K, ConfirmationHeightAnchor>, Error> {
        let mut request_spks = keychain_spks
            .into_iter()
            .map(|(k, s)| (k, s.into_iter()))
            .collect::<BTreeMap<K, _>>();
        let mut scanned_spks = BTreeMap::<(K, u32), (Script, bool)>::new();

        let txids = txids.into_iter().collect::<Vec<_>>();
        let outpoints = outpoints.into_iter().collect::<Vec<_>>();

        let update = loop {
            let mut update = ElectrumUpdate::<K, ConfirmationHeightAnchor>::new(
                prepare_chain_update(self, prev_tip.clone())?,
            );
            let anchor_block = update.chain_update.block_id();

            if !request_spks.is_empty() {
                if !scanned_spks.is_empty() {
                    scanned_spks.append(&mut populate_with_spks(
                        self,
                        anchor_block,
                        &mut update,
                        &mut scanned_spks
                            .iter()
                            .map(|(i, (spk, _))| (i.clone(), spk.clone())),
                        stop_gap,
                        batch_size,
                    )?);
                }
                for (keychain, keychain_spks) in &mut request_spks {
                    scanned_spks.extend(
                        populate_with_spks(
                            self,
                            anchor_block,
                            &mut update,
                            keychain_spks,
                            stop_gap,
                            batch_size,
                        )?
                        .into_iter()
                        .map(|(spk_i, spk)| ((keychain.clone(), spk_i), spk)),
                    );
                }
            }

            populate_with_txids(self, anchor_block, &mut update, &mut txids.iter().cloned())?;

            let _txs = populate_with_outpoints(
                self,
                anchor_block,
                &mut update,
                &mut outpoints.iter().cloned(),
            )?;

            // check for reorgs during scan process
            let server_blockhash = self
                .block_header(anchor_block.height as usize)?
                .block_hash();
            if anchor_block.hash != server_blockhash {
                continue; // reorg
            }

            update.keychain_update = request_spks
                .into_keys()
                .filter_map(|k| {
                    scanned_spks
                        .range((k.clone(), u32::MIN)..=(k.clone(), u32::MAX))
                        .rev()
                        .find(|(_, (_, active))| *active)
                        .map(|((_, i), _)| (k, *i))
                })
                .collect::<BTreeMap<_, _>>();
            break update;
        };

        Ok(update)
    }
}

/// Return a [`CheckPoint`] of the latest tip, that connects with the previous tip.
fn prepare_chain_update(
    client: &Client,
    prev_tip: Option<CheckPoint>,
) -> Result<CheckPoint, Error> {
    let HeaderNotification { height, mut header } = client.block_headers_subscribe()?;
    let mut height = height as u32;

    let (new_blocks, mut last_cp) = 'retry: loop {
        // this records new blocks, including blocks that are to be replaced
        let mut new_blocks = core::iter::once((height as _, header.block_hash()))
            .chain(
                height
                    .checked_sub(1)
                    .map(|h| (h as _, header.prev_blockhash)),
            )
            .collect::<BTreeMap<u32, BlockHash>>();

        let mut agreement_cp = Option::<CheckPoint>::None;

        for cp in prev_tip.iter().flat_map(CheckPoint::iter) {
            let cp_block = cp.block_id();
            // TODO: a batch request may be safer, as a reorg that happens when we are obtaining
            //       `block_header`s will result in inconsistencies
            let hash = client.block_header(cp_block.height as _)?.block_hash();
            if hash == cp_block.hash {
                agreement_cp = Some(cp);
                break;
            }
            new_blocks.insert(cp_block.height, hash);
        }

        // check for tip changes
        loop {
            match client.block_headers_pop()? {
                Some(new_notification) => {
                    let old_height = height;
                    height = new_notification.height as u32;
                    header = new_notification.header;
                    if height <= old_height {
                        // we may have a reorg
                        // reorg-detection logic can be improved (false positives are possible)
                        continue 'retry;
                    }
                }
                None => {
                    let new_blocks = match &agreement_cp {
                        // `new_blocks` should only include blocks that are actually new
                        Some(agreement_cp) => new_blocks.split_off(&(agreement_cp.height() + 1)),
                        None => new_blocks,
                    };

                    break 'retry (new_blocks, agreement_cp);
                }
            };
        }
    };

    // construct checkpoints
    for (height, hash) in new_blocks {
        let cp = match last_cp.clone() {
            Some(last_cp) => last_cp
                .extend(BlockId { height, hash })
                .expect("must extend checkpoint"),
            None => CheckPoint::new(BlockId { height, hash }),
        };
        last_cp = Some(cp);
    }

    Ok(last_cp.expect("must have atleast one checkpoint"))
}

fn determine_tx_anchor(
    anchor_block: BlockId,
    raw_height: i32,
    txid: Txid,
) -> Option<ConfirmationHeightAnchor> {
    // The electrum API has a weird quirk where an unconfirmed transaction is presented with a
    // height of 0. To avoid invalid representation in our data structures, we manually set
    // transactions residing in the genesis block to have height 0, then interpret a height of 0 as
    // unconfirmed for all other transactions.
    if txid
        == Txid::from_hex("4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b")
            .expect("must deserialize genesis coinbase txid")
    {
        return Some(ConfirmationHeightAnchor {
            anchor_block,
            confirmation_height: 0,
        });
    }
    match raw_height {
        h if h <= 0 => {
            debug_assert!(h == 0 || h == -1, "unexpected height ({}) from electrum", h);
            None
        }
        h => {
            let h = h as u32;
            if h > anchor_block.height {
                None
            } else {
                Some(ConfirmationHeightAnchor {
                    anchor_block,
                    confirmation_height: h,
                })
            }
        }
    }
}

fn populate_with_outpoints<K>(
    client: &Client,
    anchor_block: BlockId,
    update: &mut ElectrumUpdate<K, ConfirmationHeightAnchor>,
    outpoints: &mut impl Iterator<Item = OutPoint>,
) -> Result<HashMap<Txid, Transaction>, Error> {
    let mut full_txs = HashMap::new();
    for outpoint in outpoints {
        let txid = outpoint.txid;
        let tx = client.transaction_get(&txid)?;
        debug_assert_eq!(tx.txid(), txid);
        let txout = match tx.output.get(outpoint.vout as usize) {
            Some(txout) => txout,
            None => continue,
        };
        // attempt to find the following transactions (alongside their chain positions), and
        // add to our sparsechain `update`:
        let mut has_residing = false; // tx in which the outpoint resides
        let mut has_spending = false; // tx that spends the outpoint
        for res in client.script_get_history(&txout.script_pubkey)? {
            if has_residing && has_spending {
                break;
            }

            if res.tx_hash == txid {
                if has_residing {
                    continue;
                }
                has_residing = true;
                full_txs.insert(res.tx_hash, tx.clone());
            } else {
                if has_spending {
                    continue;
                }
                let res_tx = match full_txs.get(&res.tx_hash) {
                    Some(tx) => tx,
                    None => {
                        let res_tx = client.transaction_get(&res.tx_hash)?;
                        full_txs.insert(res.tx_hash, res_tx);
                        full_txs.get(&res.tx_hash).expect("just inserted")
                    }
                };
                has_spending = res_tx
                    .input
                    .iter()
                    .any(|txin| txin.previous_output == outpoint);
                if !has_spending {
                    continue;
                }
            };

            let anchor = determine_tx_anchor(anchor_block, res.height, res.tx_hash);

            let tx_entry = update.graph_update.entry(res.tx_hash).or_default();
            if let Some(anchor) = anchor {
                tx_entry.insert(anchor);
            }
        }
    }
    Ok(full_txs)
}

fn populate_with_txids<K>(
    client: &Client,
    anchor_block: BlockId,
    update: &mut ElectrumUpdate<K, ConfirmationHeightAnchor>,
    txids: &mut impl Iterator<Item = Txid>,
) -> Result<(), Error> {
    for txid in txids {
        let tx = match client.transaction_get(&txid) {
            Ok(tx) => tx,
            Err(electrum_client::Error::Protocol(_)) => continue,
            Err(other_err) => return Err(other_err),
        };

        let spk = tx
            .output
            .get(0)
            .map(|txo| &txo.script_pubkey)
            .expect("tx must have an output");

        let anchor = match client
            .script_get_history(spk)?
            .into_iter()
            .find(|r| r.tx_hash == txid)
        {
            Some(r) => determine_tx_anchor(anchor_block, r.height, txid),
            None => continue,
        };

        let tx_entry = update.graph_update.entry(txid).or_default();
        if let Some(anchor) = anchor {
            tx_entry.insert(anchor);
        }
    }
    Ok(())
}

fn populate_with_spks<K, I: Ord + Clone>(
    client: &Client,
    anchor_block: BlockId,
    update: &mut ElectrumUpdate<K, ConfirmationHeightAnchor>,
    spks: &mut impl Iterator<Item = (I, Script)>,
    stop_gap: usize,
    batch_size: usize,
) -> Result<BTreeMap<I, (Script, bool)>, Error> {
    let mut unused_spk_count = 0_usize;
    let mut scanned_spks = BTreeMap::new();

    loop {
        let spks = (0..batch_size)
            .map_while(|_| spks.next())
            .collect::<Vec<_>>();
        if spks.is_empty() {
            return Ok(scanned_spks);
        }

        let spk_histories = client.batch_script_get_history(spks.iter().map(|(_, s)| s))?;

        for ((spk_index, spk), spk_history) in spks.into_iter().zip(spk_histories) {
            if spk_history.is_empty() {
                scanned_spks.insert(spk_index, (spk, false));
                unused_spk_count += 1;
                if unused_spk_count > stop_gap {
                    return Ok(scanned_spks);
                }
                continue;
            } else {
                scanned_spks.insert(spk_index, (spk, true));
                unused_spk_count = 0;
            }

            for tx in spk_history {
                let tx_entry = update.graph_update.entry(tx.tx_hash).or_default();
                if let Some(anchor) = determine_tx_anchor(anchor_block, tx.height, tx.tx_hash) {
                    tx_entry.insert(anchor);
                }
            }
        }
    }
}
