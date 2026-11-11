# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.24.0](https://github.com/notmandatory/bdk/compare/electrum-0.23.2...electrum-0.24.0) - 2026-11-11

### Added

- *(core)* split SyncRequest consuming and read-only iterators
- *(chain)* [**breaking**] classify timelocked outputs as `Immature`
- *(chain)* [**breaking**] derive balance trust from output ancestry
- *(chain,electrum)* Make clippy happy
- *(chain)* Add min_confirmations parameter to CanonicalView::balance
- *(chain)* [**breaking**] Introduce `CanonicalView` and migrate API
- *(electrum)* Add `populate_anchor_cache` method
- *(electrum)* batch `transaction.get_merkle` calls via `batch_call`
- *(electrum)* batched `Header`s and `script_get_history`
- *(electrum)* optimize merkle proof validation with batching
- *(chain)* [**breaking**] Add ability to modify canonicalization algorithm
- *(electrum)* Handle spks with expected txids
- [**breaking**] Improve spk-based syncing flow
- *(core)* [**breaking**] Make `TxUpdate` non-exhaustive
- *(bdk_electrum)* add `use-openssl` as a feature
- *(electrum)* [**breaking**] depend on `bdk_core` instead of `bdk_chain`
- *(chain)* [**breaking**] `TxGraph::apply_update` auto-adds `seen_at` for unanchored
- [**breaking**] introduce `tx_graph::Update`
- [**breaking**] rework `FullScanRequest` and `SyncRequest`
- enable selecting use-rustls-ring feature on electrum client
- *(testenv)* Added `bdk_electrum` wait for Txid method
- [**breaking**] Rework sqlite, changesets, persistence and wallet-construction
- *(chain)* [**breaking**] Implement `ConfirmationBlockTime`
- *(electrum)* [**breaking**] Update `bdk_electrum` to use merkle proofs
- *(electrum)* re-export `transaction_broadcast` method
- use `Amount` on `calculate_fee`, `fee_absolute`, `fee_amount` and others
- *(electrum)* update docs and simplify logic of `ElectrumExt`
- *(electrum)* include option for previous `TxOut`s for fee calculation
- *(chain)* [**breaking**] use custom return types for `ElectrumExt` methods
- *(electrum)* [**breaking**] use new sync/full-scan structs for `ElectrumExt`
- update `wallet::Balance` to use `bitcoin::Amount`
- *(chain)* [**breaking**] rm `local_chain::Update`
- *(electrum)* impl `ElectrumExt` for all that impl `ElectrumApi`
- [**breaking**] `LocalChain` with hardwired genesis checkpoint
- *(electrum)* [**breaking**] change signature of `ElectrumExt`

### Fixed

- *(electrum)* treat batch_size of 0 as 1
- *(electrum)* return error when tip height overflows u32
- *(electrum)* check batch response lengths before pairing
- *(electrum)* propagate transport errors when syncing outpoints
- *(electrum)* refresh header cache with headers fetched for the chain update
- *(electrum)* do not pick unindexed outputs for history lookup
- full_scan covers revealed range before applying stop_gap
- *(electrum)* verify txid of server-returned transactions
- *(clippy)* use `is_none_or` instead of `map_or`
- *(ci,test)* update `chained_mempool_txs` to use 6 secs timeout
- *(electrum)* clippy warnings
- *(electrum)* fix stale anchor hash on reorg
- *(electrum)* Return error on incorrect network
- *(electrum)* improve tx validation and gap limit scanning
- *(electrum)* Handle negative heights properly
- *(electrum)* prevent `fetch_prev_txout` from querying coinbase transactions
- *(testenv)* disable downloads (bitcoind and electrsd) for docs.rs builds of crate testenv
- *(bdk_electrum)* build with `--no-default-features`
- no premature collect
- *(tx_graph)* [**breaking**] Change tx_last_seen to `Option<u64>`
- typo on `SignedAmount` instead of `Amount`
- *(electrum)* Fix `fetch_prev_txout`
- *(electrum)* fixed `sync` functionality
- *(electrum)* [**breaking**] Remove `seen_at` param from `into_tx_graph`
- *(electrum)* fixed chain sync issue
- Remove dependency on unnecessary bdk_chain features
- bump electrum version to 0.18
- improve more docs and more refactoring

### Other

