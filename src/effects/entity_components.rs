use std::marker::PhantomData;

use bevy::ecs::component::Mutable;
use bevy::ecs::query::{QueryData, ReadOnlyQueryData};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::Effect;

/// [`Effect`] that sets the `Component`s of the provided entity to the provided `Component` tuple.
///
/// If an entity with these components cannot be found, logs an error.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityComponentsPut<C> {
    entity: Entity,
    components: C,
}

impl<C> EntityComponentsPut<C> {
    /// Construct a new [`EntityComponentsPut`].
    pub fn new(entity: Entity, components: C) -> Self {
        EntityComponentsPut { entity, components }
    }
}

macro_rules! impl_effect_for_entity_components_put {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<$($C),*> Effect for EntityComponentsPut<($($C,)*)>
        where
            $($C: Component<Mutability = Mutable>,)*
        {
            type MutParam = Query<'static, 'static, ($(&'static mut $C,)*)>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let ($(mut $r,)*) = match param.get_mut(self.entity) {
                    Ok(r) => r,
                    Err(e) => {
                        error!("unable to query entity in EntityComponentsPut: {e}");
                        return ();
                    }
                };

                let ($($c,)*) = self.components;

                $(*$r = $c;)*
            }
        }
    }
}

all_tuples!(impl_effect_for_entity_components_put, 1, 15, C, c, r);

/// [`Effect`] that transforms the `Component`s of the provided entity with the provided function.
///
/// Can be parameterized by a `ReadOnlyQueryData` to access additional query data in the function.
///
/// If an entity with these components cannot be found, logs an error.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityComponentsWith<F, C, Data = ()>
where
    F: for<'a> FnOnce(C, <Data as QueryData>::Item<'a>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
{
    entity: Entity,
    f: F,
    components: PhantomData<C>,
    data: PhantomData<Data>,
}

impl<F, C, Data> EntityComponentsWith<F, C, Data>
where
    F: for<'a> FnOnce(C, <Data as QueryData>::Item<'a>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
{
    /// Construct a new [`EntityComponentsWith`].
    pub fn new(entity: Entity, f: F) -> Self {
        EntityComponentsWith {
            entity,
            f,
            components: PhantomData,
            data: PhantomData,
        }
    }
}

macro_rules! impl_effect_for_entity_components_with {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<F, $($C,)* Data> Effect for EntityComponentsWith<F, ($($C,)*), Data>
        where
            F: for<'a> FnOnce(($($C,)*), <Data as QueryData>::Item<'a>) -> ($($C,)*) + Send + Sync,
            $($C: Component<Mutability = Mutable> + Clone,)*
            Data: ReadOnlyQueryData + 'static,
        {
            type MutParam = Query<'static, 'static, (($(&'static mut $C,)*), Data)>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let (($(mut $r,)*), data) = match param.get_mut(self.entity) {
                    Ok(r) => r,
                    Err(e) => {
                        error!("unable to query entity in EntityComponentsWith: {e}");
                        return ();
                    }
                };
                let cloned = ($($r.clone(),)*);
                let ($($c,)*) = (self.f)(cloned, data);

                $(*$r = $c;)*
            }
        }
    };
}

all_tuples!(impl_effect_for_entity_components_with, 1, 15, C, c, r);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest::sample::SizeRange;

    use super::*;
    use crate::effects::number_data::{
        n0_query_data_to_n1_through_one_way_function,
        n0_to_n1_through_one_way_function,
        two_number_components_one_way_transform,
        two_number_components_one_way_transform_with_void_query_data,
        NumberComponent,
    };
    use crate::effects::one_way_fn::OneWayFn;
    use crate::prelude::affect;

    fn vec_and_index<T, R>(element: T, range: R) -> impl Strategy<Value = (Vec<T::Value>, usize)>
    where
        T: Strategy,
        <T as Strategy>::Value: Clone,
        R: Into<SizeRange>,
    {
        proptest::collection::vec(element, range).prop_flat_map(|v| {
            let len = v.len();
            (Just(v), 0..len)
        })
    }

    proptest! {
        #[test]
        fn entity_components_put_overwrites_initial_state_of_single_entity(
            (initial_bundles, index_to_put) in vec_and_index(any::<(NumberComponent<0>, NumberComponent<1>)>(), 1..4),
            put: (NumberComponent<0>, NumberComponent<1>)
        ) {
            let mut app = App::new();

            let entities = app
                .world_mut()
                .spawn_batch(initial_bundles.clone())
                .collect::<Vec<_>>();

            let entity_to_put = entities[index_to_put];

            app.add_systems(
                Update,
                (move || EntityComponentsPut::new(entity_to_put, put)).pipe(affect),
            );

            app.update();

            for (initial, entity) in initial_bundles.into_iter().zip(entities) {
                let actual0 = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual1 = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let (expected0, expected1) = if entity == entity_to_put {
                    put
                } else {
                    initial
                };

                prop_assert_eq!(actual0, &expected0);
                prop_assert_eq!(actual1, &expected1);
            }
        }

        #[test]
        fn entity_components_with_correctly_executes_one_way_function(
            (initial_bundles, index_to_put) in vec_and_index(any::<(NumberComponent<0>, NumberComponent<1>)>(), 1..4),
            f0: OneWayFn,
            f1: OneWayFn,
        ) {
            let mut app = App::new();

            let entities = app
                .world_mut()
                .spawn_batch(initial_bundles.clone())
                .collect::<Vec<_>>();

            let entity_to_transform = entities[index_to_put];

            app.add_systems(
                Update,
                (move || {
                    EntityComponentsWith::<_, _, ()>::new(
                        entity_to_transform,
                        two_number_components_one_way_transform_with_void_query_data(f0, f1)
                    )
                })
                .pipe(affect),
            );

            app.update();

            for (initial, entity) in
                initial_bundles.into_iter().zip(entities)
            {
                let actual0 = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual1 = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let (expected0, expected1) = if entity == entity_to_transform {
                    two_number_components_one_way_transform(f0, f1)(initial)
                } else {
                    initial
                };

                prop_assert_eq!(actual0, &expected0);
                prop_assert_eq!(actual1, &expected1);
            }
        }

        #[test]
        fn read_only_query_data_paramaterizes_entity_components_with_function(
            (initial_bundles, index_to_put) in vec_and_index(any::<(NumberComponent<0>, NumberComponent<1>)>(), 1..4),
            f: OneWayFn,
        ) {
            let mut app = App::new();

            let entities = app
                .world_mut()
                .spawn_batch(initial_bundles.clone())
                .collect::<Vec<_>>();

            let entity_to_transform = entities[index_to_put];

            app.add_systems(
                Update,
                (move || {
                    EntityComponentsWith::<_, _, &NumberComponent<0>>::new(
                        entity_to_transform,
                        n0_query_data_to_n1_through_one_way_function(f),
                    )
                })
                .pipe(affect),
            );

            app.update();

            for ((expected_read_component, initial_written_component), entity) in
                initial_bundles.into_iter().zip(entities)
            {
                let actual_read_component = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual_written_component = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let expected_written_component = if entity == entity_to_transform {
                    n0_to_n1_through_one_way_function(f)(expected_read_component)
                } else {
                    initial_written_component
                };

                prop_assert_eq!(actual_read_component, &expected_read_component);
                prop_assert_eq!(actual_written_component, &expected_written_component);
            }
        }
    }
}
