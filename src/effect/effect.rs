use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::all_tuples;

/// Define a state transition in `bevy`'s ECS.
///
/// Can be returned by `bevy` systems and `pipe`d into [`affect`] to perform the transition.
///
/// [`affect`]: crate::system_combinators::affect
pub trait Effect {
    /// The `SystemParam` this effect mutates.
    type MutParam: SystemParam;

    /// Perform the state transition on the parameter.
    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>);
}

macro_rules! impl_effect {
    ($(($E:ident, $e:ident, $p:ident)),*) => {
        impl<$($E),*> Effect for ($($E,)*)
        where $($E: Effect,)* {
            type MutParam = ParamSet<'static, 'static, ($(<$E as Effect>::MutParam,)*)>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let ($($e,)*) = self;
                $($e.affect(&mut param.$p());)*
            }
        }
    };
}

all_tuples!(impl_effect, 1, 8, E, e, p);

/// An [`Effect`] and an output.
///
/// Can be returned by `bevy` systems to produce effects with `E` while preserving normal pipe
/// functionality with `O`.
pub struct EffectOut<E, O>(pub E, pub O)
where
    E: Effect;

impl<E> From<E> for EffectOut<E, ()>
where
    E: Effect,
{
    fn from(effect: E) -> Self {
        EffectOut(effect, ())
    }
}
