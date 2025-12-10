# Motivations
Scrawled here is basically a blog post on functional programming, bevy, and why I made this library.
It may not be the most practical piece of documentation, but I hope it shines a light on some of the design choices for library users, and maybe even reaches out to others who are future FP+ECS enthusiasts.

## I'm an FP shill now
Here is a brief shill for writing FP Rust.
If you already are an FP shill, you can skip this.

Rust takes a lot of inspiration from purely functional languages like Haskell.
As a person who learned Rust before functional programming, I was intrigued to learn that most of its features that I found to be revelations turned out to be derivative.
Iterator chains, algebraic data types, sum-types used in place of `null` and exceptions, `?` operators, to name a few.
Similar things have been etched into purely functional languages for a long time.

I've written a lot of Rust professionally over the past few years, and gradually it has become obvious how beneficial it is to use these features.
Or, more than anything else, it has become obvious how beneficial it is to write pure functions with the aid of these features.

For the uninitiated, pure functions are those that are deterministic and have no side effects.
Like a function in mathematics, they are mere input and output, so they do not read or write anything from the state of the world at large.
A purely functional language, like Haskell, is one that only allows you to write pure functions.
This may seem limiting, but thanks to higher-order functions, plus the strength of FP's theoretical foundations in general, it really isn't.

Pure functions are easily unit tested, since you don't need to set up any state.
They are easy to compose without unexpected consequences.
There's a simplicity in functions that only have input and output, for both readers and writers of the code.

> Which function should perform this change?

> Should the data this function uses be input or read from state?

> Should the data this function calculates be output or written to state?

If you're writing pure functions, these questions aren't just foregone conclusions, they are invalid.

So in my regular programming practice now, I go to great lengths to _at least_ push the state reading/writing to the fringes of the program.
Even when designing a system of programs, I consider pushing the state to the fringes of the data flow at large.
This practice isn't that common in `bevy`.

## Practical motivation
Now, like a true software-gamedev-hipster, I also shill `bevy`.
The core framework of bevy is an ECS among many great Rust ECSs, but I especially appreciate that its systems are mere functions.
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

#[derive(Debug, PartialEq, Eq, Message)]
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
{{#rustdoc_include ../../../tests/effect-testing.rs:detect_deaths}}

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
Pipe systems also leverage this fact by composing the `SystemParam`s of two systems into one.

So far nothing about this is functionally impure.
We have functions with two arguments and an output, the first argument is `SystemInput` which is parameterized by output of another system, the second argument is `SystemParam` which is parameterized by some data in the world.
The impurity arrives when we allow that data from the world to be mutable.
And of course, in vanilla bevy, this is our only choice if we want to have any effect on the world other than heating up our computers.

Pure functions are just input and output.
We'd like to use the output instead of the side effects to have an effect on world data.
Hence the `Effect` types provided by `bevy_pipe_affect`, intended to be returned by user systems.

`Effect`s, conceptually, are almost a reflection of `SystemParam`s.
Where `SystemParam`s allow systems to express what *factor* of the world should be read, `Effect`s allow systems to express what *factor* of the world should be written (and how).
Where `SystemParam`s have an identity in the form of `()` that requests no data from the world, `Effect`s also treats `()` as an identity that has no effect on the world.
Where `SystemParam`s offer composibility with product types and derives, `Effect`s offer composibility with product types, derives, and sum-types.

Yes, not only is `Effect` implemented for tuples of effects, it can also be derived for structs of `Effect`s and enums of `Effect`s.
The latter is not a reflection of `SystemParam` behavior.
After all, it's not that common that you want a system that accepts *either* system param A *or* system param B.
It's a different story for `Effect`s, as there are many situations where you want *either* effect A to happen *or* effect B to happen.
The composibility of `Effect`s is as algebraic as algebraic data types.

## Write systems as pure functions
Of course, none of this is required with `bevy_pipe_affect`.
Nothing about it forces you to write pure systems, you could write an effectful system that pipes an `Effect` into the `affect` system.

If you choose to, you will enjoy many of the benefits of pure functions.
The consequences of your systems will be more obvious at a glance: they are in the system's return type.
If you need more specifics, their value will always be at the very bottom of your function body.
In general, these two facts make it more difficult for you to muddy your systems with effects.
You will be encouraged to separate the concerns of your systems even more.

And of course, unit tests are easier to write.
Instead of observing the effects your systems have on the bevy world, you can just observe the output of your systems.
An example, testing the `detect_deaths` system written above
```rust
{{#rustdoc_include ../../../tests/effect-testing.rs:test_detect_deaths}}
# fn main() { test_detect_deaths() }
```

Over all, game logic just becomes easier to reason about, especially *ex post facto*.
I hope you enjoy writing systems this way, and that they bring you more joy when the time comes for you to maintain them.
