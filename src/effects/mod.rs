//! [`Effect`] implementors.
//!
//! [`Effect`]: crate::Effect

mod resource;
pub use resource::{ResPut, ResWith};

mod event;
pub use event::EventWrite;

mod components;
pub use components::{ComponentsPut, ComponentsWith};

mod entity_components;
pub use entity_components::{EntityComponentsPut, EntityComponentsWith};

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
pub use iter::IterEffect;

#[cfg(test)]
mod one_way_fn;

#[cfg(test)]
mod number_data;
