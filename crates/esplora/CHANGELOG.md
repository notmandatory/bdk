# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.23.0](https://github.com/notmandatory/bdk/compare/esplora-0.22.1...esplora-0.23.0) - 2026-11-03

### Added

- *(core)* split SyncRequest consuming and read-only iterators
- *(chain)* [**breaking**] Introduce `CanonicalView` and migrate API
- *(esplora)* Handle spks with expected txids
- [**breaking**] Improve spk-based syncing flow
- *(esplora)* [**breaking**] depend on `bdk_core` instead of `bdk_chain`
- [**breaking**] introduce `tx_graph::Update`
- [**breaking**] rework `FullScanRequest` and `SyncRequest`
- *(esplora)* always fetch prevouts
- *(esplora)* make ext traits more flexible
- *(chain)* [**breaking**] Implement `ConfirmationBlockTime`
- *(wallet)* add back TxBuilder finish() and sort_tx() with thread_rng()
- use `Amount` on `calculate_fee`, `fee_absolute`, `fee_amount` and others
- *(esplora)* [**breaking**] update to use new sync/full-scan structures
- *(chain)* [**breaking**] rm `local_chain::Update`
- *(esplora)* [**breaking**] simplify chain update logic
- *(esplora)* [**breaking**] remove `EsploraExt::update_local_chain`
- *(chain)* add `get` and `range` methods to `CheckPoint`
- *(chain)* [**breaking**] wrap `TxGraph` txs with `Arc`
- *(esplora)* include previous `TxOut`s for fee calculation
- *(esplora)* greatly simplify `update_local_chain`
- [**breaking**] `LocalChain` with hardwired genesis checkpoint

### Fixed

- *(esplora)* treat parallel_requests of 0 as 1
- *(esplora)* stop paging when the server does not advance the cursor
- full_scan covers revealed range before applying stop_gap
- *(esplora)* deduplicate missing txids in fetch_txs_with_outpoints
- *(esplora)* Bump `esplora-client` to `0.12.3`
- remove panic from stop gap scan loop
- *(esplora)* `chain_update` errors if no point of connection
- *(testenv)* disable downloads (bitcoind and electrsd) for docs.rs builds of crate testenv
- *(bdk_esplora)* build with `--no-default-features`
- *(esplora+wallet+file_store)* remove remaining `println!` usage
- enable blocking-https-rustls feature on esplora client
- define and document `stop_gap`
- *(esplora)* reuse returned height instead of zipping
- Remove dependency on unnecessary bdk_chain features
- *(esplora)* use saturating_add in update_tx_graph()
- improve more docs and more refactoring
- docs and some minor refactoring

### Other

