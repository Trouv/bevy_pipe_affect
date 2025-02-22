//! Test types for various `bevy` data classes, all storing numbers, all proptest-generatable.
use bevy::prelude::*;
use proptest_derive::Arbitrary;

/// Test `Component` storing a number.
///
/// The const generic MARKER can be used to easily define as many component types as you need.
/// i.e., `Numbercomponent<0>` and `NumberComponent<1>` are different components in `bevy`.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Component, Arbitrary)]
pub struct NumberComponent<const MARKER: usize>(pub u128);

/// Test `Event` storing a number.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Event, Arbitrary)]
pub struct NumberEvent(pub u128);

/// Test `Resource` storing a number.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Resource, Arbitrary)]
pub struct NumberResource(pub u128);
