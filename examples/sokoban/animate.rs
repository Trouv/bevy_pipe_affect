use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

use crate::push::Position;
use crate::setup::BLOCK_SIZE;

const ANIMATION_DECAY_RATE: f32 = 50.;

pub fn animate_position(time: Res<Time>) -> impl Effect + use<> {
    let delta_secs = time.delta_secs();
    components_set_with_query_data::<_, _, &Position>(
        move |(mut transform,): (Transform,), position| {
            let target = (position.as_vec2() * BLOCK_SIZE).extend(0.0);

            transform
                .translation
                .smooth_nudge(&target, ANIMATION_DECAY_RATE, delta_secs);

            (transform,)
        },
    )
}
