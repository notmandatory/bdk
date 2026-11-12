# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.14.0](https://github.com/bitcoindevkit/bdk/compare/testenv-0.13.1...testenv-0.14.0)

### Added

- Add `TestEnv::mine_block` and `MineParams` to mine empty or mempool-inclusive blocks with custom timestamps and coinbase output scripts, and add `TestEnv::get_block_template` ([#2100](https://github.com/bitcoindevkit/bdk/pull/2100))
- Add `TestEnv::wait_until_electrum_tip_syncs_with_bitcoind` to wait for Electrum to match bitcoind's height and block hash ([#2231](https://github.com/bitcoindevkit/bdk/pull/2231))
- Add `TestEnv::get_block_hash` for retrieving a block hash by height ([#1826](https://github.com/bitcoindevkit/bdk/pull/1826))
- Add `persist_test_utils` with `assert_persist_changesets`, `tx_graph_changesets`, `local_chain_changesets`, and `keychain_txout_changesets` for testing persistence backends ([#2012](https://github.com/bitcoindevkit/bdk/pull/2012))
- Add `create_txout`, `create_test_tx`, `parse_descriptor`, and `spk_at_index` helpers to `utils`; enable the new `miniscript` feature by default for descriptor-based functionality ([#2012](https://github.com/bitcoindevkit/bdk/pull/2012))

### Changed

- **Breaking:** Upgrade `electrsd` from `0.28.0` to `0.38.0` and change the `download` feature to download Bitcoin Core `28.2` instead of `25.0` ([#1826](https://github.com/bitcoindevkit/bdk/pull/1826), [#2193](https://github.com/bitcoindevkit/bdk/pull/2193))
- **Breaking:** Replace the `bitcoincore_rpc` re-export and RPC interface with `corepc_client`; `TestEnv::rpc_client` now returns `&bitcoind::Client` ([#1826](https://github.com/bitcoindevkit/bdk/pull/1826), [#2193](https://github.com/bitcoindevkit/bdk/pull/2193))
- **Breaking:** Require `block_height` and optional `block_hash` arguments in `TestEnv::wait_until_electrum_sees_block` so it waits for the requested block instead of any new-block notification ([#2231](https://github.com/bitcoindevkit/bdk/pull/2231))
- **Breaking:** Return `CheckPoint<BlockHash>` from `TestEnv::make_checkpoint_tip` using the generic checkpoint API in `bdk_chain` `0.24.0` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582))
- **Breaking:** Gate `TestEnv::mine_empty_block` and `TestEnv::reorg_empty_blocks` behind the `std` feature ([#1826](https://github.com/bitcoindevkit/bdk/pull/1826))
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
