use crate::effect::Effect;
use crate::effect_composition::{combine, extend};

/// An [`Effect`] and an output.
///
/// Can be returned by `bevy` systems to produce effects with `E` while preserving normal pipe
/// functionality with `O`.
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct EffectOut<E, O>
where
    E: Effect,
{
    /// The effect to produce.
    pub effect: E,
    /// The normal pipe output.
    pub out: O,
}

impl<E, O> EffectOut<E, O>
where
    E: Effect,
{
    /// Maps a `EffectOut<E, O>` to an `EffectOut<E, O2>` by applying a function to the `output`.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "derive")] {
    /// use bevy_pipe_affect::prelude::*;
    ///
    /// #[derive(Debug, PartialEq, Eq, Effect)]
    /// struct MyEffect;
    ///
    /// let initial = effect_out(MyEffect, 5);
    /// let mapped = initial.map(|x| format!("{x}"));
    ///
    /// assert_eq!(mapped, effect_out(MyEffect, "5".to_string()));
    /// # }
    /// ```
    pub fn map<O2>(self, f: impl FnOnce(O) -> O2) -> EffectOut<E, O2> {
        let EffectOut { effect, out } = self;
        EffectOut {
            effect,
            out: f(out),
        }
    }

    /// Apply a function `f` to the `output` and return an `EffectOut` with [`Effect`] combination.
    ///
    /// i.e. `f` takes `output: O` and returns an [`Effect`] (or `EffectOut`). Then, this returns
    /// an `EffectOut` whose `effect` is the combination of `self`'s effect, and the effect
    /// returned by `f`.
    ///
    /// See [`EffectOut::and_then_compose`] for more effect composition flexibility.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "derive")] {
    /// use bevy_pipe_affect::prelude::*;
    ///
    /// #[derive(Debug, PartialEq, Eq, Effect)]
    /// struct MyEffect<const N: usize>;
    ///
    /// let initial = effect_out(MyEffect::<0>, 5);
    /// let composed = initial.and_then(|x| effect_out(MyEffect::<1>, format!("{x}")));
    ///
    /// assert_eq!(
    ///     composed,
    ///     effect_out((MyEffect::<0>, MyEffect::<1>), "5".to_string())
    /// );
    /// # }
    /// ```
    pub fn and_then<IntoEffectOut, E2, O2>(
        self,
        f: impl FnOnce(O) -> IntoEffectOut,
    ) -> EffectOut<(E, E2), O2>
    where
        E2: Effect,
        IntoEffectOut: Into<EffectOut<E2, O2>>,
    {
        self.and_then_compose(f, combine)
    }

    /// Apply a function `f` to the `output` and return an `EffectOut` with [`Effect`] composition.
    ///
    /// i.e. `f` takes `output: O` and returns an [`Effect`] (or `EffectOut`). Then, this returns
    /// an `EffectOut` whose `effect` is the composition of `self`'s effect, and the effect
    /// returned by `f`. The composition of the effects is defined by the function `compose`.
    ///
    /// Some basic effect composition functions are provided by this library in the
    /// [`effect_composition`] module.
    ///
    /// See [`EffectOut::and_then`] for a short-hand of `and_then_compose(f, combine)`.
    ///
    /// [`effect_composition`]: crate::effect_composition
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "derive")] {
    /// use bevy_pipe_affect::effect_composition;
    /// use bevy_pipe_affect::prelude::*;
    ///
    /// #[derive(Debug, PartialEq, Eq, Effect)]
    /// struct MyEffect<const N: usize>;
    ///
    /// let initial = effect_out(MyEffect::<0>, 5);
    /// let composed = initial.and_then_compose(
    ///     |x| effect_out(MyEffect::<1>, format!("{x}")),
    ///     effect_composition::enibmoc,
    /// );
    ///
    /// assert_eq!(
    ///     composed,
    ///     effect_out((MyEffect::<1>, MyEffect::<0>), "5".to_string())
    /// );
    /// # }
    /// ```
    pub fn and_then_compose<IntoEffectOut, E2, O2, E3>(
        self,
        f: impl FnOnce(O) -> IntoEffectOut,
        compose: impl FnOnce(E, E2) -> E3,
    ) -> EffectOut<E3, O2>
    where
        E2: Effect,
        E3: Effect,
        IntoEffectOut: Into<EffectOut<E2, O2>>,
    {
        self.map(f).map(Into::into).flatten_compose(compose)
    }

    /// Apply a function `f` to the `output` and return an `EffectOut` with [`Effect`] extension.
    ///
    /// i.e. `self`'s effect is an extendable iterator, and `f` takes `output: O` and returns an
    /// iterator [`Effect`] (or `EffectOut`). Then, this returns an `EffectOut` whose `effect` is
    /// the concatenation of `self`'s effect, and the effect returned by `f`.
    ///
    /// See [`EffectOut::and_then_compose`] for more effect composition flexibility.
    ///
    /// # Examples
    /// ```
    /// use bevy::prelude::*;
    /// use bevy_pipe_affect::prelude::*;
    ///
    /// #[derive(Debug, PartialEq, Eq, Message)]
    /// struct MyMessage(u32);
    ///
    /// let initial = effect_out(
    ///     vec![message_write(MyMessage(0)), message_write(MyMessage(1))],
    ///     5,
    /// );
    /// let composed =
    ///     initial.and_extend(|x| effect_out(vec![message_write(MyMessage(2))], format!("{x}")));
    ///
    /// assert_eq!(
    ///     composed,
    ///     effect_out(
    ///         vec![
    ///             message_write(MyMessage(0)),
    ///             message_write(MyMessage(1)),
    ///             message_write(MyMessage(2))
    ///         ],
    ///         "5".to_string()
    ///     )
    /// );
    /// ```
    pub fn and_extend<IntoEffectOut, EIter, O2>(
        self,
        f: impl FnOnce(O) -> IntoEffectOut,
    ) -> EffectOut<E, O2>
    where
        EIter: IntoIterator + Effect,
        EIter::Item: Effect,
        E: Extend<EIter::Item>,
        IntoEffectOut: Into<EffectOut<EIter, O2>>,
    {
        self.and_then_compose(f, extend)
    }
}

