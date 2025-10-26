use std::marker::PhantomData;

use bevy::ecs::component::Mutable;
use bevy::ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::Effect;

/// [`Effect`] that sets `Component`s of all entities in a query to the provided `Component` tuple.
///
/// Can be parameterized by a `QueryFilter` to narrow down the components updated.
///
/// Can be constructed by [`components_set`] or [`components_set_filtered`]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentsSet<C, Filter = ()>
where
    C: Clone,
    Filter: QueryFilter,
{
    components: C,
    filter: PhantomData<Filter>,
}

impl<C, Filter> ComponentsSet<C, Filter>
where
    C: Clone,
    Filter: QueryFilter,
{
    /// Construct a new [`ComponentsSet`].
    pub fn new(components: C) -> Self {
        ComponentsSet {
            components,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`ComponentsSet`] [`Effect`], with no filter.
pub fn components_set<C>(components: C) -> ComponentsSet<C, ()>
where
    C: Clone,
{
    ComponentsSet::new(components)
}

/// Construct a new [`ComponentsSet`] [`Effect`], with a custom filter.
pub fn components_set_filtered<C, Filter>(components: C) -> ComponentsSet<C, Filter>
where
    C: Clone,
    Filter: QueryFilter,
{
    ComponentsSet::new(components)
}

macro_rules! impl_effect_for_components_set {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<$($C,)* Filter> Effect for ComponentsSet<($($C,)*), Filter>
        where
            $($C: Component<Mutability = Mutable> + Clone),*,
            Filter: QueryFilter + 'static,
        {
            type MutParam = Query<'static, 'static, ($(&'static mut $C,)*), Filter>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let ($($c,)*) = self.components;
                param.par_iter_mut().for_each(|($(mut $r,)*)| {
                    $(*$r = $c.clone();)*
                });
            }
        }
    }
}

all_tuples!(impl_effect_for_components_set, 1, 15, C, c, r);

/// [`Effect`] that transforms `Component`s of all entities in a query with the provided function.
///
/// Can be parameterized by a `ReadOnlyQueryData` to access additional query data in the function.
///
/// Can be parameterized by a `QueryFilter` to narrow down the components updated.
///
/// Can be constructed by
/// - [`components_set_with`]
/// - [`components_set_filtered_with`]
/// - [`components_set_with_query_data`]
/// - [`components_set_filtered_with_query_data`]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentsSetWith<F, C, Data = (), Filter = ()>
where
    F: for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    f: F,
    components: PhantomData<C>,
    data: PhantomData<Data>,
    filter: PhantomData<Filter>,
}

impl<F, C, Data, Filter> ComponentsSetWith<F, C, Data, Filter>
where
    F: for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    /// Construct a new [`ComponentsSetWith`].
    pub fn new(f: F) -> Self {
        ComponentsSetWith {
            f,
            components: PhantomData,
            data: PhantomData,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`ComponentsSetWith`] [`Effect`], with no filter or query data.
pub fn components_set_with<F, C>(f: F) -> ComponentsSetWith<impl Fn(C, ()) -> C + Send + Sync, C>
where
    F: for<'w, 's> Fn(C) -> C + Send + Sync,
    C: Clone,
{
    ComponentsSetWith::new(move |c, _| f(c))
}

/// Construct a new [`ComponentsSetWith`] [`Effect`], with extra query data.
pub fn components_set_with_query_data<F, C, Data>(f: F) -> ComponentsSetWith<F, C, Data>
where
    F: for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
{
    ComponentsSetWith::new(f)
}

/// Construct a new [`ComponentsSetWith`] [`Effect`], with a custom filter.
pub fn components_set_filtered_with<F, C, Filter>(
    f: F,
) -> ComponentsSetWith<impl Fn(C, ()) -> C + Send + Sync, C, (), Filter>
where
    F: for<'w, 's> Fn(C) -> C + Send + Sync,
    C: Clone,
    Filter: QueryFilter,
{
    ComponentsSetWith::new(move |c, _| f(c))
}

/// Construct a new [`ComponentsSetWith`] [`Effect`], with a custom filter and extra query data.
pub fn components_set_filtered_with_query_data<F, C, Data, Filter>(
    f: F,
) -> ComponentsSetWith<F, C, Data, Filter>
where
    F: for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync,
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    ComponentsSetWith::new(f)
}

macro_rules! impl_effect_for_components_set_with {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<F, $($C,)* Data, Filter> Effect for ComponentsSetWith<F, ($($C,)*), Data, Filter>
        where
            F: for<'w, 's> Fn(($($C,)*), <Data as QueryData>::Item<'w, 's>) -> ($($C,)*) + Send + Sync,
            $($C: Component<Mutability = Mutable> + Clone),*,
            Data: ReadOnlyQueryData + 'static,
            Filter: QueryFilter + 'static,
        {
            type MutParam = Query<'static, 'static, (($(&'static mut $C,)*), Data), Filter>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                param.par_iter_mut().for_each(|(($(mut $r,)*), data)| {
                    let cloned = ($($r.clone(),)*);
                    let ($($c,)*) = (self.f)(cloned, data);
                    $(*$r = $c;)*
                });
            }
        }
    }
}

