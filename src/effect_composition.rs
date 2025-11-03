//! [`Effect`] composition functions, that can be used with [`and_compose`].
//!
//! [`and_compose`]: crate::system_combinators::and_compose

use bevy::ecs::error::{BevyError, ErrorContext};

use crate::effects::AffectOrHandle;
use crate::Effect;

/// [`Effect`] composition function that returns the first effect.
pub fn mandela<E0, E1>(e0: E0, _e1: E1) -> E0
where
    E0: Effect,
    E1: Effect,
{
    e0
}

/// [`Effect`] composition function that returns the second effect.
pub fn placebo<E0, E1>(_e0: E0, e1: E1) -> E1
where
    E0: Effect,
    E1: Effect,
{
    e1
}

/// [`Effect`] composition function that returns an effect that will apply both in order.
pub fn combine<E0, E1>(e0: E0, e1: E1) -> (E0, E1)
where
    E0: Effect,
    E1: Effect,
{
    (e0, e1)
}

/// [`Effect`] composition function that returns an effect that will apply both in reverse order.
pub fn enibmoc<E0, E1>(e0: E0, e1: E1) -> (E1, E0)
where
    E0: Effect,
    E1: Effect,
{
    (e1, e0)
}

/// Returns an [`Effect`] composition function that applies the given composition to the
/// `Some`-wrapped left effect and the right effect, otherwise `None`.
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
pub fn lhs_affect_then<E0, E1, E2, Er, Handler>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(AffectOrHandle<E0, Er, Handler>, E1) -> AffectOrHandle<E2, Er, Handler>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
    Er: Into<BevyError>,
    Handler: FnOnce(BevyError, ErrorContext),
{
    move |affect_or_handle, e1| affect_or_handle.map(|e0| composition(e0, e1))
}

/// Returns an [`Effect`] composition function that applies the given composition to the left
/// effect and the `AffectOrHandle { result: Ok, .. }`-wrapped right effect, otherwise `AffectOrHandle
/// { result: Err, .. }`.
pub fn rhs_affect_then<E0, E1, E2, Er, Handler>(
    composition: impl Fn(E0, E1) -> E2,
) -> impl Fn(E0, AffectOrHandle<E1, Er, Handler>) -> AffectOrHandle<E2, Er, Handler>
where
    E0: Effect,
    E1: Effect,
    E2: Effect,
    Er: Into<BevyError>,
    Handler: FnOnce(BevyError, ErrorContext),
{
    move |e0, affect_or_handle| affect_or_handle.map(|e1| composition(e0, e1))
}
