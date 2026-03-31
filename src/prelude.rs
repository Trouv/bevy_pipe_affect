//! `use bevy_pipe_affect::prelude::*;` to import common items.

pub use either::Either;

pub use crate::effects::command::{
    CommandInsertResource,
    CommandQueue,
    CommandRemoveResource,
    CommandSpawn,
    CommandSpawnAnd,
    CommandTrigger,
    command_insert_resource,
    command_queue,
    command_remove_resource,
    command_spawn,
    command_spawn_and,
    command_trigger,
};
pub use crate::effects::entity_command::{
    EntityCommandDespawn,
    EntityCommandInsert,
    EntityCommandInsertRecursive,
    EntityCommandQueue,
    EntityCommandRemove,
    EntityCommandRemoveRecursive,
    entity_command_despawn,
    entity_command_insert,
    entity_command_insert_recursive,
    entity_command_queue,
    entity_command_remove,
    entity_command_remove_recursive,
};
pub use crate::effects::error::{AffectOrHandle, affect_or_handle};
pub use crate::effects::iter::{AffectMany, affect_many};
pub use crate::effects::local::{LocalSetAnd, local_set_and};
pub use crate::effects::message::{
    MessageWrite,
    MessagesReadAnd,
    message_write,
    messages_read_and,
};
pub use crate::effects::query::{
    QueryAffect,
    QueryMap,
    QueryMapAnd,
    query_affect,
    query_map,
    query_map_and,
};
pub use crate::effects::query_entity::{
    QueryEntityAffect,
    QueryEntityMap,
    QueryEntityMapAnd,
    query_entity_affect,
    query_entity_map,
    query_entity_map_and,
};
pub use crate::effects::resource::{ResSet, ResSetWith, res_set, res_set_with};
#[cfg(feature = "asset")]
pub use crate::effects::{
    asset::AssetAddAnd,
    asset::AssetInsert,
    asset::AssetServerLoadAnd,
    asset::asset_add_and,
    asset::asset_insert,
    asset::asset_server_load_and,
};
pub use crate::query_data_effects::{ComponentSet, ComponentsSet, component_set, components_set};
pub use crate::system_combinators::{
    affect,
    in_and_extend,
    in_and_then,
    in_and_then_compose,
    pure,
};
pub use crate::{Effect, EffectOut, QueryDataEffect, effect_out};
