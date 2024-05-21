use crate::store::{ReadWrite, Store};
use crate::DbCommitment;
use anyhow::anyhow;
use bdk_chain::Anchor;
use bdk_persist::PersistBackend;
use serde::{Deserialize, Serialize};

impl<K, A> PersistBackend<DbCommitment<K, A>> for Store<K, A>
where
    K: Ord + for<'de> Deserialize<'de> + Serialize + Send,
    A: Anchor + for<'de> Deserialize<'de> + Serialize + Send,
{
    fn write_changes(&mut self, changeset: &DbCommitment<K, A>) -> anyhow::Result<()> {
        self.write(changeset)
            .map_err(|e| anyhow!(e).context("unable to write changes to sqlite database"))
    }

    fn load_from_persistence(&mut self) -> anyhow::Result<Option<DbCommitment<K, A>>> {
        self.read()
            .map_err(|e| anyhow!(e).context("unable to read changes from sqlite database"))
    }
}