- update release process for release-plz
- Merge bitcoindevkit/bdk#2321: feat(chain)!: classify timelocked outputs as `Immature`
- Merge bitcoindevkit/bdk#2349: fix(electrum): return error when tip height overflows u32
- Merge bitcoindevkit/bdk#2325: fix(electrum): propagate transport errors when syncing outpoints
- *(testenv)* [**breaking**] make `wait_until_electrum_sees_block` concrete
- Merge bitcoindevkit/bdk#2320: fix(electrum): refresh header cache with headers fetched for the chain update
- *(electrum)* drop stale MSRV section from README
- *(electrum)* cover transport errors when syncing outpoints
- *(electrum)* cover tx re-mined at the same height after a reorg
- Update cont_integration.yml
- *(workspace)* use new `LocalChain::canonical_view` API
- Remove example_cli
- Merge bitcoindevkit/bdk#2195: fix(electrum): do not pick unindexed outputs for history lookup
- *(deps)* [**breaking**] bump `electrsd` to v0.38.0
- Merge bitcoindevkit/bdk#2182: fix: full_scan covers revealed range before applying stop_gap
- *(bdk_esplora,bdk_electrum)* update to new `bdk_testenv` APIs
- *(deps)* bump `criterion` to `0.7`
- *(msrv)* bump MSRV to `1.85.0`
- Merge bitcoindevkit/bdk#2029: `CanonicalView`
- [**breaking**] make `CheckPoint`, `LocalChain`, and `SpkClient` take a generic
- *(release)* bump `bdk-chain` to `0.23.2`
- *(bdk-chain)* bump to `0.23.1`, update `CHANGELOG`
- Merge bitcoindevkit/bdk#1989: chore(bdk-electrum): use new `batch_transaction_get_merkle` API
- *(bdk-electrum)* use new `batch_transaction_get_merkle` API
- Merge bitcoindevkit/bdk#1986: ci: replace grcov by cargo-llvm-cov
- *(electrum)* test sync with incorrect network
- *(electrum)* remove `unwrap()`s and `expect()`s
- replace grcov by cargo-llvm-cov
- Make clippy happy
- *(electrum)* Improve benchmark
- *(electrum)* add `criterion` benchmark for `sync`
- update changelogs
- bump `bdk_chain` to 0.23.0
- apply fmt
- update rust-version to 1.86.0
- Update changelogs
- bump bdk_chain to 0.22.0
- rename example-crates directory to examples
- *(electrum)* Chained mempool sync
- Bump bdk_wallet to v1.1.0
- update `README.md` with MSRV policy
- *(electrum)* test for checking that fee calculation is correct
- Bump bdk_wallet version to 1.0.0
- Bump bdk_wallet version to 1.0.0-beta.6
- *(electrum)* `fetch_prev_txout` does not process coinbase transactions
- *(electrum)* bump `electrum-client` to 0.22.0
- Merge bitcoindevkit/bdk#1732: chore(core)!: rename `SyncResult` to `SyncResponse`
- *(core)* [**breaking**] rename `FullScanResult` to `FullScanResponse`
- *(core)* [**breaking**] rename `SyncResult` to `SyncResponse`
- Bump bdk_wallet version to 1.0.0-beta.5
- use path as `name` for cargo test targets
- Bump bdk_wallet version to 1.0.0-beta.4
- Bump bdk_wallet version to 1.0.0-beta.3
- Merge bitcoindevkit/bdk#1425: chore: add `print_stdout`/`print_stderr` lints to workspace level
- Bump bdk version to 1.0.0-beta.2
- *(core)* [**breaking**] s/tx_graph::Update/TxUpdate/
- add `print_stdout`/`print_stderr` lints to workspace level
- Merge bitcoindevkit/bdk#1491: Enable selecting use-rustls-ring feature on electrum client
- *(electrum)* enhance API docs for fetch_prev_txouts argument
- *(electrum)* Test sync in reorg and no-reorg situations
- Bump bdk version to 1.0.0-beta.1
- *(electrum)* Imported `bdk_esplora` tests into `bdk_electrum`
- *(chain)* [**breaking**] create module `indexer`
- Bump bdk version to 1.0.0-alpha.13
- Merge bitcoindevkit/bdk#1458: fix: typo on `SignedAmount` instead of `Amount`
- *(bdk)* bump `bitcoin` to `0.32.0`, miniscript to `12.0.0`
- Merge bitcoindevkit/bdk#1453: refactor(electrum) put the tx cache in electrum
- *(electrum)* [**breaking**] put the tx cache in electrum
- Bump bdk version to 1.0.0-alpha.12
- Bump bdk version to 1.0.0-alpha.11
- *(electrum)* do not use `anyhow::Result` directly
- *(electrum)* fixed `full_scan`, `sync`, and crate documentation
- *(electrum)* remove `RelevantTxids` and track txs in `TxGraph`
- Merge bitcoindevkit/bdk#1411: feat: update `keychain::Balance` to use `bitcoin::Amount`
- remove `anyhow` dev dependency from `electrum`, `esplora`, and `bitcoind_rpc`
- Bump bdk version to 1.0.0-alpha.10
- Bump bdk version to 1.0.0-alpha.9
- Merge bitcoindevkit/bdk#1177: Upgrade bitcoin/miniscript dependencies
- Upgrade miniscript/bitcoin dependency
- Bump bdk version to 1.0.0-alpha.8
- *(electrum,testenv)* move `test_reorg_is_detected_in_electrsd`
- *(electrum)* added scan and reorg tests
- extract `TestEnv` into separate crate
- Bump version to 1.0.0-alpha.7
- Bump version to 1.0.0-alpha.6
- Bump version to 1.0.0-alpha.5
- Merge bitcoindevkit/bdk#1289: doc(electrum_ext): fix docs for `RelevantTxids::into_confirmation_tim…
- Bump version to 1.0.0-alpha.4
- *(electrum_ext)* fix docs for `RelevantTxids::into_confirmation_time_tx_graph`
- *(chain)* [**breaking**] revamp `KeychainTxOutIndex` API
- Bump bdk version to 1.0.0-alpha.3
- *(electrum_ext)* rename scan_without_keychain to sync and scan to full_scan
- make clippy happy and bump clippy msrv
- Merge bitcoindevkit/bdk#1206: chore: rename `ConfirmationTimeAnchor` to `ConfirmationTimeHeightAnchor`
- rename `ConfirmationTimeAnchor` to `ConfirmationTimeHeightAnchor`
- Bump bdk version to 1.0.0-alpha.2
- improve docs, cleanup unnecessary types and improve code
- Edit ElectrumExt not to use WalletUpdate
- refactor!(chain): Unify ChangeSet nomenclature
- Update crates/electrum to bitcoin 0.30.0
- Implement linked-list `LocalChain` and update chain-src crates/examples
- Bump version to 1.0.0-alpha.1
- Update documentation
- Rename fields of `tx_graph::Additions`
- [chain_redesign] Remove old structures
- `bdk_electrum` API improvements and simplifications
- Various tweaks to code arrangement and documentation
- [examples_redesign] Introduce `example_cli` package
- [electrum_redesign] Introduce redesigned `ElectrumExt`
- Add `last_seen` to the the `ConfirmationTime::Unconfirmed` variant
- Bump bdk version to 1.0.0-alpha.0
- Add documentation fixes
- Remove transaction-based type parameters and traits
- Changed `inflate_update` logic to not depend on `Cow`
- Update workspace dependencies to be relative paths
- Fix the "repository" field in Cargo.toml
- Fix cargo clippy warnings
- Move everything else over 🎉
- Move bdk_electrum into the bdk repo :tada:

