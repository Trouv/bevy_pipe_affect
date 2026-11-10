//! [`Effect`]s that modify `Query` data.
//!
//! [`Effect`]: crate::prelude::Effect

mod query_affect;
pub use query_affect::{QueryAffect, query_affect};

mod query_map;
pub use query_map::{BoxedQueryMapFn, QueryMap, query_map};

mod query_map_and;
pub use query_map_and::{BoxedQueryMapAndFn, QueryMapAnd, query_map_and};
