//! [`Effect`] implementors.
//!
//! [`Effect`]: crate::Effect

mod resource;
pub use resource::{ResPut, ResWith};

mod event;
pub use event::EventSend;
