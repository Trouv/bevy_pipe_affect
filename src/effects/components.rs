use std::convert::identity;
use std::marker::PhantomData;

use bevy::ecs::component::Mutable;
use bevy::ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::Effect;

/// [`Effect`] that sets `Component`s of all entities in a query to the provided `Component` tuple.
///
/// If you want to parameterize the query with a filter, see [`ComponentsSetFiltered`].
///
/// Can be constructed by [`components_set`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentsSet<C>
where
    C: Clone,
{
    /// The tuple of component values to set.
    pub components: C,
}

/// Construct a new [`ComponentsSet`] [`Effect`].
pub fn components_set<C>(components: C) -> ComponentsSet<C>
where
    C: Clone,
{
    ComponentsSet { components }
}

impl<C> Effect for ComponentsSet<C>
where
    C: Clone,
    ComponentsSetFiltered<C>: Effect,
{
    type MutParam = <ComponentsSetFiltered<C> as Effect>::MutParam;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        components_set_filtered::<_, ()>(self.components).affect(param);
    }
}

/// [`Effect`] that sets `Component`s of all entities in a query to the provided `Component` tuple.
///
/// Can be parameterized by a `QueryFilter` to narrow down the components updated.
/// If you do not need a filter, see [`ComponentsSet`].
///
/// Can be constructed by [`components_set_filtered`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ComponentsSetFiltered<C, Filter = ()>
where
    C: Clone,
    Filter: QueryFilter,
{
    components: C,
    filter: PhantomData<Filter>,
}

