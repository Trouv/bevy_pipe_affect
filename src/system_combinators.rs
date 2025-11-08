//! `bevy` systems and higher-order system constructors related to effects piping and composition.

use bevy::ecs::system::StaticSystemParam;
use bevy::prelude::*;

use crate::effect_composition::combine;
use crate::{Effect, EffectOut};

/// `bevy` system that accepts [`Effect`]s as pipe input and performs their state transition.
///
/// Technically, this actually accepts [`I: Into<EffectOut<E, O>>`] as pipe-input, and performs the
/// conversion implicitly. Returns the output `O` of the [`EffectOut`], so piping can continue.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy::ecs::system::assert_is_system;
/// use bevy_pipe_affect::prelude::*;
///
/// fn system_with_effects() -> impl Effect {
///     res_set(ClearColor(Color::BLACK))
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
    let EffectOut { effect, out } = into_effect_out.into();
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
/// Some basic effect composition functions are provided by this library in the
/// [`effect_composition`] module.
///
/// In this example, the effect composition just ignores the effect of the original system:
/// ```
/// # use bevy::prelude::*;
/// # use bevy::ecs::system::assert_is_system;
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
///         .pipe(and_compose(another_system_with_effects, |_, e| e))
///         .pipe(affect),
/// )
/// ```
///
/// [`EffectOut<E, O>`]: EffectOut
/// [`effect_composition`]: crate::effect_composition
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
        let EffectOut {
            effect: e1,
            out: input,
        } = into_effect_out.into();
        let EffectOut { effect: e2, out } = s.run(input, params.into_inner()).into();

        EffectOut {
            effect: compose_fn(e1, e2),
            out,
        }
    }
}

/// Higher-order `bevy` system constructor for combining the effects of two systems via piping.
///
/// Accepts a system with effects and returns a system that combines the effects of the given
/// system and the piped-in system.
///
/// This function is just [`and_compose`] used with [`combine`] for convenience. See [`and_compose`] for more
/// effect composition flexibility.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy::ecs::system::assert_is_system;
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
///         .pipe(and_combine(another_system_with_effects))
///         .pipe(affect), // applies both effects
/// )
/// ```
#[expect(clippy::type_complexity)]
pub fn and_combine<I1, E1, O1, System, Marker, I2, E2, O2>(
    s: System,
) -> impl FnMut(In<I1>, StaticSystemParam<System::Param>) -> EffectOut<(E1, E2), O2>
where
    System: SystemParamFunction<Marker, Out = I2>,
    I1: Into<EffectOut<E1, O1>>,
    I2: Into<EffectOut<E2, O2>>,
    E1: Effect,
    E2: Effect,
    for<'a> System::In: SystemInput<Inner<'a> = O1>,
{
    and_compose(s, combine)
}

/// Identity function for read-only-systems.
///
/// This totally-optional function can be used if you want the pureness of your systems to be
/// checked at compile time.
///
/// This only fails for bevy system parameters that aren't read-only. There may be other side
/// effects in your system still that may be unrelated to bevy, like print statements, or global
/// rust state like `OnceCell`s.
///
/// An anti-example that failes to compile:
/// ```compile_fail
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// fn my_non_read_only_system(_color: ResMut<ClearColor>) -> impl Effect {
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
