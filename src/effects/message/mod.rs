//! [`Effect`]s that modify `MessageReader`s and `MessageWriter`s.
//!
//! [`Effect`]: crate::prelude::Effect

mod message_write;
pub use message_write::{MessageWrite, message_write};

mod messages_read_and;
pub use messages_read_and::{MessagesReadAnd, messages_read_and};

#[cfg(test)]
mod tests {
    use bevy::prelude::*;
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberMessage;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn message_write_produces_messages(messages in prop::collection::vec(any::<NumberMessage>(), 1..10)) {
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
