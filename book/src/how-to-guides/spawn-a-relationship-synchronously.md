# Spawn a Relationship Synchronously
Bevy relationships may be spawned using specialized commands, like `with_children` or `with_related_entities`.
`bevy_pipe_affect` APIs are more minimal, but these situations can still be handled ergonomically:
1. Return an effect that spawns the entity that will be the `RelationshipTarget` with `command_spawn_and`. In the `Parent`/`ChildOf` relationship, this will become the `Parent` entity. That component does not need to be provided, it will be created by Bevy.
2. Provide a closure to the second argument of the `command_spawn_and` call that returns another `command_spawn-` effect that will spawn the `Relationship` entity. In the `Parent`/`ChildOf` relationship, this is the `ChildOf` entity. This time, you do need to provide that component with the `Entity` provided to the closure (this will be `RelationshipTarget` `Entity`, spawned in step 1).

The `relationship` example does this, while also bundling some sprites/marker components:
```rust
# #[derive(Component)] struct Spinny;
use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

{{#include ../../../examples/relationship.rs:spawn_relationship}}
# fn main() { bevy::ecs::system::assert_is_system(spawn_relationship.pipe(affect)) }
```