impl<E1, E2, O> EffectOut<E1, EffectOut<E2, O>>
where
    E1: Effect,
    E2: Effect,
{
    /// Flattens a nested `EffectOut` with [`Effect`] combination.
    ///
    /// i.e. `EffectOut<E1, EffectOut<E2, 0>>` becomes `EffectOut<(E1, E2), O>`.
    ///
    /// See [`EffectOut::flatten_compose`] for more effect composition flexibility.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "derive")] {
    /// use bevy_pipe_affect::prelude::*;
    ///
    /// #[derive(Debug, PartialEq, Eq, Effect)]
    /// struct MyEffect<const N: usize>;
    ///
    /// let nested = effect_out(MyEffect::<0>, effect_out(MyEffect::<1>, 5));
    /// let flattened = nested.flatten();
    ///
    /// assert_eq!(flattened, effect_out((MyEffect::<0>, MyEffect::<1>), 5));
    /// # }
    /// ```
    pub fn flatten(self) -> EffectOut<(E1, E2), O> {
        self.flatten_compose(combine)
    }

    /// Flattens a nested `EffectOut` with [`Effect`] composition.
    ///
    /// i.e. `EffectOut<E1, EffectOut<E2, 0>>` becomes `EffectOut<E3, O>` using the effect
    /// composition function `compose`.
    ///
    /// Some basic effect composition functions are provided by this library in the
    /// [`effect_composition`] module.
    ///
    /// See [`EffectOut::flatten`] for a short-hand of `flatten_compose(combine)`.
    ///
    /// [`effect_composition`]: crate::effect_composition
    ///
    /// # Examples
    /// ```
    /// # #[cfg(feature = "derive")] {
    /// use bevy_pipe_affect::effect_composition;
    /// use bevy_pipe_affect::prelude::*;
    ///
    /// #[derive(Debug, PartialEq, Eq, Effect)]
    /// struct MyEffect<const N: usize>;
    ///
    /// let nested = effect_out(MyEffect::<0>, effect_out(MyEffect::<1>, 5));
    /// let flattened = nested.flatten_compose(effect_composition::enibmoc);
    ///
    /// assert_eq!(flattened, effect_out((MyEffect::<1>, MyEffect::<0>), 5));
    /// # }
    /// ```
    pub fn flatten_compose<E3>(self, compose: impl FnOnce(E1, E2) -> E3) -> EffectOut<E3, O>
    where
        E3: Effect,
    {
        let EffectOut {
            effect: effect_1,
            out: EffectOut {
                effect: effect_2,
                out,
            },
        } = self;
        EffectOut {
            effect: compose(effect_1, effect_2),
            out,
        }
    }
}

impl<E> From<E> for EffectOut<E, ()>
where
    E: Effect,
{
    fn from(effect: E) -> Self {
        EffectOut { effect, out: () }
    }
}

/// Construct a new [`EffectOut`].
pub fn effect_out<E, O>(effect: E, out: O) -> EffectOut<E, O>
where
    E: Effect,
{
    EffectOut { effect, out }
}

impl<E, O, EIter, OIter> FromIterator<EffectOut<E, O>> for EffectOut<EIter, OIter>
where
    E: Effect,
    EIter: Effect + Default + Extend<E>,
    OIter: Default + Extend<O>,
{
    fn from_iter<T: IntoIterator<Item = EffectOut<E, O>>>(iter: T) -> Self {
        let (e_iter, o_iter): (EIter, OIter) = iter
            .into_iter()
            .map(|EffectOut { effect, out }| (effect, out))
            .unzip();

        EffectOut {
            effect: e_iter,
            out: o_iter,
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::message_write;
    use crate::effects::number_data::NumberMessage;

    proptest! {
        #[test]
        fn we_can_collect_effect_outs_with_unit_out(
            messages in proptest::collection::vec(any::<NumberMessage>(), 0..1000usize)
        ) {
            let actual = messages
                .clone()
                .into_iter()
                .map(|message| effect_out(message_write(message), ()))
                .collect::<EffectOut<_, _>>();

            let expected = effect_out(
                messages.into_iter().map(message_write).collect::<Vec<_>>(),
                (),
            );

            assert_eq!(actual, expected);
        }

        #[test]
        fn we_can_collect_effect_outs_with_vec_out(
            message_outs in proptest::collection::vec(any::<(NumberMessage, f32)>(), 0..1000usize),
        ) {
            let actual = message_outs
                .clone()
                .into_iter()
                .map(|(message, out)| effect_out(message_write(message), out))
                .collect::<EffectOut<_, _>>();

            let (expected_messages, expected_outs): (Vec<_>, Vec<_>) = message_outs
                .into_iter()
                .map(|(m, o)| (message_write(m), o))
                .unzip();

            let expected = effect_out(expected_messages, expected_outs);

            assert_eq!(actual, expected);
        }
    }
}
