use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that reads all messages in a `MessageReader`, supplying them to the provided
/// effect-producing function to cause another effect.
///
/// The cursor of the message reader is updated.
///
/// Can be constructed with [`messages_read_and`].
///
/// # Example
/// In this example, a system is written that increments an entity's `Wins` counter if they are
/// declared a winner by the `Winner` message.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Wins(u8);
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Message)]
/// struct Winner(Entity);
///
/// /// Pure system using effects.
/// fn tally_wins_pure() -> MessagesReadAnd<Winner, QueryEntityMap<&'static Wins, ComponentSet<Wins>>> {
///     messages_read_and(|&Winner(entity)| {
///         query_entity_map(entity, |&Wins(n)| component_set(Wins(n.saturating_add(1))))
///     })
/// }
///
/// /// Equivalent impure system.
/// fn tally_wins_impure(mut query: Query<&mut Wins>, mut reader: MessageReader<Winner>) {
///     reader.read().for_each(|Winner(entity)| {
///         if let Ok(mut wins) = query.get_mut(*entity) {
///             wins.0 = wins.0.saturating_add(1);
///         }
///     });
/// }
/// #
/// # use bevy::ecs::error::{ignore, FallbackErrorHandler};
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(
/// #     component_table: Vec<Option<Wins>>,
/// #     mut winner_indices_per_update: Vec<Vec<usize>>,
/// # ) -> App {
/// #     let mut app = App::new();
/// #     app.add_message::<Winner>()
/// #         .insert_resource(FallbackErrorHandler(ignore));
/// #
/// #     let entities = component_table
/// #         .into_iter()
/// #         .map(|wins| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #             if let Some(wins) = wins {
/// #                 entity.insert(wins);
/// #             }
/// #
/// #             entity.id()
/// #         })
/// #         .collect::<Vec<_>>();
/// #
/// #     app.add_systems(
/// #         PreUpdate,
/// #         (move || {
/// #             winner_indices_per_update.pop().map(|winner_indices| {
/// #                 winner_indices
/// #                     .into_iter()
/// #                     .map(|winner_index| {
/// #                         message_write(Winner(entities[winner_index % entities.len()]))
/// #                     })
/// #                     .collect::<Vec<_>>()
/// #             })
/// #         })
/// #         .pipe(affect),
/// #     );
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Wins>)> {
/// #     let mut query = world.query::<(Entity, Option<&Wins>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table in proptest::collection::vec(any::<Option<Wins>>(), 1..64), winner_indices: Vec<Vec<usize>>) {
/// #         let mut pure_app = app_setup(component_table.clone(), winner_indices.clone());
/// #         pure_app.add_systems(Update, tally_wins_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table.clone(), winner_indices.clone());
/// #         impure_app.add_systems(Update, tally_wins_impure);
/// #
/// #         for _ in 0..winner_indices.len() + 1 {
/// #             prop_assert_eq!(test_state(pure_app.world_mut()), test_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - in this example, [`QueryEntityMap`] is used as the [`Effect`], but any other effect could be produced.
///
/// [`QueryEntityMap`]: crate::prelude::QueryEntityMap
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
