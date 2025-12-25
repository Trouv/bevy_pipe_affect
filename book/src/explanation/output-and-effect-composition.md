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
`bevy_pipe_affect` sort of hijacks bevy's system piping.
So, at first glance, it may seem like there's no way to go about typical system pipe usage while making effects.
The `EffectOut` type aims to give system piping back to the people.
It also providing some composibility of its own that may be useful beyond systems.
More on this in the following sections.

Structurally, it's just an `effect` field containing an effect, and an `out` field containing additional output.

You may be interested to know that the higher-order systems provided by `bevy_pipe_affect` actually only ever expect a type that can convert into `EffectOut`, not just a mere `Effect`:

```rust
# use bevy::prelude::*;
# use bevy_pipe_affect::prelude::*;
#[derive(Resource)]
struct Score(u32);

#[derive(Deref, DerefMut, Resource)]
struct StartTime(f32);

fn update_score(time: Res<Time>, start_time: Res<StartTime>) -> EffectOut<ResSet<Score>, f32> {
    let level_time = time.elapsed_secs() - **start_time;
    effect_out(res_set(Score(level_time as u32)), level_time)
}

fn main() {
    bevy::ecs::system::assert_is_system(update_score.pipe(affect))
}
```

Notice that we can still pipe `update_score` into `affect`, even though `update_score` returns an `EffectOut` instead of an `Effect`.

### `EffectOut::and_then`
`EffectOut`s compose in a few different ways, with the main goal of letting users process the `out` field while continuing to collect effects.
For example, the `and_then` method takes a function for processing the `out` into another `Effect`/`EffectOut`, and returns an `EffectOut` with the `effect` combining the original/new effect, and an `out` being the new output.

This code example shows `and_then` being used to process the `EffectOut` returned by one function into more effects:
```rust
# use bevy::prelude::*;
# use bevy_pipe_affect::prelude::*;
# // A simple marker component for players.
# #[derive(Component)]
# struct Player<const N: usize>;
# // A logical component for player level.
# #[derive(Component, Deref, DerefMut)]
# struct FightLevel(u32);
# /// A simple message we can write with effects.
# #[derive(Message)]
# struct Log(String);
/// Will be used as an intermediate result of the fight logic.
enum FightOutcome {
    Player1Wins,
    Player2Wins,
    Draw,
}

/// This is not a system! It describes the logic for the upcoming fight system and also has a logging effect.
fn fight_outcome(
    player_1: &FightLevel,
    player_2: &FightLevel,
) -> EffectOut<MessageWrite<Log>, FightOutcome> {
    if **player_1 > **player_2 {
        effect_out(
            message_write(Log("player 1 wins!".to_string())),
            FightOutcome::Player1Wins,
        )
    } else if **player_1 < **player_2 {
        effect_out(
            message_write(Log("player 2 wins!".to_string())),
            FightOutcome::Player2Wins,
        )
    } else {
        effect_out(
            message_write(Log("fight came to a draw!".to_string())),
            FightOutcome::Draw,
        )
    }
}

/// This is a system! It defines the effects of a player 1 and 2 fighting (logging the result and despawning).
fn fight(
    player_1: Single<(Entity, &FightLevel), With<Player<1>>>,
    player_2: Single<(Entity, &FightLevel), With<Player<2>>>,
) -> EffectOut<(MessageWrite<Log>, Option<EntityCommandDespawn>), ()> {
    let (player_1_entity, player_1_level) = *player_1;
    let (player_2_entity, player_2_level) = *player_2;

    // here's an EffectOut-returning call
    fight_outcome(player_1_level, player_2_level)
        // and now we're composing it w/ a closure processing the `out`
        .and_then(|outcome| match outcome {
            FightOutcome::Player1Wins => Some(entity_command_despawn(player_2_entity)),
            FightOutcome::Player2Wins => Some(entity_command_despawn(player_1_entity)),
            FightOutcome::Draw => None,
            // we can return an EffectOut here, but a mere Effect works too (in this case Option<EntityCommandDespawn>)
        })
    // we could continue composing it with more `.and_then`s if we had more output to process into effects.
}
# fn main() { bevy::ecs::system::assert_is_system(fight.pipe(affect)) }
```
You may notice that, if we _did_ want to create the "despawn" effects in the `fight_outcome` system, we'd have to complicate its function signature with more `Entity` inputs.
This composition of `EffectOut`s keeps each function smaller and simpler, but allows for a grander logic that is much more complicated and powerful.

Rust users will recognize this API as being similar in name and purpose to `Option::and_then` and `Result::and_then`, and they'd roughly be correct.
Functional programmers, on the other hand, may recognize this API as being similar to a monad's bind operation.
It is _kind of_ like that, but not quite.
While the `out: O` type gets mapped in a monadic way, the `effect: E` type changes to be a tuple of the new and old effects.
If it were truly monadic, only one type parameter (`out: O`) of `EffectOut` would be changed by bind, not both.

### `EffectOut::and_extend`
Unlike `and_then` there is another `EffectOut` composition function that is actually more monadic.
If the `effect: E` of the original `EffectOut` is an extendable iterator, and the new effect is an iterator, they can be concatenated with `and_extend`.

In this example, we take advantage of this to write a system with recursive logic.
This recursion is only possible because the recursive function's typing stays consistent.
I.e., the effect type doesn't need to change with `and_extend` like it does with `and_then`:
```rust
{{#rustdoc_include ../../../examples/sokoban/push.rs:push_logic}}
# fn main() { bevy::ecs::system::assert_is_system(push.pipe(affect)) }
```

This is a bit of a side-note, but notice how this system takes advantage of the fact that we _are not_ actually performing side effects directly when using `Effect`s.
We create a hypothetical set of entity movements with `push_and_weigh`, but those movements aren't performed until the `affect` system runs.
So, we can decide to discard them for whatever reason between now and then, in this case the reason being that the total weight is too heavy.

### `EffectOut::and_then_compose`

### EffectOut iterators

### System-level composition
