//! [`Effect`] implementors.
//!
//! [`Effect`]: crate::Effect

mod resource;
pub use resource::{res_set, res_set_with, ResSet, ResSetWith};

mod message;
pub use message::{message_write, MessageWrite};

mod components;
pub use components::{
    components_set,
    components_set_filtered,
    components_set_filtered_with,
    components_set_filtered_with_query_data,
    components_set_with,
    components_set_with_query_data,
    ComponentsSet,
    ComponentsSetWith,
};

mod entity_components;
pub use entity_components::{
    entity_components_set,
    entity_components_set_with,
    entity_components_set_with_query_data,
    EntityComponentsSet,
    EntityComponentsSetWith,
};

mod command;
pub use command::{
    command_insert_resource,
    command_queue,
    command_remove_resource,
    command_spawn,
    command_spawn_and,
    CommandInsertResource,
    CommandQueue,
    CommandRemoveResource,
    CommandSpawnAnd,
};

mod entity_command;
pub use entity_command::{
    entity_command_despawn,
    entity_command_insert,
    entity_command_queue,
    entity_command_remove,
    EntityCommandDespawn,
    EntityCommandInsert,
    EntityCommandQueue,
    EntityCommandRemove,
};

mod asset_server;
pub use asset_server::{asset_server_load_and, AssetServerLoadAnd};

mod algebra;

mod iter;
pub use iter::{affect_many, AffectMany};

mod error;
pub use error::{affect_or_handle, AffectOrHandle};

#[cfg(test)]
mod one_way_fn;

#[cfg(test)]
mod number_data;
