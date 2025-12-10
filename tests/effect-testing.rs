//! This test is mostly used to demonstrate testing in the book

// ANCHOR: import_preludes
use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;
// ANCHOR_END: import_preludes

// ANCHOR: detect_deaths
#[derive(Component)]
struct Health(u32);

#[derive(Debug, PartialEq, Eq, Message)]
struct DeathMessage(Entity);

fn detect_deaths(query: Query<(Entity, &Health)>) -> Vec<MessageWrite<DeathMessage>> {
    query
        .iter()
        .flat_map(|(entity, health)| {
            if health.0 == 0 {
                Some(DeathMessage(entity))
            } else {
                None
            }
        })
        .map(message_write)
        .collect()
}
// ANCHOR_END: detect_deaths

// ANCHOR: test_detect_deaths
use bevy::ecs::system::RunSystemOnce;

#[derive(Resource)]
struct UnhealthyEntity(Entity);

fn test_detect_deaths() {
    let mut world = World::new();

    // We still need to setup the initial state of the world.
    let _setup = world
        .run_system_once(
            (|| {
                command_spawn_and(Health(100), |_| {
                    command_spawn_and(Health(0), |entity| {
                        command_insert_resource(UnhealthyEntity(entity))
                    })
                })
            })
            .pipe(affect)
            .pipe(ApplyDeferred),
        )
        .unwrap();

    // Now we can just assert against system output instead of state changes
    let dead_entity_messages = world.run_system_once(detect_deaths).unwrap();

    let UnhealthyEntity(entity) = world.get_resource::<UnhealthyEntity>().unwrap();

    assert_eq!(
        dead_entity_messages,
        vec![message_write(DeathMessage(*entity))]
    );
}
// ANCHOR_END: test_detect_deaths

#[test]
fn cargo_test_detect_deaths() {
    test_detect_deaths()
}
