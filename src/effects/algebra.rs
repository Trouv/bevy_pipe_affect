//! Implements [`Effect`] for tuples of effects up to size 8.

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use either::Either;
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

impl<E0, E1> Effect for Either<E0, E1>
where
    E0: Effect,
    E1: Effect,
{
    type MutParam = ParamSet<'static, 'static, (E0::MutParam, E1::MutParam)>;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        match self {
            Either::Left(e0) => e0.affect(&mut param.p0()),
            Either::Right(e1) => e1.affect(&mut param.p1()),
        }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberResource;
    use crate::effects::ResSet;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn either_can_simulate_collatz(mut current_value in 1..(1u128 << 32), num_rounds in 1..(1u128 << 8)) {
            let mut app = App::new();

            app.insert_resource(NumberResource(current_value)).add_systems(
                Update,
                (|num: Res<NumberResource>| if num.0 % 2 == 0 {
                    Either::Left(ResSet(NumberResource(num.0 / 2)))
                } else {
                    Either::Right(ResSet(NumberResource(3 * num.0 + 1)))
                })
                .pipe(affect),
            );

            for _ in 0..num_rounds {
                app.update();

                current_value = if current_value % 2 == 0 { current_value / 2 } else { 3 * current_value + 1 };

                assert_eq!(app.world().resource::<NumberResource>().0, current_value);
            }
        }
    }
}
