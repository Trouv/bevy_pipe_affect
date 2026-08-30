//! [`Effect`]s that queue `Commands`.

mod command_queue;
pub use command_queue::{CommandQueue, command_queue};

mod command_insert_resource;
pub use command_insert_resource::{CommandInsertResource, command_insert_resource};

mod command_remove_resource;
pub use command_remove_resource::{CommandRemoveResource, command_remove_resource};

mod command_spawn_and;
pub use command_spawn_and::{CommandSpawn, CommandSpawnAnd, command_spawn, command_spawn_and};

mod command_trigger;
pub use command_trigger::{CommandTrigger, command_trigger};

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::{NumberComponent, NumberEvent, NumberResource};
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

        #[test]
        fn command_trigger_correctly_triggers_observers(event in any::<NumberEvent>()) {
            let mut app = App::new();

            app.add_systems(
                Startup,
                (move || command_spawn(Observer::new((|event: On<NumberEvent>| command_insert_resource(NumberResource(event.0))).pipe(affect)))).pipe(affect),
            );

            app.update();
            assert!(app.world().get_resource::<NumberResource>().is_none());

            app.update();
            assert!(app.world().get_resource::<NumberResource>().is_none());

            app.add_systems(
                Update,
                (move || command_trigger(event)).pipe(affect)
            );

            app.update();

            assert_eq!(app.world().get_resource::<NumberResource>(), Some(&NumberResource(event.0)));
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
