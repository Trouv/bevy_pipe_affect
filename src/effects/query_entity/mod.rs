//! [`Effect`]s that modify `Query` data for particular entities.
//!
//! [`Effect`]: crate::prelude::Effect

mod query_entity_affect;
pub use query_entity_affect::{QueryEntityAffect, query_entity_affect};

mod query_entity_map;
pub use query_entity_map::{QueryEntityMap, query_entity_map};

mod query_entity_map_and;
pub use query_entity_map_and::{QueryEntityMapAnd, query_entity_map_and};
