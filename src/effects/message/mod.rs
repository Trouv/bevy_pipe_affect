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
    use crate::effects::number_data::{NumberMessage, NumberResource};
    use crate::effects::resource::res_set_with;
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

        #[test]
        fn messages_read_and_reads_all_available_and_produces_effects(message_bundles in prop::collection::vec(prop::collection::vec(any::<NumberMessage>(), 0..10), 0..10)) {
            let mut app = App::new();

            let mut message_bundles_write_clone = message_bundles.clone();

            app.add_message::<NumberMessage>()
                .init_resource::<NumberResource>()
                .add_systems(
                    Update,
                    (
                        // An effect that writes the messages, one bundle at a time
                        (move || {
                            message_bundles_write_clone
                                .remove(0)
                                .into_iter()
                                .map(message_write)
                                .collect::<Vec<_>>()
                        })
                        .pipe(affect),
                        // An effect that reads the messages and sums their values into a resource
                        (|| {
                            messages_read_and(move |m: &NumberMessage| {
                                let m = m.0;
                                res_set_with(move |n: &NumberResource| NumberResource(n.0.wrapping_add(m)))
                            })
                        })
                        .pipe(affect),
                    )
                        // Chained to make sure the writing system comes before the reading system
                        .chain(),
                );

            assert_eq!(app.world().resource::<NumberResource>().0, 0);

            for i in 0..message_bundles.len() {
                app.update();

                let expected_written_so_far = message_bundles
                    .iter()
                    // i + 1 updates have occurred, so i + 1 bundles have been written.
                    .take(i + 1)
                    .flatten()
                    .map(|m| m.0)
                    .reduce(u128::wrapping_add)
                    .unwrap_or_default();

                assert_eq!(
                    app.world().resource::<NumberResource>().0,
                    expected_written_so_far as u128
                );
            }
        }
    }
}
