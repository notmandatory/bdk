# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.24.0](https://github.com/bitcoindevkit/bdk/compare/chain-0.23.2...chain-0.24.0)

### Added

- Add `CanonicalView` to canonicalize once and reuse the result for transaction/output queries and balance calculation ([#2029](https://github.com/bitcoindevkit/bdk/pull/2029))
- Add the generic `Canonical<A, P>` container, `CanonicalTxs`, `CanonicalTask`, and `CanonicalViewTask` for two-phase sans-IO canonicalization through `ChainQuery`, driven by `LocalChain::canonicalize` or `LocalChain::canonical_view` ([#2038](https://github.com/bitcoindevkit/bdk/pull/2038))
- Add `CanonicalView::classify_outpoints`, `Eligibility`, `Trust`, and the `taints_unowned` predicate helper for ancestry-aware output classification ([#2246](https://github.com/bitcoindevkit/bdk/pull/2246))
- Add `ChainPosition::confirmations_lower_bound`, returning `0` for unconfirmed positions or confirmation heights above the supplied tip ([#2263](https://github.com/bitcoindevkit/bdk/pull/2263))
- Add `spent_txouts` and `created_txouts` to `SpkTxOutIndex` and `KeychainTxOutIndex`, returning `SpentTxOut` and `CreatedTxOut` values with script indices and input/output positions ([#2081](https://github.com/bitcoindevkit/bdk/pull/2081))
- **Breaking:** Add defaulted `Indexer::rescan` for indexer-controlled whole-graph rescans; the generic method makes `Indexer` no longer object-safe ([#2262](https://github.com/bitcoindevkit/bdk/pull/2262))
- Make `Balance` implement `Copy`, `AddAssign`, and `Sum` ([#2367](https://github.com/bitcoindevkit/bdk/pull/2367))

### Changed

- **Breaking:** Generalize `LocalChain` and `local_chain::ChangeSet` over checkpoint data: `from_genesis_hash` becomes `from_genesis`, `insert_block` accepts height and data instead of `BlockId`, and chain operations require `D: Clone` ([#1582](https://github.com/bitcoindevkit/bdk/pull/1582), [#2115](https://github.com/bitcoindevkit/bdk/pull/2115))
- **Breaking:** Move balance calculation to `CanonicalView::balance`, which accepts plain `OutPoint`s plus `does_taint`, `is_settled`, and `is_locked` predicates; derive pending trust from unsettled ancestry and count timelocked outputs as `Balance::immature` ([#2029](https://github.com/bitcoindevkit/bdk/pull/2029), [#2246](https://github.com/bitcoindevkit/bdk/pull/2246), [#2321](https://github.com/bitcoindevkit/bdk/pull/2321))
- **Breaking:** Replace `TxGraph::get_last_evicted` with the public `TxNode::last_evicted` field and add `TxNode::is_evicted` for comparing eviction and last-seen timestamps ([#2028](https://github.com/bitcoindevkit/bdk/pull/2028), [#2240](https://github.com/bitcoindevkit/bdk/pull/2240))
- **Breaking:** Change `SpkTxOutIndex::index_of_spk` and `KeychainTxOutIndex::index_of_spk` to accept generic `AsRef<Script>` arguments instead of owned `ScriptBuf`s ([#2052](https://github.com/bitcoindevkit/bdk/pull/2052))
- **Breaking:** Use human-readable `Display` output for `CalculateFeeError` and `InsertDescriptorError<K>` instead of `Debug` formatting, requiring `K: Display` for the latter ([#2083](https://github.com/bitcoindevkit/bdk/pull/2083))
- **Breaking:** Validate `prev_blockhash` links when constructing, extending, and merging `LocalChain`; return `ApplyBlockError` from `from_blocks`/`from_changeset`, `ApplyChangeSetError` from `apply_changeset`, and `InsertBlockError` from `insert_block` ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115), [#2380](https://github.com/bitcoindevkit/bdk/pull/2380))
- **Breaking:** Update public dependency baselines to `bdk_core` `0.7.0` and `miniscript` `13.0.0` ([3ec1b327](https://github.com/bitcoindevkit/bdk/commit/3ec1b327), [#2106](https://github.com/bitcoindevkit/bdk/pull/2106))
- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))

### Fixed

- Keep genesis immutable during `LocalChain` updates and changeset application, and make `insert_block` report replacement or `prev_blockhash` failures accurately ([#2115](https://github.com/bitcoindevkit/bdk/pull/2115), [#2380](https://github.com/bitcoindevkit/bdk/pull/2380))
- Order `ChainPosition::Unconfirmed` values with mempool timestamps before never-seen transactions, using `last_seen` as a tiebreaker ([#2074](https://github.com/bitcoindevkit/bdk/pull/2074))
- Make `FullScanRequestBuilderExt::spks_from_indexer` cover the full revealed range before applying `stop_gap` ([#2182](https://github.com/bitcoindevkit/bdk/pull/2182))
- Prevent integer overflow at range boundaries in `SpkIterator::new_with_range`, including `0..=u32::MAX` ([#2180](https://github.com/bitcoindevkit/bdk/pull/2180))
- Skip evicted transactions during leftover canonicalization so stale anchors cannot make them unconfirmed again ([#2240](https://github.com/bitcoindevkit/bdk/pull/2240))
- Make `IndexedTxGraph::reindex` deterministic by rescanning `KeychainTxOutIndex` until its revealed frontier stops growing, preventing missed lookahead outputs ([#2262](https://github.com/bitcoindevkit/bdk/pull/2262))
- Clamp `KeychainTxOutIndex` derivation indices to `BIP32_MAX_INDEX` and fix overflow and off-by-one behavior in `lookahead_to_target` and lookahead replenishment ([#2265](https://github.com/bitcoindevkit/bdk/pull/2265))
- Ignore `last_seen` on coinbase transactions during canonicalization and return `false` from `CanonicalTxOut::is_mature` when maturity cannot be proven instead of panicking in debug builds ([#2372](https://github.com/bitcoindevkit/bdk/pull/2372))
- Reject an occupied index in `SpkTxOutIndex::insert_spk` without changing existing script, output, or used-status mappings ([#2335](https://github.com/bitcoindevkit/bdk/pull/2335))
- Return `None` from `KeychainTxOutIndex::reveal_next_spk` instead of panicking in release builds for overlapping descriptors ([#2377](https://github.com/bitcoindevkit/bdk/pull/2377))
- Prevent the `serde` feature from implicitly enabling `std`, and implement `core::error::Error` for public errors without requiring `std` ([#2137](https://github.com/bitcoindevkit/bdk/pull/2137), [0790f040](https://github.com/bitcoindevkit/bdk/commit/0790f040))

### Removed

- **Breaking:** Remove `CanonicalIter`, `ChainOracle`, and `FullTxOut`; use the two-phase canonicalization API, `LocalChain::canonical_view`, and `CanonicalTxOut` instead ([#2029](https://github.com/bitcoindevkit/bdk/pull/2029), [#2038](https://github.com/bitcoindevkit/bdk/pull/2038))
- **Breaking:** Remove `AlterCheckPointError` in favor of `ApplyChangeSetError` and `InsertBlockError` ([#2380](https://github.com/bitcoindevkit/bdk/pull/2380))
- **Breaking:** Remove `Anchor::confirmation_height_upper_bound`; use `anchor_block().height` because an anchor identifies the confirming block ([#2299](https://github.com/bitcoindevkit/bdk/pull/2299))

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
