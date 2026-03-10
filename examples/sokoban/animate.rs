use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::push::Position;
use crate::setup::BLOCK_SIZE;

/// How quickly Position items smooth to their destination.
const ANIMATION_DECAY_RATE: f32 = 50.;

/// Defines how visual positions of entities update based off their logical position.
pub fn animate_position(
    time: Res<Time>,
) -> QueryMap<(&'static Transform, &'static Position), ComponentSet<Transform>> {
    let delta_secs = time.delta_secs();
    query_map(move |(transform, position): (&Transform, &Position)| {
        // note: even though we we use mut here, there is still no side effect to this closure.
        // This is because we take ownership of the transform instead of mutable reference.
        let target = (position.as_vec2() * BLOCK_SIZE).extend(0.0);

        let mut new_transform = *transform;

        new_transform
            .translation
            .smooth_nudge(&target, ANIMATION_DECAY_RATE, delta_secs);

        component_set(new_transform)
    })
}
