use bevy::ecs::system::SystemParam;

/// Define a state transition in `bevy`'s ECS.
///
/// Can be returned by `bevy` systems and `pipe`d into [`affect`] to perform the transition.
///
/// # Derive
/// *Requires the `derive` feature to be enabled.*
///
/// More complex effects can be derived for structs and enums whose fields also implement `Effect`,
/// if the `derive` cargo feature is enabled.
///
/// In the `enum` case, all params for all variants will still be accessed by the `affect` system,
/// but only the affects for the value's variant will be executed.
///
/// ```no_run
/// # #[cfg(feature = "derive")] {
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Component)]
/// struct Health(f32);
///
/// #[derive(Effect)]
/// struct DeathEffect<T: Effect> {
///     // If there are multiple fields, they will be affected in field order.
///     despawn: EntityCommandDespawn,
///     // Generic effects can be present as well
///     bonus_effect: T,
/// }
///
/// #[derive(Effect)]
/// enum HealthProcessEffect<T: Effect> {
///     Died(DeathEffect<T>),
///     Regenerating {
///         new_health: EntityComponentsSet<(Health,)>,
///     },
///     // Unit structs/variants will do nothing.
///     HealthFull,
/// }
/// # }
/// ```
///
/// [`affect`]: crate::system_combinators::affect
pub trait Effect {
    /// The `SystemParam` this effect mutates.
    type MutParam: SystemParam;

    /// Perform the state transition on the parameter.
    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>);
}
