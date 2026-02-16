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

/// [`Effect`] that reads all messages in a `MessageReader`, supplying them to the provided
/// effect-producing function to cause another effect.
///
/// The cursor of the message reader is updated.
///
/// Can be constructed with [`messages_read_and`].
#[derive(derive_more::Debug)]
pub struct MessagesReadAnd<M, E>
where
    M: Message,
    E: Effect,
{
    /// The `&Message -> Effect` function that may cause another effect.
    #[debug("{0} -> {1}", std::any::type_name::<&M>(), std::any::type_name::<E>())]
    pub f: Box<dyn FnMut(&M) -> E>,
}

/// Construct a new [`MessagesReadAnd`] [`Effect`].
pub fn messages_read_and<M, E, F>(f: F) -> MessagesReadAnd<M, E>
where
    M: Message,
    E: Effect,
    F: FnMut(&M) -> E + 'static,
{
    MessagesReadAnd { f: Box::new(f) }
}

impl<M, E> Default for MessagesReadAnd<M, E>
where
    M: Message,
    E: Effect + Default,
{
    fn default() -> Self {
        messages_read_and(|_| default())
    }
}

impl<M, E> Effect for MessagesReadAnd<M, E>
where
    M: Message,
    E: Effect,
{
    type MutParam = (MessageReader<'static, 'static, M>, E::MutParam);

    fn affect(
        mut self,
        param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) {
        param
            .0
            .read()
            .for_each(|message| (self.f)(message).affect(&mut param.1))
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
                .add_systems(Update, (move || message_write(messages_clone.remove(0))).pipe(affect));

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
