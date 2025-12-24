//! Test types for various `bevy` data classes, all storing numbers, all proptest-generatable.
use bevy::prelude::*;
use proptest_derive::Arbitrary;

use super::one_way_fn::OneWayFn;

/// Test `Component` storing a number.
///
/// The const generic MARKER can be used to easily define as many component types as you need.
/// i.e., `Numbercomponent<0>` and `NumberComponent<1>` are different components in `bevy`.
#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Component, Arbitrary)]
pub struct NumberComponent<const MARKER: usize>(pub u128);

/// Returns a transform for a pair of [`NumberComponent`]s using the [`OneWayFn`]s.
///
/// Can be used for validating a `-ComponentsSetWith` that uses
/// [`two_number_components_one_way_transform_with_void_query_data`].
pub fn two_number_components_one_way_transform(
    f0: OneWayFn,
    f1: OneWayFn,
) -> impl Fn((NumberComponent<0>, NumberComponent<1>)) -> (NumberComponent<0>, NumberComponent<1>) {
    move |(NumberComponent(n0), NumberComponent(n1))| {
        (NumberComponent(f0.call(n0)), NumberComponent(f1.call(n1)))
    }
}

/// Returns a function taking a [`NumberComponent`] with `MARKER=0` and returning a `MARKER=1`
/// using the [`OneWayFn`].
///
/// Can be used for validating a `-ComponentsSetWith` that uses
/// [`n0_query_data_to_n1_through_one_way_function`].
pub fn n0_to_n1_through_one_way_function(
    f: OneWayFn,
) -> impl Fn(NumberComponent<0>) -> NumberComponent<1> {
    move |NumberComponent(n)| NumberComponent(f.call(n))
}

/// Returns a function transforming a `(NumberComponent<1>,)` from a `&NumberComponent<0>` using
/// the [`OneWayFn`].
///
/// Can be used as the function stored in a `-ComponentsSetWith` effect.
pub fn n0_query_data_to_n1_through_one_way_function(
    f: OneWayFn,
) -> impl Fn((NumberComponent<1>,), &NumberComponent<0>) -> (NumberComponent<1>,) {
    move |_, n0| (n0_to_n1_through_one_way_function(f)(*n0),)
}

/// Test `Message` storing a number.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Message, Arbitrary)]
pub struct NumberMessage(pub u128);

/// Test `Event` storing a number.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Event, Arbitrary)]
pub struct NumberEvent(pub u128);

/// Test `Resource` storing a number.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Resource, Arbitrary)]
pub struct NumberResource(pub u128);
