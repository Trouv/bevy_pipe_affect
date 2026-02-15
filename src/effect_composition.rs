//! [`Effect`] composition functions, that can be used with [`in_and_then_compose`] and
//! [`EffectOut::and_then_compose`].
//!
//! [`in_and_then_compose`]: crate::system_combinators::in_and_then_compose
//! [`EffectOut::and_then_compose`]: crate::EffectOut::and_then_compose

use bevy::ecs::error::BevyError;

use crate::Effect;
use crate::effects::AffectOrHandle;

/// [`Effect`] composition function that returns the first effect.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::mandela;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let effect = mandela(MyEffect::<0>, MyEffect::<1>);
///
/// assert_eq!(effect, MyEffect::<0>);
/// # }
/// ```
pub fn mandela<E0, E1>(e0: E0, _e1: E1) -> E0
where
    E0: Effect,
    E1: Effect,
{
    e0
}

/// [`Effect`] composition function that returns the second effect.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::placebo;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let effect = placebo(MyEffect::<0>, MyEffect::<1>);
///
/// assert_eq!(effect, MyEffect::<1>);
/// # }
/// ```
pub fn placebo<E0, E1>(_e0: E0, e1: E1) -> E1
where
    E0: Effect,
    E1: Effect,
{
    e1
}

/// [`Effect`] composition function that returns an effect that will apply both in order.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::combine;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let effect = combine(MyEffect::<0>, MyEffect::<1>);
///
/// assert_eq!(effect, (MyEffect::<0>, MyEffect::<1>));
/// # }
/// ```
pub fn combine<E0, E1>(e0: E0, e1: E1) -> (E0, E1)
where
    E0: Effect,
    E1: Effect,
{
    (e0, e1)
}

/// [`Effect`] composition function that returns an effect that will apply both in reverse order.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::enibmoc;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let effect = enibmoc(MyEffect::<0>, MyEffect::<1>);
///
/// assert_eq!(effect, (MyEffect::<1>, MyEffect::<0>));
/// # }
/// ```
pub fn enibmoc<E0, E1>(e0: E0, e1: E1) -> (E1, E0)
where
    E0: Effect,
    E1: Effect,
{
    (e1, e0)
}

/// [`Effect`] composition function for extendable iterator effects that concatenates them.
///
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::extend;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Message)]
/// struct MyMessage(u32);
///
/// let effect = extend(
///     vec![message_write(MyMessage(0)), message_write(MyMessage(1))],
///     vec![message_write(MyMessage(2))],
/// );
///
/// assert_eq!(
///     effect,
///     vec![
///         message_write(MyMessage(0)),
///         message_write(MyMessage(1)),
///         message_write(MyMessage(2))
///     ]
/// );
/// ```
pub fn extend<E0, E1>(mut e0: E0, e1: E1) -> E0
where
    E0: Extend<E1::Item> + Effect,
    E1: IntoIterator + Effect,
{
    e0.extend(e1);
    e0
}

/// Returns an [`Effect`] composition function that applies the given composition to the
/// `Some`-wrapped left effect and the right effect, otherwise `None`.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::lhs_some_then;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let composition = lhs_some_then(|_, _| MyEffect::<2>);
///
/// let some_effect = composition(Some(MyEffect::<0>), MyEffect::<1>);
/// assert_eq!(some_effect, Some(MyEffect::<2>));
///
/// let none_effect = composition(None::<MyEffect<0>>, MyEffect::<1>);
/// assert!(none_effect.is_none());
/// # }
/// ```
pub fn lhs_some_then<E0, E1, E2>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(Option<E0>, E1) -> Option<E2>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
{
    move |option, e1| option.map(|e0| composition(e0, e1))
}

/// Returns an [`Effect`] composition function that applies the given composition to the left
/// effect and the `Some`-wrapped right effect, otherwise `None`.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::rhs_some_then;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let composition = rhs_some_then(|_, _| MyEffect::<2>);
///
/// let some_effect = composition(MyEffect::<0>, Some(MyEffect::<1>));
/// assert_eq!(some_effect, Some(MyEffect::<2>));
///
/// let none_effect = composition(MyEffect::<0>, None::<MyEffect<1>>);
/// assert!(none_effect.is_none());
/// # }
/// ```
pub fn rhs_some_then<E0, E1, E2>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(E0, Option<E1>) -> Option<E2>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
{
    move |e0, option| option.map(|e1| composition(e0, e1))
}

