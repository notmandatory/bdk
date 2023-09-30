//! Async Blockchain
//!
//! This module provides async traits that can be used to fetch [`Update`] data from the bitcoin
//! blockchain and perform other common functions needed by a [`Wallet`] user.
//!
//! Also provides async implementations of these traits for the commonly used blockchain client protocol
//! [Esplora]. Creators of new or custom blockchain clients should implement which ever of these
//! traits that are applicable.
//!
//! [Esplora]: TBD

use async_trait::async_trait;
use std::boxed::Box;

use crate::wallet::Update;
use crate::FeeRate;
use crate::Wallet;
use bitcoin::Transaction;

/// Trait that defines broadcasting a transaction by a blockchain client
#[async_trait]
pub trait Broadcast {
    type Error: core::fmt::Debug;

    /// Broadcast a transaction
    async fn broadcast(&self, tx: &Transaction) -> Result<(), Self::Error>;
}

/// Trait that defines a function for estimating the fee rate by a blockchain backend or service.
#[async_trait]
pub trait EstimateFee {
    type Error: core::fmt::Debug;
    type Target: core::fmt::Debug;

    /// Estimate the fee rate required to confirm a transaction in a given `target` number of blocks.
    /// The `target` parameter can be customized with implementation specific features.
    async fn estimate_fee(&self, target: Self::Target) -> Result<FeeRate, Self::Error>;
}

/// Trait that defines a function to scan all script pub keys (spks).
#[async_trait]
pub trait ScanSpks {
    type Error: core::fmt::Debug;

    /// Iterate through script pub keys (spks) of all [`Wallet`] keychains and scan each spk for
    /// transactions. Scanning starts with the lowest derivation index spk and stop scanning after
    /// `stop_gap` number of consecutive spks have no transaction history. A Scan is usually done
    /// restoring a previously used wallet. It is a special case. Applications should use "sync"
    /// style updates after an initial scan or when creating a wallet from new and never used
    /// keychains. An ['Update'] is returned with newly found or updated transactions.
    ///
    /// The returned update must be applied to the wallet and persisted (if a data store is being use).
    /// For example:
    /// ```no_run
    /// # use bdk::{Wallet, wallet::Update};
    /// # let mut wallet: Wallet = todo!();
    /// # let update: Update = todo!();
    /// wallet.apply_update(update).expect("update applied");
    /// wallet.commit().expect("updates commited to db");
    /// ```
    async fn scan_spks(&self, wallet: &Wallet, stop_gap: usize) -> Result<Update, Self::Error>;
}

/// Trait that defines a function to sync the status of script pub keys (spks), UTXOs, and
/// unconfirmed transactions. The data to be synced can be adjusted, in an implementation specific
/// way, by providing a sync mode.
#[async_trait]
pub trait ModalSyncSpks {
    type Error: core::fmt::Debug;
    type SyncMode: core::fmt::Debug;

    /// Iterate through derived script pub keys (spks) of [`Wallet`] keychains and get their current
    /// transaction history. Also iterate through transaction UTXOs and unconfirmed transactions
    /// know by the [`Wallet`] to get their current status. An [`Update`] is returned with new
    /// or updated transactions and utxos. The data to be synced can be adjusted, in an
    /// implementation specific way, by providing a sync mode.
    ///
    /// The returned update must be applied to the wallet and persisted (if a data store is being use).
    /// For example:
    /// ```no_run
    /// # use bdk::{Wallet, wallet::Update};
    /// # let mut wallet: Wallet = todo!();
    /// # let update: Update = todo!();
    /// wallet.apply_update(update).expect("update applied");
    /// wallet.commit().expect("updates commited to db");
    /// ```
    async fn sync_spks(
        &self,
        wallet: &Wallet,
        sync_mode: Self::SyncMode,
    ) -> Result<Update, Self::Error>;
}

/// Trait that defines a function to sync the status of script pub keys (spks), UTXOs, and
/// unconfirmed transactions.
#[async_trait]
pub trait SyncSpks {
    type Error: core::fmt::Debug;

    /// Iterate through derived script pub keys (spks) of [`Wallet`] keychains and get their current
    /// transaction history. Also iterate through transaction UTXOs and unconfirmed transactions
    /// know by the [`Wallet`] to get their current status. An [`Update`] is returned with new
    /// or updated transactions and UTXOs.
    ///
    /// The returned update must be applied to the wallet and persisted (if a data store is being use).
    /// For example:
    /// ```no_run
    /// # use bdk::{Wallet, wallet::Update};
    /// # let mut wallet: Wallet = todo!();
    /// # let update: Update = todo!();
    /// wallet.apply_update(update).expect("update applied");
    /// wallet.commit().expect("updates commited to db");
    /// ```
    async fn sync_spks(&self, wallet: &Wallet) -> Result<Update, Self::Error>;
}
