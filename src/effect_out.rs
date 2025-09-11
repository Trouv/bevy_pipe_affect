use crate::effect::Effect;

/// An [`Effect`] and an output.
///
/// Can be returned by `bevy` systems to produce effects with `E` while preserving normal pipe
/// functionality with `O`.
pub struct EffectOut<E, O>
where
    E: Effect,
{
    pub effect: E,
    pub out: O,
}

impl<E> From<E> for EffectOut<E, ()>
where
    E: Effect,
{
    fn from(effect: E) -> Self {
        EffectOut { effect, out: () }
    }
}
