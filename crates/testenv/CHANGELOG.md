# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.0](https://github.com/bitcoindevkit/bdk/compare/testenv-0.13.1...testenv-0.14.0) - 2026-11-03

### Added

- Add a `persist_test_utils` module with `assert_persist_changesets` and sample `tx_graph`, `keychain_txout`, and `local_chain` changesets for testing persistence backends, plus `utils` helpers (`create_txout`, `create_test_tx`, `parse_descriptor`, `spk_at_index`) and a default-on `miniscript` feature ([#2012](https://github.com/bitcoindevkit/bdk/pull/2012))
- Add `MineParams` and `TestEnv::mine_block` to mine a block with a custom timestamp, coinbase address, and optionally mempool transactions; also add `TestEnv::get_block_template` ([#2100](https://github.com/bitcoindevkit/bdk/pull/2100))
- Add `TestEnv::wait_until_electrum_tip_syncs_with_bitcoind` to wait until Electrum's tip matches bitcoind's height and hash ([#2231](https://github.com/bitcoindevkit/bdk/pull/2231))

### Changed

- **Breaking:** `TestEnv::wait_until_electrum_sees_block` now takes a `block_height` and an optional `block_hash` in addition to the timeout ([#2231](https://github.com/bitcoindevkit/bdk/pull/2231))
- **Breaking:** Bump `electrsd` to `0.38.0`; the node RPC client is now the `corepc` client, so `TestEnv::rpc_client` returns `&electrsd::bitcoind::Client`, `electrsd::corepc_client` replaces the `electrsd::bitcoind::bitcoincore_rpc` re-export, and RPC responses are wrapped in `corepc` model types ([#2193](https://github.com/bitcoindevkit/bdk/pull/2193))
- **Breaking:** `mine_empty_block` and `reorg_empty_blocks` are now only available with the `std` feature ([9393832](https://github.com/bitcoindevkit/bdk/commit/93938326))
- **Breaking:** `TestEnv::make_checkpoint_tip` now returns `CheckPoint<BlockHash>`; bump `bdk_chain` to `0.24.0` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))
- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))

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