- update release process for release-plz
- Merge bitcoindevkit/bdk#2350: docs(esplora): explain prevout trust assumptions
- *(esplora)* explain prevout trust assumptions
- *(workspace)* use new `LocalChain::canonical_view` API
- Remove example_cli
- *(deps)* [**breaking**] bump `electrsd` to v0.38.0
- *(esplora)* allow(dead_code) at file level in common
- *(bdk_esplora,bdk_electrum)* update to new `bdk_testenv` APIs
- *(example_cli,esplora)* Fix no-std compilation
- *(msrv)* bump MSRV to `1.85.0`
- [**breaking**] make `CheckPoint`, `LocalChain`, and `SpkClient` take a generic
- *(bdk-chain)* bump to `0.23.1`, update `CHANGELOG`
- replace grcov by cargo-llvm-cov
- *(esplora)* Allow `dead_code` for helper methods
- Make clippy happy
- *(esplora)* in debug build assert that `latest_blocks` is not empty
- update changelogs
- bump `bdk_chain` to 0.23.0
- apply fmt
- Update changelogs
- bump bdk_chain to 0.22.0
- rename example-crates directory to examples
- Bump bdk_wallet version to 1.0.0
- Bump bdk_wallet version to 1.0.0-beta.6
- *(esplora)* update README.md
- *(esplora)* bump `esplora-client` to 0.11.0
- Merge bitcoindevkit/bdk#1732: chore(core)!: rename `SyncResult` to `SyncResponse`
- *(core)* [**breaking**] rename `FullScanResult` to `FullScanResponse`
- *(core)* [**breaking**] rename `SyncResult` to `SyncResponse`
- Bump bdk_wallet version to 1.0.0-beta.5
- Merge bitcoindevkit/bdk#1626: deps(esplora): bump esplora-client to 0.10.0
- *(esplora)* bump esplora-client to 0.10.0
- use path as `name` for cargo test targets
- Bump bdk_wallet version to 1.0.0-beta.4
- Bump bdk_wallet version to 1.0.0-beta.3
- Merge bitcoindevkit/bdk#1425: chore: add `print_stdout`/`print_stderr` lints to workspace level
- Bump bdk version to 1.0.0-beta.2
- *(core)* [**breaking**] s/tx_graph::Update/TxUpdate/
- add `print_stdout`/`print_stderr` lints to workspace level
- *(esplora)* README example, uncomment async import
- *(esplora)* Fix typo
- *(esplora)* Simplify crate docs
- Bump bdk version to 1.0.0-beta.1
- *(chain)* [**breaking**] create module `indexer`
- Bump bdk version to 1.0.0-alpha.13
- *(chain)* Standardise KeychainTxOutIndex return types
- *(chain)* add type IndexSpk, fix clippy type complexity warning
- *(bdk)* bump `bitcoin` to `0.32.0`, miniscript to `12.0.0`
- Bump bdk version to 1.0.0-alpha.12
- bump build_docs rust version to nightly-2024-05-12
- Bump bdk version to 1.0.0-alpha.11
- Merge bitcoindevkit/bdk#1414: chore: clean up `electrsd` and `anyhow` dev dependencies
- *(esplora)* fixed `full_scan` and `sync` documentation
- remove `anyhow` dev dependency from `electrum`, `esplora`, and `bitcoind_rpc`
- Bump bdk version to 1.0.0-alpha.10
- Merge bitcoindevkit/bdk#1380: Simplified `EsploraExt` API
- *(esplora)* move esplora tests into src files
- *(esplora)* add `test_finalize_chain_update`
- Bump bdk version to 1.0.0-alpha.9
- Upgrade miniscript/bitcoin dependency
- Merge bitcoindevkit/bdk#1373: Wrap transactions as `Arc<Transaction>` in `TxGraph`
- Bump bdk version to 1.0.0-alpha.8
- *(esplora)* rm custom WASM target spec test dependency
- extract `TestEnv` into separate crate
- Bump version to 1.0.0-alpha.7
- Bump version to 1.0.0-alpha.6
- Merge bitcoindevkit/bdk#1308: feat(esplora): include previous `TxOut`s for fee calculation
- Merge bitcoindevkit/bdk#1186: Clean up clippy allows
- *(esplora)* `Box` a large `esplora_client::Error`
- Bump version to 1.0.0-alpha.5
- Merge bitcoindevkit/bdk#1267: Simplify `Esplora::update_local_chain` and add tests
- *(esplora)* better variable naming and docs
- *(esplora)* Clarify consistency guarantees
- Merge bitcoindevkit/bdk#1291: doc(chain,esplora): minor documentation improvements
- Merge bitcoindevkit/bdk#1288: doc(esplora): fix broken link in README
- *(esplora)* introduce test cases for `update_local_chain`
- Bump version to 1.0.0-alpha.4
- *(chain,esplora)* minor documentation improvements
- *(esplora)* fix broken link in README
- Bump bdk_esplora and bdk_file_store versions for 1.0.0-alpha.3 release
- Bump bdk version to 1.0.0-alpha.3
- *(esplora_ext)* rename scan_txs to sync and scan_txs_with_keychains to full_scan
- Merge bitcoindevkit/bdk#1179: build(esplora): Add async-https-rustls flag to esplora client
- also test the gap limit bounds in the async client
- also fix the gap limit check in the async client
- scan gap limit bounds testing
- fix incorrect gap limit check in blocking client
- *(esplora)* Add async-https-rustls flag to esplora client
- rename `ConfirmationTimeAnchor` to `ConfirmationTimeHeightAnchor`
- Bump bdk version to 1.0.0-alpha.2
- *(esplora)* fix wasm cargo check by setting workspace resolver to version 2
- *(esplora)* add async_ext and blocking_ext integration tests
- fix spelling errors
- rename methods in EsploraExt and EsploraExtAsync
- Update crates/esplora to rust-bitcoin 0.30.0
- Implement linked-list `LocalChain` and update chain-src crates/examples
- Bump version to 1.0.0-alpha.1
- Better no-std support
- [chain_redesign] Remove old structures
- Implement redesigned versions of `EsploraExt` and `EsploraAsyncExt`
- Add `last_seen` to the the `ConfirmationTime::Unconfirmed` variant
- Unpin base64
- Pin base64 to 0.21.0 to keep the MSRV to 1.57.0
- Bump bdk version to 1.0.0-alpha.0
- Add docs for `EsploraAsyncExt` and make doctests runnable
- Add documentation fixes
- Make async esplora futures Send
- More documentation improvements
- Add `wallet_esplora_async` example and various fixes
- Implement EsploraExt for Async client
- Make bdk_esplora wasm compatible again
- Update workspace dependencies to be relative paths
- Fix the "repository" field in Cargo.toml
- Fix cargo clippy warnings
- Move everything else over 🎉

## [esplora-0.22.1]

### Added
- Introduce usage of `cfg_attr(coverage_nightly)` in order to not consider tests under coverage. #1986

### Changed
- deps: bump `esplora-client` to 0.12.1
- deps: bump `bdk_core` to 0.6.1

### Fixed
- Esplora `chain_update` returns early with `esplora_client::Error::HeaderHashNotFound` if no point of agreement is found in the local checkpoint #1971

## [esplora-0.22.0]

### Changed

- bump bdk_core to 0.6.0

## [esplora-0.21.0]

### Changed

- Make full-scan/sync flow easier to reason about. #1838
- Change `bdk_esplora` to understand `SpkWithExpectedTxids`. #1839
- deps: bump `esplora-client` to 0.12.0
- deps: bump `bdk_core` to 0.5.0
- deps: remove optional dependency on `miniscript`

## [esplora-0.20.1]

### Changed

- Minor updates to fix new rustc 1.83.0 clippy warnings #1776

[esplora-0.20.1]: https://github.com/bitcoindevkit/bdk/releases/tag/esplora-0.20.1
[esplora-0.21.0]: https://github.com/bitcoindevkit/bdk/releases/tag/esplora-0.21.0
[esplora-0.22.0]: https://github.com/bitcoindevkit/bdk/releases/tag/esplora-0.22.0
[esplora-0.22.1]: https://github.com/bitcoindevkit/bdk/releases/tag/esplora-0.22.1
