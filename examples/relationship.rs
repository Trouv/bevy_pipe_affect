use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

/// Spins entities around
#[derive(Component)]
struct Spinny;

fn spawn_camera() -> impl Effect {
    command_spawn(Camera2d)
}

fn spawn_relationship() -> impl Effect {
    // We will give both entities a sprite, which we can load using this effect
    // We can use its handle to create more effects with a closure
    asset_server_load_and("player.png", |image_handle| {
        // Similarly, this effect will spawn our components
        // Then, we use the resulting Entity to create more effects with a closure
        command_spawn_and(
            (
                Spinny,
                Sprite::from_image(image_handle.clone()),
                Transform::from_scale(Vec3::splat(10.0)),
            ),
            |parent| {
                command_spawn((
                    Spinny,
                    Sprite::from_image(image_handle),
                    ChildOf(parent),
                    Transform::from_xyz(20.0, 20.0, 0.0).with_scale(Vec3::splat(0.5)),
                ))
            },
        )
    })
}

// This system defines the rotation of entities with a Spinny component to be a function of time.
fn spin(time: Res<Time>) -> impl Effect + use<> {
    let theta = time.elapsed_secs();
    components_set_filtered_with::<_, _, With<Spinny>>(move |(transform,): (Transform,)| {
        (transform.with_rotation(Quat::from_axis_angle(Vec3::Z, theta)),)
    })
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(
            Startup,
            (spawn_camera.pipe(affect), spawn_relationship.pipe(affect)),
        )
        .add_systems(Update, spin.pipe(affect))
        .run();
}
