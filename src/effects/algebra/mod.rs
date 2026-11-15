//! [`Effect`] implementations for generic algebraic data types.
//!
//! The implementations here are on these foreign types...
//! - tuples of `Effect`s
//! - `Either<Left, Right>` where the `Left` and `Right` are both effects
//! - the unit type `()` (trivial no-op effect)
//!
//! It's also worth noting that `Option<T>` where `T: Effect` also implements [`Effect`], but that is
//! implemented in the [`iter`] module.
//!
//! Also relevant, [`Effect`] [can be derived] for any custom `struct`/`enum` where the internal
//! types implement [`Effect`].
//!
//! [`iter`]: crate::effects::iter
//! [can be derived]: crate::effect::Effect#derive
//! [`Effect`]: crate::prelude::Effect

mod tuple;

mod either;
