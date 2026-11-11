# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.24.0](https://github.com/notmandatory/bdk/compare/chain-0.23.2...chain-0.24.0) - 2026-11-11

### Added

- *(chain)* derive Copy and implement AddAssign, Sum for Balance
- *(chain)* [**breaking**] classify timelocked outputs as `Immature`
- *(chain)* [**breaking**] derive balance trust from output ancestry
- *(chain)* add ChainPosition::confirmations_lower_bound
- *(chain)* classify outpoints by chain-level spend eligibility
- *(chain)* add TxNode::is_evicted
- *(core,chain)* introduce `CanonicalizationTask` and `ChainQuery`
- *(chain,electrum)* Make clippy happy
- *(chain)* [**breaking**] Add `ApplyBlockError` for `prev_blockhash` validation
- *(chain)* [**breaking**] Relax the generic parameter for `LocalChain<D>`
- *(chain)* add spent_txouts and created_txouts methods to SPK and keychain indexes
- *(chain)* add input/output indices to sent_and_received_txouts
- *(chain)* add sent_and_received_txouts method to SPK and keychain indexes
- *(chain)* [**breaking**] Remove `CanonicalTx`
- *(chain)* Add min_confirmations parameter to CanonicalView::balance
- *(chain)* [**breaking**] Introduce `CanonicalView` and migrate API
- *(chain)* [**breaking**] Add `last_evicted` field to `TxNode`
- *(chain)* Txs that conflict with relevant txs are also relevant
- *(chain)* Add `TxGraph::get_last_evicted`
- *(chain)* [**breaking**] Clean up ergonomics of `IndexedTxGraph`
- *(chain)* `KeychainTxOutIndex`: Make spk cache optional
- *(chain)* Add `KeychainTxOutIndex::from_changeset` constructor
- *(chain)* `KeychainTxOutIndex`: Debug build checks
- *(chain)* [**breaking**] Persist spks derived from `KeychainTxOutIndex`
- feat!(chain): implement `first_seen` tracking
- *(chain)* Add convenience conversions for `CanonicalTx`
- *(graph)* add convenience function for inserting relevant `evicted_at`s
- *(chain)* [**breaking**] Add ability to modify canonicalization algorithm
- *(chain)* Signed txs should displace unsigned txs in `TxGraph`.
- *(chain)* Add method for constructing `TxGraph` from a `ChangeSet`
- *(chain)* Add `TxGraph` methods that handle expected spk txids
- *(chain)* [**breaking**] Change `TxGraph` to understand evicted-at & last-evicted
- [**breaking**] Improve spk-based syncing flow
- *(core)* [**breaking**] Make `TxUpdate` non-exhaustive
- *(chain)* Derive `Clone` on `IndexedTxGraph`
- *(chain)* [**breaking**] rm `get_chain_position` and associated methods
- *(chain)* [**breaking**] `O(n)` canonicalization algorithm
- *(chain)* [**breaking**] Add `run_until_finished` methods
- *(chain)* [**breaking**] `TxGraph` contain anchors in one field
- *(chain,wallet)* [**breaking**] Transitive `ChainPosition`
- *(chain,wallet)* [**breaking**] rm `ConfirmationTime`
- *(chain,core)* [**breaking**] move `Merge` to `bdk_core`
- *(chain,wallet)* [**breaking**] change methods to take in generic instead of `&Transaction`
- *(chain)* make various insert tx methods more generic
- [**breaking**] move `spk_client`s to `bdk_core`
- [**breaking**] move `tx_graph::Update` to `bdk_core`
- [**breaking**] move `CheckPoint` to `bdk_core`
- [**breaking**] introduce `bdk_core`
- *(chain)* [**breaking**] `TxGraph::apply_update` auto-adds `seen_at` for unanchored
- [**breaking**] introduce `tx_graph::Update`
- *(chain)* get rid of `TxGraph::determine_changeset`
- *(chain,wallet)* [**breaking**] publicize `.init_sqlite_tables` changeset methods
- [**breaking**] rework `FullScanRequest` and `SyncRequest`
- *(chain,wallet)* [**breaking**] Change persist-traits to be "safer"
- [**breaking**] Rework sqlite, changesets, persistence and wallet-construction
- *(chain)* [**breaking**] Implement `ConfirmationBlockTime`
- *(tx_graph)* Add method `txs_with_no_anchor_or_last_seen`
- *(chain)* [**breaking**] add `take` convenience method to `Append` trait
- [**breaking**] rm `persist` submodule
- *(chain)* reintroduce a way to stage changesets before persisting
- *(persist)* add PersistAsync trait and StagedPersistAsync struct
- use `Amount` on `calculate_fee`, `fee_absolute`, `fee_amount` and others
- *(chain)* introduce `TxCache` to `SyncRequest` and `FullScanRequest`
- *(chain)* `TxGraph::insert_tx` reuses `Arc`
- use `Amount` on `TxBuilder::add_recipient`
- use `Amount` on `spk_txout_index` and related
- update `wallet::Balance` to use `bitcoin::Amount`
- *(chain)* `SyncRequest` now uses `ExactSizeIterator`s
- *(bdk-persist)* extract persistence traits to new crate
- *(chain)* add sync/full-scan structures for spk-based syncing
- *(chain)* refactor `merge_chains`
- *(chain)* [**breaking**] make `KeychainTxOutIndex` more range based
- *(chain)* optimize `merge_chains`
- *(chain)* [**breaking**] rm `local_chain::Update`
- *(esplora)* [**breaking**] simplify chain update logic
- *(chain)* introduce `CheckPoint::insert`
- *(tx_graph)* Add method update_last_seen_unconfirmed
- *(chain)* impl `PartialEq` on `CheckPoint`
- *(chain)* add `get` and `range` methods to `CheckPoint`
- *(chain)* [**breaking**] wrap `TxGraph` txs with `Arc`
- *(chain)* add `map_anchors` for `TxGraph` and `ChangeSet`
- *(chain)* expose `SpkIterator::new_with_range`
- *(persist)* Add stage_and_commit to Persist
- *(chain)* add txout methods to `KeychainTxOutIndex`
- *(chain)* add `SpkIterator::descriptor` method
- *(chain)* [**breaking**] make `IndexedTxGraph::apply_block_relevant` more efficient
- *(chain)* add `apply_header..` methods to `LocalChain`
- *(chain)* add `CheckPoint::from_block_ids` convenience method
- *(chain)* add `LocalChain::disconnect_from` method
- *(chain)* make output of `SpkTxOutIndex::unused_spks` cloneable
- *(chain)* debug_assert non-wildcard desc. to only cache index 0
- *(chain)* [**breaking**] `KeychainTxOutIndex` uses a universal lookahead
- [**breaking**] change `load_from_persistence` to return an option
- *(chain, file_store)* add `is_empty` method to `PersistBackend` trait
- [**breaking**] `LocalChain` with hardwired genesis checkpoint
- *(chain)* add `AnchorFromBlockPosition` trait
- *(chain)* Introduce TxAncestors, walk_ancestors
- impl Append for lots of tuples
- add cli example for `esplora`
- *(wallet)* add Wallet::insert_txout function and updated docs for fee functions
- *(tx_graph)* [**breaking**] change TxGraph::calculate_fee to return Result<u64,CalculateFeeError>
- *(chain)* Add `initial_changeset` to graphs
- Implement Anchor for BlockId

