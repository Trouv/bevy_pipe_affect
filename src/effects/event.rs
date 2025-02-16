use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that sends an event `E` to the corresponding `EventWriter`.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct EventSend<E>(pub E)
where
    E: Event;

impl<E> Effect for EventSend<E>
where
    E: Event,
{
    type MutParam = EventWriter<'static, E>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.send(self.0);
    }
}
