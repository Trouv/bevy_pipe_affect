use bevy::prelude::*;

use crate::Effect;
/// [`Effect`] that queues a command for triggering the given event.
///
/// Can be constructed with [`command_trigger`].
///
/// # Example
/// In this example, a system is written that triggers a `Winner` event if any entity has a score
/// above 100.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Score(u8);
///
/// #[derive(Copy, Clone, Debug, PartialEq, Eq, Event)]
/// struct Winner(Entity);
///
/// /// Pure system using effects.
/// fn declare_winner_pure(query: Query<(Entity, &Score)>) -> Option<CommandTrigger<Winner>> {
///     query
///         .iter()
///         .find(|(_, score)| score.0 >= 100)
///         .map(|(entity, _)| command_trigger(Winner(entity)))
/// }
///
/// /// Equivalent impure system.
/// fn declare_winner_impure(query: Query<(Entity, &Score)>, mut commands: Commands) {
///     if let Some((entity, _)) = query.iter().find(|(_, score)| score.0 >= 100) {
///         commands.trigger(Winner(entity))
///     }
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # fn reset_winner_score(winner: On<Winner>) -> QueryEntityAffect<ComponentSet<Score>> {
/// #     query_entity_affect(winner.0, component_set(Score(0)))
/// # }
/// #
/// # fn app_setup(component_table: Vec<Option<Score>>) -> App {
/// #     let mut app = App::new();
/// #     app.add_systems(
/// #         Update,
/// #         (|| command_spawn(Observer::new(reset_winner_score.pipe(affect)))).pipe(affect),
/// #     );
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
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Score>)> {
/// #     let mut query = world.query::<(Entity, Option<&Score>)>();
/// #     query.iter(world).collect()
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
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandTrigger<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    /// The event being triggered.
    pub event: E,
}

/// Construct a new [`CommandTrigger`] [`Effect`].
pub fn command_trigger<E>(event: E) -> CommandTrigger<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    CommandTrigger { event }
}

impl<E> Effect for CommandTrigger<E>
where
    E: Event,
    for<'a> E::Trigger<'a>: Default,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.trigger(self.event);
    }
}