### Fixed

- *(chain)* reject an occupied index in insert_spk
- *(chain)* [**breaking**] report `prev_blockhash` mismatches from `insert_block`
- *(chain)* [**breaking**] reject changesets that alter the genesis block
- *(chain)* don't panic on overlapping descriptors in release
- *(chain)* skip coinbase txs with a `last_seen` in canonicalization
- *(chain,core)* prevent serde from implicitly leaking std into no_std environments
- *(chain)* fix integer overflow in lookahead_to_target
- *(chain)* fix off-by-one in replenish_inner_index stop bound
- *(chain)* clamp derivation index before apply_changeset
- *(chain)* let the indexer decide how far to rescan
- *(chain)* skip evicted leftover transactions
- *(chain)* position resolution for assumed txs
- full_scan covers revealed range before applying stop_gap
- *(chain)* prevent integer overflow in `SpkIterator::new_with_range`
- *(chain)* [**breaking**] make genesis immutable in `merge_chains`
- *(chain)* `merge_chains` now takes account of `prev_blockhash`es
- *(docs)* in keychain_txout.rs and spk_client.rs
- *(chain)* forward `confirmation_height_upper_bound` in `Anchor` impl for `&A`
- *(chain)* [**breaking**] remove Debug usage and implement Display for CalculateFeeError
- *(chain)* [**breaking**] remove Debug usage and implement Display for InsertDescriptorError
- *(chain)* correct unconfirmed `ChainPosition` `last_seen` tiebreaker
- *(chain)* correct ChainPosition ordering for wallet transaction display
- *(clippy)* use `is_none_or` instead of `map_or`
- *(chain)* Unconfirmed coinbase txs should never be canonical
- *(rusqlite_impl)* Fix derived spks create table statement
- *(chain)* persist `first_seen`
- *(chain)* [**breaking**] API and logical issues in `KeychainTxOutIndex`
- *(docs)* `merge_chains` documentation
- *(chain)* Correctly handle txs that double spend themselves
- *(chain)* Sqlite - allow persisting anchor without tx
- *(wallet, chain)* [**breaking**] use `bitcoin::constants::COINBASE_MATURITY`
- *(chain)* check time when persisting
- *(chain)* type constraint on list canonical tx
- *(tests)* [**breaking**] remove println! usage from tests
- *(chain)* correct `Iterator::size_hint` impl
- no premature collect
- *(wallet)* Change FromSql type to `Option<_>`
- typos
- *(tx_graph)* [**breaking**] Change tx_last_seen to `Option<u64>`
- *(chain)* Make `Anchor` type in `FullScanResult` generic
- *(chain)* introduce keychain-variant-ranking to `KeychainTxOutIndex`
- *(chain)* simplify `Append::append` impl for `keychain::ChangeSet`
- Run tests only if the miniscript feature is..
- Cargo clippy lints after rust 1.78
- *(wallet)* remove generic from wallet
- *(chain)* `KeychainTxOutIndex::lookahead_to_target`
- *(chain)* [**breaking**] rm duplicate `is_empty` method in tx graph changeset
- *(chain)* tx_graph::ChangeSet::is_empty
- fix(chain)! Re-order fields in anchors so Ord DWIM
- *(chain)* convert to `ConfirmationTime` fix
- *(chain)* avoid using `BTreeMap::append`
- *(chain)* filter coinbase tx not in best chain
- *(chain)* Consider conflicting ancestors in...
- *(chain)* TxDescendants performs a BFS
- *(chain)* [**breaking**] rm weird `From` impl
- non-wildcard descriptors should return an..
- SpkIterator::new_with_range takes wildcards..
- s/index_tx_graph/indexed_tx_graph/g
- Even more refactoring to code and documentation
- `TxGraph::missing_blocks` logic
- improve more docs and more refactoring
- docs and some minor refactoring

