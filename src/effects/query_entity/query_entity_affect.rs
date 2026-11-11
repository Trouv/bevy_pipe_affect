use std::marker::PhantomData;

use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use crate::Effect;
use crate::query_data_effect::QueryDataEffect;

/// [`Effect`] that applies the given [`QueryDataEffect`] to the given entity.
///
/// Produces an error (handled by `bevy`'s `FallbackErrorHandler`) if the entity does not exist in
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
/// # use bevy::ecs::error::{ignore, FallbackErrorHandler};
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(entity_table: Vec<Option<Defense>>, top_player_index: usize) -> App {
/// #     let mut app = App::new();
/// #
/// #     app.insert_resource(FallbackErrorHandler(ignore));
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
///
/// Not shown...
/// - Other [`QueryDataEffect`]s are available
/// - a filter can be applied using the `Filter` generic parameter.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
    QueryDataE: QueryDataEffect,
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

        self.query_data_effect.affect(&mut query_data);
    }
}
