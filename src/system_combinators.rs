//! `bevy` systems and higher-order system constructors related to effects piping and composition.

use bevy::ecs::system::StaticSystemParam;
use bevy::prelude::*;

use crate::{Effect, EffectOut};

/// `bevy` system that accepts [`Effect`]s as pipe input and performs their state transition.
///
/// Technically, this actually accepts [`I: Into<EffectOut<E, O>>`] as pipe-input, and performs the
/// conversion implicitly. Returns the output `O` of the [`EffectOut`], so piping can continue.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy::ecs::system::assert_is_system;
/// # use bevy_pipe_affect::system_combinators::affect;
/// # use bevy_pipe_affect::{Effect, effects::ResPut};
/// fn system_with_effects() -> impl Effect {
///     ResPut(ClearColor(Color::BLACK))
/// }
///
/// assert_is_system(system_with_effects.pipe(affect))
/// ```
///
/// [`I: Into<EffectOut<E, O>>`]: EffectOut
pub fn affect<I, E, O>(In(into_effect_out): In<I>, param: StaticSystemParam<E::MutParam>) -> O
where
    I: Into<EffectOut<E, O>>,
    E: Effect,
{
    let EffectOut(effect, out) = into_effect_out.into();
    effect.affect(&mut param.into_inner());

    out
}

/// Higher-order `bevy` system constructor for composing two systems with effects via piping.
///
/// Accepts a system with effects and an effect composition function and returns a system that
/// composes their effects.
///
/// Normal pipe input can be passed into `S` by returning [`EffectOut<E, O>`] in the source system
/// instead of a plain [`Effect`]. The output `O` will be passed into `S`.
///
/// Similarly, normal pipe output can be passed out of the resulting system if `S` returns
/// [`EffectOut<E, O>`].
///
/// In this example, the effect composition just ignores the effect of the original system:
/// ```
/// # use bevy::prelude::*;
/// # use bevy::ecs::system::assert_is_system;
/// # use bevy_pipe_affect::system_combinators::{affect, and_compose};
/// # use bevy_pipe_affect::{Effect, effects::ResPut};
/// fn system_with_effects() -> impl Effect {
///     ResPut(ClearColor(Color::BLACK))
/// }
///
/// fn another_system_with_effects() -> impl Effect {
///     ResPut(UiScale(2.0))
/// }
///
/// assert_is_system(
///     system_with_effects
///         .pipe(and_compose(another_system_with_effects, |_, e| e))
///         .pipe(affect),
/// )
/// ```
///
/// [`EffectOut<E, O>`]: EffectOut
pub fn and_compose<I1, E1, O1, System, Marker, I2, E2, O2, E3>(
    mut s: System,
    compose_fn: impl Fn(E1, E2) -> E3,
) -> impl FnMut(In<I1>, StaticSystemParam<System::Param>) -> EffectOut<E3, O2>
where
    System: SystemParamFunction<Marker, Out = I2>,
    I1: Into<EffectOut<E1, O1>>,
    I2: Into<EffectOut<E2, O2>>,
    E1: Effect,
    E2: Effect,
    E3: Effect,
    for<'a> System::In: SystemInput<Inner<'a> = O1>,
{
    move |In(into_effect_out), params| {
        let EffectOut(e1, input) = into_effect_out.into();
        let EffectOut(e2, out) = s.run(input, params.into_inner()).into();

        EffectOut(compose_fn(e1, e2), out)
    }
}
