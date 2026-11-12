# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

## [0.7.0](https://github.com/bitcoindevkit/bdk/compare/core-0.6.2...core-0.7.0)

### Added

- Generalize `CheckPoint<D = BlockHash>` to store block data through `ToBlockHash`, expose it with `data` and `data_ref`, and add checkpoint-data parameters to sync/full-scan requests, builders, and responses ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))
- Add `ToBlockTime` and `CheckPoint::median_time_past` for BIP-0113 median-time-past calculations ([#2037](https://github.com/bitcoindevkit/bdk/pull/2037))
- Add `ToBlockHash::prev_blockhash` and prev-blockhash-aware `CheckPointEntry` and `CheckPointEntryIter` iteration through `CheckPoint::entry_iter`, `to_entry`, and `into_entry` ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115))
- Add `ChainQuery`, `ChainRequest`, and `ChainResponse` for sans-I/O chain-verification tasks ([#2038](https://github.com/bitcoindevkit/bdk/pull/2038))
- Implement `serde::Serialize` and `serde::Deserialize` for `CheckPoint`, `TxUpdate`, `SyncResponse`, and `FullScanResponse` under the `serde` feature, using non-recursive flat checkpoint serialization ([#2232](https://github.com/bitcoindevkit/bdk/pull/2232))
- Add `Clone` and `PartialEq` implementations for `SyncResponse` and `FullScanResponse`, and `PartialEq` for `TxUpdate` ([#2417](https://github.com/bitcoindevkit/bdk/pull/2417))

### Changed

- **Breaking:** Change `CheckPoint::new`, `push`, `extend`, and `insert` to accept height and block data instead of `BlockId` values ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))
- **Breaking:** Validate `prev_blockhash` links between contiguous checkpoints: `CheckPoint::push`, `extend`, and `from_blocks` reject mismatches, while `insert` evicts checkpoints invalidated by a mismatch ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115))
- Speed up `CheckPoint::get`, `range`, `floor_at`, `floor_below`, and `CheckPointIter::nth` and `last` with logarithmic pskip traversal; expose `CheckPoint::index` and `skip`, and implement `ExactSizeIterator` for `CheckPointIter` ([#2048](https://github.com/bitcoindevkit/bdk/pull/2048))
- Split `SyncRequest` iteration into read-only `spks`, `txids`, and `outpoints` accessors and consuming `drain_spks_with_expected_txids`, `drain_txids`, and `drain_outpoints` iterators; deprecate the old `iter_*` methods ([#2236](https://github.com/bitcoindevkit/bdk/pull/2236))
- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))

### Fixed

- Fix memory leaks when dropping `CheckPoint` chains ([#2056](https://github.com/bitcoindevkit/bdk/pull/2056))
- Fix full scans applying `stop_gap` before covering all revealed scripts by tracking per-keychain indices with `FullScanRequestBuilder::last_revealed_for_keychain` and `FullScanRequest::last_revealed` ([#2182](https://github.com/bitcoindevkit/bdk/pull/2182))
- Stop the `serde` feature from implicitly enabling `std` in `no_std` builds ([#2227](https://github.com/bitcoindevkit/bdk/pull/2227))
- Make `SyncResponse::is_empty` and `FullScanResponse::is_empty` available for responses with custom checkpoint data types ([#2417](https://github.com/bitcoindevkit/bdk/pull/2417))

### Removed

- **Breaking:** Remove `CheckPoint::from_block_ids` and `CheckPoint::from_header`; use `CheckPoint::from_blocks` with generic block data instead ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))

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
