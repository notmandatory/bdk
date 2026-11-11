# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.23.0](https://github.com/notmandatory/bdk/compare/bitcoind_rpc-0.22.0...bitcoind_rpc-0.23.0) - 2026-11-11

### Added

- *(chain)* [**breaking**] classify timelocked outputs as `Immature`
- *(chain)* [**breaking**] derive balance trust from output ancestry
- *(chain)* Add min_confirmations parameter to CanonicalView::balance
- *(chain)* [**breaking**] Introduce `CanonicalView` and migrate API
- *(rpc)* [**breaking**] `FilterIter` API redesign
- *(bitcoind_rpc)* [**breaking**] Use `getrawmempool` without verbose
- *(chain)* [**breaking**] Clean up ergonomics of `IndexedTxGraph`
- *(chain)* `KeychainTxOutIndex`: Make spk cache optional
- *(chain)* [**breaking**] Persist spks derived from `KeychainTxOutIndex`
- *(bitcoind_rpc)* [**breaking**] Reduce friction of `Emitter` API.
- *(graph)* add convenience function for inserting relevant `evicted_at`s
- *(chain)* [**breaking**] Add ability to modify canonicalization algorithm
- *(rpc)* introduce `FilterIter`
- *(chain)* [**breaking**] `O(n)` canonicalization algorithm
- *(bitcoind_rpc)* [**breaking**] depend on `bdk_core` instead of `bdk_chain`
- [**breaking**] Rework sqlite, changesets, persistence and wallet-construction
- update `wallet::Balance` to use `bitcoin::Amount`
- *(chain)* [**breaking**] rm `local_chain::Update`
- *(chain)* add `get` and `range` methods to `CheckPoint`
- *(chain)* [**breaking**] make `IndexedTxGraph::apply_block_relevant` more efficient
- *(bitcoind_rpc)* [**breaking**] emissions include checkpoint and connected_to data
- [**breaking**] `LocalChain` with hardwired genesis checkpoint
- *(bitcoind_rpc)* introduce `bitcoind_rpc` crate

### Fixed

- *(bitcoind_rpc)* allow filter_iter example to compile on Windows
- *(rpc)* verify fetched mempool tx txids
- *(bitcoind_rpc)* emit invalidated heights when start_height is above agreement point
- *(bitcoind_rpc)* [**breaking**] Simplify emitter
- *(testenv)* disable downloads (bitcoind and electrsd) for docs.rs builds of crate testenv
- *(tests)* [**breaking**] remove println! usage from tests
- *(tx_graph)* [**breaking**] Change tx_last_seen to `Option<u64>`

### Other

- update release process for release-plz
- Merge bitcoindevkit/bdk#2401: fix(bitcoind_rpc): allow filter_iter example to compile on Windows
- *(rpc)* add Emitter::next_block and Emitter::mempool examples
- *(workspace)* use new `LocalChain::canonical_view` API
- *(deps)* [**breaking**] bump `electrsd` to v0.38.0
- use `core::error::Error`
- *(bdk_bitcoind_rpc)* update it's tests to use `ClientExt`
- *(msrv)* bump MSRV to `1.85.0`
- *(example)* Reuse `CanonicalView` in filter iter example
- [**breaking**] make `CheckPoint`, `LocalChain`, and `SpkClient` take a generic
- *(release)* bump `bdk-chain` to `0.23.2`
- *(bitcoind)* Test 6-block reorg to `filter_iter_detects_reorgs` test
- *(bip158)* Update `FilterIter` documentation
- *(bdk-chain)* bump to `0.23.1`, update `CHANGELOG`
- Merge bitcoindevkit/bdk#1986: ci: replace grcov by cargo-llvm-cov
- *(bitcoind_rpc)* fixed typo in docs
- replace grcov by cargo-llvm-cov
- *(bitcoind_rpc)* Make clippy happy
- *(bitcoind_rpc)* Detect new mempool txs
- Make clippy happy
- update changelogs
- bump `bdk_chain` to 0.23.0
- apply fmt
- Update changelogs
- bump bdk_chain to 0.22.0
- *(rpc)* [**breaking**] update `mempool` interface and test code
- Bump bdk_wallet to v1.1.0
- *(rpc)* add README and print ext address for FilterIter example
- Bump bdk_wallet version to 1.0.0
- Bump bdk_wallet version to 1.0.0-beta.6
- Bump bdk_wallet version to 1.0.0-beta.5
- Bump bdk_wallet version to 1.0.0-beta.4
- Bump bdk_wallet version to 1.0.0-beta.3
- Merge bitcoindevkit/bdk#1425: chore: add `print_stdout`/`print_stderr` lints to workspace level
- Bump bdk version to 1.0.0-beta.2
- add `print_stdout`/`print_stderr` lints to workspace level
- Bump bdk version to 1.0.0-beta.1
- *(chain)* [**breaking**] Rename `Append` to `Merge`
- *(chain)* [**breaking**] create module `indexer`
- Bump bdk version to 1.0.0-alpha.13
- *(bdk)* bump `bitcoin` to `0.32.0`, miniscript to `12.0.0`
- Bump bdk version to 1.0.0-alpha.12
- bump build_docs rust version to nightly-2024-05-12
- Bump bdk version to 1.0.0-alpha.11
- Merge bitcoindevkit/bdk#1411: feat: update `keychain::Balance` to use `bitcoin::Amount`
- remove `anyhow` dev dependency from `electrum`, `esplora`, and `bitcoind_rpc`
- Bump bdk version to 1.0.0-alpha.10
- Bump bdk version to 1.0.0-alpha.9
- Upgrade miniscript/bitcoin dependency
- Bump bdk version to 1.0.0-alpha.8
- extract `TestEnv` into separate crate
- Bump version to 1.0.0-alpha.7
- Bump version to 1.0.0-alpha.6
- Bump version to 1.0.0-alpha.5
- Merge bitcoindevkit/bdk#1172: Introduce block-by-block API to `bdk::Wallet` and add RPC wallet example
- Bump version to 1.0.0-alpha.4
- Bump bdk version to 1.0.0-alpha.3
- Improve TxGraph & co docs
- change MSRV to 1.63.0
- typos fixed
- *(bitcoind_rpc)* update docs for `Emitter::new`
- add meta data to bitcoind_rpc crate
- Bump bdk version to 1.0.0-alpha.2
- *(bitcoind_rpc)* better `Emitter::mempool` explanation
- [**breaking**] bring back `CheckPoint`s to `Emitter`
- rm `BlockHash` from `Emitter::last_mempool_tip`
- improvements to `IndexedTxGraph` and `TxGraph` APIs
- *(bitcoind_rpc)* add no_agreement_point test
- *(bitcoind_rpc)* initial tests for `Emitter`

