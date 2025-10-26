use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that pushes a generic command to the command queue.
///
/// Can be constructed with [`command_queue`].
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandQueue<C>
where
    C: Command,
{
    /// The command to push onto the queue.
    pub command: C,
}

/// Construct a new [`CommandQueue`] [`Effect`].
pub fn command_queue<C>(command: C) -> CommandQueue<C>
where
    C: Command,
{
    CommandQueue { command }
}

impl<C> Effect for CommandQueue<C>
where
    C: Command,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.queue(self.command)
    }
}

/// [`Effect`] that queues a command for inserting the provided `Resource` in the `World`.
///
/// Can be constucted with [`command_insert_resource`].
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandInsertResource<R>
where
    R: Resource,
{
    /// The initial value of the inserted resource.
    pub resource: R,
}

/// Construct a new [`CommandInsertResource`] [`Effect`].
pub fn command_insert_resource<R>(resource: R) -> CommandInsertResource<R>
where
    R: Resource,
{
    CommandInsertResource { resource }
}

impl<R> Effect for CommandInsertResource<R>
where
    R: Resource,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.insert_resource(self.resource);
    }
}

/// [`Effect`] that queues a command for removing a `Resource` from the `World`.
///
/// Can be constructed with [`command_remove_resource`].
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandRemoveResource<R>
where
    R: Resource,
{
    resource: PhantomData<R>,
}

impl<R> CommandRemoveResource<R>
where
    R: Resource,
{
    /// Construct a new [`CommandRemoveResource`]
    pub fn new() -> Self {
        CommandRemoveResource {
            resource: PhantomData,
        }
    }
}

/// Construct a new [`CommandRemoveResource`] [`Effect`].
pub fn command_remove_resource<R>() -> CommandRemoveResource<R>
where
    R: Resource,
{
    CommandRemoveResource::new()
}

impl<R> Effect for CommandRemoveResource<R>
where
    R: Resource,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.remove_resource::<R>();
    }
}

/// [`Effect`] that queues a command for spawning an entity with the provided `Bundle`, then
/// supplies the entity id to the provided effect-producing function to cause another effect.
///
/// Can be constructed with [`command_spawn`] or [`command_spawn_and`].
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CommandSpawnAnd<B, F, E>
where
    B: Bundle,
    F: FnOnce(Entity) -> E,
    E: Effect,
{
    /// The bundle to spawn.
    pub bundle: B,
    /// The `Entity -> Effect` function that may cause another effect.
    pub f: F,
}

/// Construct a new [`CommandSpawnAnd`] [`Effect`], without an extra effect.
pub fn command_spawn<B>(bundle: B) -> CommandSpawnAnd<B, impl FnOnce(Entity) -> (), ()>
where
    B: Bundle,
{
    command_spawn_and(bundle, |_| ())
}

/// Construct a new [`CommandSpawnAnd`] [`Effect`], with an extra effect using the `Entity`.
pub fn command_spawn_and<B, F, E>(bundle: B, f: F) -> CommandSpawnAnd<B, F, E>
where
    B: Bundle,
    F: FnOnce(Entity) -> E,
    E: Effect,
{
    CommandSpawnAnd { bundle, f }
}

impl<B, F, E> Effect for CommandSpawnAnd<B, F, E>
where
    B: Bundle,
    F: FnOnce(Entity) -> E,
    E: Effect,
{
    type MutParam = (Commands<'static, 'static>, E::MutParam);

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let entity = param.0.spawn(self.bundle).id();

        (self.f)(entity).affect(&mut param.1);
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::{NumberComponent, NumberResource};
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn command_queue_can_spawn_entities_non_exclusively(component in any::<NumberComponent<0>>()) {
            let mut app = App::new();

            let component_count = app.world_mut().query_filtered::<(), With<NumberComponent<0>>>().iter(app.world()).count();

            assert_eq!(component_count, 0);

            let spawn_component_system = move || {
                command_queue(move |world: &mut World| {
                    world.spawn(component.clone());
                })
            };


            assert!(!IntoSystem::into_system(spawn_component_system.pipe(affect)).is_exclusive());

            app.add_systems(
                Update,
                spawn_component_system.pipe(affect),
            );

            app.update();

            let component_count = app.world_mut().query_filtered::<(), With<NumberComponent<0>>>().iter(app.world()).count();

            assert_eq!(component_count, 1);

            app.update();

            let component_count = app.world_mut().query_filtered::<(), With<NumberComponent<0>>>().iter(app.world()).count();

            assert_eq!(component_count, 2);
        }

        #[test]
        fn resource_commands_correctly_insert_and_remove(resource in any::<NumberResource>()) {
            let mut app = App::new();

            assert!(app.world().get_resource::<NumberResource>().is_none());

            #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
            struct InsertSystem;

            app.add_systems(
                Update,
                (move || command_insert_resource(resource)).pipe(affect).in_set(InsertSystem),
            );

            app.update();

            assert_eq!(app.world().get_resource::<NumberResource>(), Some(&resource));

            app.add_systems(
                Update,
                (move || command_remove_resource::<NumberResource>()).pipe(affect).after(InsertSystem),
            );

            app.update();

            assert!(app.world().get_resource::<NumberResource>().is_none());
        }
    }

    #[test]
    fn command_spawn_effect_can_create_parent_child_relationship() {
        let mut app = App::new();

        let children_count = app
            .world_mut()
            .query::<&ChildOf>()
            .iter(app.world())
            .count();

        assert_eq!(children_count, 0);

        #[derive(Resource)]
        struct ParentEntity(Entity);

        app.add_systems(
            Update,
            (move || {
                command_spawn_and((), move |parent| {
                    (
                        command_spawn(ChildOf(parent)),
                        command_insert_resource(ParentEntity(parent)),
                    )
                })
            })
            .pipe(affect),
        );

        app.update();

        let children_count = app
            .world_mut()
            .query::<&ChildOf>()
            .iter(app.world())
            .count();

        assert_eq!(children_count, 1);

        let parent_entity = app.world().resource::<ParentEntity>().0;

        app.world_mut()
            .query::<&ChildOf>()
            .iter(app.world())
            .for_each(|child_of| {
                assert_eq!(child_of.0, parent_entity);
            });

        let children_of_parent_count = app
            .world()
            .entity(parent_entity)
            .get::<Children>()
            .iter()
            .count();

        assert_eq!(children_of_parent_count, 1);
    }
}
