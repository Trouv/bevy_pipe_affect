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
It also provides some composibility of its own that may be useful beyond systems.
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
However, be aware that `affect` will actually have an output too; the `f32` is passed along.
This would prevent us from scheduling the system (without further piping to drop the `f32`).
However, it is inconsequential if the `out` type is `()`.

### `EffectOut::and_then`
[`EffectOut`](#effectout)s compose in a few different ways, with the main goal of letting users process the `out` field while continuing to collect effects.
For example, the `and_then` method takes a function for processing the `out` into another `Effect`/`EffectOut`, and returns an `EffectOut` with the `effect` [combining the original/new effect](#combined-effects), and an `out` being the new output.

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
[`and_then`](#effectoutand_then) may not be monadic, but there is another [`EffectOut`](#effectout) composition function that is.
If the `effect: E` of the original `EffectOut` is an extendable iterator, and the new effect is an iterator, they can be concatenated with `and_extend`.

In this excerpt from the `sokoban` example, we take advantage of this to write a system with recursive logic.
This recursion is only possible because the recursive function's typing stays consistent.
I.e., the `effect: E` type parameter doesn't need to change with `and_extend` like it does with `and_then`:
```rust
{{#rustdoc_include ../../../examples/sokoban/push.rs:push_logic}}
# fn main() { bevy::ecs::system::assert_is_system(push.pipe(affect)) }
```

This is a bit of a side-note, but notice how this system takes advantage of the fact that we _are not_ actually performing side effects directly when using `Effect`s.
We create a hypothetical set of entity movements with `push_and_weigh`, but those movements aren't performed until the `affect` system runs.
So, we can decide to discard them for whatever reason between now and then (in this case, the reason being that the total weight is too heavy).

### `EffectOut::and_then_compose`
So, [`EffectOut::and_then`](#effectoutand_then) and [`EffectOut::and_extend`](#effectoutand_extend) define two common ways to process output into more effects, and compose those effects.
What about less common effect composition strategies?

Well, `EffectOut::and_then_compose` simply allows the user to pass in an effect composition function, with a signature like `Fn(E1, E2) -> E3`.
It is actually used internally by `and_then` and `and_extend`.
Some common effect composition functions are provided in the crate's `effect_composition` module.

### `EffectOut` iterators
Often, you will find yourself iterating through queries or event readers, and trying to produce effects during that iteration.
As you enjoy FP, you'll probably be trying to do this by mapping the iterator to effects.
This works well for mere `Effect`s, [you can just collect into a `Vec`](#effect-iterators) and call it a day.
However, you may be dealing with `EffectOut`s, and a system returning `Vec<EffectOut>` cannot be piped into `affect`.

The crate provides a simple answer for this issue.
If the `effect` and `out` are both extendable iterators, then you can collect an iterator of `EffectOut`s into an `EffectOut` of iterators.

This is actually demonstrated in the code example of [the next section](#system-level-composition).

### System-level composition
[`EffectOut::and_then`](#effectoutand_then), [`EffectOut::and_extend`](#effectoutand_extend), and [`EffectOut::and_then_compose`](#effectoutand_then_compose) are all methods for processing the `out` of an effect out into more effects and composing them.
Their functionality is also available at the system level, so you can return an [`EffectOut`](#effectout) from one system, and then process its output into more effects with another.
These are the `in_and_then`, `in_and_extend`, and `in_and_then_compose` system combinators, respectively.
They each accept a system that takes the `out: O` type as system input and returns an effect, and they each return a system that accepts the whole `EffectOut` as system input and composes the effects.

Here, we use `in_and_then` to compose a system that creates explosions while calculating the explosion size with a system that plays the explosion sound effect.
```rust
# use bevy::prelude::*;
# use bevy_pipe_affect::prelude::*;
# use bevy::audio::Volume;
#[derive(Deref, DerefMut, Component)]
struct InternalPressure(u32);

#[derive(Deref, DerefMut, Resource)]
struct ExplosionSound(Handle<AudioSource>);

fn explosion(
    explodable: Query<(Entity, &InternalPressure)>,
) -> EffectOut<Vec<EntityCommandDespawn>, u32> {
    explodable
        .iter()
        .flat_map(|(entity, internal_pressure)| {
            let explosion_size = internal_pressure.saturating_sub(100);

            if explosion_size > 0 {
                Some(effect_out(entity_command_despawn(entity), explosion_size))
            } else {
                None
            }
        })
        .collect::<EffectOut<_, Vec<_>>>()
        // this maps the `out` value (functoriality!)
        .map(|sizes| sizes.into_iter().sum())
}

fn explosion_sound(In(explosion_size): In<u32>, sound: Res<ExplosionSound>) -> Option<impl Effect> {
    if explosion_size > 0 {
        Some(command_spawn((
            AudioPlayer::new(sound.clone()),
            PlaybackSettings::default().with_volume(Volume::Linear(explosion_size as f32)),
        )))
    } else {
        None
    }
}

fn main() {
    bevy::ecs::system::assert_is_system(explosion.pipe(in_and_then(explosion_sound)).pipe(affect));
}
```

Notice how the `explosion` and `explosion_sound` systems have completely disjoint system parameters.
Just like functions, the benefit of composing the effects of multiple systems in this way is often that you can simplify each individual system.

As mentioned before, if there remains an `out` type at the end of all this piping, then it will be passed out of the `affect` system, allowing for output processing to continue.
