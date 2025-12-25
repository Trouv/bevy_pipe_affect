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
