# Output and Effect Composition
This chapter will cover the ways that effects compose, starting with the most basic and getting more advanced.

## Combined effects
The canonical form of "effect composition" is the combined effect, which is simply a tuple of effects.
The `Effect` trait is implemented for tuples where each element of the tuple also implements `Effect`.
The `affect` system will perform their effects from left to right.

So, if you want a system that has 2 or more effects of heterogenous type, you can just return their tuple:

```rust
# use bevy::prelude::*;
# use bevy_pipe_affect::prelude::*;
#[derive(Resource)]
struct Score(u32);

fn setup() -> impl Effect {
    (
        command_spawn(Camera2d::default()),
        command_insert_resource(Score(0)),
    )
}
# fn main() { bevy::ecs::system::assert_is_system(setup.pipe(affect)) }
```

## Effect iterators
`Effect` is implemented for a couple of important iterators, `Option` and `Vec`.
There's also the `affect_many` effect, which can wrap any iterator.

So, if you want a system that has 2 or more effects of homogenous type, you can return them as a `Vec`:

```rust
{{#rustdoc_include ../../../tests/effect-testing.rs:detect_deaths}}
# fn main() { bevy::ecs::system::assert_is_system(detect_deaths.pipe(affect)); }
```

## EffectOut

## EffectOut composition

## EffectOut iterators

## System-level composition