## [electrum-0.23.2]

### Added

- Add a new `populate_anchor_cache` method. #2005

### Changed

- Update the `populate_tx_cache` method documentation. #2005

### Fixed

- Update the `batch_fetch_anchors` to no longer uses a potentially stale hash as the anchor. #2011

## [electrum-0.23.1]

### Added
- Introduce usage of `cfg_attr(coverage_nightly)` in order to not consider tests under coverage. #1986

### Changed

- Add transaction anchor cache to prevent redundant network calls. #1957
- Batch Merkle Proof, script history, and header requests. #1957
- Fix `clippy` warnings on electrum-client batch call #1990
- deps: bump bdk_core to 0.6.1
- deps: bump electrum-client to 0.24.0

### Fixed
- Removed all `unwrap()`s and `expect()`s from `bdk_electrum_client.rs` #1981

## [electrum-0.23.0]

### Changed

- bump bdk_core to 0.6.0

## [electrum-0.22.0]

### Fixed

- Fix `bdk_electrum` handling of negative spk-history height. #1837

### Changed

- Make full-scan/sync flow easier to reason about. #1838
- Change `bdk_electrum` to understand `SpkWithExpectedTxids`. #1839
- deps: bump `electrum-client` to 0.23.1
- deps: bump `bdk_core` to 0.5.0

## [electrum-0.21.0]

### Changed

- Bump crate MSRV to 1.75.0 #1803
- deps: bump `electrum-client` to 0.23.0
- add test for checking that fee calculation is correct #1685

## [electrum-0.20.1]

### Changed

- Minor updates to fix new rustc 1.83.0 clippy warnings #1776

[electrum-0.20.1]: https://github.com/bitcoindevkit/bdk/releases/tag/electrum-0.20.1
[electrum-0.21.0]: https://github.com/bitcoindevkit/bdk/releases/tag/electrum-0.21.0
[electrum-0.22.0]: https://github.com/bitcoindevkit/bdk/releases/tag/electrum-0.22.0
[electrum-0.23.0]: https://github.com/bitcoindevkit/bdk/releases/tag/electrum-0.23.0
[electrum-0.23.1]: https://github.com/bitcoindevkit/bdk/releases/tag/electrum-0.23.1
[electrum-0.23.2]: https://github.com/bitcoindevkit/bdk/releases/tag/electrum-0.23.2
