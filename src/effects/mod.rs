//! [`Effect`] implementors.
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