/// Returns an [`Effect`] composition function that applies the given composition to the
/// `Ok`-wrapped left effect and the right effect, otherwise `Err`.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::lhs_ok_then;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let composition = lhs_ok_then(|_, _| MyEffect::<2>);
///
/// let ok_effect = composition(Ok::<_, &str>(MyEffect::<0>), MyEffect::<1>);
/// assert_eq!(ok_effect, Ok(MyEffect::<2>));
///
/// let err_effect = composition(Err::<MyEffect<0>, _>("snafu"), MyEffect::<1>);
/// assert!(err_effect.is_err());
/// # }
/// ```
pub fn lhs_ok_then<E0, E1, E2, Er>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(Result<E0, Er>, E1) -> Result<E2, Er>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
    Er: Into<BevyError>,
{
    move |result, e1| result.map(|e0| composition(e0, e1))
}

/// Returns an [`Effect`] composition function that applies the given composition to the left
/// effect and the `Ok`-wrapped right effect, otherwise `Err`.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::rhs_ok_then;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let composition = rhs_ok_then(|_, _| MyEffect::<2>);
///
/// let ok_effect = composition(MyEffect::<0>, Ok::<_, &str>(MyEffect::<1>));
/// assert_eq!(ok_effect, Ok(MyEffect::<2>));
///
/// let err_effect = composition(MyEffect::<0>, Err::<MyEffect<1>, _>("snafu"));
/// assert!(err_effect.is_err());
/// # }
/// ```
pub fn rhs_ok_then<E0, E1, E2, Er>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(E0, Result<E1, Er>) -> Result<E2, Er>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
    Er: Into<BevyError>,
{
    move |e0, result| result.map(|e1| composition(e0, e1))
}

/// Returns an [`Effect`] composition function that applies the given composition to the
/// `AffectOrHandle { result: Ok, .. }`-wrapped left effect and the right effect, otherwise
/// `AffectOrHandle { result: Err, .. }`.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::lhs_affect_then;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let composition = lhs_affect_then(|_, _| MyEffect::<2>);
///
/// let affect_effect = composition(
///     affect_or_handle(Ok::<_, &str>(MyEffect::<0>), bevy::ecs::error::warn),
///     MyEffect::<1>,
/// );
/// assert_eq!(affect_effect.result, Ok(MyEffect::<2>));
///
/// let handle_effect = composition(
///     affect_or_handle(Err::<MyEffect<0>, _>("snafu"), bevy::ecs::error::warn),
///     MyEffect::<1>,
/// );
/// assert!(handle_effect.result.is_err());
/// # }
/// ```
pub fn lhs_affect_then<E0, E1, E2, Er>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(AffectOrHandle<E0, Er>, E1) -> AffectOrHandle<E2, Er>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
    Er: Into<BevyError>,
{
    move |affect_or_handle, e1| affect_or_handle.map(|e0| composition(e0, e1))
}

/// Returns an [`Effect`] composition function that applies the given composition to the left
/// effect and the `AffectOrHandle { result: Ok, .. }`-wrapped right effect, otherwise `AffectOrHandle
/// { result: Err, .. }`.
///
/// ```
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::effect_composition::rhs_affect_then;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, PartialEq, Eq, Effect)]
/// struct MyEffect<const N: u8>;
///
/// let composition = rhs_affect_then(|_, _| MyEffect::<2>);
///
/// let affect_effect = composition(
///     MyEffect::<0>,
///     affect_or_handle(Ok::<_, &str>(MyEffect::<1>), bevy::ecs::error::warn),
/// );
/// assert_eq!(affect_effect.result, Ok(MyEffect::<2>));
///
/// let handle_effect = composition(
///     MyEffect::<0>,
///     affect_or_handle(Err::<MyEffect<1>, _>("snafu"), bevy::ecs::error::warn),
/// );
/// assert!(handle_effect.result.is_err());
/// # }
/// ```
pub fn rhs_affect_then<E0, E1, E2, Er>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(E0, AffectOrHandle<E1, Er>) -> AffectOrHandle<E2, Er>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
    Er: Into<BevyError>,
{
    move |e0, affect_or_handle| affect_or_handle.map(|e1| composition(e0, e1))
}
