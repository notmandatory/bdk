# BDK SQLite Store

This is a simple [SQLite] relational database schema backed implementation of
[`PersistBackend`](`bdk_persist::PersistBackend`).

The main structure is [`Store`](`store::Store`) which works with any `bdk_chain` based changeset to persist data into a SQLite database file. To use `Store` with [`bdk`]'s `Wallet` enable the `bdk` feature.

[`bdk`]: https://docs.rs/bdk/latest
[`bdk_chain`]: https://docs.rs/bdk_chain/latest
[SQLite]: https://www.sqlite.org/index.html
