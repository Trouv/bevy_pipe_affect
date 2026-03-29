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
/// In this example, a system is written that nerfs the `TopPlayer` by setting her `Defense` to 0.5.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Resource)]
/// struct TopPlayer(Entity);
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Defense(f32);
///
/// /// Pure system using effects.
/// fn nerf_top_player_pure(
///     top_player: Res<TopPlayer>
/// )  -> QueryEntityAffect<ComponentSet<Defense>> {
///     query_entity_affect(top_player.0, component_set(Defense(0.5)))
/// }
///
/// /// Equivalent impure system.
/// fn nerf_top_player_impure(
///     top_player: Res<TopPlayer>,
///     mut query: Query<&mut Defense>,
/// ) -> Result<(), BevyError> {
///     query.get_mut(top_player.0)?.0 = 0.5;
///     Ok(())
/// }
/// # use bevy::ecs::error::{ignore, DefaultErrorHandler};
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(entity_table: Vec<Option<Defense>>, top_player_index: usize) -> App {
/// #     let mut app = App::new();
/// #
/// #     app.insert_resource(DefaultErrorHandler(ignore));
/// #
/// #     let entities = entity_table.into_iter().fold(
/// #         vec![app.world_mut().spawn_empty().id()],
/// #         |mut entities, defense| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #
/// #             if let Some(defense) = defense {
/// #                 entity.insert(defense);
/// #             }
/// #
/// #             entities.push(entity.id());
/// #             entities
/// #         },
/// #     );
/// #
/// #     app.insert_resource(TopPlayer(entities[top_player_index % entities.len()]));
/// #
/// #     app
/// # }
/// #
/// # fn query_state(world: &mut World) -> Vec<(Entity, Option<&Defense>)> {
/// #     let mut query = world.query::<(Entity, Option<&Defense>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(entity_table: Vec<Option<Defense>>, top_player_index: usize) {
/// #         let mut pure_app = app_setup(entity_table.clone(), top_player_index);
/// #         pure_app.add_systems(Update, nerf_top_player_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(entity_table.clone(), top_player_index);
/// #         impure_app.add_systems(Update, nerf_top_player_impure);
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
