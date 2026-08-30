use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that queues a command for removing a `Resource` from the `World`.
///
/// Can be constructed with [`command_remove_resource`].
///
/// # Example
/// In this example, a system is written that removes the `GoToLevel` resource (presumably, after
/// some level transition has completed).
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Resource)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct GoToLevel(usize);
///
/// /// Pure system using effects.
/// fn complete_level_transition_pure() -> CommandRemoveResource<GoToLevel> {
///     command_remove_resource::<GoToLevel>()
/// }
///
/// /// Equivalent impure system.
/// fn complete_level_transition_impure(mut commands: Commands) {
///     commands.remove_resource::<GoToLevel>()
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(resource: Option<GoToLevel>) -> App {
/// #     let mut app = App::new();
/// #
/// #     if let Some(resource) = resource {
/// #         app.insert_resource(resource);
/// #     }
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &World) -> Option<&GoToLevel> {
/// #     world.get_resource::<GoToLevel>()
/// # }
/// #
/// # proptest! {
/// #     fn main(resource: Option<GoToLevel>) {
/// #         let mut pure_app = app_setup(resource);
/// #         pure_app.add_systems(Update, complete_level_transition_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(resource);
/// #         impure_app.add_systems(Update, complete_level_transition_impure);
/// #
/// #         for _ in 0..3 {
/// #              prop_assert_eq!(test_state(pure_app.world_mut()), test_state(impure_app.world_mut()));
/// #              pure_app.update();
/// #              impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
#[doc = include_str!("../defer_command_note.md")]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CommandRemoveResource<R>
where
    R: Resource,
{
    resource: PhantomData<R>,
}

impl<R> CommandRemoveResource<R>
where
    R: Resource,
{
    /// Construct a new [`CommandRemoveResource`]
    pub fn new() -> Self {
        CommandRemoveResource {
            resource: PhantomData,
        }
    }
}

/// Construct a new [`CommandRemoveResource`] [`Effect`].
pub fn command_remove_resource<R>() -> CommandRemoveResource<R>
where
    R: Resource,
{
    CommandRemoveResource::new()
}

impl<R> Effect for CommandRemoveResource<R>
where
    R: Resource,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.remove_resource::<R>();
    }
}
