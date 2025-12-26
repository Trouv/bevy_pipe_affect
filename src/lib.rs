#![doc = include_str!("../book/src/blurb.md")]
//!
//! ## This API Reference
//! The purpose of this API reference is to describe the API provided by this library.
//! More explanation-oriented documentation, tutorials, and guides are available in the
//! [`bevy_pipe_affect` book](https://trouv.github.io/bevy_pipe_affect/v0.1.0). <!-- x-release-please-version -->
//!
//! The following are good jumping-off points for beginners:
//! - [*Motivations* explanation](https://trouv.github.io/bevy_pipe_affect/v0.1.0/explanation/motivations.html) <!-- x-release-please-version -->
//! - [*effects* module api reference](effects) (a list of effects and constructors provided by the library)
//!
//! Cargo examples are also available in this library's
//! [github repository](https://github.com/Trouv/bevy_pipe_affect/tree/v0.1.0/examples). <!-- x-release-please-version -->
//!
//! ## Feature flags
//! This crate provides the following set of [feature flags]:
//! - `derive`: enables the [`Effect`] derive macro for structs and enums of effects
//! - `asset_server`: enables the `bevy/bevy_asset` feature and [`AssetServer`-related effects]
//!
//! None of these are enabled by default.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/features.html#the-features-section
//! [`AssetServer`-related effects]: effects::AssetServerLoadAnd
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
///
/// *Requires the `derive` feature to be enabled.*
#[cfg(feature = "derive")]
pub use bevy_pipe_affect_derive::Effect;
