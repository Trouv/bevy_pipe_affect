use bevy::{ecs::system::SystemParam, prelude::*, utils::all_tuples};

pub trait Effect {
    type MutParam: SystemParam;

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
