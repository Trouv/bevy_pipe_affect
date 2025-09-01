use bevy::ecs::system::SystemParam;

/// Define a state transition in `bevy`'s ECS.
///
/// Can be returned by `bevy` systems and `pipe`d into [`affect`] to perform the transition.
///
/// [`affect`]: crate::system_combinators::affect
pub trait Effect {
    /// The `SystemParam` this effect mutates.
    type MutParam: SystemParam;

    /// Perform the state transition on the parameter.
    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>);
}
