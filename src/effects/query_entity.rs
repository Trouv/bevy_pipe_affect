use std::marker::PhantomData;

use bevy::ecs::query::{QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;

use crate::query_data_effect::QueryDataEffect;
use crate::{Effect, EffectOut};

/// [`Effect`] that applies the given [`QueryDataEffect`] to the given entity.
///
/// Produces an error (handled by `bevy`'s `DefaultErrorHandler`) if the entity does not exist in
/// the [`QueryDataEffect::Filter`] (and the optional `Filter` generic).
///
/// Can be constructed by [`query_entity_affect`].
///
/// # Example
/// In this example, a system is written that sets all `ImmovableObject` entities' parent's `Speed`
/// to 0.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct ImmovableObject;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Speed(f32);
///
/// /// Pure system using effects.
/// fn stop_immovable_carriers_pure(
///     query: Query<&ChildOf, With<ImmovableObject>>,
/// ) -> Vec<QueryEntityAffect<ComponentSet<Speed>>> {
///     query
///         .iter()
///         .map(|child_of| query_entity_affect(child_of.parent(), component_set(Speed(0.0))))
///         .collect()
/// }
///
/// /// Equivalent impure system (apart from error handling).
/// fn stop_immovable_carriers_impure(
///     immovable_query: Query<&ChildOf, With<ImmovableObject>>,
///     mut speed_query: Query<&mut Speed>,
/// ) {
///     for child_of in immovable_query.iter() {
///         if let Ok(mut speed) = speed_query.get_mut(child_of.parent()) {
///             speed.0 = 0.0;
///         }
///     }
/// }
/// #
/// # use bevy::ecs::error::{DefaultErrorHandler, ignore};
/// # use proptest::prelude::*;
/// # #[derive(Debug, Copy, Clone, proptest_derive::Arbitrary)]
/// # struct ChildOfIndex(usize);
/// #
/// # fn app_setup(
/// #     entity_table: Vec<(Option<Speed>, Option<ImmovableObject>, Option<ChildOfIndex>)>,
/// # ) -> App {
/// #     let mut app = App::new();
/// #
/// #     app.insert_resource(DefaultErrorHandler(ignore));
/// #
/// #     entity_table.into_iter().fold(
/// #         vec![app.world_mut().spawn_empty().id()],
/// #         |mut entities, (speed, immovable, child_of)| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #
/// #             if let Some(speed) = speed {
/// #                 entity.insert(speed);
/// #             }
/// #             if let Some(immovable) = immovable {
/// #                 entity.insert(immovable);
/// #             }
/// #
/// #             if let Some(ChildOfIndex(index)) = child_of {
/// #                 entity.insert(ChildOf(entities[index % entities.len()]));
/// #             }
/// #
/// #             entities.push(entity.id());
/// #             entities
/// #         },
/// #     );
/// #
/// #     app
/// # }
/// #
/// # fn query_state(
/// #     world: &mut World,
/// # ) -> Vec<(
/// #     Entity,
/// #     Option<&Speed>,
/// #     Option<&ImmovableObject>,
/// #     Option<&ChildOf>,
/// # )> {
/// #     let mut query = world.query::<(
/// #         Entity,
/// #         Option<&Speed>,
/// #         Option<&ImmovableObject>,
/// #         Option<&ChildOf>,
/// #     )>();
/// #     query.iter(world).collect()
/// # }
/// # proptest! {
/// #     fn main(entity_table: Vec<(Option<Speed>, Option<ImmovableObject>, Option<ChildOfIndex>)>) {
/// #         let mut pure_app = app_setup(entity_table.clone());
/// #         pure_app.add_systems(Update, stop_immovable_carriers_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(entity_table.clone());
/// #         impure_app.add_systems(Update, stop_immovable_carriers_impure);
/// #
/// #         for _ in 0..3 {
/// #             assert_eq!(query_state(pure_app.world_mut()), query_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
pub struct QueryEntityAffect<QueryDataE, Filter = ()>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// The entity to apply the [`QueryDataEffect`] to.
    pub entity: Entity,
    /// The [`QueryDataEffect`] to apply to the entity.
    pub query_data_effect: QueryDataE,
    filter: PhantomData<Filter>,
}