impl<C, Filter> ComponentsSetFiltered<C, Filter>
where
    C: Clone,
    Filter: QueryFilter,
{
    /// Construct a new [`ComponentsSetFiltered`].
    pub fn new(components: C) -> Self {
        ComponentsSetFiltered {
            components,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`ComponentsSetFiltered`] [`Effect`].
pub fn components_set_filtered<C, Filter>(components: C) -> ComponentsSetFiltered<C, Filter>
where
    C: Clone,
    Filter: QueryFilter,
{
    ComponentsSetFiltered::new(components)
}

macro_rules! impl_effect_for_components_set {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<$($C,)* Filter> Effect for ComponentsSetFiltered<($($C,)*), Filter>
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
/// For more query parameterization options, see:
/// - [`ComponentsSetWith`]
/// - [`ComponentsSetFilteredWith`]
/// - [`ComponentsSetWithQueryData`]
///
/// Can be constructed by [`components_set_filtered_with_query_data`].
#[derive(derive_more::Debug)]
pub struct ComponentsSetFilteredWithQueryData<C, Data = (), Filter = ()>
where
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    #[debug("{0}, {1} -> {0}", std::any::type_name::<C>(), std::any::type_name::<Data::Item<'static, 'static>>())]
    #[expect(clippy::type_complexity)]
    f: Box<dyn for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync>,
    filter: PhantomData<Filter>,
}

impl<C, Data, Filter> ComponentsSetFilteredWithQueryData<C, Data, Filter>
where
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    /// Construct a new [`ComponentsSetFilteredWithQueryData`].
    #[expect(clippy::type_complexity)]
    pub fn new(f: Box<dyn Fn(C, <Data as QueryData>::Item<'_, '_>) -> C + Send + Sync>) -> Self {
        ComponentsSetFilteredWithQueryData {
            f,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`ComponentsSetFilteredWithQueryData`] [`Effect`].
pub fn components_set_filtered_with_query_data<F, C, Data, Filter>(
    f: F,
) -> ComponentsSetFilteredWithQueryData<C, Data, Filter>
where
    F: for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync + 'static,
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    ComponentsSetFilteredWithQueryData::new(Box::new(f))
}

impl<C, Data, Filter> Default for ComponentsSetFilteredWithQueryData<C, Data, Filter>
where
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    fn default() -> Self {
        components_set_filtered_with_query_data(|c, _| c)
    }
}

macro_rules! impl_effect_for_components_set_filtered_with_query_data {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<$($C,)* Data, Filter> Effect for ComponentsSetFilteredWithQueryData<($($C,)*), Data, Filter>
        where
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

all_tuples!(
    impl_effect_for_components_set_filtered_with_query_data,
    1,
    15,
    C,
    c,
    r
);

/// [`Effect`] that transforms `Component`s of all entities in a query with the provided function.
///
/// Can be parameterized by a `ReadOnlyQueryData` to access additional query data in the function.
///
/// For more query parameterization options, see:
/// - [`ComponentsSetWith`]
/// - [`ComponentsSetFilteredWith`]
/// - [`ComponentsSetFilteredWithQueryData`]
///
/// Can be constructed by [`components_set_with_query_data`].
#[derive(derive_more::Debug)]
pub struct ComponentsSetWithQueryData<C, Data = ()>
where
    C: Clone,
    Data: ReadOnlyQueryData,
{
    /// The function that is applied to the components `C`.
    #[debug("{0}, {1} -> {0}", std::any::type_name::<C>(), std::any::type_name::<Data::Item<'static, 'static>>())]
    #[expect(clippy::type_complexity)]
    pub f: Box<dyn for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync>,
}

/// Construct a new [`ComponentsSetWithQueryData`] [`Effect`].
pub fn components_set_with_query_data<F, C, Data>(f: F) -> ComponentsSetWithQueryData<C, Data>
where
    F: for<'w, 's> Fn(C, <Data as QueryData>::Item<'w, 's>) -> C + Send + Sync + 'static,
    C: Clone,
    Data: ReadOnlyQueryData,
{
    ComponentsSetWithQueryData { f: Box::new(f) }
}

impl<C, Data> Default for ComponentsSetWithQueryData<C, Data>
where
    C: Clone,
    Data: ReadOnlyQueryData,
{
    fn default() -> Self {
        components_set_with_query_data(|c, _| c)
    }
}

impl<C, Data> Effect for ComponentsSetWithQueryData<C, Data>
where
    C: Clone,
    Data: ReadOnlyQueryData,
    ComponentsSetFilteredWithQueryData<C, Data, ()>: Effect,
{
    type MutParam = <ComponentsSetFilteredWithQueryData<C, Data, ()> as Effect>::MutParam;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        ComponentsSetFilteredWithQueryData::new(self.f).affect(param);
    }
}

/// [`Effect`] that transforms `Component`s of all entities in a query with the provided function.
///
/// Can be parameterized by a `QueryFilter` to narrow down the components updated.
///
/// For more query parameterization options, see:
/// - [`ComponentsSetWith`]
/// - [`ComponentsSetWithQueryData`]
/// - [`ComponentsSetFilteredWithQueryData`]
///
/// Can be constructed by [`components_set_filtered_with`].
#[derive(derive_more::Debug)]
pub struct ComponentsSetFilteredWith<C, Filter = ()>
where
    C: Clone,
    Filter: QueryFilter,
{
    #[debug("{0} -> {0}", std::any::type_name::<C>())]
    f: Box<dyn Fn(C) -> C + Send + Sync>,
    filter: PhantomData<Filter>,
}

impl<C, Filter> ComponentsSetFilteredWith<C, Filter>
where
    C: Clone,
    Filter: QueryFilter,
{
    /// Construct a new [`ComponentsSetFilteredWith`].
    pub fn new(f: Box<dyn Fn(C) -> C + Send + Sync>) -> Self {
        ComponentsSetFilteredWith {
            f,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`ComponentsSetFilteredWith`] [`Effect`].
pub fn components_set_filtered_with<F, C, Filter>(f: F) -> ComponentsSetFilteredWith<C, Filter>
where
    F: Fn(C) -> C + Send + Sync + 'static,
    C: Clone,
    Filter: QueryFilter,
{
    ComponentsSetFilteredWith::new(Box::new(f))
}

impl<C, Filter> Default for ComponentsSetFilteredWith<C, Filter>
where
    C: Clone + 'static,
    Filter: QueryFilter,
{
    fn default() -> Self {
        components_set_filtered_with(identity)
    }
}

macro_rules! impl_effect_for_components_set_filtered_with {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<$($C,)* Filter> Effect for ComponentsSetFilteredWith<($($C,)*), Filter>
        where
            $($C: Component<Mutability = Mutable> + Clone),*,
            Filter: QueryFilter + 'static,
        {
            type MutParam = Query<'static, 'static, ($(&'static mut $C,)*), Filter>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                param.par_iter_mut().for_each(|($(mut $r,)*)| {
                    let cloned = ($($r.clone(),)*);
                    let ($($c,)*) = (self.f)(cloned);
                    $(*$r = $c;)*
                });
            }
        }
    }
}

all_tuples!(impl_effect_for_components_set_filtered_with, 1, 15, C, c, r);

/// [`Effect`] that transforms `Component`s of all entities in a query with the provided function.
///
/// For more query parameterization options, see:
/// - [`ComponentsSetFilteredWith`]
/// - [`ComponentsSetWithQueryData`]
/// - [`ComponentsSetFilteredWithQueryData`]
///
/// Can be constructed by [`components_set_with`].
#[derive(derive_more::Debug)]
pub struct ComponentsSetWith<C>
where
    C: Clone,
{
    /// The function that is applied to the components `C`.
    #[debug("{0} -> {0}", std::any::type_name::<C>())]
    pub f: Box<dyn Fn(C) -> C + Send + Sync>,
}

/// Construct a new [`ComponentsSetWith`] [`Effect`].
pub fn components_set_with<F, C>(f: F) -> ComponentsSetWith<C>
where
    F: Fn(C) -> C + Send + Sync + 'static,
    C: Clone,
{
    ComponentsSetWith { f: Box::new(f) }
}

impl<C> Default for ComponentsSetWith<C>
where
    C: Clone + 'static,
{
    fn default() -> Self {
        components_set_with(identity)
    }
}

impl<C> Effect for ComponentsSetWith<C>
where
    C: Clone,
    ComponentsSetFilteredWith<C, ()>: Effect,
{
    type MutParam = <ComponentsSetFilteredWith<C, ()> as Effect>::MutParam;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        ComponentsSetFilteredWith::new(self.f).affect(param);
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    use super::*;
    use crate::effects::number_data::{
        NumberComponent,
        n0_query_data_to_n1_through_one_way_function,
        n0_to_n1_through_one_way_function,
        two_number_components_one_way_transform,
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
        fn components_set_overwrites_initial_state(
            test_bundles in proptest::collection::vec(any::<BundleToBeMarked<(NumberComponent<0>, NumberComponent<1>)>>(), 0..16),
            put: (NumberComponent<0>, NumberComponent<1>),
        ) {
            let mut app = App::new();

            let entities = test_bundles
                .iter()
                .copied()
                .map(|bundle| spawn_bundle_and_mark(app.world_mut(), bundle))
                .collect::<Vec<_>>();

            app.add_systems(Update, (move || components_set(put)).pipe(affect));

            app.update();

            for entity in entities {
                let actual0 = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual1 = app.world().get::<NumberComponent<1>>(entity).unwrap();

                prop_assert_eq!(actual0, &put.0);
                prop_assert_eq!(actual1, &put.1);
            }
        }

        #[test]
        fn components_set_filtered_overwrites_initial_state(
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
                (move || components_set_filtered::<_, With<MarkerComponent>>(put)).pipe(affect),
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
        fn components_set_with_correctly_executes_one_way_function(
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
                    components_set_with(
                        two_number_components_one_way_transform(f0, f1)
                    )
                })
                .pipe(affect),
            );

            app.update();

            for (
                BundleToBeMarked {
                    initial,
                    ..
                },
                entity,
            ) in test_bundles.into_iter().zip(entities)
            {
                let actual0 = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual1 = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let (expected0, expected1) = two_number_components_one_way_transform(f0, f1)(initial);

                prop_assert_eq!(actual0, &expected0);
                prop_assert_eq!(actual1, &expected1);
            }
        }

        #[test]
        fn components_set_filtered_with_correctly_executes_one_way_function(
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
                    components_set_filtered_with::<_, _, With<MarkerComponent>>(
                        two_number_components_one_way_transform(f0, f1)
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
        fn read_only_query_data_paramaterizes_components_set_with_function(
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
                    components_set_with_query_data::<_, _, &NumberComponent<0>>(
                        n0_query_data_to_n1_through_one_way_function(f),
                    )
                })
                .pipe(affect),
            );

            app.update();

            for (
                BundleToBeMarked {
                    initial: (expected_read_component, _),
                    ..
                },
                entity,
            ) in initial_bundles.into_iter().zip(entities)
            {
                let actual_read_component = app.world().get::<NumberComponent<0>>(entity).unwrap();
                let actual_written_component = app.world().get::<NumberComponent<1>>(entity).unwrap();

                let expected_written_component =
                    n0_to_n1_through_one_way_function(f)(expected_read_component);

                prop_assert_eq!(actual_read_component, &expected_read_component);
                prop_assert_eq!(actual_written_component, &expected_written_component);
            }
        }

        #[test]
        fn read_only_query_data_paramaterizes_components_set_filtered_with_function(
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
                    components_set_filtered_with_query_data::<_, _, &NumberComponent<0>, With<MarkerComponent>>(
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
