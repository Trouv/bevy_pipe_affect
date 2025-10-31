//! [`Effect`] composition functions, that can be used with [`and_compose`].
//!
//! [`and_compose`]: crate::system_combinators::and_compose

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
