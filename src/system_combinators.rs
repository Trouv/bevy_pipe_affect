//! `bevy` systems and higher-order system constructors related to effects piping and composition.

use bevy::ecs::system::StaticSystemParam;
use bevy::prelude::*;

use crate::effect_composition::{combine, extend};
use crate::{Effect, EffectOut};

/// `bevy` system that accepts [`Effect`]s as pipe input and performs their state transition.
///
/// Technically, this actually accepts `IntoEffectOut: Into<EffectOut<E, O>>` as pipe-input, and
/// performs the conversion implicitly (with `O=()` in the simple converted-effect case). The
/// `output: O` of the [`EffectOut<E, O>`] is returned, so this system can be piped into more
/// systems if [`EffectOut<E, O>`] output exists.
///
/// # Examples
/// ```
/// use bevy::ecs::system::assert_is_system;
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// fn system_with_effects() -> impl Effect {
///     res_set(ClearColor(Color::BLACK))
/// }
///
/// assert_is_system(system_with_effects.pipe(affect))
/// ```
///
/// [`EffectOut<E, O>>`]: EffectOut
pub fn affect<IntoEffectOut, E, O>(
    In(into_effect_out): In<IntoEffectOut>,
    param: StaticSystemParam<E::MutParam>,
) -> O
where
    IntoEffectOut: Into<EffectOut<E, O>>,
    E: Effect,
{
    let EffectOut { effect, out } = into_effect_out.into();
    effect.affect(&mut param.into_inner());

    out
}

/// Higher-order `bevy` system constructor for composing two systems with effects via piping.
///
/// Accepts an effect-returning system `s` and returns a system that composes the effects of the
/// piped-in system and `s` using the `compose` function.
///
/// Some basic effect composition functions are provided by this library in the
/// [`effect_composition`] module.
///
/// See [`in_and_then`] for a short-hand of `in_and_then_compose(s, combine)`.
///
/// If the piped-in system returns [`EffectOut<E, O>`] instead of a simple effect, then the
/// `output: O` is passed into the given system `s`. This allows for a monad-ish API of chaining
/// many systems and piping their outputs while composing their effects.
///
/// # Examples
/// ## No output piping
/// ```
/// use bevy::ecs::system::assert_is_system;
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition;
/// use bevy_pipe_affect::prelude::*;
///
/// fn system_with_effects() -> impl Effect {
///     res_set(ClearColor(Color::BLACK))
/// }
///
/// fn another_system_with_effects() -> impl Effect {
///     res_set(UiScale(2.0))
/// }
///
/// assert_is_system(
///     system_with_effects
///         .pipe(in_and_then_compose(
///             another_system_with_effects,
///             effect_composition::enibmoc, // |e1, e2| (e2, e1)
///         ))
///         .pipe(affect), // applies both effects, in reverse
/// )
/// ```
///
/// ## [`EffectOut<E, O>`] piping
/// ```
/// use bevy::ecs::system::assert_is_system;
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition;
/// use bevy_pipe_affect::prelude::*;
///
/// fn system_with_effects() -> EffectOut<impl Effect, f32> {
///     effect_out(res_set(ClearColor(Color::BLACK)), 2.0)
/// }
///
/// fn another_system_with_effects(In(value): In<f32>) -> impl Effect {
///     res_set(UiScale(value))
/// }
///
/// assert_is_system(
///     system_with_effects
///         .pipe(in_and_then_compose(
///             another_system_with_effects,
///             effect_composition::enibmoc, // |e1, e2| (e2, e1)
///         ))
///         .pipe(affect), // applies both effects, in reverse
/// )
/// ```
///
/// [`EffectOut<E, O>`]: EffectOut
/// [`effect_composition`]: crate::effect_composition
pub fn in_and_then_compose<IntoEffectOut1, E1, O1, System, Marker, IntoEffectOut2, E2, O2, E3>(
    mut s: System,
    compose_fn: impl Fn(E1, E2) -> E3 + Clone,
) -> impl FnMut(In<IntoEffectOut1>, StaticSystemParam<System::Param>) -> EffectOut<E3, O2>
where
    System: SystemParamFunction<Marker, Out = IntoEffectOut2>,
    IntoEffectOut1: Into<EffectOut<E1, O1>>,
    IntoEffectOut2: Into<EffectOut<E2, O2>>,
    E1: Effect,
    E2: Effect,
    E3: Effect,
    for<'a> System::In: SystemInput<Inner<'a> = O1>,
{
    move |In(into_effect_out), params| {
        into_effect_out.into().and_then_compose(
            |input| s.run(input, params.into_inner()),
            compose_fn.clone(),
        )
    }
}