## [bitcoind_rpc-0.22.0]

### Fixed
- `FilterIter` now handles reorgs to ensure consistency of the header chain. #2000

### Changed
- `Event` is now a struct instead of enum #2000

### Added
- `FilterIter::new` constructor that takes as input a reference to the RPC client, checkpoint, and a list of SPKs. #2000
- `Error::ReorgDepthExceeded` variant. #2000
- `Error::TryFromInt` variant. #2000

### Removed
- `FilterIter::new_with_height` method #2000
- `FilterIter::new_with_checkpoint` method #2000
- `EventInner` type #2000
- `FilterIter::get_tip` method #2000
- `FilterIter::chain_update` method #2000
- `Error::NoScripts` variant #2000

## [bitcoind_rpc-0.21.0]

### Added

- Introduce usage of `cfg_attr(coverage_nightly)` in order to not consider tests under coverage. #1986

### Changed

- deps: bump bdk_core to 0.6.1

### Fixed

- Some mempool transactions not being emitted at all, it was fixed by simplifying the emitter, and replaced the avoid-re-emission-logic with a new one that emits all mempool transactions. #1988
- Use the `getrawmempool` without verbose, as a more performant method. `Emitter::mempool` method now requires the `std` feature. A non-std version of this is added: `Emitter::mempool_at` #1988

## [bitcoind_rpc-0.20.0]

### Changed

- bump bdk_core to 0.6.0

## [bitcoind_rpc-0.19.0]

### Changed

- feat(rpc)!: Update Emitter::mempool to support evicted_at #1857
- Change Emitter::mempool to return MempoolEvents which contain mempool-eviction data.
- Change Emitter::client to have more relaxed generic bounds. C: Deref, C::Target: RpcApi are the new bounds.
- deps: bump `bdk_core` to 0.5.0

## [bitcoind_rpc-0.18.0]

### Added

- Added `bip158` module as a means of updating `bdk_chain` structures #1614

## [bitcoind_rpc-0.17.1]

### Changed

- Minor updates to fix new rustc 1.83.0 clippy warnings #1776

[bitcoind_rpc-0.17.1]: https://github.com/bitcoindevkit/bdk/releases/tag/bitcoind_rpc-0.17.1
[bitcoind_rpc-0.18.0]: https://github.com/bitcoindevkit/bdk/releases/tag/bitcoind_rpc-0.18.0
[bitcoind_rpc-0.19.0]: https://github.com/bitcoindevkit/bdk/releases/tag/bitcoind_rpc-0.19.0
[bitcoind_rpc-0.20.0]: https://github.com/bitcoindevkit/bdk/releases/tag/bitcoind_rpc-0.20.0
[bitcoind_rpc-0.21.0]: https://github.com/bitcoindevkit/bdk/releases/tag/bitcoind_rpc-0.21.0
[bitcoind_rpc-0.22.0]: https://github.com/bitcoindevkit/bdk/releases/tag/bitcoind_rpc-0.22.0