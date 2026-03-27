use std::marker::PhantomData;

use bevy::ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;

use crate::query_data_effect::QueryDataEffect;
use crate::{Effect, EffectOut, effect_out};

/// [`Effect`] that applies a [`QueryDataEffect`] to all entities in a query.
///
/// The query can be filtered with the `Filter` generic.
///
/// Can be constructed with [`query_affect`].
///
/// # Example
/// In this example, a system is written that sets all entities' `Speed` to 0 if they have a
/// `Brake` component.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Speed(f32);
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Brake;
///
/// // Pure system using effects.
/// fn stop_all_pure() -> QueryAffect<ComponentSet<Speed>, With<Brake>> {
///     query_affect(component_set(Speed(0.0)))
/// }
///
/// // Equivalent impure system.
/// fn stop_all_impure(mut query: Query<&mut Speed, With<Brake>>) {
///     for (mut number_component) in query.iter_mut() {
///         *number_component = Speed(0.0);
///     }
/// }
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(component_table: Vec<(Option<Speed>, Option<Brake>)>) -> App {
/// #     let mut app = App::new();
/// #     component_table.into_iter().for_each(|(speed, brake)| {
/// #         let mut entity = app.world_mut().spawn_empty();
/// #         if let Some(speed) = speed {
/// #             entity.insert(speed);
/// #         }
/// #         if let Some(brake) = brake {
/// #             entity.insert(brake);
/// #         }
/// #     });
/// #
/// #     app
/// # }
/// #
/// # fn query_state(world: &mut World) -> Vec<(Entity, Option<&Speed>, Option<&Brake>)> {
/// #     let mut query = world.query::<(Entity, Option<&Speed>, Option<&Brake>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<(Option<Speed>, Option<Brake>)>) {
/// #         let mut pure_app = app_setup(component_table.clone());
/// #         pure_app.add_systems(Update, stop_all_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table.clone());
/// #         impure_app.add_systems(Update, stop_all_impure);
/// #
/// #         for _ in 0..3 {
/// #             assert_eq!(query_state(pure_app.world_mut()), query_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - other [`QueryDataEffect`]s are available
/// - the `Filter` parameter can be omitted
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct QueryAffect<QueryDataE, Filter = ()>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// The [`QueryDataEffect`] that is applied to all entities in the query.
    pub query_data_effect: QueryDataE,
    filter: PhantomData<Filter>,
}

impl<QueryDataE, Filter> QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryAffect`].
    pub fn new(query_data_effect: QueryDataE) -> Self {
        QueryAffect {
            query_data_effect,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`QueryAffect`] [`Effect`].
pub fn query_affect<QueryDataE, Filter>(
    query_data_effect: QueryDataE,
) -> QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    QueryAffect::new(query_data_effect)
}

impl<QueryDataE, Filter> Effect for QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect + Clone,
    QueryDataE::MutQueryData: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = Query<'static, 'static, QueryDataE::MutQueryData, Filter>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param
            .iter_mut()
            .for_each(|mut query_data| self.query_data_effect.clone().affect(&mut query_data));
    }
}

/// Type alias for the mapping function in [`QueryMap`].
pub type BoxedQueryMapFn<QueryDataIn, QueryDataE> =
    Box<dyn for<'w, 's> Fn(<QueryDataIn as QueryData>::Item<'w, 's>) -> QueryDataE>;

/// [`Effect`] that applies a mapping of `QueryData` to [`QueryDataEffect`] to all entities in a
/// query.
///
/// This query can be filtered with the `Filter` generic.
///
/// Can be constructed with [`query_map`].
///
/// # Example
/// In this example, a system is written that updates all entities' `Speed` component according to
/// their `Acceleration` component.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Default, Debug, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Acceleration(f32);
///
/// #[derive(Copy, Clone, Default, Debug, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Speed(f32);
///
/// // Pure system using effects.
/// fn accelerate_pure() -> QueryMap<(&'static Acceleration, &'static Speed), ComponentSet<Speed>> {
///     query_map(move |(acceleration, speed): (&Acceleration, &Speed)| {
///         component_set(Speed(speed.0 + acceleration.0))
///     })
/// }
///
/// // Equivalent impure system.
/// fn accelerate_impure(mut query: Query<(&Acceleration, &mut Speed)>) {
///     for (acceleration, mut speed) in query.iter_mut() {
///         speed.0 += acceleration.0
///     }
/// }
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(component_table: Vec<(Option<Acceleration>, Option<Speed>)>) -> App {
/// #     let mut app = App::new();
/// #     component_table.into_iter().for_each(|(acceleration, speed)| {
/// #         let mut entity = app.world_mut().spawn_empty();
/// #         if let Some(acceleration) = acceleration {
/// #             entity.insert(acceleration);
/// #         }
/// #         if let Some(speed) = speed {
/// #             entity.insert(speed);
/// #         }
/// #     });
/// #
/// #     app
/// # }
/// #
/// # fn query_state(world: &mut World) -> Vec<(Entity, Option<&Acceleration>, Option<&Speed>)> {
/// #     let mut query = world.query::<(Entity, Option<&Acceleration>, Option<&Speed>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<(Option<Acceleration>, Option<Speed>)>) {
/// #         let mut pure_app = app_setup(component_table.clone());
/// #         pure_app.add_systems(Update, accelerate_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table.clone());
/// #         impure_app.add_systems(Update, accelerate_impure);
/// #
/// #         for _ in 0..3 {
/// #             assert_eq!(query_state(pure_app.world_mut()), query_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - other [`QueryDataEffect`]s are available
/// - any `QueryData` (including single components) can be input to the map function
/// - a filter can be applied using the `Filter` generic parameter.
#[derive(derive_more::Debug)]
pub struct QueryMap<QueryDataIn, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// The `QueryData -> QueryDataEffect` function that is applied to all entities in the query.
    #[debug("{0} -> {1}", std::any::type_name::<QueryDataIn>(), std::any::type_name::<QueryDataE>())]
    pub f: BoxedQueryMapFn<QueryDataIn, QueryDataE>,
    filter: PhantomData<Filter>,
}

impl<QueryDataIn, QueryDataE, Filter> QueryMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryMap`].
    pub fn new(f: BoxedQueryMapFn<QueryDataIn, QueryDataE>) -> Self {
        QueryMap {
            f,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`QueryMap`] [`Effect`].
pub fn query_map<QueryDataIn, QueryDataE, Filter, F>(
    f: F,
) -> QueryMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE + 'static,
{
    QueryMap::new(Box::new(f))
}

impl<QueryDataIn, QueryDataE, Filter> Effect for QueryMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData + 'static,
    QueryDataE: QueryDataEffect,
    QueryDataE::MutQueryData: 'static,
    QueryDataE::Filter: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = ParamSet<
        'static,
        'static,
        (
            Query<'static, 'static, (Entity, QueryDataIn), (QueryDataE::Filter, Filter)>,
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let query_data_effects = param
            .p0()
            .iter()
            .map(|(entity, data_in)| (entity, (self.f)(data_in)))
            .collect::<Vec<_>>();

        query_data_effects
            .into_iter()
            .for_each(|(entity, query_data_effect)| {
                query_data_effect.affect(&mut param.p1().get_mut(entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));
            })
    }
}