/// Higher-order `bevy` system constructor for combining the effects of two systems via piping.
///
/// Accepts an effect-returning system `s` and returns a system that combines the effects of the
/// piped-in system and `s`.
///
/// To "combine" these effects just means to apply them in order, first the effect of the piped-in
/// system, then the effect of the given system. See [`in_and_then_compose`] for more effect
/// composition flexibility.
///
/// If the piped-in system returns [`EffectOut<E, O>`] instead of a simple effect, then the
/// `output: O` is passed into the given system `s`. This allows for a monad-ish API of chaining
/// many systems and piping their outputs while combining their effects.
///
/// # Examples
/// ## No output
/// ```
/// use bevy::ecs::system::assert_is_system;
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// fn system_with_effects() -> impl Effect {
///     res_set(ClearColor(Color::BLACK))
/// }
///
/// fn another_system_with_effects() -> impl Effect {
///     res_set(UiScale(2.0))
/// }
///
/// assert_is_system(
///     system_with_effects
///         .pipe(in_and_then(another_system_with_effects))
///         .pipe(affect), // applies both effects
/// )
/// ```
///
/// ## [`EffectOut<E, O>`] piping
/// ```
/// use bevy::ecs::system::assert_is_system;
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition;
/// use bevy_pipe_affect::prelude::*;
///
/// fn system_with_effects() -> EffectOut<impl Effect, f32> {
///     effect_out(res_set(ClearColor(Color::BLACK)), 2.0)
/// }
///
/// fn another_system_with_effects(In(value): In<f32>) -> impl Effect {
///     res_set(UiScale(value))
/// }
///
/// assert_is_system(
///     system_with_effects
///         .pipe(in_and_then(another_system_with_effects))
///         .pipe(affect), // applies both effects
/// )
/// ```
///
/// [`EffectOut<E, O>`]: EffectOut
#[expect(clippy::type_complexity)]
pub fn in_and_then<IntoEffectOut1, E1, O1, System, Marker, IntoEffectOut2, E2, O2>(
    s: System,
) -> impl FnMut(In<IntoEffectOut1>, StaticSystemParam<System::Param>) -> EffectOut<(E1, E2), O2>
where
    System: SystemParamFunction<Marker, Out = IntoEffectOut2>,
    IntoEffectOut1: Into<EffectOut<E1, O1>>,
    IntoEffectOut2: Into<EffectOut<E2, O2>>,
    E1: Effect,
    E2: Effect,
    for<'a> System::In: SystemInput<Inner<'a> = O1>,
{
    in_and_then_compose(s, combine)
}

pub fn in_and_extend<IntoEffectOut1, E1, O1, System, Marker, IntoEffectOut2, E2, O2>(
    s: System,
) -> impl FnMut(In<IntoEffectOut1>, StaticSystemParam<System::Param>) -> EffectOut<E1, O2>
where
    System: SystemParamFunction<Marker, Out = IntoEffectOut2>,
    IntoEffectOut1: Into<EffectOut<E1, O1>>,
    IntoEffectOut2: Into<EffectOut<E2, O2>>,
    E1: Extend<E2::Item> + Effect,
    E2: IntoIterator + Effect,
    for<'a> System::In: SystemInput<Inner<'a> = O1>,
{
    in_and_then_compose(s, extend)
}

/// Identity function for read-only-systems.
///
/// This totally-optional function can be used if you want the pureness of your systems to be
/// checked at compile time.
///
/// This only fails for bevy system parameters that aren't read-only. There may be other side
/// effects in your system still that may be unrelated to bevy, like print statements, or global
/// rust state like `OnceCell`s. There are even some `bevy` things with interior mutability that
/// will not get caught, notably `Res<AssetServer>`.
///
/// # Examples
/// An anti-example that failes to compile:
/// ```compile_fail
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// fn my_non_read_only_system(_color: ResMut<ClearColor>) -> impl Effect + use <> {
///     // potential mutation
///
///     res_set(ClearColor::default())
/// }
///
/// App::new()
///     .add_systems(Update, pure(my_non_read_only_system).pipe(affect))
///     .run();
/// ```
pub fn pure<In, Out, Marker, S>(system: S) -> S
where
    In: SystemInput,
    Out: 'static,
    S: IntoSystem<In, Out, Marker>,
    S::System: ReadOnlySystem,
{
    system
}
