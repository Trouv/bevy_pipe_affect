use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that pushes a generic entity command to the command queue.
///
/// This effect is mostly intended to "fill the gaps" of `bevy_pipe_affect`, as not all entity
/// commands or entity world mutations provided by bevy have an equivalent effect.
/// Using this typically leads to some excessive generics, and often leads to writing some
/// impure code anyway, so users should also consider writing their own custom [`Effect`] instead.
/// (Then, if it's general-purpose enough, consider contributing it upstream!)
///
/// Can be constructed with [`entity_command_queue`].
///
/// # Example
/// In this example, a system is written that removes all components from the `Player` entity
/// except for the `Player` component.
/// ```rust
/// use bevy::ecs::system::entity_command::retain;
/// use bevy::ecs::world::error::EntityMutableFetchError;
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Component)]
/// struct Player;
///
/// /// Pure system using effects.
/// fn reset_player_pure(
///     player: Single<Entity, With<Player>>,
/// ) -> EntityCommandQueue<impl EntityCommand + use<>> {
///     entity_command_queue(*player, retain::<Player>())
/// }
///
/// /// Equivalent impure system.
/// fn reset_player_impure(player: Single<Entity, With<Player>>, mut commands: Commands) {
///     commands.entity(*player).retain::<Player>();
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component, proptest_derive::Arbitrary)]
/// # struct A(u8);
/// #
/// # #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component, proptest_derive::Arbitrary)]
/// # struct B(u8);
/// #
/// # fn app_setup(component_table: Vec<(Option<A>, Option<B>)>, player_index: usize) -> App {
/// #     let mut app = App::new();
/// #
/// #     let once_entity = app.world_mut().spawn_empty().id();
/// #
/// #     let entities = component_table
/// #         .into_iter()
/// #         .map(|(a, b)| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #             if let Some(a) = a {
/// #                 entity.insert(a);
/// #             }
/// #
/// #             if let Some(b) = b {
/// #                 entity.insert(b);
/// #             }
/// #
/// #             entity.id()
/// #         })
/// #         .chain(std::iter::once(once_entity))
/// #         .collect::<Vec<_>>();
/// #
/// #     let player = entities[player_index % entities.len()];
/// #
/// #     app.world_mut().entity_mut(player).insert(Player);
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&A>, Option<&B>, Option<&Player>)> {
/// #     let mut query = world.query::<(Entity, Option<&A>, Option<&B>, Option<&Player>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<(Option<A>, Option<B>)>, player_index: usize) {
/// #         let mut pure_app = app_setup(component_table.clone(), player_index);
/// #         pure_app.add_systems(Update, reset_player_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table.clone(), player_index);
/// #         impure_app.add_systems(Update, reset_player_impure);
/// #
/// #         for _ in 0..3 {
/// #             prop_assert_eq!(test_state(pure_app.world_mut()), test_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityCommandQueue<C>
where
    C: EntityCommand,
{
    entity: Entity,
    command: C,
}

impl<C> EntityCommandQueue<C>
where
    C: EntityCommand,
{
    /// Construct a new [`EntityCommandQueue`]
    pub fn new(entity: Entity, command: C) -> Self {
        EntityCommandQueue { entity, command }
    }
}

/// Construct a new [`EntityCommandQueue`] [`Effect`].
pub fn entity_command_queue<C>(entity: Entity, command: C) -> EntityCommandQueue<C>
where
    C: EntityCommand,
{
    EntityCommandQueue::new(entity, command)
}

impl<C> Effect for EntityCommandQueue<C>
where
    C: EntityCommand,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).queue(self.command);
    }
}
