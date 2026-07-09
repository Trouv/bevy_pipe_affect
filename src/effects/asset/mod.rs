//! [`Effect`]s that operate on `Assets` stores and the `AssetServer`.

mod asset_server_load_and;
pub use asset_server_load_and::{AssetServerLoadAnd, asset_server_load_and};

mod asset_add_and;
pub use asset_add_and::{AssetAddAnd, asset_add_and};

mod asset_insert;
pub use asset_insert::{AssetInsert, asset_insert};
