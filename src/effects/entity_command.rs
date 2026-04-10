//! [`Effect`]s that queue entity-specific `Commands`.
use std::marker::PhantomData;

use bevy::ecs::error::CommandWithEntity;
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
/// use bevy::ecs::error::CommandWithEntity;
/// use bevy::ecs::system::entity_command::retain;
/// use bevy::ecs::world::error::EntityMutableFetchError;
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Component)]
/// struct Player;
///
/// fn reset_player_pure(
///     player: Single<Entity, With<Player>>,
/// ) -> EntityCommandQueue<
///     impl EntityCommand<()> + CommandWithEntity<Result<(), EntityMutableFetchError>> + use<>,
///     (),
///     Result<(), EntityMutableFetchError>,
/// > {
///     entity_command_queue(*player, retain::<Player>())
/// }
///
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
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityCommandQueue<C, T, M>
where
    C: EntityCommand<T> + CommandWithEntity<M>,
{
    entity: Entity,
    command: C,
    entity_command_out: PhantomData<T>,
    command_with_entity_out: PhantomData<M>,
}

impl<C, T, M> EntityCommandQueue<C, T, M>
where
    C: EntityCommand<T> + CommandWithEntity<M>,
{
    /// Construct a new [`EntityCommandQueue`]
    pub fn new(entity: Entity, command: C) -> Self {
        EntityCommandQueue {
            entity,
            command,
            entity_command_out: PhantomData,
            command_with_entity_out: PhantomData,
        }
    }
}

/// Construct a new [`EntityCommandQueue`] [`Effect`].
pub fn entity_command_queue<C, T, M>(entity: Entity, command: C) -> EntityCommandQueue<C, T, M>
where
    C: EntityCommand<T> + CommandWithEntity<M>,
{
    EntityCommandQueue::new(entity, command)
}

impl<C, T, M> Effect for EntityCommandQueue<C, T, M>
where
    C: EntityCommand<T> + CommandWithEntity<M>,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).queue(self.command);
    }
}

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
/// be a `Bundle` struct or tuple of components
#[doc = include_str!("defer_command_note.md")]
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
/// a `Bundle` struct or tuple of components
#[doc = include_str!("defer_command_note.md")]
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

/// [`Effect`] that queues a command for despawning an `Entity`.
///
/// Can be constructed with [`entity_command_despawn`].
#[doc = include_str!("defer_command_note.md")]
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

/// [`Effect`] that inserts a component/bundle recursively on an entity and its relationships.
///
/// Can be constructed with [`entity_command_insert_recursive`].
#[doc = include_str!("defer_command_note.md")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct EntityCommandInsertRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle + Clone,
{
    /// The entity that is inserted to recursively.
    pub entity: Entity,
    /// The bundle being inserted on relationship targets.
    pub bundle: B,
    relationship_target: PhantomData<RT>,
}

impl<RT, B> EntityCommandInsertRecursive<RT, B>
where
    B: Bundle + Clone,
    RT: RelationshipTarget,
{
    /// Construct a new [`EntityCommandInsertRecursive`].
    pub fn new(entity: Entity, bundle: B) -> Self {
        Self {
            entity,
            bundle,
            relationship_target: PhantomData,
        }
    }
}

/// Construct a new [`EntityCommandInsertRecursive`] [`Effect`].
pub fn entity_command_insert_recursive<RT, B>(
    entity: Entity,
    bundle: B,
) -> EntityCommandInsertRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle + Clone,
{
    EntityCommandInsertRecursive::new(entity, bundle)
}

impl<RT, B> Effect for EntityCommandInsertRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle + Clone,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param
            .entity(self.entity)
            .insert_recursive::<RT>(self.bundle);
    }
}

/// [`Effect`] that removes a component/bundle recursively from an entity and its relationships.
///
/// Can be constructed with [`entity_command_remove_recursive`].
#[doc = include_str!("defer_command_note.md")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct EntityCommandRemoveRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle,
{
    /// The entity that the bundle is removed from recursively.
    pub entity: Entity,
    bundle: PhantomData<B>,
    relationship_target: PhantomData<RT>,
}

impl<RT, B> EntityCommandRemoveRecursive<RT, B>
where
    B: Bundle,
    RT: RelationshipTarget,
{
    /// Construct a new [`EntityCommandRemoveRecursive`].
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            bundle: PhantomData,
            relationship_target: PhantomData,
        }
    }
}

