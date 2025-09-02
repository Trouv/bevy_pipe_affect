//! Implements [`Effect`] for tuples of effects up to size 8.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::Effect;

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

impl Effect for () {
    type MutParam = ();

    fn affect(self, _: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {}
}
