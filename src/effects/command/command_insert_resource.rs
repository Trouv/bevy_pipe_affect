use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that queues a command for inserting the provided `Resource` in the `World`.
///
/// Can be constructed with [`command_insert_resource`].
///
/// # Example
/// In this example, a system is written that inserts a `Score` of 0.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Resource)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Score(u32);
///
/// /// Pure system using effects.
/// fn init_score_pure() -> CommandInsertResource<Score> {
///     command_insert_resource(Score(0))
/// }
///
/// /// Equivalent impure system.
/// fn init_score_impure(mut commands: Commands) {
///     commands.insert_resource(Score(0))
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(score: Option<Score>) -> App {
/// #     let mut app = App::new();
/// #     if let Some(score) = score {
/// #         app.insert_resource(score);
/// #     }
/// #
/// #     app
/// # }
/// #
/// # fn resource_state(world: &World) -> Option<&Score> {
/// #     world.get_resource::<Score>()
/// # }
/// #
/// # proptest! {
/// #     fn main(score: Option<Score>) {
/// #         let mut pure_app = app_setup(score);
/// #         pure_app.add_systems(Update, init_score_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(score);
/// #         impure_app.add_systems(Update, init_score_impure);
/// #
/// #         for _ in 0..3 {
/// #              prop_assert_eq!(resource_state(pure_app.world_mut()), resource_state(impure_app.world_mut()));
/// #              pure_app.update();
/// #              impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandInsertResource<R>
where
    R: Resource,
{
    /// The initial value of the inserted resource.
    pub resource: R,
}

/// Construct a new [`CommandInsertResource`] [`Effect`].
pub fn command_insert_resource<R>(resource: R) -> CommandInsertResource<R>
where
    R: Resource,
{
    CommandInsertResource { resource }
}

impl<R> Effect for CommandInsertResource<R>
where
    R: Resource,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.insert_resource(self.resource);
    }
}
