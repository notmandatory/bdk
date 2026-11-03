# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.7.0](https://github.com/notmandatory/bdk/compare/core-0.6.2...core-0.7.0) - 2026-11-03

### Added

- *(core)* split SyncRequest consuming and read-only iterators
- *(core,chain)* introduce `CanonicalizationTask` and `ChainQuery`
- *(core)* Optimize `CheckPointIter` using pskip
- *(core)* [**breaking**] Remove unused methods of `CheckPointEntry`
- *(core)* add skiplist to CheckPoint for faster traversal
- *(chain,electrum)* Make clippy happy
- *(chain)* [**breaking**] Add `ApplyBlockError` for `prev_blockhash` validation
- *(core)* Initial work on `CheckPointEntry`
- *(core)* Add `prev_blockhash` method to `ToBlockHash` trait
- *(core)* add median-time-past calculation to CheckPoint
- *(core)* Add `is_empty` methods to `TxUpdate` and `{}_Response` types
- *(core)* [**breaking**] Remove redundant `SyncRequest` methods
- *(core)* Add expected txids to `SyncRequest` spks
- *(core)* Add `TxUpdate::evicted_ats`
- [**breaking**] Improve spk-based syncing flow
- *(core)* [**breaking**] Make `TxUpdate` non-exhaustive
- *(chain,core)* [**breaking**] move `Merge` to `bdk_core`
- *(core)* add `TxUpdate::map_anchors`
- [**breaking**] move `spk_client`s to `bdk_core`
- [**breaking**] move `tx_graph::Update` to `bdk_core`
- [**breaking**] move `CheckPoint` to `bdk_core`
- [**breaking**] introduce `bdk_core`

### Fixed

- *(core)* generalize is_empty for SyncResponse and FullScanResponse and derive Clone, PartialEq
- *(chain,core)* prevent serde from implicitly leaking std into no_std environments
- full_scan covers revealed range before applying stop_gap
- *(core)* `Checkpoint::insert` now evicts on `prev_blockhash` mismatch
- *(core)* `push` now errors on `prev_blockhash` mismatch
- *(docs)* in keychain_txout.rs and spk_client.rs
- *(core)* Memory leak bugs in `CheckPoint::drop` impl
- *(core)* Fix checkpoint Drop stack overflow
- *(core)* calling `CheckPoint::insert` with existing block must succeed
- typos

### Other

