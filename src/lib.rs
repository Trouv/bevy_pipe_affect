#![doc = include_str!("../book/src/blurb.md")]
#![warn(missing_docs)]
#![deny(rustdoc::all)]

mod effect;
pub use effect::Effect;

mod effect_out;
pub use effect_out::{EffectOut, effect_out};

pub mod effects;

pub mod system_combinators;

pub mod effect_composition;

pub mod prelude;

/// Derive macro for the [`Effect`] trait. See that trait for more details.
#[cfg(feature = "derive")]
pub use bevy_pipe_affect_derive::Effect;
