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

## License

Except where noted, all code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.
