//! [`Effect`]s that modify resources.
//!
//! [`Effect`]: crate::prelude::Effect

mod res_set;
pub use res_set::{ResSet, res_set};

mod res_set_with;
pub use res_set_with::{ResSetWith, res_set_with};
