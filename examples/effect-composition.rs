use std::fmt::Display;

use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Message)]
struct Log(String);

#[derive(Copy, Clone, Debug, Default, PartialEq, Deref, DerefMut, Component)]
struct Velocity(Vec2);

#[derive(Copy, Clone, Debug, Default, PartialEq, Deref, DerefMut, Component)]
struct RectangleCollider {
    half_extents: Vec2,
}

const LOGO_SIZE: Vec2 = Vec2::new(160.0, 90.0);

struct WindowColliders {
    right: Aabb2d,
    up: Aabb2d,
    left: Aabb2d,
    down: Aabb2d,
}

enum WindowColliderDirection {
    Right,
    Up,
    Left,
    Down,
}

impl Display for WindowColliderDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WindowColliderDirection::Right => {
                write!(f, "right")
            }
            WindowColliderDirection::Up => {
                write!(f, "up")
            }
            WindowColliderDirection::Left => {
                write!(f, "left")
            }
            WindowColliderDirection::Down => {
                write!(f, "down")
            }
        }
    }
}

fn window_colliders(window: Single<&Window>) -> EffectOut<MessageWrite<Log>, WindowColliders> {
    let width = window.width();
    let height = window.height();

    let log = Log(format!("window size is currently {width} by {height}"));

    let right = Aabb2d::new(Vec2::new(width / 2.0, 0.0), Vec2::new(1.0, height / 2.0));
    let up = Aabb2d::new(Vec2::new(0.0, height / 2.0), Vec2::new(width / 2.0, 1.0));
    let left = Aabb2d::new(Vec2::new(-width / 2.0, 0.0), Vec2::new(1.0, height / 2.0));
    let down = Aabb2d::new(Vec2::new(0.0, -height / 2.0), Vec2::new(width / 2.0, 1.0));

    effect_out(
        message_write(log),
        WindowColliders {
            right,
            up,
            left,
            down,
        },
    )
}

fn collides_with_window(
    window_colliders: &WindowColliders,
    collider: &Aabb2d,
) -> Option<EffectOut<MessageWrite<Log>, WindowColliderDirection>> {
    let collision_direction = if window_colliders.right.intersects(collider) {
        WindowColliderDirection::Right
    } else if window_colliders.up.intersects(collider) {
        WindowColliderDirection::Up
    } else if window_colliders.left.intersects(collider) {
        WindowColliderDirection::Left
    } else if window_colliders.down.intersects(collider) {
        WindowColliderDirection::Down
    } else {
        return None;
    };

    let log = message_write(Log(format!("collided with {collision_direction} wall!")));

    Some(effect_out(log, collision_direction))
}

fn reflect_velocity_off_window(
    velocity: &Velocity,
    collision_direction: WindowColliderDirection,
) -> Velocity {
    match collision_direction {
        WindowColliderDirection::Right if velocity.x > 0.0 => {
            Velocity(**velocity * Vec2::new(-1.0, 1.0))
        }
        WindowColliderDirection::Left if velocity.x < 0.0 => {
            Velocity(**velocity * Vec2::new(-1.0, 1.0))
        }
        WindowColliderDirection::Up if velocity.y > 0.0 => {
            Velocity(**velocity * Vec2::new(1.0, -1.0))
        }
        WindowColliderDirection::Down if velocity.y < 0.0 => {
            Velocity(**velocity * Vec2::new(1.0, -1.0))
        }
        _ => *velocity,
    }
}

fn bounce_off_window(
    In(window_colliders): In<WindowColliders>,
    colliders: Query<(Entity, &Velocity, &RectangleCollider, &Transform)>,
) -> EffectOut<Vec<impl Effect + use<>>, ()> {
    colliders
        .iter()
        .flat_map(|(entity, velocity, rectangle, transform)| {
            let collider = Aabb2d::new(transform.translation.xy(), **rectangle);
            let log_collision = collides_with_window(&window_colliders, &collider)?;

            Some((entity, velocity, log_collision))
        })
        .map(|(entity, velocity, log_collision)| {
            log_collision.and_then(|collision_direction| {
                entity_components_set(
                    entity,
                    (reflect_velocity_off_window(velocity, collision_direction),),
                )
            })
        })
        .collect()
}

fn velocity<'s>(
) -> ComponentsSetWith<impl Fn((Transform,), &Velocity) -> (Transform,), (Transform,), &'s Velocity>
{
    components_set_with_query_data::<_, _, &Velocity>(|(transform,): (Transform,), velocity| {
        (transform.with_translation(transform.translation + velocity.extend(0.0)),)
    })
}

fn print_logs(mut logs: MessageReader<Log>) {
    logs.read().for_each(|log| {
        println!("{}", log.0);
    });
}

fn setup() -> impl Effect {
    (
        command_spawn((
            Velocity(Vec2::new(2.0, 1.0)),
            Sprite::from_color(Color::WHITE, LOGO_SIZE),
            RectangleCollider {
                half_extents: LOGO_SIZE / 2.0,
            },
        )),
        command_spawn(Camera2d::default()),
    )
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<Log>()
        .add_systems(Startup, setup.pipe(affect))
        .add_systems(
            FixedUpdate,
            (
                velocity.pipe(affect),
                window_colliders
                    .pipe(in_and_then(bounce_off_window))
                    .pipe(affect),
                print_logs,
            ),
        )
        .run();
}
