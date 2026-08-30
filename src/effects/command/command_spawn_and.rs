use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that queues a command for spawning an entity with the provided `Bundle`.
///
/// See [`CommandSpawnAnd`] if you need to produce an extra effect with the spawned `Entity` id.
///
/// Can be constructed with [`command_spawn`].
///
/// # Example
/// In this example, a system is written that spawns an `Enemy`.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// struct Enemy;
///
/// /// Pure system using effects.
/// fn spawn_enemy_pure() -> CommandSpawn<Enemy> {
///     command_spawn(Enemy)
/// }
///
/// /// Equivalent impure system.
/// fn spawn_enemy_impure(mut commands: Commands) {
///     commands.spawn(Enemy);
/// }
/// #
/// # fn app_setup() -> App {
/// #     App::new()
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Enemy>)> {
/// #     let mut query = world.query::<(Entity, Option<&Enemy>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # fn main() {
/// #     let mut pure_app = app_setup();
/// #     pure_app.add_systems(Update, spawn_enemy_pure.pipe(affect));
/// #
/// #     let mut impure_app = app_setup();
/// #     impure_app.add_systems(Update, spawn_enemy_impure);
/// #
/// #     for _ in 0..32 {
/// #         assert_eq!(
/// #             test_state(pure_app.world_mut()),
/// #             test_state(impure_app.world_mut())
/// #         );
/// #         pure_app.update();
/// #         impure_app.update();
/// #     }
/// # }
/// ```
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandSpawn<B>
where
    B: Bundle,
{
    /// The bundle to spawn.
    pub bundle: B,
}

/// Construct a new [`CommandSpawn`] [`Effect`], without an extra effect.
pub fn command_spawn<B>(bundle: B) -> CommandSpawn<B>
where
    B: Bundle,
{
    CommandSpawn { bundle }
}

impl<B> Effect for CommandSpawn<B>
where
    B: Bundle,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.spawn(self.bundle);
    }
}

/// [`Effect`] that queues a command for spawning an entity with the provided `Bundle`, then
/// supplies the entity id to the provided effect-producing function to cause another effect.
///
/// See [`CommandSpawn`] if you do not need to produce an extra effect.
///
/// Can be constructed with [`command_spawn_and`].
///
/// # Example
/// In this example, a system is written that spawns a `Player`, and a `Sword` as a child of the
/// player.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// struct Player;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// struct Sword;
///
/// /// Pure system using effects.
/// fn spawn_armed_player_pure() -> CommandSpawnAnd<Player, CommandSpawn<(Sword, ChildOf)>> {
///     command_spawn_and(Player, |player_entity| {
///         command_spawn((Sword, ChildOf(player_entity)))
///     })
/// }
///
/// /// Equivalent impure system.
/// fn spawn_armed_player_impure(mut commands: Commands) {
///     commands.spawn(Player).with_children(|parent| {
///         parent.spawn(Sword);
///     });
/// }
/// #
/// # fn app_setup() -> App {
/// #     App::new()
/// # }
/// #
/// # fn test_state(
/// #     world: &mut World,
/// # ) -> Vec<(Entity, Option<&Player>, Option<&Sword>, Option<&ChildOf>)> {
/// #     let mut query =
/// #         world.query::<(Entity, Option<&Player>, Option<&Sword>, Option<&ChildOf>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # fn main() {
/// #     let mut pure_app = app_setup();
/// #     pure_app.add_systems(Update, spawn_armed_player_pure.pipe(affect));
/// #
/// #     let mut impure_app = app_setup();
/// #     impure_app.add_systems(Update, spawn_armed_player_impure);
/// #
/// #     for _ in 0..32 {
/// #         assert_eq!(
/// #             test_state(pure_app.world_mut()),
/// #             test_state(impure_app.world_mut())
/// #         );
/// #         pure_app.update();
/// #         impure_app.update();
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - In this example, [`CommandSpawn`] is used as the additional [`Effect`], but any other effect
/// could be produced.
#[doc = include_str!("../defer_command_note.md")]
#[derive(derive_more::Debug)]
pub struct CommandSpawnAnd<B, E>
where
    B: Bundle,
    E: Effect,
{
    /// The bundle to spawn.
    pub bundle: B,
    /// The `Entity -> Effect` function that may cause another effect.
    #[debug("Entity -> {}", std::any::type_name::<E>())]
    pub f: Box<dyn FnOnce(Entity) -> E>,
}

/// Construct a new [`CommandSpawnAnd`] [`Effect`], with an extra effect using the `Entity`.
pub fn command_spawn_and<B, F, E>(bundle: B, f: F) -> CommandSpawnAnd<B, E>
where
    B: Bundle,
    F: FnOnce(Entity) -> E + 'static,
    E: Effect,
{
    CommandSpawnAnd {
        bundle,
        f: Box::new(f),
    }
}

impl<B, E> Default for CommandSpawnAnd<B, E>
where
    B: Bundle + Default,
    E: Effect + Default,
{
    fn default() -> Self {
        CommandSpawnAnd {
            bundle: default(),
            f: Box::new(|_| default()),
        }
    }
}

impl<B, E> Effect for CommandSpawnAnd<B, E>
where
    B: Bundle,
    E: Effect,
{
    type MutParam = (Commands<'static, 'static>, E::MutParam);

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let entity = param.0.spawn(self.bundle).id();

        (self.f)(entity).affect(&mut param.1);
    }
}
