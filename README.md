# `bevy_pipe_affect`<br>Write systems as pure functions
Normally, Bevy systems perform some state changes as side effects.
This crate enables you to instead return `Effect`s as system output.

`Effect`s define an ECS state transition.
All common ECS operations have one or more `Effect` types provided in the library.

These "systems with effects" can then be `.pipe(affect)`-ed.
The `affect` system will perform the state transition.

This enables a more functional code-style in `bevy` app development.
User-written systems can all be read-only, pure functions.
All mutability can be _piped out_ of your code.

*From the example `rainbow-clear-color`:*
```rust
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
fn rainbow_clear_color(time: Res<Time>) -> ResSet<ClearColor> {
    let color = Color::hsv(time.elapsed_secs() * 20.0, 0.7, 0.7);
    res_set(ClearColor(color))
}

// One benefit of writing systems as pure function is that testing becomes much easier
#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;

    #[test]
    fn test_rainbow_clear() {
        // Create a world to test against
        let mut world = World::new();
        world.insert_resource::<Time>(Time::default());

        // Now that we have a Res<Time>, we can run our system and get an output
        let output = world.run_system_once(rainbow_clear_color).unwrap();

        // Then, instead of checking that the world and its resources changed in a particular way,
        // all we have to do is check that the output is correct
        assert_eq!(output.value.0, Color::hsva(0.0, 0.7, 0.7, 1.0));
    }
}
```

## Documentation
Documentation for this library is available in two main places.
- API reference on [docs.rs](https://docs.rs/bevy_pipe_affect/0.1.0/bevy_pipe_affect/) <!-- x-release-please-version -->
- Tutorials, Explanation, and Guides in the [`bevy_pipe_affect` book](https://trouv.github.io/bevy_pipe_affect/v0.1.0/index.html) <!-- x-release-please-version -->

The following are good jumping-off points for beginners:
- [*Motivations* explanation](https://trouv.github.io/bevy_pipe_affect/v0.1.0/explanation/motivations.html) <!-- x-release-please-version -->
- [*effects* module api reference](https://docs.rs/bevy_pipe_affect/0.1.0/bevy_pipe_affect/effects/index.html) (a list of effects and constructors provided by the library) <!-- x-release-please-version -->

Cargo examples are also available in this repository:
```sh
$ cargo run --release --all-features --example example-name
```

## Compatibility
| bevy | bevy_pipe_affect |
| --- | --- |
| 0.17 | 0.1 |

## License

Except where noted, all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
