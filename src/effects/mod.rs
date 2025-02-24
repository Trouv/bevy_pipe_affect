//! [`Effect`] implementors.
//!
//! [`Effect`]: crate::Effect

mod resource;
pub use resource::{ResPut, ResWith};

mod event;
pub use event::EventSend;

mod components;
pub use components::{ComponentsPut, ComponentsWith};

mod entity_components;
pub use entity_components::{EntityComponentsPut, EntityComponentsWith};

#[cfg(test)]
mod one_way_fn;

#[cfg(test)]
mod number_data;
