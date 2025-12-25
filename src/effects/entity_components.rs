use std::marker::PhantomData;

use bevy::ecs::component::Mutable;
use bevy::ecs::query::{QueryData, ReadOnlyQueryData};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::Effect;

/// [`Effect`] that sets the `Component`s of the provided entity to the provided `Component` tuple.
///
/// If an entity with these components cannot be found, handles the `QueryEntityError` with
/// `bevy`'s `DefaultErrorHandler`.
///
/// Can be constructed with [`entity_components_set`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityComponentsSet<C> {
    entity: Entity,
    components: C,
}

impl<C> EntityComponentsSet<C> {
    /// Construct a new [`EntityComponentsSet`].
    pub fn new(entity: Entity, components: C) -> Self {
        EntityComponentsSet { entity, components }
    }
}

/// Construct a new [`EntityComponentsSet`] [`Effect`].
pub fn entity_components_set<C>(entity: Entity, components: C) -> EntityComponentsSet<C> {
    EntityComponentsSet::new(entity, components)
}

macro_rules! impl_effect_for_entity_components_set {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<$($C),*> Effect for EntityComponentsSet<($($C,)*)>
        where
            $($C: Component<Mutability = Mutable>,)*
        {
            type MutParam = (Query<'static, 'static, ($(&'static mut $C,)*)>, <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam);

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let ($(mut $r,)*) = match param.0.get_mut(self.entity) {
                    Ok(r) => r,
                    Err(e) => {
                        Err::<(), _>(e).affect(&mut param.1);
                        return ();
                    }
                };

                let ($($c,)*) = self.components;

                $(*$r = $c;)*
            }
        }
    }
}

all_tuples!(impl_effect_for_entity_components_set, 1, 15, C, c, r);

/// [`Effect`] that transforms the `Component`s of the provided entity with the provided function.
///
/// Can be parameterized by a `ReadOnlyQueryData` to access additional query data in the function.
///
/// If an entity with these components cannot be found, handles the `QueryEntityError` with
/// `bevy`'s `DefaultErrorHandler`.
///
/// Can be constructed with [`entity_components_set_with`] or [`entity_components_set_with_query_data`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityComponentsSetWith<F, C, Data = ()>
where
    F: for<'w, 's> FnOnce(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
{
    entity: Entity,
    f: F,
    components: PhantomData<C>,
    data: PhantomData<Data>,
}

impl<F, C, Data> EntityComponentsSetWith<F, C, Data>
where
    F: for<'w, 's> FnOnce(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
{
    /// Construct a new [`EntityComponentsSetWith`].
    pub fn new(entity: Entity, f: F) -> Self {
        EntityComponentsSetWith {
            entity,
            f,
            components: PhantomData,
            data: PhantomData,
        }
    }
}

/// Construct a new [`EntityComponentsSetWith`] [`Effect`], without extra query data.
pub fn entity_components_set_with<F, C>(
    entity: Entity,
    f: F,
) -> EntityComponentsSetWith<impl FnOnce(C, ()) -> C + Send + Sync, C>
where
    F: for<'w, 's> FnOnce(C) -> C + Send + Sync,
    C: Clone,
{
    EntityComponentsSetWith::new(entity, move |c, _| f(c))
}

/// Construct a new [`EntityComponentsSetWith`] [`Effect`], with extra query data.
pub fn entity_components_set_with_query_data<F, C, Data>(
    entity: Entity,
    f: F,
) -> EntityComponentsSetWith<F, C, Data>
where
    F: for<'w, 's> FnOnce(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
{
    EntityComponentsSetWith::new(entity, f)
}

macro_rules! impl_effect_for_entity_components_set_with {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<F, $($C,)* Data> Effect for EntityComponentsSetWith<F, ($($C,)*), Data>
        where
            F: for<'w, 's> FnOnce(($($C,)*), <Data as QueryData>::Item<'w, 's>) -> ($($C,)*) + Send + Sync,
            $($C: Component<Mutability = Mutable> + Clone,)*
            Data: ReadOnlyQueryData + 'static,
        {
            type MutParam = (Query<'static, 'static, (($(&'static mut $C,)*), Data)>, <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam);

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let (($(mut $r,)*), data) = match param.0.get_mut(self.entity) {
                    Ok(r) => r,
                    Err(e) => {
                        Err::<(), _>(e).affect(&mut param.1);
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

all_tuples!(impl_effect_for_entity_components_set_with, 1, 15, C, c, r);

#[cfg(test)]
mod tests {
    use bevy::ecs::error::DefaultErrorHandler;
    use proptest::prelude::*;
    use proptest::sample::SizeRange;

    use super::*;
    use crate::effects::command_spawn_and;
    use crate::effects::number_data::{
        NumberComponent,
        n0_query_data_to_n1_through_one_way_function,
        n0_to_n1_through_one_way_function,
        two_number_components_one_way_transform,
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
                (move || entity_components_set(entity_to_put, put)).pipe(affect),
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
                    entity_components_set_with::<_, _>(
                        entity_to_transform,
                        two_number_components_one_way_transform(f0, f1)
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
                    entity_components_set_with_query_data::<_, _, &NumberComponent<0>>(
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

    #[test]
    #[should_panic]
    fn entity_components_set_uses_default_error_handler_panic() {
        let mut app = App::new();

        app.add_systems(
            Startup,
            (|| {
                command_spawn_and((), |entity| {
                    entity_components_set(entity, (NumberComponent::<0>(0),))
                })
            })
            .pipe(affect),
        );

        app.update();
    }

    #[test]
    fn entity_components_set_uses_default_error_handler_overridden_doesnt_panic() {
        let mut app = App::new();

        app.world_mut()
            .insert_resource(DefaultErrorHandler(bevy::ecs::error::warn));

        app.add_systems(
            Startup,
            (|| {
                command_spawn_and((), |entity| {
                    entity_components_set(entity, (NumberComponent::<0>(0),))
                })
            })
            .pipe(affect),
        );

        app.update();
    }

    #[test]
    #[should_panic]
    fn entity_components_set_with_uses_default_error_handler_panic() {
        let mut app = App::new();

        app.add_systems(
            Startup,
            (|| {
                command_spawn_and((), |entity| {
                    entity_components_set_with(entity, |_| (NumberComponent::<0>(0),))
                })
            })
            .pipe(affect),
        );

        app.update();
    }

    #[test]
    fn entity_components_set_with_uses_default_error_handler_overridden_doesnt_panic() {
        let mut app = App::new();

        app.world_mut()
            .insert_resource(DefaultErrorHandler(bevy::ecs::error::warn));

        app.add_systems(
            Startup,
            (|| {
                command_spawn_and((), |entity| {
                    entity_components_set_with(entity, |_| (NumberComponent::<0>(0),))
                })
            })
            .pipe(affect),
        );

        app.update();
    }
}
