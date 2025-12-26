# Spawn and Trigger an Observer
While `bevy_pipe_affect` effects are somewhat minimal and don't include a `command_add_observer`, observers still integrate well with effects.

The following code examples are pulled from the `observer` cargo example.

1. Create an event type (`InflateEvent` in this example)
```rust
{{#rustdoc_include ../../../examples/observer/inflatable.rs:event}}
# fn main() {}
```
2. Create a system that accepts an `On<InflateEvent>` and returns an effect that will be ran by the observer.
```rust
{{#rustdoc_include ../../../examples/observer/inflatable.rs:observer_system}}
# fn main() { bevy::ecs::system::assert_is_system(inflate.pipe(affect)) }
```
3. Create a system that spawns an `Observer` component with `command_spawn`, using the previous system `.pipe(affect)`-ed as the observer system.
```rust
{{#rustdoc_include ../../../examples/observer/inflatable.rs:spawn_observer}}
# fn main() { bevy::ecs::system::assert_is_system(spawn_observer.pipe(affect)) }
```
4. Trigger your observer with the `command_trigger` effect.
```rust
{{#rustdoc_include ../../../examples/observer/inflatable.rs:trigger_observer}}
# fn main() { bevy::ecs::system::assert_is_system(trigger_observer.pipe(affect)) }
```
