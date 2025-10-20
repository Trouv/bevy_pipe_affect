//! [`Effect`] implementors.
//!
//! [`Effect`]: crate::Effect

mod resource;
pub use resource::{ResSet, ResSetWith};

mod event;
pub use event::MessageWrite;

mod components;
pub use components::{ComponentsSet, ComponentsSetWith};

mod entity_components;
pub use entity_components::{EntityComponentsSet, EntityComponentsSetWith};

mod command;
pub use command::{CommandInsertResource, CommandQueue, CommandRemoveResource, CommandSpawnAnd};

mod entity_command;
pub use entity_command::{
    EntityCommandDespawn,
    EntityCommandInsert,
    EntityCommandQueue,
    EntityCommandRemove,
};

mod algebra;

mod iter;
pub use iter::AffectMany;

mod error;
pub use error::AffectOrHandle;

#[cfg(test)]
mod one_way_fn;

#[cfg(test)]
mod number_data;
