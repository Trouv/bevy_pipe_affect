#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

mod effect;
pub use effect::Effect;

mod effect_out;
pub use effect_out::EffectOut;

pub mod effects;

pub mod system_combinators;
