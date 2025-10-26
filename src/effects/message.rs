use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that sends a message `M` to the corresponding `MessageWriter`.
///
/// Can be constructed with [`message_write`].
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct MessageWrite<M>
where
    M: Message,
{
    /// The message data that will be written to the `MessageWriter`.
    pub message: M,
}

/// Construct a new [`MessageWrite`] [`Effect`].
pub fn message_write<M>(message: M) -> MessageWrite<M>
where
    M: Message,
{
    MessageWrite { message }
}

impl<M> Effect for MessageWrite<M>
where
    M: Message,
{
    type MutParam = MessageWriter<'static, M>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.write(self.message);
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberMessage;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn message_send_produces_messages(messages in prop::collection::vec(any::<NumberMessage>(), 1..10)) {
            let mut app = App::new();

            let mut messages_clone = messages.clone();
            app.add_message::<NumberMessage>()
                .add_systems(Update, (move || MessageWrite { message: messages_clone.remove(0) }).pipe(affect));

            for expected in messages {
                app.update();

                let messages_in_update = app.world().resource::<Messages<NumberMessage>>().iter_current_update_messages().collect::<Vec<_>>();

                prop_assert_eq!(messages_in_update.len(), 1);

                let message_sent = messages_in_update.first().unwrap();

                prop_assert_eq!(message_sent, &&expected);
            }
        }
    }
}
