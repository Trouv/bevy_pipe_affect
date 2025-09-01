use std::marker::PhantomData;

use bevy::ecs::error::CommandWithEntity;
use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that pushes a generic entity command to the command queue.
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
#[doc = include_str!("defer_command_note.md")]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EntityCommandInsert<B>(pub Entity, pub B)
where
    B: Bundle;

impl<B> Effect for EntityCommandInsert<B>
where
    B: Bundle,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.0).insert(self.1);
    }
}

/// [`Effect`] that queues a command for removing the `Bundle` from the `Entity`.
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

impl<B> Effect for EntityCommandRemove<B>
where
    B: Bundle,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).remove::<B>();
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberComponent;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn entity_command_queue_can_insert_component_non_exclusively(component in any::<NumberComponent<0>>()) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(()).id();

            let actual_component = app.world().get_entity(entity).unwrap().get_components::<&NumberComponent<0>>();

            assert!(actual_component.is_none());

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
                (move || EntityCommandInsert(entity, (component_0, component_1))).pipe(affect).in_set(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert_eq!(actual_component_0, Some(&component_0));
            assert_eq!(actual_component_1, Some(&component_1));

            app.add_systems(
                Update,
                (move || EntityCommandRemove::<(NumberComponent<1>, NumberComponent<2>)>::new(entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert_eq!(actual_component_0, Some(&component_0));
            assert!(actual_component_1.is_none());

            app.add_systems(
                Update,
                (move || EntityCommandRemove::<(NumberComponent<0>, NumberComponent<1>)>::new(entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert!(actual_component_0.is_none());
            assert!(actual_component_1.is_none());
        }
    }
}
