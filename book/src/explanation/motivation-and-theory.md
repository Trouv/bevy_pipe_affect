# The Pipe Affect Scrawl
Scrawled here is a blog post on functional programming, bevy, or why I made this library.
It may not be the most practical piece of documentation, but I hope it shines a light on some things for library users and maybe others.

## I'm an FP shill now
Here is a brief shill for writing FP rust.
If you already are an FP shill, you can skip this.

Rust takes a lot of inspiration from purely functional languages like haskell.
As a person who learned rust before functional programming, I was intrigued to learn that most of its features that I found to be revelations turned out to be derivative.
Iterator chains, algebraic data types, sum-types used in place of `null` and exceptions, `?` operators, to name a few.
Similar things have been etched into purely functional languages for a long time.

I've written a lot of rust professionally over the past few years, and gradually it has become obvious how beneficial it is to use these features.
Or, more than anything else, it has become obvious how beneficial it is to write pure functions with the aid of these features.

For the uninitiated, pure functions are those that are deterministic and have no side effects.
Like a function in mathematics, they are mere input and output, so they do not read or write anything from the state of the world at large.
A purely functional language, like haskell, is one that only allows you to write pure functions.
This may seem limiting, but thanks to higher-order functions, plus the strength of FP's theoretical foundations in general, it really isn't.

Pure functions are easily unit tested.
They are easy to compose without unexpected consequences.
If you must read from state or write to state, go to great lengths to push these things to the fringes of the program.
Even if you are designing a system of programs, push the state to the fringes of the data flow at large.

## Practical motivation
Now, like a true software-gamedev-hipster, I also shill `bevy`.
The core framework of bevy is an ECS among many great rust ECSs, but I especially appreciate that its systems are mere functions.
Its system scheduler may be particularly attractive to FP shills as well.
It is declarative, it leverages higher-order functions for scheduling your systems, it provides system composition with piping and mapping, and it does its best to abstract away the parallel execution of systems safely.

However, the main way to interact with the world in vanilla bevy is by writing systems that have side effects.
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

So, in this example, I have a pure system `detect_deaths` that produces messages as output, and then a system that actually writes the messages `write_messages`.
I've gone from 0% of my systems being pure to 50%.
Since `write_messages` is generic, I can now write more pure systems that produce messages and reuse it.

Wouldn't it be nice if 100% of user-written systems could be pure?
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

Rather than returning a list of messages, `detect_deaths` now returns `MessageWrite`s, which is an `Effect`.
A `Vec` of `Effect`s is also an `Effect`.
Then, the `affect` system can take any `Effect` and do the necessary writing.
The user no longer has to write words like `mut` and `for`.

## Theoretical motivation
Bevy's system scheduling APIs are higher-order functions that allow you to register system-functions to the App.
We can basically think of these higher-order functions as taking functions with two arguments, a `SystemInput` and a `SystemParam`, and then having an output.
Technically there's an extra wrinkle to this for two reasons, but both are just a bit of sugar that carmelize down to these two arguments:
- the `SystemInput` can be omitted, but the `bevy` scheduling traits just use the unit type `()` in these cases
- the `SystemParam` can occupy more than 1 arguments to the function (or even 0), but the `bevy` scheduling traits just convert these cases to a tuple `SystemParam`

This is elegant.
Our `SystemParam` argument not only serves as normal function input, but it also expresses to the higher-order scheduling APIs what *factor* of the world needs to be input to the system.
I say *factor* in the sense of algebraic data types.
In the language of algebraic data types, an ECS world is sort of like a *product* of component storages and resources, and our `SystemParam` identifies a *factor* of this *product*.
Again, the reality of bevy is more complicated (this time, much more complicated) than this theoretical framework.

The `SystemParam` is even composable.
The *factor* of the world that a system gets as input can actually be a larger product of system params.
As in, it can be a tuple of other system params, which again, is what the sugar of multi-system-param-argument functions carmelizes into.
Pipe systems also leverage this fact by composing the `SystemParam`s of two systems into one (this time using a `ParamSet` type to guarantee safe memory access).

So far nothing about this is functionally impure.
We have functions with two arguments and an output, the first argument is `SystemInput` which is parameterized by output of another system, the second argument is `SystemParam` which is parameterized by some data in the world.
The impurity arrives when we allow that data from the world to be mutable.
And of course, in vanilla bevy, this is our only choice if we want to have any effect on the world other than heating up our computers.
