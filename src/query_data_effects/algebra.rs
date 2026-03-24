use bevy::ecs::query::QueryData;
use either::Either;
use variadics_please::all_tuples;

use crate::query_data_effect::QueryDataEffect;

macro_rules! impl_query_data_effect {
    ($(($QDE:ident, $qde:ident, $qd:ident)),*) => {
        impl<$($QDE),*> QueryDataEffect for ($($QDE,)*)
        where $($QDE: QueryDataEffect,)*
        {
            type MutQueryData = ($(<$QDE as QueryDataEffect>::MutQueryData,)*);
            type Filter = ($(<$QDE as QueryDataEffect>::Filter,)*);

            fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
                let ($($qde,)*) = self;
                let ($($qd,)*) = query_data;

                $($qde.affect($qd));*
            }
        }
    }
}

all_tuples!(impl_query_data_effect, 0, 15, QDE, qde, qd);

impl<QDE0, QDE1> QueryDataEffect for Either<QDE0, QDE1>
where
    QDE0: QueryDataEffect,
    QDE1: QueryDataEffect,
{
    type MutQueryData = (
        <QDE0 as QueryDataEffect>::MutQueryData,
        <QDE1 as QueryDataEffect>::MutQueryData,
    );
    type Filter = (
        <QDE0 as QueryDataEffect>::Filter,
        <QDE1 as QueryDataEffect>::Filter,
    );

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        match self {
            Either::Left(query_data_effect) => query_data_effect.affect(&mut query_data.0),
            Either::Right(query_data_effect) => query_data_effect.affect(&mut query_data.1),
        }
    }
}

impl<QDE> QueryDataEffect for Option<QDE>
where
    QDE: QueryDataEffect,
{
    type MutQueryData = <Either<QDE, ()> as QueryDataEffect>::MutQueryData;
    type Filter = <Either<QDE, ()> as QueryDataEffect>::Filter;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        let as_either = match self {
            Some(query_data_effect) => Either::Left(query_data_effect),
            None => Either::Right(()),
        };

        as_either.affect(query_data);
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberComponent;
    use crate::query_data_effects::{ComponentSet, component_set};

    proptest! {
        #[test]
        fn pair_of_effects_affects_both(initial: (NumberComponent<0>, NumberComponent<1>), components: (NumberComponent<0>, NumberComponent<1>)) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(initial).id();

            let component_sets = (component_set(components.0), component_set(components.1));

            app.world_mut()
                .query::<<(ComponentSet<NumberComponent<0>>, ComponentSet<NumberComponent<1>>) as QueryDataEffect>::MutQueryData>()
                .iter_mut(app.world_mut())
                .for_each(|mut query_data| component_sets.affect(&mut query_data));

            assert_eq!(
                app.world().get::<NumberComponent<0>>(entity).unwrap(),
                &components.0
            );

            assert_eq!(
                app.world().get::<NumberComponent<1>>(entity).unwrap(),
                &components.1
            );
        }

        #[test]
        fn either_of_effects_affects_one(initial: (NumberComponent<0>, NumberComponent<1>), components: (NumberComponent<0>, NumberComponent<1>)) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(initial).id();

            let either_component_set: Either<ComponentSet<NumberComponent<0>>, ComponentSet<NumberComponent<1>>> = Either::Left(component_set(components.0));

            app.world_mut()
                .query::<<Either<ComponentSet<NumberComponent<0>>, ComponentSet<NumberComponent<1>>> as QueryDataEffect>::MutQueryData>()
                .iter_mut(app.world_mut())
                .for_each(|mut query_data| either_component_set.affect(&mut query_data));

            assert_eq!(
                app.world().get::<NumberComponent<0>>(entity).unwrap(),
                &components.0
            );

            assert_eq!(
                app.world().get::<NumberComponent<1>>(entity).unwrap(),
                &initial.1
            );

            let either_component_set: Either<ComponentSet<NumberComponent<0>>, ComponentSet<NumberComponent<1>>> = Either::Right(component_set(components.1));

            app.world_mut()
                .query::<<Either<ComponentSet<NumberComponent<0>>, ComponentSet<NumberComponent<1>>> as QueryDataEffect>::MutQueryData>()
                .iter_mut(app.world_mut())
                .for_each(|mut query_data| either_component_set.affect(&mut query_data));

            assert_eq!(
                app.world().get::<NumberComponent<0>>(entity).unwrap(),
                &components.0
            );

            assert_eq!(
                app.world().get::<NumberComponent<1>>(entity).unwrap(),
                &components.1
            );
        }

        #[test]
        fn option_of_effects_may_or_may_not_affect(initial: NumberComponent<0>, component: NumberComponent<0>) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(initial).id();

            let component_set_none: Option<ComponentSet<NumberComponent<0>>> = None;

            app.world_mut()
                .query::<<Option<ComponentSet<NumberComponent<0>>> as QueryDataEffect>::MutQueryData>()
                .iter_mut(app.world_mut())
                .for_each(|mut query_data| component_set_none.affect(&mut query_data));

            assert_eq!(
                app.world().get::<NumberComponent<0>>(entity).unwrap(),
                &initial
            );

            let component_set_some = Some(component_set(component));

            app.world_mut()
                .query::<<Option<ComponentSet<NumberComponent<0>>> as QueryDataEffect>::MutQueryData>()
                .iter_mut(app.world_mut())
                .for_each(|mut query_data| component_set_some.affect(&mut query_data));

            assert_eq!(
                app.world().get::<NumberComponent<0>>(entity).unwrap(),
                &component
            );
        }
    }
}
