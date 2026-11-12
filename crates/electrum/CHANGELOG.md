# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.24.0](https://github.com/bitcoindevkit/bdk/compare/electrum-0.23.2...electrum-0.24.0)

### Changed

- **Breaking:** Update `bdk_core` to `0.7.0` ([3ec1b327](https://github.com/bitcoindevkit/bdk/commit/3ec1b327))
- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))

### Fixed

- Treat a `batch_size` of `0` as `1` in `full_scan` and `sync` instead of returning empty results ([#2360](https://github.com/bitcoindevkit/bdk/pull/2360))
- Return an error when the Electrum server's tip height exceeds `u32::MAX` instead of truncating it ([#2349](https://github.com/bitcoindevkit/bdk/pull/2349))
- Return an error when an Electrum batch response length differs from the request instead of silently dropping results or panicking ([#2334](https://github.com/bitcoindevkit/bdk/pull/2334))
- Propagate non-protocol errors when `sync` fetches transactions for requested outpoints ([#2325](https://github.com/bitcoindevkit/bdk/pull/2325))
- Fix stale anchors for transactions re-mined at the same height after a reorg by refreshing cached headers during chain updates ([#2320](https://github.com/bitcoindevkit/bdk/pull/2320))
- Fix `sync` missing confirmation updates for transactions whose first output is an unindexed `OP_RETURN` or `OP_FALSE OP_RETURN` script ([#2195](https://github.com/bitcoindevkit/bdk/pull/2195))
- Reject transactions returned by the server when their computed txid does not match the txid requested by `fetch_tx` ([#2188](https://github.com/bitcoindevkit/bdk/pull/2188))
- Ensure `full_scan` covers the entire revealed range before applying `stop_gap` ([#2182](https://github.com/bitcoindevkit/bdk/pull/2182))

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
