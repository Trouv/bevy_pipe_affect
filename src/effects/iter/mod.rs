//! [`Effect`] implementations for generic iterators.
//!
//! On top of the types shown here, this implements [`Effect`] for...
//! - `Vec<T>` where `T: Effect`
//! - `Option<T>` where `T: Effect`
//!
//! [`Effect`]: crate::prelude::Effect

mod affect_many;
pub use affect_many::{AffectMany, affect_many};

mod vec;

mod option;
