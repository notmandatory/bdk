# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.7.0](https://github.com/bitcoindevkit/bdk/compare/core-0.6.2...core-0.7.0) - 2026-11-03

### Added

- Add a Bitcoin Core-style skip list to `CheckPoint`, making `get` and `floor_at` `O(log n)` and letting `range` jump to its upper bound in `O(log n)`; expose `CheckPoint::index` and `CheckPoint::skip`, and give `CheckPointIter` `O(log n)` `nth`/`last`, constant-time `count`, and `ExactSizeIterator` ([#2048](https://github.com/bitcoindevkit/bdk/pull/2048))
- Add the `ToBlockHash` and `ToBlockTime` traits so checkpoints can carry typed block data such as `Header` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582), [#2037](https://github.com/bitcoindevkit/bdk/pull/2037))
- Add `ToBlockHash::prev_blockhash` and `CheckPointEntry` (with `CheckPoint::entry_iter`) for `prev_blockhash`-aware iteration, including placeholder entries for heights inferred between sparse checkpoints ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115))
- Add `CheckPoint::median_time_past`, which computes the BIP-113 median time past over the preceding 11 blocks and returns `None` when any block is missing ([#2037](https://github.com/bitcoindevkit/bdk/pull/2037))
- Add the `ChainQuery` trait with `ChainRequest` and `ChainResponse` types, enabling sans-IO chain queries for `bdk_chain`'s `CanonicalTask` ([#2038](https://github.com/bitcoindevkit/bdk/pull/2038))
- Add read-only `SyncRequest::spks`, `SyncRequest::txids`, and `SyncRequest::outpoints` iterators, and rename the consuming `iter_*` methods to `drain_*` (the old names remain as deprecated wrappers) ([#2236](https://github.com/bitcoindevkit/bdk/pull/2236))
- Add `FullScanRequest::last_revealed` and `FullScanRequestBuilder::last_revealed_for_keychain`, so chain sources scan the revealed range before applying `stop_gap` ([#2182](https://github.com/bitcoindevkit/bdk/pull/2182))
- Derive `Clone` and `PartialEq` for `SyncResponse`, `FullScanResponse`, and `TxUpdate` ([#2417](https://github.com/bitcoindevkit/bdk/pull/2417))

### Changed

- **Breaking:** `CheckPoint` is generic over its data type `D` (defaulting to `BlockHash`); `CheckPoint::new`, `push`, and `insert` now take `(height, data)` instead of a `BlockId` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))
- Make `SyncRequest`, `SyncRequestBuilder`, `SyncResponse`, `FullScanRequest`, and `FullScanResponse` generic over the checkpoint data type `D` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))
- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))

### Fixed

- **Breaking:** `CheckPoint::push`, `extend`, and `from_blocks` now reject a `prev_blockhash` mismatch on contiguous heights, and `insert` evicts the conflicting checkpoint and everything above it; `insert` panics if the insertion would replace or omit the genesis checkpoint ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115))
- Fix a memory leak when dropping a `CheckPoint` chain ([#2056](https://github.com/bitcoindevkit/bdk/pull/2056))
- Prevent the `serde` feature from implicitly enabling `std` in `no_std` builds; the `std` feature now enables `serde?/std` ([0790f04](https://github.com/bitcoindevkit/bdk/commit/0790f040))
- Make `SyncResponse::is_empty` and `FullScanResponse::is_empty` available for any checkpoint data type `D` ([#2417](https://github.com/bitcoindevkit/bdk/pull/2417))

### Removed

- Remove `CheckPoint::from_block_ids` and `CheckPoint::from_header`; construct chains with `CheckPoint::new` and `CheckPoint::from_blocks` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))

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
