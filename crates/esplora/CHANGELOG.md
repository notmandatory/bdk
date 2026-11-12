# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.23.0](https://github.com/bitcoindevkit/bdk/compare/esplora-0.22.1...esplora-0.23.0)

### Changed

- **Breaking:** Bump `bdk_core` to `0.7.0` ([3ec1b327](https://github.com/bitcoindevkit/bdk/commit/3ec1b327))
- Bump `esplora-client` to `0.12.3` ([#2136](https://github.com/bitcoindevkit/bdk/pull/2136))
- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))

### Fixed

- Treat `parallel_requests` of 0 as 1 instead of silently skipping `full_scan` or `sync` requests ([#2360](https://github.com/bitcoindevkit/bdk/pull/2360))
- Return `esplora_client::Error::InvalidResponse` from `full_scan` and `sync` when script-history pagination repeats a cursor instead of looping indefinitely ([#2361](https://github.com/bitcoindevkit/bdk/pull/2361))
- Ensure `full_scan` covers the entire revealed range before applying `stop_gap` ([#2182](https://github.com/bitcoindevkit/bdk/pull/2182))
- Avoid a panic when calculating the `full_scan` stop-gap boundary and treat `stop_gap` of 0 as 1 ([#2053](https://github.com/bitcoindevkit/bdk/pull/2053))
- Deduplicate spending-transaction fetches when multiple synced outpoints have the same spender ([#2104](https://github.com/bitcoindevkit/bdk/pull/2104))
- Fix builds with default features disabled by enabling `serde_json`'s `alloc` feature ([#2099](https://github.com/bitcoindevkit/bdk/pull/2099))

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
