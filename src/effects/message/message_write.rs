use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that sends a message `M` to the corresponding `MessageWriter`.
///
/// Can be constructed with [`message_write`].
///
/// # Example
/// In this example, a system is written that writes a `Winner` message if any entity has a score
/// above 100.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Score(u8);
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Message)]
/// struct Winner(Entity);
///
/// /// Pure system using effects.
/// fn declare_winner_pure(query: Query<(Entity, &Score)>) -> Option<MessageWrite<Winner>> {
///     query
///         .iter()
///         .find(|(_, score)| score.0 >= 100)
///         .map(|(entity, _)| message_write(Winner(entity)))
/// }
///
/// /// Equivalent impure system.
/// fn declare_winner_impure(query: Query<(Entity, &Score)>, mut writer: MessageWriter<Winner>) {
///     if let Some((entity, _)) = query.iter().find(|(_, score)| score.0 >= 100) {
///         writer.write(Winner(entity));
///     }
/// }
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(component_table: Vec<Option<Score>>) -> App {
/// #     let mut app = App::new();
/// #     app.add_message::<Winner>();
/// #     component_table.into_iter().for_each(|score| {
/// #         let mut entity = app.world_mut().spawn_empty();
/// #         if let Some(score) = score {
/// #             entity.insert(score);
/// #         }
/// #     });
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &World) -> Vec<&Winner> {
/// #     world
/// #         .resource::<Messages<Winner>>()
/// #         .iter_current_update_messages()
/// #         .collect::<Vec<_>>()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<Option<Score>>) {
/// #         let mut pure_app = app_setup(component_table.clone());
/// #         pure_app.add_systems(Update, declare_winner_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table.clone());
/// #         impure_app.add_systems(Update, declare_winner_impure);
/// #
/// #         for _ in 0..3 {
/// #             prop_assert_eq!(test_state(pure_app.world_mut()), test_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
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
