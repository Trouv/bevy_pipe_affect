//! [`Effect`] implementors.
//!
//! [`Effect`]: crate::Effect

mod resource;
pub use resource::{ResSet, ResSetWith, res_set, res_set_with};

mod message;
pub use message::{MessageWrite, message_write};

mod components;
pub use components::{
    ComponentsSet,
    ComponentsSetWith,
    components_set,
    components_set_filtered,
    components_set_filtered_with,
    components_set_filtered_with_query_data,
    components_set_with,
    components_set_with_query_data,
};

mod entity_components;
pub use entity_components::{
    EntityComponentsSet,
    EntityComponentsSetWith,
    entity_components_set,
    entity_components_set_with,
    entity_components_set_with_query_data,
};

mod command;
pub use command::{
    CommandInsertResource,
    CommandQueue,
    CommandRemoveResource,
    CommandSpawnAnd,
    command_insert_resource,
    command_queue,
    command_remove_resource,
    command_spawn,
    command_spawn_and,
};

mod entity_command;
pub use entity_command::{
    EntityCommandDespawn,
    EntityCommandInsert,
    EntityCommandQueue,
    EntityCommandRemove,
    entity_command_despawn,
    entity_command_insert,
    entity_command_queue,
    entity_command_remove,
};

mod asset_server;
pub use asset_server::{AssetServerLoadAnd, asset_server_load_and};

mod algebra;

mod iter;
pub use iter::{AffectMany, affect_many};

mod error;
pub use error::{AffectOrHandle, affect_or_handle};

#[cfg(test)]
mod one_way_fn;

#[cfg(test)]
mod number_data;
