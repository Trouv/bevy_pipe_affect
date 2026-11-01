use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that queues a command for removing the `Bundle` from the `Entity`.
///
/// Can be constructed with [`entity_command_remove`].
///
/// # Example
/// In this example, a system is written that nerfs the `TopPlayer` by removing her `Shield`.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Resource)]
/// struct TopPlayer(Entity);
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Shield;
///
/// /// Pure system using effects.
/// fn nerf_top_player_pure(top_player: Res<TopPlayer>) -> EntityCommandRemove<Shield> {
///     entity_command_remove::<Shield>(top_player.0)
/// }
///
/// /// Equivalent impure system.
/// fn nerf_top_player_impure(top_player: Res<TopPlayer>, mut commands: Commands) {
///     commands.entity(top_player.0).remove::<Shield>();
/// }
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(component_table: Vec<Option<Shield>>, top_player_index: usize) -> App {
/// #     let mut app = App::new();
/// #
/// #     let once_entity = app.world_mut().spawn_empty().id();
/// #
/// #     let entities = component_table
/// #         .into_iter()
/// #         .map(|shield| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #             if let Some(shield) = shield {
/// #                 entity.insert(shield);
/// #             }
/// #
/// #             entity.id()
/// #         })
/// #         .chain(std::iter::once(once_entity))
/// #         .collect::<Vec<_>>();
/// #
/// #     let top_player = entities[top_player_index % entities.len()];
/// #
/// #     app.world_mut().insert_resource(TopPlayer(top_player));
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Shield>)> {
/// #     let mut query = world.query::<(Entity, Option<&Shield>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<Option<Shield>>, player_index: usize) {
/// #         let mut pure_app = app_setup(component_table.clone(), player_index);
/// #         pure_app.add_systems(Update, nerf_top_player_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table, player_index);
/// #         impure_app.add_systems(Update, nerf_top_player_impure);
/// #
/// #         for _ in 0..3 {
/// #             prop_assert_eq!(test_state(pure_app.world_mut()), test_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - A single component is used in this example, but the removed type is a `Bundle`, so it can be
/// a `Bundle` struct or tuple of components.
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityCommandRemove<B>
where
    B: Bundle,
{
    entity: Entity,
    bundle: PhantomData<B>,
}

impl<B> EntityCommandRemove<B>
where
    B: Bundle,
{
    /// Construct a new [`EntityCommandRemove`]
    pub fn new(entity: Entity) -> Self {
        EntityCommandRemove {
            entity,
            bundle: PhantomData,
        }
    }
}

/// Construct a new [`EntityCommandRemove`] [`Effect`].
pub fn entity_command_remove<B>(entity: Entity) -> EntityCommandRemove<B>
where
    B: Bundle,
{
    EntityCommandRemove::new(entity)
}

impl<B> Effect for EntityCommandRemove<B>
where
    B: Bundle,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).remove::<B>();
    }
}
