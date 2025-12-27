use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Update,
            pure(rainbow_clear_color) // pure() is optional, just forces the system to be read-only
                .pipe(affect),
        )
        .run()
}

/// This system defines the clear color as a pure function of time.
fn rainbow_clear_color(time: Res<Time<Real>>) -> ResSet<ClearColor> {
    let color = Color::hsv(time.elapsed_secs() * 20.0, 0.7, 0.7);
    res_set(ClearColor(color))
}

#[cfg(test)]
mod tests {

    use bevy::camera::ClearColor;
    use bevy::color::Color;
    use bevy::ecs::system::{Res, SystemState};
    use bevy::ecs::world::World;
    use bevy::time::{Real, Time};

    use crate::rainbow_clear_color;

    #[test]
    fn test_rainbow_clear() {
        // Create a Res<Time> for our system
        let mut world = World::new();
        world.insert_resource(Time::new_with(Real::default()));

        let mut state: SystemState<(Res<Time<Real>>,)> = SystemState::new(&mut world);
        let (time,) = state.get(&mut world);

        // Now that we have a Res<Time>, we can run our system and get an output
        let output = rainbow_clear_color(time);

        // Then all we have to do is check that output has the proper value
        assert_eq!(output.value.0, Color::hsva(0.0, 0.7, 0.7, 1.0));
    }
}
