use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that queues a command for despawning an `Entity`.
///
/// Can be constructed with [`entity_command_despawn`].
///
/// # Example
/// In this example, a system is written that despawns entites with 0 `Health`.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Health {
///     # #[proptest(strategy = "0..3 as u32")]
///     value: u32,
/// }
///
/// /// Pure system using effects.
/// fn death_pure(query: Query<(Entity, &Health)>) -> Vec<EntityCommandDespawn> {
///     query
///         .iter()
///         .filter(|(_, health)| health.value == 0)
///         .map(|(entity, _)| entity_command_despawn(entity))
///         .collect()
/// }
///
/// /// Equivalent impure system.
/// fn death_impure(query: Query<(Entity, &Health)>, mut commands: Commands) {
///     for (entity, health) in query.iter() {
///         if health.value == 0 {
///             commands.entity(entity).despawn();
///         }
///     }
/// }
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(component_table: Vec<Option<Health>>) -> App {
/// #     let mut app = App::new();
/// #
/// #     component_table
/// #         .into_iter()
/// #         .for_each(|health| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #
/// #             if let Some(health) = health {
/// #                 entity.insert(health);
/// #             }
/// #         });
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Health>)> {
/// #     let mut query = world.query::<(Entity, Option<&Health>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<Option<Health>>) {
/// #         let mut pure_app = app_setup(component_table.clone());
/// #         pure_app.add_systems(Update, death_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table);
/// #         impure_app.add_systems(Update, death_impure);
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
pub struct EntityCommandDespawn {
    /// The entity to despawn.
    pub entity: Entity,
}

/// Construct a new [`EntityCommandDespawn`] [`Effect`].
pub fn entity_command_despawn(entity: Entity) -> EntityCommandDespawn {
    EntityCommandDespawn { entity }
}

impl Effect for EntityCommandDespawn {
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).despawn();
    }
}
