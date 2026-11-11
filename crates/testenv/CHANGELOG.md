# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.0](https://github.com/notmandatory/bdk/compare/testenv-0.13.1...testenv-0.14.0) - 2026-11-11

### Added

- *(testenv)* add `persist_test_utils` for testing persistence backends
- *(testenv)* add `mine_block` with custom timestamp and coinbase address

### Fixed

- *(ci,test)* update `chained_mempool_txs` to use 6 secs timeout

### Other

- update release process for release-plz
- *(testenv)* [**breaking**] make `wait_until_electrum_sees_block` concrete
- *(deps)* [**breaking**] bump `electrsd` to v0.38.0
- *(testenv)* fix rustdocs for `corepc` types
- *(testenv)* `mine_empty_blocks` requires `std` feature
- *(bdk_testenv)* bump `electrsd` to `0.36.1`
- *(msrv)* bump MSRV to `1.85.0`
- [**breaking**] make `CheckPoint`, `LocalChain`, and `SpkClient` take a generic

## [testenv-0.13.1]

### Changed

- Introduce usage of `cfg_attr(coverage_nightly)` in order to not consider tests under coverage. #1986
- deps: bump `bdk_chain` to 0.23.1

## [testenv-0.13.0]

### Changed

- deps: bump `bdk_chain` to 0.23.0

## [testenv-0.12.0]

### Changed

- deps: bump `bdk_chain` to 0.22.0

## [testenv-0.11.1]

### Changed

- Minor updates to fix new rustc 1.83.0 clippy warnings #1776

[testenv-0.11.1]: https://github.com/bitcoindevkit/bdk/releases/tag/testenv-0.11.1
[testenv-0.12.0]: https://github.com/bitcoindevkit/bdk/releases/tag/testenv-0.12.0
[testenv-0.13.0]: https://github.com/bitcoindevkit/bdk/releases/tag/testenv-0.13.0
[testenv-0.13.1]: https://github.com/bitcoindevkit/bdk/releases/tag/testenv-0.13.1
