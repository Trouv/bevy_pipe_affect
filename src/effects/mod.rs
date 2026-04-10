//! [`Effect`] implementors and their constructors.
//!
//! Exploring this module is an easy way to discover what effects are available!
//! The effects are roughly organized by the system parameter they mutate.
//!
//! All of the native effect types have code examples, typically comparing a system that uses them
//! to an equivalent impure system.
//!
//! [`Effect`]: crate::Effect

pub mod resource;

pub mod local;

pub mod message;

pub mod command;

pub mod entity_command;

pub mod query;

pub mod query_entity;

#[cfg(feature = "asset")]
pub mod asset;

pub mod algebra;

pub mod iter;

pub mod error;

#[cfg(test)]
mod one_way_fn;

#[cfg(test)]
pub mod number_data;
