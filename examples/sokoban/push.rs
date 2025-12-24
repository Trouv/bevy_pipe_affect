use bevy::prelude::*;
use bevy_pipe_affect::prelude::*;

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Component, Deref, DerefMut)]
pub struct Position(pub IVec2);

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Component, Deref, DerefMut)]
pub struct Weight(pub u32);

/// An observer event for triggering the push system
#[derive(Event)]
pub struct PushEntity {
    pub direction: IVec2,
    pub entity: Entity,
}

/// The recursive function that creates the effects for pushing entities and also sums their weights.
fn push_and_weigh(
    positions: &Query<(Entity, &Position, &Weight)>,
    position_pushed: Position,
    direction: IVec2,
) -> EffectOut<Vec<EntityComponentsSet<(Position,)>>, Weight> {
    match positions
        .iter()
        .find(|(_, position, _)| **position == position_pushed)
    {
        // base case
        None => effect_out(vec![], Weight(0)),
        // recursive case
        Some((entity, _, weight)) => {
            let new_position = Position(*position_pushed + direction);

            push_and_weigh(&positions, new_position.clone(), direction)
                // Here's the composition!
                .and_extend(|acc_weight| {
                    effect_out(
                        vec![entity_components_set(entity, (new_position,))],
                        Weight(*acc_weight + **weight),
                    )
                })
        }
    }
}

/// The system that triggers the above pushing logic.
///
/// In this case, we happen to only use `EffectOut` for intermediate computation, and return a normal `Effect` in the system.
pub fn push(
    push: On<PushEntity>,
    positions: Query<(Entity, &Position, &Weight)>,
) -> Vec<EntityComponentsSet<(Position,)>> {
    let (_first_entity, position_pushed, _weight) = positions.get(push.entity).unwrap();

    let EffectOut {
        effect: pushes,
        out: weight,
    } = push_and_weigh(&positions, *position_pushed, push.direction);

    if *weight > 10 {
        // too heavy, do nothing.
        vec![]
    } else {
        pushes
    }
}
