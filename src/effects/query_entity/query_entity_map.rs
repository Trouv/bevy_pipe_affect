use std::marker::PhantomData;

use bevy::ecs::query::{QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;

use crate::Effect;
use crate::effects::query::BoxedQueryMapFn;
use crate::query_data_effect::QueryDataEffect;

/// [`Effect`] that applies the given mapping of `QueryData` to [`QueryDataEffect`] to the given
/// entity, and applies the [`QueryDataEffect`].
///
/// Produces an error (handled by `bevy`'s `FallbackErrorHandler`) if the entity isn't selected by
/// `QueryDataIn`'s or `QueryDataE`'s filters or the optional `Filter` generic.
///
/// Can be constructed by [`query_entity_map`].
///
/// # Example
/// In this example, a system is written that buffs the `BottomPlayer` by making their
/// `AttackMultiplier` inversely proportional to their `Health`.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Resource)]
/// struct BottomPlayer(Entity);
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Health(f32);
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct AttackMultiplier(f32);
///
/// /// Pure system using effects.
/// fn buff_bottom_player_pure(
///     bottom_player: Res<BottomPlayer>,
/// ) -> QueryEntityMap<&'static Health, ComponentSet<AttackMultiplier>> {
///     query_entity_map(bottom_player.0, |health: &Health| {
///         component_set(AttackMultiplier(1.0 + ((100.0 - health.0) / 100.0)))
///     })
/// }
///
/// /// Equivalent impure system.
/// fn buff_bottom_player_impure(
///     bottom_player: Res<BottomPlayer>,
///     mut query: Query<(&Health, &mut AttackMultiplier)>,
/// ) -> Result<(), BevyError> {
///     let (health, mut attack_multiplier) = query.get_mut(bottom_player.0)?;
///     attack_multiplier.0 = 1.0 + ((100.0 - health.0) / 100.0);
///     Ok(())
/// }
/// # use bevy::ecs::error::{ignore, FallbackErrorHandler};
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(entity_table: Vec<(Option<Health>, Option<AttackMultiplier>)>, bottom_player_index: usize) -> App {
/// #     let mut app = App::new();
/// #
/// #     app.insert_resource(FallbackErrorHandler(ignore));
/// #
/// #     let entities = entity_table.into_iter().fold(
/// #         vec![app.world_mut().spawn_empty().id()],
/// #         |mut entities, (health, attack_multiplier)| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #
/// #             if let Some(health) = health {
/// #                 entity.insert(health);
/// #             }
/// #             if let Some(attack_multiplier) = attack_multiplier {
/// #                 entity.insert(attack_multiplier);
/// #             }
/// #
/// #             entities.push(entity.id());
/// #             entities
/// #         },
/// #     );
/// #
/// #     app.insert_resource(BottomPlayer(entities[bottom_player_index % entities.len()]));
/// #
/// #     app
/// # }
/// #
/// # fn query_state(world: &mut World) -> Vec<(Entity, Option<&Health>, Option<&AttackMultiplier>)> {
/// #     let mut query = world.query::<(Entity, Option<&Health>, Option<&AttackMultiplier>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(entity_table: Vec<(Option<Health>, Option<AttackMultiplier>)>, bottom_player_index: usize) {
/// #         let mut pure_app = app_setup(entity_table.clone(), bottom_player_index);
/// #         pure_app.add_systems(Update, buff_bottom_player_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(entity_table.clone(), bottom_player_index);
/// #         impure_app.add_systems(Update, buff_bottom_player_impure);
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
/// - any `QueryData` (including component tuples), can be input to the map function
/// - a filter can be applied using the `Filter` generic parameter.
#[derive(derive_more::Debug)]
pub struct QueryEntityMap<QueryDataIn, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// The entity that the mapping is applied to.
    pub entity: Entity,
    /// The `QueryData -> QueryDataEffect` function that is applied to the entity.
    #[debug("{0} -> {1}", std::any::type_name::<QueryDataIn>(), std::any::type_name::<QueryDataE>())]
    pub f: BoxedQueryMapFn<QueryDataIn, QueryDataE>,
    filter: PhantomData<Filter>,
}

impl<QueryDataIn, QueryDataE, Filter> QueryEntityMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryEntityMap`] [`Effect`].
    pub fn new(entity: Entity, f: BoxedQueryMapFn<QueryDataIn, QueryDataE>) -> Self {
        QueryEntityMap {
            entity,
            f,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`QueryEntityMap`] [`Effect`].
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