### Other

- update release process for release-plz
- *(chain)* use confirmations_lower_bound
- Merge bitcoindevkit/bdk#2012: Add functions to test persistence
- Merge bitcoindevkit/bdk#2367: feat(chain): derive Copy and implement AddAssign, Sum for Balance
- Merge bitcoindevkit/bdk#2411: test(chain): simplify `test_classify_locked_descriptor_timelocks`
- Merge bitcoindevkit/bdk#2380: fix(chain)!: reject changesets that alter the genesis block
- *(chain)* simplify the locked descriptor timelocks test
- Merge bitcoindevkit/bdk#2321: feat(chain)!: classify timelocked outputs as `Immature`
- *(chain)* cover timelocked outputs classified as `Immature`
- *(chain)* test rusqlite persistence with `assert_persist_changesets`
- *(chain)* reuse indexer helper in apply_changeset clamp test
- Merge bitcoindevkit/bdk#2377: fix(chain): don't panic on overlapping descriptors in release
- *(chain)* cover `Display` for `ApplyChangeSetError` and `InsertBlockError`
- Merge bitcoindevkit/bdk#2376: docs(chain): document that `apply_block` trusts the given block
- Merge bitcoindevkit/bdk#2265: fix(chain): sanitize and bound-check derivation indices near BIP32_MAX_INDEX
- Merge bitcoindevkit/bdk#2372: fix(chain): skip coinbase txs with a `last_seen` in canonicalization
- *(chain)* [**breaking**] remove `Anchor::confirmation_height`
- *(chain)* document that `apply_block` trusts the given block
- *(chain)* note false-pending in CanonicalView::balance
- *(chain)* [**breaking**] rename Anchor::confirmation_height_upper_bound
- *(chain)* state that an anchor names the confirming block
- Merge bitcoindevkit/bdk#2246: feat(chain)!: taint-aware `CanonicalView::balance`
- *(chain)* benchmark classify_outpoints across canonical topologies
- *(chain)* cover taint propagation and eligibility classification
- Merge bitcoindevkit/bdk#2262: fix(chain): let the indexer decide how far to rescan
- *(chain,example)* [**breaking**] remove `ChainOracle`, update docs
- *(chain)* [**breaking**] split `canonical_view` into `canonical`,`canonical_view_task`
- *(chain)* [**breaking**] split canonicalization into two tasks with generic `Canonical<A, P>`
- *(chain)* [**breaking**] remove `CanonicalIter` APIs
- *(workspace)* use new `LocalChain::canonical_view` API
- *(deps)* [**breaking**] bump `electrsd` to v0.38.0
- Merge bitcoindevkit/bdk#2182: fix: full_scan covers revealed range before applying stop_gap
- address review feedback on `prev_blockhash` validation
- *(core)* Add module-level docs for `checkpoint_entry`
- *(chain)* Test `apply_update` with a single `CheckPoint<Header>`
- *(chain)* make `TestLocalChain` generic and add `prev_blockhash` test
- Merge bitcoindevkit/bdk#2135: docs: Document TxUpdate temporal context requirements (supports Wizardsardine audit recommendation)
- document TxUpdate temporal context requirements
- use `core::error::Error`
- Merge bitcoindevkit/bdk#2081: Add spent/created_txouts to SPK and Keychain TxOut indexes
- Merge bitcoindevkit/bdk#2083: fix: no `Debug` on `Display` implementations
- *(bdk_chain)* [**breaking**] bump `miniscript` to `13.0.0`
- *(bdk_chain)* update to new `bdk_testenv` APIs
- *(deps)* bump `criterion` to `0.7`
- *(msrv)* bump MSRV to `1.85.0`
- *(chain)* replace `ScriptBuf` with `AsRef<Script>` in SPK index methods
- *(chain)* [**breaking**] Change trust_predicate to accept FullTxOut
- *(chain)* Tighten `CanonicalView` documentation
- *(chain)* [**breaking**] Rename `CanonicalViewTx` to `CanonicalTx`
- *(chain)* Improve CanonicalView documentation
- *(chain)* Add comprehensive tests for min_confirmations parameter
- Merge bitcoindevkit/bdk#2028: Add `last_evicted` field to `TxNode`
- *(chain)* Add test for inserting `Header` into `LocalChain`
- [**breaking**] make `CheckPoint`, `LocalChain`, and `SpkClient` take a generic
- *(release)* bump `bdk-chain` to `0.23.2`
- *(chain)* add doctest for min confirmation balance filtering
- Merge bitcoindevkit/bdk#2008: `IndexedTxGraph`: Transactions that conflict with relevant txs are also relevant.
- *(chain)* fix typo in method doc comment
- *(chain)* `relevant_conflicts`
- *(bdk-chain)* bump to `0.23.1`, update `CHANGELOG`
- Merge bitcoindevkit/bdk#1986: ci: replace grcov by cargo-llvm-cov
- *(keychain_txout)* `apply_changeset` restores spk cache before last revealed
- replace grcov by cargo-llvm-cov
- *(chain)* Add scenario: coinbase tx must not become unconfirmed
- Make clippy happy
- Merge bitcoindevkit/bdk#1968: bench: Add `reindex_tx_graph` benchmark
- *(chain)* Improve API docs
- *(keychain_txout)* test spk cache
- *(chain)* Add `reindex_tx_graph` benchmark
- test for excluded bounds in outputs_in_range
- update changelogs
- bump `bdk_chain` to 0.23.0
- Merge bitcoindevkit/bdk#1963: Persist spks derived from `KeychainTxOutIndex`
- *(chain)* Adds docs for persisting spks in `KeychainTxOutIndex`
- apply fmt
- Merge bitcoindevkit/bdk#1955: chore: Update rust-version to 1.86.0
- update rust-version to 1.86.0
- Update changelogs
- bump bdk_chain to 0.22.0
- *(graph)* Add additional witness scenarios
- *(chain)* Make private fields in `CanonicalIter` concise.
- bump `miniscript` to 12.3.1
- *(chain)* Fix docs for `KeychainTxOutIndex`
- Bump bdk_wallet version to 1.0.0
- *(clippy)* fix new stricter needless_lifetime errors for rust 1.83
- *(chain,wallet)* fix build warnings
- Bump bdk_wallet version to 1.0.0-beta.6
- *(chain)* [**breaking**] use Amount for DescriptorExt::dust_value()
- *(chain)* Update test docs to stop referencing `get_chain_position`
- Fix typos
- *(chain)* Reorganize `TxGraph::insert_anchor` logic for clarity
- *(chain)* Address `CanonicalIter` nitpicks
- *(chain)* Add canonicalization test
- *(chain)* [**breaking**] Rename `LastSeenIn` to `ObservedIn`
- *(chain)* add benchmarks for canonicalization logic
- *(chain)* Add transitive anchor tests
- *(chain)* [**breaking**] change String by &str for versioned scripts in migrate_schema
- *(chain)* add compatibility test for v0 to v1 sqlite schema migration
- *(chain)* [**breaking**] move schemas to their own methods
- *(chain)* use ChangeSet<ConfirmationBlockTime> instead of ChangeSet<BlockId>
- *(chain)* remove serde_json dependency
- *(chain)* [**breaking**] remove AnchorImpl wrapper for Anchor implementors
- *(chain)* [**breaking**] impl sqlite for ConfirmationBlockTime anchored changesets
- *(sqlite)* test persisting anchors and txs independently
- *(chain)* add test `insert_anchor_without_tx`
- *(chain)* document module `rusqlite_impl`
- Merge bitcoindevkit/bdk#1734: chore(chain)!: use `debug_assert!` on `apply_update`
- Merge bitcoindevkit/bdk#1730: Check time when persisting in `rusqlite` impl
- Merge bitcoindevkit/bdk#1727: Use `bitcoin::constants::COINBASE_MATURITY`
- *(chain)* [**breaking**] use `debug_assert!` on `apply_update`
- Merge bitcoindevkit/bdk#1652: refactor(chain)!: remove `inner` method from KeychainTxOutIndex
- Merge bitcoindevkit/bdk#1643: feat(chain,wallet)!: rm `ConfirmationTime`
- *(chain)* [**breaking**] remove `inner` method from KeychainTxOutIndex
- *(test)* move tx test utils to testenv
- Bump bdk_wallet version to 1.0.0-beta.5
- Merge bitcoindevkit/bdk#1594: Replace trait `AnchorFromBlockPosition` with new struct
- *(chain)* [**breaking**] Replace trait `AnchorFromBlockPosition` with new struct
- Bump bdk_wallet version to 1.0.0-beta.4
- Bump bdk_wallet version to 1.0.0-beta.3
- Merge bitcoindevkit/bdk#1425: chore: add `print_stdout`/`print_stderr` lints to workspace level
- Bump bdk version to 1.0.0-beta.2
- *(core)* [**breaking**] s/tx_graph::Update/TxUpdate/
- *(chain)* change `CheckPoint::apply_changeset`
- *(chain)* `TxGraph` and `tx_graph::Update` conversion tests
- *(chain)* [**breaking**] rm `TxGraph::update_last_seen_unconfirmed`
- *(chain)* use `doc_cfg` feature
- add `print_stdout`/`print_stderr` lints to workspace level
- *(chain)* [**breaking**] rm `persit` module
- *(chain)* [**breaking**] Rename `spks_with_labels` to `spks_with_indices`
- *(chain)* improve `SyncRequestBuilder::spks_with_labels` docs
- *(chain)* remove crate-renaming
- Merge bitcoindevkit/bdk#1530: chore: fix clippy lints
- fix clippy lints
- Bump bdk version to 1.0.0-beta.1
- *(chain)* [**breaking**] update KeychainTxOutIndex methods to use owned ScriptBuf
- *(chain)* [**breaking**] update KeychainTxOutIndex methods to use owned K
- *(chain,wallet)* [**breaking**] move rusqlite things into it's own file
- *(chain)* rm unused `sqlite` types
- *(wallet)* ensure checks work when loading wallet
- [**breaking**] move `WalletChangeSet` to `bdk_wallet` and fix import paths
- *(chain)* [**breaking**] Rename `Append` to `Merge`
- *(chain)* [**breaking**] create module `indexer`
- Merge bitcoindevkit/bdk#1416: [chain] Change tx_last_seen to `Option<u64>`
- *(tx_graph)* Change last_seen to `HashMap<Txid, u64>`
- *(tx_graph)* Add test for `list_canonical_txs`
- *(tx_graph)* [**breaking**] Rename `list_chain_txs` to `list_canonical_txs`
- *(indexed_tx_graph)* Add test_get_chain_position
- *(chain)* calculate DescriptorId as sha256 hash of spk at index 0
- Bump bdk version to 1.0.0-alpha.13
- remove PersistBackend from docs, Cargo.toml and wallet README.md
- *(chain)* relax `miniscript` feature flag scope
- *(persist)* update file_store, sqlite, wallet to use bdk_chain::persist
- *(persist)* rename PersistBackend to Persist, move to chain crate
- *(chain)* compute txid once for `KeychainTxOutIndex::index_tx`
- *(chain)* change field names to be more sane
- *(chain)* fix docs
- *(chain)* `KeychainTxOutIndex` use `HashMap` for fields
- *(chain)* s/replenish_lookahead/replenish_inner_index/
- *(chain)* document insert_descriptor invariants better
- *(chain)* improve replenish lookeahd internals
- *(chain)* Fix Indexed and KeychainIndexed documentaion
- *(chain)* Improve documentation of keychain::ChangeSet
- *(chain)* Standardise KeychainTxOutIndex return types
- *(chain)* add type IndexSpk, fix clippy type complexity warning
- *(chain)* misc docs and insert_descriptor fixes
- *(keychain)* Fix KeychainTxOutIndex range queries
- Write failing test for keychain range querying
- *(bdk)* bump `bitcoin` to `0.32.0`, miniscript to `12.0.0`
- Merge bitcoindevkit/bdk#1453: refactor(electrum) put the tx cache in electrum
- *(electrum)* [**breaking**] put the tx cache in electrum
- Bump bdk version to 1.0.0-alpha.12
- bump build_docs rust version to nightly-2024-05-12
- Bump bdk version to 1.0.0-alpha.11
- *(chain)* add section for non-recommended K to descriptor assignments
- Only the highest ord keychain is returned
- *(chain)* move `use` in `indexed_tx_graph.rs` so clippy is happy
- *(chain)* update test so clippy does not complain
- *(chain)* applying changesets one-by-one vs aggregate should be same
- :ChangeSet includes the descriptor
- *(chain)* Define test descriptors, use them...
- *(chain)* move `keychain::ChangeSet` into `txout_index.rs`
- Merge bitcoindevkit/bdk#1411: feat: update `keychain::Balance` to use `bitcoin::Amount`
- Bump bdk version to 1.0.0-alpha.10
- Merge bitcoindevkit/bdk#1413: Introduce universal sync/full-scan structures for spk-based syncing
- Merge bitcoindevkit/bdk#1380: Simplified `EsploraExt` API
- *(chain)* fix comment
- Merge bitcoindevkit/bdk#1387: fix(wallet): remove the generic from wallet
- *(chain)* fix incorrect test case
- *(chain)* [**breaking**] rm `missing_heights` and `missing_heights_from` methods
- Bump bdk version to 1.0.0-alpha.9
- Merge bitcoindevkit/bdk#1177: Upgrade bitcoin/miniscript dependencies
- *(chain)* introduce proptest for `CheckPoint::range`
- Merge bitcoindevkit/bdk#1385: Fix last seen unconfirmed
- Upgrade miniscript/bitcoin dependency
- *(chain)* [**breaking**] rm `From<LocalChain> for BTreeMap<u32, BlockHash>`
- Bump version to 1.0.0-alpha.7
- Merge bitcoindevkit/bdk#1349: Fix `KeychainTxOutIndex::lookahead_to_target`
- *(chain)* change test case comments to docstring
- Merge bitcoindevkit/bdk#1325: Add `map_anchors` for `TxGraph`
- Bump version to 1.0.0-alpha.6
- Merge bitcoindevkit/bdk#1335: fix(chain): tx_graph::ChangeSet::is_empty
- *(chain)* Improve TxGraph::ChangeSet docs
- test(chain) use `Anchor` generic on `init_graph`
- typos
- Bump version to 1.0.0-alpha.5
- Merge bitcoindevkit/bdk#1301: Fix `ConfirmationTime` conversion from `ChainPosition`
- Merge pull request #1296 from yukibtc/change-default-lookahead
- Merge bitcoindevkit/bdk#1294: Expose `SpkIterator::new_with_range`
- set `DEFAULT_LOOKAHEAD` to 25
- Merge bitcoindevkit/bdk#1291: doc(chain,esplora): minor documentation improvements
- Merge bitcoindevkit/bdk#1292: fix(store): Remove lifetime
- Merge bitcoindevkit/bdk#1172: Introduce block-by-block API to `bdk::Wallet` and add RPC wallet example
- Bump version to 1.0.0-alpha.4
- Merge bitcoindevkit/bdk#1269: Revamp `KeychainTxOutIndex` API to be safer
- *(chain,esplora)* minor documentation improvements
- *(chain)* improve `KeychainTxOutIndex` docs
- *(chain)* [**breaking**] revamp `KeychainTxOutIndex` API
- improve LocalChain::apply_header_connected_to doc
- *(chain)* `LocalChain` test for update that is shorter than original
- *(chain)* Refactor reveal_to_target
- *(chain)* Refactor next_store_index
- Bump bdk version to 1.0.0-alpha.3
- *(esplora_ext)* rename scan_txs to sync and scan_txs_with_keychains to full_scan
- make clippy happy and bump clippy msrv
- improve documentation of lookahead
- Improve TxGraph & co docs
- change MSRV to 1.63.0
- *(bdk)* [**breaking**] add context specific error types, remove top level error mod
- typos fixed
- make clippy happy
- Merge bitcoindevkit/bdk#1202: fix(chain): filter coinbase tx not in best chain
- rename `ConfirmationTimeAnchor` to `ConfirmationTimeHeightAnchor`
- *(fix)* conflict resolution for txs with same last_seen
- *(chain)* downgrade hashbrown dependency to 0.9.1 to fix ahash related MSRV issue
- Bump bdk version to 1.0.0-alpha.2
- *(chain)* Rename direct_conflicts_of_tx to...
- split `IndexedTxGraph::insert_tx` into 3 methods
- improvements to `IndexedTxGraph` and `TxGraph` APIs
- add batch-insert methods for `IndexedTxGraph`
- add helper methods on `CheckPoint`
- Merge bitcoindevkit/bdk#1104: refactor: use set_lookahead in set_lookahead_for_all
- *(chain)* Add test for conflicting transactions
- *(chain)* Add test for walk_ancestors
- *(chain)* Introduce TxTemplate
- *(chain)* add block_id! utility macro
- *(chain)* Clarify direct_conflicts_of_tx's docs
- *(chain)* Fix typos
- fix spelling errors
- improve docs, cleanup unnecessary types and improve code
- use set_lookahead in set_lookahead_for_all
- Remove `scan` and `scan_txout` from SpkTxoutIndex and KeychainTxoutIndex
- move WalletChangeset to wallet module
- Move WalletUpdate to wallet module
- Remove ForEachTxout trait
- Merge bitcoindevkit/bdk#1093: fix: spks_of_all_keychains() shouldn't return an infinite iterator for non-wildcard descriptors
- Make test errors more readable
- Merge bitcoindevkit/bdk#1069: Implement Anchor for BlockId
- Add doctest for Anchor implementation on BlockId
- *(chain)* use `apply_update` instead of `determine_changeset` + `apply_changeset`
- refactor!(chain): Unify ChangeSet nomenclature
- Update bdk_chain to bitcoin 0.30.0
- Merge bitcoindevkit/bdk#1034: Implement linked-list `LocalChain` and update chain-src crates/examples
- Refactor `debug_assertions` checks for `LocalChain`
- Implement linked-list `LocalChain` and update chain-src crates/examples
- Merge bitcoindevkit/bdk#1031: remove duplicate `is_empty`
- remove duplicate `is_empty` from DerivationAdditions
- Bump version to 1.0.0-alpha.1
- Fix cargo features
- Better no-std support
- Update documentation
- Rename fields of `tx_graph::Additions`
- [chain_redesign] Remove old structures
- Introduce `keychain::LocalChangeSet`
- [examples_redesign] Introduce `example_cli` package
- Add `last_seen` to the the `ConfirmationTime::Unconfirmed` variant
- [wallet_redesign] Move the majority of `Update` to `bdk_chain`
- [wallet_redesign] Update `Wallet` with redesigned structures
- Improve txout filter/listing method docs for `TxGraph`
- Clarify `TxGraph::try_filter_chain_unspents` logic
- Fix and improve `Persist::commit` method
- Improve `txout` listing and balance APIs for redesigned structures
- Add more `impl`s for `Append` and docs for file store `magic`
- [persist_redesign] Introduce redesigned `persist` types
- [chain_redesign] Rename `LocalChain::inner()` to `blocks()`
- [chain_redesign] Various `LocalChain` improvements
- [chain_redesign] `BlockId` should not implement `Anchor`
- [chain_redesign] Relax generic constraints
- [chain_redesign] Add `LocalChain::insert_block`
- [chain_redesign] Change behavior of `try_get_chain_position`
- Merge pull request #927 from LagginTimes/custom_spk_iterator
- Implement SpkIterator
- Test cleanup and added documentation.
- Merge bitcoindevkit/bdk#926: Introduce redesigned `bdk_chain` structures
- [bdk_chain_redesign] Add tests for `IndexedTxGraph` with `LocalChain`
- [bdk_chain_redesign] Implement `OwnedIndexer` for indexers
- [bdk_chain_redesign] Fix calculation bugs.
- [bdk_chain_redesign] Fix `tx_graph::Additions::append` logic
- [bdk_chain_redesign] Fix `tx_graph::Additions::append` logic
- [bdk_chain_redesign] Change `insert_relevant_txs` method
- [bdk_chain_redesign] Docs for `is_mature` and `is_confirmed_and_spendable`
- [bdk_chain_redesign] Make `insert_relevant_txs` topologically-agnostic
- [bdk_chain_redesign] Add test for `insert_relevant_txs`
- [bdk_chain_redesign] Various tweaks and fixes
- [bdk_chain_redesign] Revert changes to `SparseChain`
- [bdk_chain_redesign] Rm unnecessary code and premature optimisation
- [bdk_chain_redesign] Rm `HashSet` from `TxGraph::relevant_heights`
- [bdk_chain_redesign] Test `LocalChain`
- [bdk_chain_redesign] Add tests for `TxGraph::relevant_heights`
- [bdk_chain_redesign] Remove incomplete logic
- [bdk_chain_redesign] Fix `Anchor` definition + docs
- Include tests for new updates of TxGraph
- [bdk_chain_redesign] MOVE `TxIndex` into `indexed_chain_graph.rs`
- [bdk_chain_redesign] Remove `IndexedTxGraph::last_height`
- [bdk_chain_redesign] Documentation improvements
- [bdk_chain_redesign] Simplify `LocalChain`
- [bdk_chain_redesign] Consistent `ChainOracle`
- [bdk_chain_redesign] Improve `BlockAnchor` docs
- [bdk_chain_redesign] More tweaks and renamings
- [bdk_chain_redesign] Modify signature of `TxIndex`
- [bdk_chain_redesign] Introduce `Append` trait for additions
- [bdk_chain_redesign] Better names, comments and generic bounds
- [bdk_chain_redesign] `chain_oracle::Cache`
- [bdk_chain_redesign] Add serde support for `IndexedAdditions`
- [bdk_chain_redesign] Added methods to `LocalChain`
- [bdk_chain_redesign] `TxGraph` tweaks
- [bdk_chain_redesign] Simplify `TxIndex`
- [bdk_chain_redesign] Revert some API changes
- [bdk_chain_redesign] Make default anchor for `TxGraph` as `()`
- [bdk_chain_redesign] Initial work on `LocalChain`.
- [bdk_chain_redesign] Rm anchor type param for structs that don't use it
- [bdk_chain_redesign] Add docs
- [bdk_chain_redesign] `IndexedTxGraph` keeps track of the last synced height
- [bdk_chain_redesign] Fix `apply_additions` logic for `IndexedTxGraph`.
- [bdk_chain_redesign] `mut_index` should be `index_mut`
- [bdk_chain_redesign] Add balance methods to `IndexedTxGraph`
- [bdk_chain_redesign] Add `apply_additions` to `IndexedTxGraph`
- [bdk_chain_redesign] MOVE: `IndexedTxGraph` into submodule
- [bdk_chain_redesign] List chain data methods can be try/non-try
- [bdk_chain_redesign] Add `..in_chain` methods
- [bdk_chain_redesign] Introduce `ChainOracle` and `TxIndex` traits
- [bdk_chain_redesign] Introduce `BlockAnchor` trait
- Add bdk_chain README
- Bump bdk version to 1.0.0-alpha.0
- Simplify `reveal_to_target` logic
- Add documentation fixes
- Fix cargo clippy
- Remove transaction-based type parameters and traits
- Changed `inflate_update` logic to not depend on `Cow`
- Make bdk_file_store use bincode v1
- Make bdk and bdk_chain work under 1.57.0
- Fix the "repository" field in Cargo.toml
- Fix cargo clippy warnings
- Rust fmt
- Move bdk_chain into the bdk repo :tada:

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