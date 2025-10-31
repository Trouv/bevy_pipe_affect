//! [`Effect`] composition functions, that can be used with [`and_compose`].
//!
//! [`Effect`]: crate::Effect
//! [`and_compose`]: crate::system_combinators::and_compose

/// [`Effect`] composition function that returns the first effect.
///
/// [`Effect`]: crate::Effect
pub fn mandela<E0, E1>(e0: E0, _e1: E1) -> E0 {
    e0
}

/// [`Effect`] composition function that returns the second effect.
///
/// [`Effect`]: crate::Effect
pub fn placebo<E0, E1>(_e0: E0, e1: E1) -> E1 {
    e1
}

/// [`Effect`] composition function that returns an effect that will apply both in order.
///
/// [`Effect`]: crate::Effect
pub fn combine<E0, E1>(e0: E0, e1: E1) -> (E0, E1) {
    (e0, e1)
}

/// [`Effect`] composition function that returns an effect that will apply both in reverse order.
///
/// [`Effect`]: crate::Effect
pub fn enibmoc<E0, E1>(e0: E0, e1: E1) -> (E1, E0) {
    (e1, e0)
}
