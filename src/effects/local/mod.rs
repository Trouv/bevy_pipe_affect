//! [`Effect`]s that modify `Local` parameters.
//!
//! [`Effect`]: crate::prelude::Effect

mod local_set_and;
pub use local_set_and::{BoxedLocalSetAndFn, LocalSetAnd, local_set_and};
