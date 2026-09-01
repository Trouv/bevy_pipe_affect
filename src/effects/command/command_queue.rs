use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that pushes a generic command to the command queue.
///
/// This effect is mostly intended to "fill the gaps" of `bevy_pipe_affect`, as not all commands or
/// world mutations provided by bevy have an equivalent effect.
/// Using this typically requires writing some impure code anyway, so users should also consider
/// writing their own custom [`Effect`] instead.
/// (Then, if it's general-purpose enough, consider contributing it upstream!)
///
/// Can be constructed with [`command_queue`].
///
/// # Example
/// In this example, a system is written that gives all `Player`s a `Score` of 0 if they do not
/// already have a score.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Player;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Score(u32);
///
/// #[derive(Clone, Debug, Default, PartialEq, Eq)]
/// struct InitScores(Vec<Entity>);
///
/// impl Command for InitScores {
///     type Out = ();
///
///     fn apply(self, world: &mut World) {
///         world.insert_batch_if_new(self.0.into_iter().map(|entity| (entity, Score(0))));
///     }
/// }
///
/// /// Pure-ish system using effects.
/// fn init_player_score_pureish(query: Query<Entity, With<Player>>) -> CommandQueue<InitScores> {
///     command_queue(InitScores(query.iter().collect()))
/// }
///
/// /// Equivalent impure system.
/// fn init_player_score_impure(query: Query<Entity, With<Player>>, mut commands: Commands) {
///     commands.queue(InitScores(query.iter().collect()));
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(component_table: Vec<(Option<Player>, Option<Score>)>) -> App {
/// #     let mut app = App::new();
/// #
/// #     component_table.into_iter().for_each(|(player, score)| {
/// #         let mut entity = app.world_mut().spawn_empty();
/// #         if let Some(player) = player {
/// #             entity.insert(player);
/// #         }
/// #         if let Some(score) = score {
/// #             entity.insert(score);
/// #         }
/// #     });
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Player>, Option<&Score>)> {
/// #     let mut query = world.query::<(Entity, Option<&Player>, Option<&Score>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<(Option<Player>, Option<Score>)>) {
/// #          let mut pure_app = app_setup(component_table.clone());
/// #          pure_app.add_systems(Update, init_player_score_pureish.pipe(affect));
/// #
/// #          let mut impure_app = app_setup(component_table.clone());
/// #          impure_app.add_systems(Update, init_player_score_impure);
/// #
/// #          for _ in 0..3 {
/// #              prop_assert_eq!(test_state(pure_app.world_mut()), test_state(impure_app.world_mut()));
/// #              pure_app.update();
/// #              impure_app.update();
/// #          }
/// #     }
/// # }
/// ```
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandQueue<C>
where
    C: Command,
{
    /// The command to push onto the queue.
    pub command: C,
}

/// Construct a new [`CommandQueue`] [`Effect`].
pub fn command_queue<C>(command: C) -> CommandQueue<C>
where
    C: Command,
{
    CommandQueue { command }
}

impl<C> Effect for CommandQueue<C>
where
    C: Command,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.queue(self.command)
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberComponent;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn command_queue_can_spawn_entities_non_exclusively(component in any::<NumberComponent<0>>()) {
            let mut app = App::new();

            let component_count = app.world_mut().query_filtered::<(), With<NumberComponent<0>>>().iter(app.world()).count();

            assert_eq!(component_count, 0);

            let spawn_component_system = move || {
                command_queue(move |world: &mut World| {
                    world.spawn(component.clone());
                })
            };


            assert!(!IntoSystem::into_system(spawn_component_system.pipe(affect)).is_exclusive());

            app.add_systems(
                Update,
                spawn_component_system.pipe(affect),
            );

            app.update();

            let component_count = app.world_mut().query_filtered::<(), With<NumberComponent<0>>>().iter(app.world()).count();

            assert_eq!(component_count, 1);

            app.update();

            let component_count = app.world_mut().query_filtered::<(), With<NumberComponent<0>>>().iter(app.world()).count();

            assert_eq!(component_count, 2);
        }
    }
}
