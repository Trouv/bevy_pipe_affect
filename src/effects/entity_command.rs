use std::marker::PhantomData;

use bevy::ecs::error::CommandWithEntity;
use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that pushes a generic entity command to the command queue.
///
/// Can be constructed with [`entity_command_queue`].
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
    use crate::effects::number_data::NumberComponent;
    use crate::effects::{command_insert_resource, command_spawn_and};
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
