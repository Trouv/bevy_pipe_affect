# The Pipe Affect Scrawl
Scrawled here is a blog post on functional programming, bevy, or why I made this library.
It may not be the most practical piece of documentation, but I hope it shines a light on some things for library users and maybe others.

## I'm an FP shill now
Rust takes a lot of inspiration from purely functional languages like haskell.
As a person who learned rust before functional programming, most of its features that I found to be revelations turned out to be derivative.
Iterator chains, algebraic data types, sum-types used in place of `null` and exceptions, `?` operators, to name a few.

I've written a lot of rust professionally over the past few years, and gradually it has become obvious how beneficial it is to use these features.
Or, more than anything else, it has become obvious how beneficial it is to write pure functions.
Pure functions are easily unit tested.
They are easy to compose without unexpected consequences.
In professional software, if you must read from state or write to state, go to great lengths to push these things to the fringes of the program.
Even if you are designing a system of programs, push the state to the fringes of the data flow at large.

Every time you write `mut`, a puppy dies.
`for` loops kill kittens instead.

## Bevy's side effects
Now, like a true software-gamedev-hipster, I also shill `bevy`.
The core framework of bevy is an ECS among many great rust ECSs, but I especially appreciate that its systems are mere functions.
Its system scheduler may be particularly attractive to FP shills as well.
It is declarative, it leverages higher-order functions for scheduling your systems, it provides system composition with piping and mapping, and it does its best to abstract away the parallel execution of systems.

However, the only way to interact with the world in vanilla bevy is by writing systems that have side effects.
If you want to update a resource, you must parameterize a `ResMut`.
If you want to edit components in-place, you must query them `&mut`-ably.
If you want to load an asset, you must interact with the internally mutable `AssetServer`.
Even if you do everything with `Commands`, not only do the intended effects require exclusive world access, you're still having a side effect on the command queue.

In my feable attempts to write more immutable systems, I would simply write systems that output messages, or bundles, and then have generic systems to handle these as pipe input and actually do the writing or spawning. For example:
```rust
# #[derive(Component)]
# struct Health(u32);
use bevy::prelude::*;

#[derive(Message)]
struct DeathMessage(Entity);

fn detect_deaths(query: Query<(Entity, &Health)>) -> Vec<DeathMessage> {
    query
        .iter()
        .flat_map(|(entity, health)| {
            if health.0 == 0 {
                Some(DeathMessage(entity))
            } else {
                None
            }
        })
        .collect()
}

fn write_messages<M: Message>(
    In(messages): In<impl IntoIterator<Item = M>>,
    mut writer: MessageWriter<M>,
) {
    messages.into_iter().for_each(|message| {
        writer.write(message);
    });
}

fn main() {
    bevy::ecs::system::assert_is_system(detect_deaths.pipe(write_messages));
}
```

So, in this example, I've gone from 0% of my systems being pure to 50%.
Wouldn't it be nice if it could be 100%?
If somebody provided all the systems you may ever need to do the "writing" so that you only have to worry about writing ECS effects declaratively?
`bevy_pipe_affect` aims to provide these systems.
Or rather, a single system for all ECS mutation.
Her name is `affect`:

```rust
# #[derive(Component)]
# struct Health(u32);
use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

#[derive(Message)]
struct DeathMessage(Entity);

fn detect_deaths(query: Query<(Entity, &Health)>) -> Vec<MessageWrite<DeathMessage>> {
    query
        .iter()
        .flat_map(|(entity, health)| {
            if health.0 == 0 {
                Some(DeathMessage(entity))
            } else {
                None
            }
        })
        .map(message_write)
        .collect()
}

fn main() {
    bevy::ecs::system::assert_is_system(detect_deaths.pipe(affect));
}
```