/// Construct a new [`EntityCommandRemoveRecursive`] [`Effect`].
pub fn entity_command_remove_recursive<RT, B>(entity: Entity) -> EntityCommandRemoveRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle,
{
    EntityCommandRemoveRecursive::new(entity)
}

impl<RT, B> Effect for EntityCommandRemoveRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).remove_recursive::<RT, B>();
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::command::{command_insert_resource, command_spawn_and};
    use crate::effects::number_data::NumberComponent;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn entity_command_queue_can_insert_component_non_exclusively(component in any::<NumberComponent<0>>()) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(()).id();

            let actual_component = app.world().get_entity(entity).unwrap().get_components::<&NumberComponent<0>>();

            assert!(actual_component.is_err());

            let insert_component_system = move || {
                EntityCommandQueue::new(entity, move |mut entity_world: EntityWorldMut<'_>| {
                    entity_world.insert(component.clone());
                })
            };

            assert!(!IntoSystem::into_system(insert_component_system.pipe(affect)).is_exclusive());

            app.add_systems(
                Update,
                insert_component_system.pipe(affect),
            );

            app.update();

            let actual_component = app.world().get_entity(entity).unwrap().get_components::<&NumberComponent<0>>().unwrap();

            assert_eq!(actual_component, &component);
        }

        #[test]
        fn bundle_commands_correctly_insert_and_remove(component_0 in any::<NumberComponent<0>>(), component_1 in any::<NumberComponent<1>>()) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(()).id();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert!(actual_component_0.is_none());
            assert!(actual_component_1.is_none());

            #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
            struct InsertSystem;

            app.add_systems(
                Update,
                (move || entity_command_insert(entity,  (component_0, component_1))).pipe(affect).in_set(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert_eq!(actual_component_0, Some(&component_0));
            assert_eq!(actual_component_1, Some(&component_1));

            app.add_systems(
                Update,
                (move || entity_command_remove::<(NumberComponent<1>, NumberComponent<2>)>(entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert_eq!(actual_component_0, Some(&component_0));
            assert!(actual_component_1.is_none());

            app.add_systems(
                Update,
                (move || entity_command_remove::<(NumberComponent<0>, NumberComponent<1>)>(entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert!(actual_component_0.is_none());
            assert!(actual_component_1.is_none());
        }

        #[test]
        fn bundle_recursively_commands_correctly_insert_and_remove(component_0 in any::<NumberComponent<0>>(), component_1 in any::<NumberComponent<1>>()) {
            let mut app = App::new();

            let parent_entity = app.world_mut().spawn(()).id();
            let child_entity_a = app.world_mut().spawn(ChildOf(parent_entity)).id();
            let child_entity_b = app.world_mut().spawn(ChildOf(parent_entity)).id();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), None);

            #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
            struct InsertSystem;

            app.add_systems(
                Update,
                (move || entity_command_insert_recursive::<Children, _>(parent_entity,  (component_0, component_1))).pipe(affect).in_set(InsertSystem),
            );

            app.update();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), Some(&component_1));

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), Some(&component_1));

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), Some(&component_1));

            app.add_systems(
                Update,
                (move || entity_command_remove_recursive::<Children, (NumberComponent<1>, NumberComponent<2>)>(parent_entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), None);

            app.add_systems(
                Update,
                (move || entity_command_remove_recursive::<Children, (NumberComponent<0>, NumberComponent<1>)>(parent_entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), None);
        }
    }

    #[test]
    fn commands_can_spawn_and_despawn_entities() {
        #[derive(Resource, Clone)]
        struct EntityHolder(Entity);

        let mut app = App::new();

        app.add_systems(
            Update,
            (move || command_spawn_and((), |entity| command_insert_resource(EntityHolder(entity))))
                .pipe(affect)
                .run_if(not(resource_exists::<EntityHolder>)),
        );

        app.update();

        let EntityHolder(entity) = app.world().resource::<EntityHolder>().clone();

        assert!(app.world().get_entity(entity).is_ok());

        app.add_systems(
            Update,
            (move |entity_holder: Res<EntityHolder>| entity_command_despawn(entity_holder.0))
                .pipe(affect),
        );

        app.update();

        assert!(app.world().get_entity(entity).is_err());
    }
}
