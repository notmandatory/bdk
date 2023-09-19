use crate::store::{ReadWrite, Store};
use crate::ChangeSet;
use anyhow::anyhow;
use bdk_chain::ConfirmationTimeHeightAnchor;
use bdk_persist::PersistBackend;
use bdk_wallet::KeychainKind;

impl PersistBackend<bdk_wallet::wallet::ChangeSet>
    for Store<KeychainKind, ConfirmationTimeHeightAnchor>
{
    fn write_changes(&mut self, changeset: &bdk_wallet::wallet::ChangeSet) -> anyhow::Result<()> {
        let changeset = ChangeSet {
            network: changeset.network,
            chain: changeset.chain.clone(),
            tx_graph: changeset.indexed_tx_graph.clone(),
        };
        self.write(&changeset)
            .map_err(|e| anyhow!(e).context("unable to write wallet changes"))
    }

    fn load_from_persistence(&mut self) -> anyhow::Result<Option<bdk_wallet::wallet::ChangeSet>> {
        let changeset = self
            .read()
            .map_err(|e| anyhow!(e).context("unable to load wallet changes"))?;

        Ok(changeset.map(|changeset| bdk_wallet::wallet::ChangeSet {
            network: changeset.network,
            chain: changeset.chain,
            indexed_tx_graph: changeset.tx_graph,
        }))
    }
}
