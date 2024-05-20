# BDK SQLite Store

This is a simple [SQLite] relational database schema backed implementation of [`PersistBackend`](bdk_persist::PersistBackend).

The main structure is `Store` which works with any [`bdk_chain`] based changesets to persist data into a SQLite database file. 

To use `Store` with [`Wallet`](bdk_wallet::wallet::Wallet) enable the `wallet` feature.

[`bdk_chain`]:https://docs.rs/bdk_chain/latest/bdk_chain/
[SQLite]: https://www.sqlite.org/index.html