- update release process for release-plz
- Merge bitcoindevkit/bdk#2417: fix(core): generalize is_empty for SyncResponse and FullScanResponse and derive Clone, PartialEq
- cargo fmt `crates/core/src/lib.rs`
- Merge bitcoindevkit/bdk#2354: docs(core): document hashbrown feature's fixed-key hashing
- *(core)* document hashbrown feature's fixed-key hashing
- *(core)* require `CheckPoint` data to be determined by its block hash
- Merge bitcoindevkit/bdk#2233: test(core): cover SyncRequest inspect progress and expected txid merging
- *(core)* cover SyncRequest inspect progress and expected txid merging
- *(core)* add random-access skiplist benchmark
- Merge bitcoindevkit/bdk#2182: fix: full_scan covers revealed range before applying stop_gap
- address review feedback on `prev_blockhash` validation
- *(core)* address review feedback on docs and tests
- *(core)* address review feedback on checkpoint tests
- *(core)* Add module-level docs for `checkpoint_entry`
- *(core)* add tests for CheckPoint::push and insert methods
- *(chain)* make `TestLocalChain` generic and add `prev_blockhash` test
- document TxUpdate temporal context requirements
- Merge bitcoindevkit/bdk#2083: fix: no `Debug` on `Display` implementations
- *(core)* add detailed SyncRequest output example
- *(msrv)* bump MSRV to `1.85.0`
- *(chain)* update docs for `CheckPoint` and `SyncRequest`
- [**breaking**] make `CheckPoint`, `LocalChain`, and `SpkClient` take a generic
- *(release)* bump `bdk-chain` to `0.23.2`
- *(chain)* add doctest for min confirmation balance filtering
- *(bdk-chain)* bump to `0.23.1`, update `CHANGELOG`
- Make clippy happy
- update changelogs
- bump `bdk_chain` to 0.23.0
- Merge bitcoindevkit/bdk#1961: Add `is_empty` methods to `TxUpdate` and `{}_Response` types
- apply fmt
- Update changelogs
- bump bdk_chain to 0.22.0
- Bump bdk_wallet to v1.1.0
- add tests for  trait of operations on various data structures, including conflict scenarios and checking for empty structures
- Bump bdk_wallet version to 1.0.0
- *(clippy)* fix new stricter needless_lifetime errors for rust 1.83
- Bump bdk_wallet version to 1.0.0-beta.6
- Merge bitcoindevkit/bdk#1721: chore(deps): bump hashbrown to v0.14.5
- Merge bitcoindevkit/bdk#1731: fix(core): Fix checkpoint Drop stack overflow
- *(core)* [**breaking**] rename `FullScanResult` to `FullScanResponse`
- *(deps)* bump hashbrown to v0.14.5, disable default features
- *(core)* test `Drop` implementation of `CPInner`
- *(core)* [**breaking**] rename `SyncResult` to `SyncResponse`
- *(test)* move tx test utils to testenv
- Bump bdk_wallet version to 1.0.0-beta.5
- Bump bdk_wallet version to 1.0.0-beta.4
- Bump bdk_wallet version to 1.0.0-beta.3
- Merge bitcoindevkit/bdk#1601: fix(core): calling `CheckPoint::insert` with existing block must succeed
- fix off-by-one in `checkpoint_insert_existing`
- *(core)* document panic for `CheckPoint::insert`
- *(core)* remove bdk_chain dev-dependency version
- *(core)* add missing README.md
- Bump bdk version to 1.0.0-beta.2
- *(core)* [**breaking**] s/tx_graph::Update/TxUpdate/

## [core-0.6.2]

### Added

- Added two new convenience methods: `floor_at` and `floor_below` for getting the checkpoint at a floored height or by an offset from the tip. #2007

## [core-0.6.1]

### Changed

- Fix `clippy` warnings for `SpkItem` on `spk_client` #1990

## [core-0.6.0]

### Added

- Add `is_empty` methods to `TxUpdate` and `{}_Response` types #1961

## [core-0.5.0]

### Added

- Add `FullScanRequest::builder_at` and `SyncRequest::builder_at` methods which are the non-std version of the `..Request::builder` methods.
- Add `TxUpdate::evicted_ats` which tracks transactions that have been replaced and are no longer present in mempool.
- Add `SpkWithExpectedTxids` in `spk_client` which keeps track of expected `Txid`s for each `spk`.
- Add `SyncRequestBuilder::expected_txids_of_spk` method which adds an association between `txid`s and `spk`s.
- test: add tests for `Merge` trait #1738

### Changed

- Make full-scan/sync flow easier to reason about. #1838
- Change `FullScanRequest::builder` and `SyncRequest::builder` methods to depend on `feature = "std"`.
This is because requests now have a `start_time`, instead of specifying a `seen_at` when applying the update.
- Change `TxUpdate` to be `non-exhaustive`.
- Change `TxUpdate::seen_ats` field to be a `HashSet` of `(Txid, u64)`. This allows a single update to have multiple `seen_at`s per tx.
- Introduce `evicted-at`/`last-evicted` timestamps #1839

## [core-0.4.1]

### Changed

- Minor updates to fix new rustc 1.83.0 clippy warnings #1776

[core-0.4.1]: https://github.com/bitcoindevkit/bdk/releases/tag/core-0.4.1
[core-0.5.0]: https://github.com/bitcoindevkit/bdk/releases/tag/core-0.5.0
[core-0.6.0]: https://github.com/bitcoindevkit/bdk/releases/tag/core-0.6.0
[core-0.6.1]: https://github.com/bitcoindevkit/bdk/releases/tag/core-0.6.1
[core-0.6.2]: https://github.com/bitcoindevkit/bdk/releases/tag/core-0.6.2
