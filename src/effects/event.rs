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

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    use super::*;
    use crate::prelude::affect;

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Event, Arbitrary)]
    struct NumberEvent(u128);

    proptest! {
        #[test]
        fn event_send_produces_events(events in prop::collection::vec(any::<NumberEvent>(), 1..10)) {
            let mut app = App::new();

            let mut events_clone = events.clone();
            app.add_event::<NumberEvent>()
                .add_systems(Update, (move || EventSend(events_clone.remove(0))).pipe(affect));

            for expected in events {
                app.update();

                let events_in_update = app.world().resource::<Events<NumberEvent>>().iter_current_update_events().collect::<Vec<_>>();

                prop_assert_eq!(events_in_update.len(), 1);

                let event_sent = events_in_update.first().unwrap();

                prop_assert_eq!(event_sent, &&expected);
            }
        }
    }
}
