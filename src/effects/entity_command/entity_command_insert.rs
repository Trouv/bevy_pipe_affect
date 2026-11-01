use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that queues a command for inserting the provided `Bundle` onto the `Entity`.
///
/// Can be constructed with [`entity_command_insert`].
///
/// # Example
/// In this example, a system is written that gives the `TopPlayer` a `Crown`.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Copy, Clone, PartialEq, Eq, Resource)]
/// struct TopPlayer(Entity);
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// struct Crown;
///
/// /// Pure system using effects.
/// fn crown_top_player_pure(top_player: Res<TopPlayer>) -> EntityCommandInsert<Crown> {
///     entity_command_insert(top_player.0, Crown)
/// }
///
/// /// Equivalent impure system.
/// fn crown_top_player_impure(top_player: Res<TopPlayer>, mut commands: Commands) {
///     commands.entity(top_player.0).insert(Crown);
/// }
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(entity_count: u8, top_player_index: usize) -> App {
/// #     let mut app = App::new();
/// #
/// #     let once_entity = app.world_mut().spawn_empty().id();
/// #
/// #     let entities = (0..entity_count)
/// #         .map(|_| app.world_mut().spawn_empty().id())
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
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Crown>)> {
/// #     let mut query = world.query::<(Entity, Option<&Crown>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(entity_count: u8, player_index: usize) {
/// #         let mut pure_app = app_setup(entity_count, player_index);
/// #         pure_app.add_systems(Update, crown_top_player_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(entity_count, player_index);
/// #         impure_app.add_systems(Update, crown_top_player_impure);
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
/// - A single component is used in this example, but the inserted value is a `Bundle`, so it can
/// be a `Bundle` struct or tuple of components.
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityCommandInsert<B>
where
    B: Bundle,
{
    /// The entity to insert to.
    pub entity: Entity,
    /// The bundle to insert.
    pub bundle: B,
}

/// Construct a new [`EntityCommandInsert`] [`Effect`].
pub fn entity_command_insert<B>(entity: Entity, bundle: B) -> EntityCommandInsert<B>
where
    B: Bundle,
{
    EntityCommandInsert { entity, bundle }
}

impl<B> Effect for EntityCommandInsert<B>
where
    B: Bundle,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).insert(self.bundle);
    }
}
