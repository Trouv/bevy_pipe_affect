//! [`Effect`]s that queue entity-specific `Commands`.

mod entity_command_queue;
pub use entity_command_queue::{EntityCommandQueue, entity_command_queue};

mod entity_command_insert;
pub use entity_command_insert::{EntityCommandInsert, entity_command_insert};

mod entity_command_remove;
pub use entity_command_remove::{EntityCommandRemove, entity_command_remove};

mod entity_command_despawn;
pub use entity_command_despawn::{EntityCommandDespawn, entity_command_despawn};

mod entity_command_insert_recursive;
pub use entity_command_insert_recursive::{
    EntityCommandInsertRecursive,
    entity_command_insert_recursive,
};

mod entity_command_remove_recursive;
pub use entity_command_remove_recursive::{
    EntityCommandRemoveRecursive,
    entity_command_remove_recursive,
};

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
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

            assert!(!IntoSystem::into_system(insert_component_system.pipe(affect)).initialize(app.world_mut()).is_exclusive());

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
