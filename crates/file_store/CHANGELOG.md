# Changelog

All notable changes to this project can be found here and in each release's git tag and can be viewed with `git tag -ln100 "v*"`. See also [DEVELOPMENT_CYCLE.md](../../DEVELOPMENT_CYCLE.md) for more details.

This file is updated automatically by [release-plz](https://release-plz.dev) from [Conventional Commits](https://www.conventionalcommits.org/). Do not edit it manually.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.23.0](https://github.com/bitcoindevkit/bdk/compare/file_store-0.22.0...file_store-0.23.0)

### Changed

- Raise MSRV to `1.85.0` ([#2055](https://github.com/bitcoindevkit/bdk/pull/2055))
- **Breaking:** Raise the minimum `bdk_core` version to `0.7.0` ([3ec1b327](https://github.com/bitcoindevkit/bdk/commit/3ec1b327394f179cb683e66d4be06684400bafff))

### Fixed

- **Breaking:** Update `StoreError` display messages and format `StoreError::InvalidMagicBytes` expected and actual bytes as `0x`-prefixed hexadecimal ([#2083](https://github.com/bitcoindevkit/bdk/pull/2083))

## [file_store-0.22.0]

### Changed
- `Changeset` field of `StoreErrorWithDump` is now of type `Option<Box<C>>` #1998

## [file_store-0.21.1]

### Added

- Introduce usage of `cfg_attr(coverage_nightly)` in order to not consider tests under coverage. #1986

### Changed

- Fix `clippy` formatting warnings for `StoreError` and test assertions #1990
- Bump `bdk_core` to `0.6.1`

## [file_store-0.21.0]

### Changed

- bump bdk_core to 0.6.0

## [file_store-0.20.0]

### Changed

- deps: bump `bdk_core` to 0.5.0

## [file_store-0.19.0]

### Added:

- `StoreError` enum, which includes `Io`, `Bincode` and `InvalidMagicBytes` #1684.
- docs: add "not intended for production" note in `README`.

### Changed:

- `Store::create_new` to `Store::create`, with new return type: `Result<Self, StoreError>`
- `Store::open` to `Store::load`, with new return type: `Result<(Self, Option<C>), StoreErrorWithDump<C>>`
- `Store::open_or_create` to `Store::load_or_create`, with new return type: `Result<(Option<C>, Self), StoreErrorWithDump<C>>`
- `Store::aggregate_changesets` to `Store::dump`, with new return type: `Result<Option<C>, StoreErrorWithDump<C>>`
- `FileError` to `StoreError`
- `AggregateChangesetsError` to `StoreErrorWithDump`, which now can include all the variants of `StoreError` in the error field.

#### Removed:

- `IterError` deleted.

## [file_store-0.18.1]

### Changed

- Minor updates to fix new rustc 1.83.0 clippy warnings #1776

[file_store-0.18.1]: https://github.com/bitcoindevkit/bdk/releases/tag/file_store-0.18.1
[file_store-0.19.0]: https://github.com/bitcoindevkit/bdk/releases/tag/file_store-0.19.0
[file_store-0.20.0]: https://github.com/bitcoindevkit/bdk/releases/tag/file_store-0.20.0
[file_store-0.21.0]: https://github.com/bitcoindevkit/bdk/releases/tag/file_store-0.21.0
[file_store-0.21.1]: https://github.com/bitcoindevkit/bdk/releases/tag/file_store-0.21.1
[file_store-0.22.0]: https://github.com/bitcoindevkit/bdk/releases/tag/file_store-0.22.0