all_tuples!(impl_effect_for_components_set_with, 1, 15, C, c, r);

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    use super::*;
    use crate::effects::number_data::{
        n0_query_data_to_n1_through_one_way_function,
        n0_to_n1_through_one_way_function,
        two_number_components_one_way_transform,
        two_number_components_one_way_transform_with_void_query_data,
        NumberComponent,
    };
    use crate::effects::one_way_fn::OneWayFn;
    use crate::system_combinators::affect;

    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Component)]
    struct MarkerComponent;

    #[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Arbitrary)]
    struct BundleToBeMarked<B: Bundle + Arbitrary> {
        initial: B,
        to_be_marked: bool,
    }

    fn spawn_bundle_and_mark<B: Bundle + Arbitrary>(
        world: &mut World,
        BundleToBeMarked {
            initial,
            to_be_marked,
        }: BundleToBeMarked<B>,
    ) -> Entity {
        let mut entity_commands = world.spawn(initial);

        if to_be_marked {
            entity_commands.insert(MarkerComponent);
        }

        entity_commands.id()
    }

    proptest! {
        #[test]
        fn components_put_overwrites_initial_state(
            test_bundles in proptest::collection::vec(any::<BundleToBeMarked<(NumberComponent<0>, NumberComponent<1>)>>(), 0..16),
            put: (NumberComponent<0>, NumberComponent<1>)
        ) {
            let mut app = App::new();

            let entities = test_bundles
                .iter()
                .copied()
                .map(|bundle| spawn_bundle_and_mark(app.world_mut(), bundle))
                .collect::<Vec<_>>();

            app.add_systems(
                Update,
                (move || ComponentsSet::<_, With<MarkerComponent>>::new(put)).pipe(affect),
            );

            app.update();

            for (
                BundleToBeMarked {
                    initial,
                    to_be_marked,
                },
                entity,
            ) in test_bundles.into_iter().zip(entities)
            {
                let actual0 = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual1 = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let (expected0, expected1) = if to_be_marked { put } else { initial };

                prop_assert_eq!(actual0, &expected0);
                prop_assert_eq!(actual1, &expected1);
            }
        }

        #[test]
        fn components_with_correctly_executes_one_way_function(
            test_bundles in proptest::collection::vec(any::<BundleToBeMarked<(NumberComponent<0>, NumberComponent<1>)>>(), 0..16),
            f0: OneWayFn,
            f1: OneWayFn
        ) {
            let mut app = App::new();

            let entities = test_bundles
                .iter()
                .copied()
                .map(|bundle| spawn_bundle_and_mark(app.world_mut(), bundle))
                .collect::<Vec<_>>();

            app.add_systems(
                Update,
                (move || {
                    ComponentsSetWith::<_, _, (), With<MarkerComponent>>::new(
                        two_number_components_one_way_transform_with_void_query_data(f0, f1)
                    )
                })
                .pipe(affect),
            );

            app.update();

            for (
                BundleToBeMarked {
                    initial,
                    to_be_marked,
                },
                entity,
            ) in test_bundles.into_iter().zip(entities)
            {
                let actual0 = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual1 = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let (expected0, expected1) = if to_be_marked {
                    two_number_components_one_way_transform(f0, f1)(initial)
                } else {
                    initial
                };

                prop_assert_eq!(actual0, &expected0);
                prop_assert_eq!(actual1, &expected1);
            }
        }

        #[test]
        fn read_only_query_data_paramaterizes_components_with_function(
            initial_bundles in proptest::collection::vec(any::<BundleToBeMarked<(NumberComponent<0>, NumberComponent<1>)>>(), 0..16),
            f: OneWayFn,
        ) {
            let mut app = App::new();

            let entities = initial_bundles
                .iter()
                .copied()
                .map(|bundle| spawn_bundle_and_mark(app.world_mut(), bundle))
                .collect::<Vec<_>>();

            app.add_systems(
                Update,
                (move || {
                    ComponentsSetWith::<_, _, &NumberComponent<0>, With<MarkerComponent>>::new(
                        n0_query_data_to_n1_through_one_way_function(f)
                    )
                })
                .pipe(affect),
            );

            app.update();

            for (
                BundleToBeMarked {
                    initial: (expected_read_component, initial_written_component),
                    to_be_marked,
                },
                entity,
            ) in initial_bundles.into_iter().zip(entities)
            {
                let actual_read_component = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual_written_component = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let expected_written_component = if to_be_marked {
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
