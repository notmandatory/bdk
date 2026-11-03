# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.24.0](https://github.com/bitcoindevkit/bdk/compare/chain-0.23.2...chain-0.24.0) - 2026-11-03

### Added

- Add `CanonicalView`, a reusable canonical (ordered and conflict-resolved) view of a `TxGraph` with `txs`, `tx`, `txout`, `filter_outpoints`, `filter_unspent_outpoints` and `list_expected_spk_txids` methods ([#2029](https://github.com/bitcoindevkit/bdk/pull/2029))
- Add a two-phase sans-IO canonicalization pipeline: `CanonicalTask` selects the canonical transactions and `CanonicalViewTask` resolves their `ChainPosition`s, sharing the generic `Canonical<A, P>` container and the reworked `CanonicalTx<P>` and new `CanonicalTxOut<P>` types; `LocalChain::canonicalize` drives any `bdk_core::ChainQuery`, and `LocalChain::canonical_view` runs both phases ([#2038](https://github.com/bitcoindevkit/bdk/pull/2038))
- Add `CanonicalView::classify_outpoints` and the `Eligibility` (`Settled`, `Immature`, `Unsettled(Trust)`) and `Trust` types, plus the `taints_unowned` helper predicate ([#2246](https://github.com/bitcoindevkit/bdk/pull/2246))
- Add `ChainPosition::confirmations_lower_bound` ([#2263](https://github.com/bitcoindevkit/bdk/pull/2263))
- Add the `TxNode::last_evicted` field and `TxNode::is_evicted`, replacing `TxGraph::get_last_evicted` ([#2028](https://github.com/bitcoindevkit/bdk/pull/2028), [#2240](https://github.com/bitcoindevkit/bdk/pull/2240))
- Add `SpentTxOut` and `CreatedTxOut` plus `spent_txouts`/`created_txouts` methods to `SpkTxOutIndex` and `KeychainTxOutIndex` ([#2081](https://github.com/bitcoindevkit/bdk/pull/2081))
- Add `Indexer::rescan`, which indexes a whole `TxGraph` and lets the indexer decide how many passes are needed; **Breaking:** `Indexer` is no longer object-safe ([#2262](https://github.com/bitcoindevkit/bdk/pull/2262))
- Derive `Copy` for `Balance` and implement `AddAssign` and `Sum` ([#2367](https://github.com/bitcoindevkit/bdk/pull/2367))
- Add `CheckPointEntry` for `prev_blockhash`-aware checkpoint iteration, plus `ToBlockTime` and `CheckPoint::median_time_past` (re-exported from `bdk_core`) ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115), [#2037](https://github.com/bitcoindevkit/bdk/pull/2037))

### Changed

- **Breaking:** `CanonicalView::balance` and `classify_outpoints` now take plain `OutPoint`s plus `does_taint`, `is_settled` and `is_locked` predicates; trust is derived from an output's unconfirmed ancestry, and outputs with unmet timelocks are classified as `Immature` and counted in `Balance::immature` ([#2246](https://github.com/bitcoindevkit/bdk/pull/2246), [#2321](https://github.com/bitcoindevkit/bdk/pull/2321))
- **Breaking:** Correct `ChainPosition` ordering: confirmed positions sort before unconfirmed, same-anchor transitive confirmations before direct ones, never-in-mempool positions last, and unconfirmed positions by `first_seen` then `last_seen` ([#2074](https://github.com/bitcoindevkit/bdk/pull/2074))
- **Breaking:** `CheckPoint` and `LocalChain` are generic over the block data type, and `LocalChain<D>` now requires only `D: Clone` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582), [#2115](https://github.com/bitcoindevkit/bdk/pull/2115))
- **Breaking:** `CheckPoint::push` returns a `Result`, `CheckPoint::insert` evicts conflicting checkpoints on a `prev_blockhash` mismatch, and `LocalChain::apply_update` validates `prev_blockhash` and rejects a replacement genesis ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115))
- Accept `impl AsRef<Script>` in `SpkTxOutIndex::index_of_spk` and `KeychainTxOutIndex::index_of_spk` instead of `ScriptBuf` ([#2052](https://github.com/bitcoindevkit/bdk/pull/2052))
- **Breaking:** Bump `bdk_core` to `0.7.0`
- **Breaking:** Bump `miniscript` to `13.0.0`; `bdk_chain` re-exports `miniscript` ([#2106](https://github.com/bitcoindevkit/bdk/pull/2106))
- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))

### Fixed

- Fix `ChainPosition` resolution for assumed-canonical transactions that have an anchored or confirmed descendant ([#2038](https://github.com/bitcoindevkit/bdk/pull/2038))
- **Breaking:** `LocalChain::from_blocks`/`from_changeset` return `ApplyBlockError`, `apply_changeset` returns `ApplyChangeSetError`, and `insert_block` returns `InsertBlockError`; changesets can no longer replace or remove genesis, `prev_blockhash` mismatches are reported instead of a bogus genesis error, and `AlterCheckPointError` is removed ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115), [#2380](https://github.com/bitcoindevkit/bdk/pull/2380))
- Fix `KeychainTxOutIndex` derivation-index handling near `BIP32_MAX_INDEX`: clamp `last_revealed` in `apply_changeset`, saturate `lookahead_to_target`, and include the maximum index in the replenished lookahead range ([#2265](https://github.com/bitcoindevkit/bdk/pull/2265))
- Fix `IndexedTxGraph::reindex` to rescan until `KeychainTxOutIndex`'s revealed frontier stops moving, removing its dependence on `HashMap` iteration order ([#2262](https://github.com/bitcoindevkit/bdk/pull/2262))
- Fix `FullScanRequestBuilderExt::spks_from_indexer` to record each keychain's last revealed index, so full scans cover the revealed range before applying `stop_gap` ([#2182](https://github.com/bitcoindevkit/bdk/pull/2182))
- Reject an occupied index in `SpkTxOutIndex::insert_spk` without changing the existing script or output state, and make `KeychainTxOutIndex::reveal_next_spk` return `None` instead of panicking in release builds on overlapping descriptors ([#2335](https://github.com/bitcoindevkit/bdk/pull/2335), [#2377](https://github.com/bitcoindevkit/bdk/pull/2377))
- Skip evicted leftover transactions and coinbase transactions with a `last_seen` timestamp during canonicalization ([#2240](https://github.com/bitcoindevkit/bdk/pull/2240), [#2372](https://github.com/bitcoindevkit/bdk/pull/2372))
- Fix integer overflow in `SpkIterator::new_with_range` for bounds near `u32::MAX` ([#2180](https://github.com/bitcoindevkit/bdk/pull/2180))
- Stop `serde` from pulling in `std` in `no_std` builds ([0790f04](https://github.com/bitcoindevkit/bdk/commit/0790f040))
- Improve `Display` output for `CalculateFeeError` and `InsertDescriptorError` ([#2083](https://github.com/bitcoindevkit/bdk/pull/2083))

### Removed

- **Breaking:** Remove the `ChainOracle` trait, `CanonicalIter`, `TxGraph::canonical_iter`, and the `ChainOracle`-based `TxGraph` query methods (`list_canonical_txs`, `filter_chain_txouts`, `filter_chain_unspents`, `balance`, `list_expected_spk_txids` and their `try_*` variants); use `CanonicalTask` and `CanonicalView` instead ([#2029](https://github.com/bitcoindevkit/bdk/pull/2029), [#2038](https://github.com/bitcoindevkit/bdk/pull/2038))
- **Breaking:** Remove `Anchor::confirmation_height` and `Anchor::confirmation_height_upper_bound`; use `anchor_block().height` or `ChainPosition::confirmation_height_upper_bound` ([#2299](https://github.com/bitcoindevkit/bdk/pull/2299))

## [chain-0.23.2]

### Added

- Added a doctest illustrating how to filter confirmed balance results by simulating a minimum confirmation threshold via `chain_tip` height. #2007

### Fixed

- Behavior of `IndexedTxGraph` methods (`apply_block_relevant`, `batch_insert_relevant` and `batch_insert_relevant_unconfirmed`) to also consider conflicts of spk-relevant transactions as relevant. #2008

## [chain-0.23.1]

### Added

- Add new tests covering excluded bounds and the `SpkTxOutIndex::outputs_in_range` #1897
- Add new `reindex_tx_graph` benchmark, and unit test to cover `spk_cache` behavior #1968
- Add new `TxGraph::get_last_evicted` method #1977

### Changed

- Improve API docs, fixed parameter names to match the function or struct definition, and correct some spelling mistakes. #1968
- `KeychainTxOutIndex::apply_changeset` restores spk_cache before last revealed #1993
- deps: bump bdk_core to 0.6.1

### Fixed
- During canonicalization, exclude coinbase transactions when considering txs that are anchored in stale blocks #1976

## [chain-0.23.0]

### Added

- feat!(chain): implement `first_seen` tracking #1950
- fix(chain): persist `first_seen` #1966
- Persist spks derived from KeychainTxOutIndex #1963

### Changed

- fix(docs): `merge_chains` outdated documentation #1806
- chore: create and apply rustfmt.toml #1946
- chore: Update rust-version to 1.86.0 #1955
- bump bdk_core to 0.6.0

## [chain-0.22.0]

### Added

- Introduce evicted-at/last-evicted timestamps #1839
- Add method for constructing TxGraph from a ChangeSet #1930
- docs: Architectural Decision Records #1592
- Introduce canonicalization parameters #1808
- Add conversion impls for CanonicalTx to Txid/Arc<Transaction>.
- Add ChainPosition::is_unconfirmed method.

### Changed

- Make full-scan/sync flow easier to reason about. #1838
- Change `TxGraph` to track `last_evicted` timestamps. This is when a transaction is last marked as missing from the mempool.
- The canonicalization algorithm now disregards transactions with a `last_evicted` timestamp greater than or equal to it's `last_seen` timestamp, except when a canonical descendant exists due to rules of transitivity. #1839
- deps: bump miniscript to 12.3.1 #1924

### Fixed

- Fix canonicalization mess-up when transactions that conflict with itself are inserted. #1917

### Removed

- Remove `apply_update_at` as we no longer need to apply with a timestamp after-the-fact.

## [chain-0.21.1]

### Changed

- Minor updates to fix new rustc 1.83.0 clippy warnings #1776

[chain-0.21.1]: https://github.com/bitcoindevkit/bdk/releases/tag/chain-0.21.1
[chain-0.22.0]: https://github.com/bitcoindevkit/bdk/releases/tag/chain-0.22.0
[chain-0.23.0]: https://github.com/bitcoindevkit/bdk/releases/tag/chain-0.23.0
[chain-0.23.1]: https://github.com/bitcoindevkit/bdk/releases/tag/chain-0.23.1
[chain-0.23.2]: https://github.com/bitcoindevkit/bdk/releases/tag/chain-0.23.2