/// Type alias for the mapping function in [`QueryMapAnd`].
pub type BoxedQueryMapAndFn<QueryDataIn, E, QueryDataE> =
    Box<dyn for<'w, 's> Fn(<QueryDataIn as QueryData>::Item<'w, 's>) -> EffectOut<E, QueryDataE>>;

/// [`Effect`] that applies a mapping from `QueryData` to a [`QueryDataEffect`] + [`Effect`] (as an
/// `EffectOut<Effect, QueryDataEffect>`) to all entities in a query.
///
/// The query can be filtered with the `Filter` generic.
///
/// Can be constructed by [`query_map_and`].
#[derive(derive_more::Debug)]
pub struct QueryMapAnd<QueryDataIn, E, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// The `QueryData -> EffectOut<Effect, QueryDataEffect>` function that applies to all entities
    /// in the query.
    #[debug("{0} -> {1}", std::any::type_name::<QueryDataIn>(), std::any::type_name::<EffectOut<E, QueryDataE>>())]
    pub f: BoxedQueryMapAndFn<QueryDataIn, E, QueryDataE>,
    filter: PhantomData<Filter>,
}

impl<QueryDataIn, E, QueryDataE, Filter> QueryMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryMapAnd`].
    pub fn new(f: BoxedQueryMapAndFn<QueryDataIn, E, QueryDataE>) -> Self {
        QueryMapAnd {
            f,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`QueryMapAnd`] [`Effect`].
pub fn query_map_and<QueryDataIn, E, QueryDataE, Filter, F>(
    f: F,
) -> QueryMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE> + 'static,
{
    QueryMapAnd::new(Box::new(f))
}

impl<QueryDataIn, E, QueryDataE, Filter> Effect for QueryMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData + 'static,
    E: Effect,
    QueryDataE: QueryDataEffect,
    QueryDataE::MutQueryData: 'static,
    QueryDataE::Filter: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = ParamSet<
        'static,
        'static,
        (
            Query<'static, 'static, (Entity, QueryDataIn), (QueryDataE::Filter, Filter)>,
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
            <Vec<E> as Effect>::MutParam,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let EffectOut {
            effect: effects,
            out: query_data_effects,
        }: EffectOut<Vec<E>, Vec<(Entity, QueryDataE)>> = param
            .p0()
            .iter()
            .map(|(entity, data_in)| {
                let EffectOut {
                    effect,
                    out: query_data_effect,
                } = (self.f)(data_in);
                effect_out(effect, (entity, query_data_effect))
            })
            .collect();

        query_data_effects
            .into_iter()
            .for_each(|(entity, query_data_effect)| {
                query_data_effect.affect(&mut param.p1().get_mut(entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));
            });

        effects.affect(&mut param.p2())
    }
}
