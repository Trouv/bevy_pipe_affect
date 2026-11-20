//! [`Effect`]s for error handling.
//!
//! On top of the types shown here, this implements [`Effect`] for...
//! - `Result<T, E>` where `T: Effect` and `E: Into<BevyError>`
//!
//! [`Effect`]: crate::prelude::Effect

mod affect_or_handle;
pub use affect_or_handle::{AffectOrHandle, affect_or_handle};

mod result;
