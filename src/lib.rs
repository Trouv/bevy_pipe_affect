#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
#![deny(rustdoc::all)]

mod effect;
pub use effect::Effect;

mod effect_out;
pub use effect_out::EffectOut;

pub mod effects;

pub mod system_combinators;

pub mod effect_composition;

pub mod prelude;
