//! `use bevy_pipe_affect::prelude::*;` to import common items.

pub use either::Either;

pub use crate::effects::{
    AffectMany,
    AffectOrHandle,
    CommandInsertResource,
    CommandQueue,
    CommandRemoveResource,
    CommandSpawn,
    CommandSpawnAnd,
    CommandTrigger,
    ComponentsSet,
    ComponentsSetWith,
    EntityCommandDespawn,
    EntityCommandInsert,
    EntityCommandQueue,
    EntityCommandRemove,
    EntityComponentsSet,
    EntityComponentsSetWithQueryData,
    MessageWrite,
    ResSet,
    ResSetWith,
    affect_many,
    affect_or_handle,
    command_insert_resource,
    command_queue,
    command_remove_resource,
    command_spawn,
    command_spawn_and,
    command_trigger,
    components_set,
    components_set_filtered,
    components_set_filtered_with,
    components_set_filtered_with_query_data,
    components_set_with,
    components_set_with_query_data,
    entity_command_despawn,
    entity_command_insert,
    entity_command_queue,
    entity_command_remove,
    entity_components_set,
    entity_components_set_with,
    entity_components_set_with_query_data,
    message_write,
    res_set,
    res_set_with,
};
#[cfg(feature = "asset_server")]
pub use crate::effects::{AssetServerLoadAnd, asset_server_load_and};
pub use crate::system_combinators::{
    affect,
    in_and_extend,
    in_and_then,
    in_and_then_compose,
    pure,
};
pub use crate::{Effect, EffectOut, effect_out};
