use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

// ANCHOR: event
#[derive(Event)]
pub struct InflateEvent;
// ANCHOR_END: event

// ANCHOR: observer_system
#[derive(Component)]
pub struct Inflatable;

fn inflate(_event: On<InflateEvent>) -> impl Effect + use<> {
    components_set_filtered_with::<_, _, With<Inflatable>>(|(transform,): (Transform,)| {
        (transform.with_scale(transform.scale * 1.1),)
    })
}
// ANCHOR_END: observer_system

// ANCHOR: spawn_observer
pub fn spawn_observer() -> CommandSpawn<Observer> {
    command_spawn(Observer::new(inflate.pipe(affect)))
}
// ANCHOR_END: spawn_observer

// ANCHOR: trigger_observer
pub fn trigger_observer(input: Res<ButtonInput<KeyCode>>) -> Option<CommandTrigger<InflateEvent>> {
    if input.just_pressed(KeyCode::Space) {
        Some(command_trigger(InflateEvent))
    } else {
        None
    }
}
// ANCHOR_END: trigger_observer
