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

## Development
`bevy_pipe_affect` is nearing release.
The initial set of effects and utilities have been created.
See #47 for the remainder of the planned work before the initial release.

## Documentation
Documentation for this library is available in two main places.
- API reference on [docs.rs](https://docs.rs/bevy_pipe_affect/0.1.0/bevy_pipe_affect/) <!-- x-release-please-version -->
- Tutorials, Explanation, and Guides in the [`bevy_pipe_affect` book](https://trouv.github.io/bevy_pipe_affect/main/index.html)

The following are good jumping-off points for beginners:
- [*Motivations* explanation](https://trouv.github.io/bevy_pipe_affect/main/explanation/motivations.html)
- [*effects* module api reference](https://docs.rs/bevy_pipe_affect/0.1.0/bevy_pipe_affect/effects/index.html) (a list of effects and constructors provided by the library) <!-- x-release-please-version -->

Cargo examples are also available in this repository:
```sh
$ cargo run --example example-name --release --features bevy/default
```

## Compatibility
| bevy | bevy_pipe_affect |
| --- | --- |
| 0.17 | main |

## License

Except where noted, all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
