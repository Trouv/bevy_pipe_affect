//! `use bevy_pipe_affect::prelude::*;` to import common items.

pub use crate::effects::{
    CommandQueue,
    ComponentsPut,
    ComponentsWith,
    EntityComponentsPut,
    EntityComponentsWith,
    EventWrite,
    ResPut,
    ResWith,
};
pub use crate::system_combinators::{affect, and_compose};
pub use crate::{Effect, EffectOut};
