//! [`Effect`] implementors.
//!
//! [`Effect`]: crate::Effect

mod resource;
pub use resource::{ResPut, ResWith};

mod event;
pub use event::EventSend;

mod components;
pub use components::{ComponentsPut, ComponentsWith};

#[cfg(test)]
mod one_way_fn;