impl<QueryDataE, Filter> QueryEntityAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryEntityAffect`] [`Effect`].
    pub fn new(entity: Entity, query_data_effect: QueryDataE) -> Self {
        QueryEntityAffect {
            entity,
            query_data_effect,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`QueryEntityAffect`] [`Effect`].
pub fn query_entity_affect<QueryDataE, Filter>(
    entity: Entity,
    query_data_effect: QueryDataE,
) -> QueryEntityAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    QueryEntityAffect::new(entity, query_data_effect)
}

impl<QueryDataE, Filter> Effect for QueryEntityAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect + Clone,
    QueryDataE::MutQueryData: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = (
        Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
        <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam,
    );

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let mut query_data = match param.0.get_mut(self.entity) {
            Ok(d) => d,
            Err(e) => {
                Err::<(), _>(e).affect(&mut param.1);
                return;
            }
        };

        self.query_data_effect.clone().affect(&mut query_data);
    }
}

pub struct QueryEntityMap<QueryDataIn, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub entity: Entity,
    pub f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE>,
    filter: PhantomData<Filter>,
}

impl<QueryDataIn, QueryDataE, Filter> QueryEntityMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryEntityMap`] [`Effect`].
    pub fn new(
        entity: Entity,
        f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE>,
    ) -> Self {
        QueryEntityMap {
            entity,
            f,
            filter: PhantomData,
        }
    }
}

pub fn query_entity_map<QueryDataIn, QueryDataE, Filter, F>(
    entity: Entity,
    f: F,
) -> QueryEntityMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE + 'static,
{
    QueryEntityMap::new(entity, Box::new(f))
}

impl<QueryDataIn, QueryDataE, Filter> Effect for QueryEntityMap<QueryDataIn, QueryDataE, Filter>
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
            (
                Query<'static, 'static, QueryDataIn, (QueryDataE::Filter, Filter)>,
                <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam,
            ),
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (input_query_param, mut result_param) = param.p0();

        let input = match input_query_param.get(self.entity) {
            Ok(d) => d,
            Err(e) => {
                Err::<(), _>(e).affect(&mut result_param);
                return;
            }
        };

        let query_data_effect = (self.f)(input);

        query_data_effect.affect(&mut param.p1().get_mut(self.entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));
    }
}

pub struct QueryEntityMapAnd<QueryDataIn, E, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub entity: Entity,
    pub f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE>>,
    filter: PhantomData<Filter>,
}

impl<QueryDataIn, E, QueryDataE, Filter> QueryEntityMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryEntityMapAnd`] [`Effect`].
    pub fn new(
        entity: Entity,
        f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE>>,
    ) -> Self {
        QueryEntityMapAnd {
            entity,
            f,
            filter: PhantomData,
        }
    }
}

pub fn query_entity_map_and<QueryDataIn, E, QueryDataE, Filter, F>(
    entity: Entity,
    f: F,
) -> QueryEntityMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE> + 'static,
{
    QueryEntityMapAnd::new(entity, Box::new(f))
}

impl<QueryDataIn, E, QueryDataE, Filter> Effect
    for QueryEntityMapAnd<QueryDataIn, E, QueryDataE, Filter>
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
            (
                Query<'static, 'static, QueryDataIn, (QueryDataE::Filter, Filter)>,
                <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam,
            ),
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
            <E as Effect>::MutParam,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (input_query_param, mut result_param) = param.p0();

        let input = match input_query_param.get(self.entity) {
            Ok(d) => d,
            Err(e) => {
                Err::<(), _>(e).affect(&mut result_param);
                return;
            }
        };

        let EffectOut {
            effect,
            out: query_data_effect,
        } = (self.f)(input);

        query_data_effect.affect(&mut param.p1().get_mut(self.entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));

        effect.affect(&mut param.p2())
    }
}
