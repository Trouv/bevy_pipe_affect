//! `use bevy_pipe_affect::prelude::*;` to import common items.

pub use either::Either;

pub use crate::effects::{
    AffectMany,
    AffectOrHandle,
    CommandInsertResource,
    CommandQueue,
    CommandRemoveResource,
    CommandSpawnAnd,
    ComponentsSet,
    ComponentsSetWith,
    EntityCommandDespawn,
    EntityCommandInsert,
    EntityCommandQueue,
    EntityCommandRemove,
    EntityComponentsSet,
    EntityComponentsSetWith,
    MessageWrite,
    ResSet,
    ResSetWith,
};
pub use crate::system_combinators::{affect, and_compose};
pub use crate::{Effect, EffectOut};
