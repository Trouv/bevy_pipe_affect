use bevy::ecs::component::Mutable;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::effect::Effect;

/// [`Effect`] that sets a `Resource` to the provided value.
///
/// Can be constructed by [`res_set`].
///
/// # Example
/// In this example, a system is written that resets the `Score` to 0.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Resource)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Score(u32);
///
/// /// Pure system using effects.
/// fn reset_score_pure() -> ResSet<Score> {
///     res_set(Score(0))
/// }
///
/// /// Equivalent impure system.
/// fn reset_score_impure(mut score: ResMut<Score>) {
///     score.0 = 0;
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(score: Score) -> App {
/// #     let mut app = App::new();
/// #     app.insert_resource(score);
/// #     app
/// # }
/// #
/// # fn resource_state(world: &World) -> &Score {
/// #     world.get_resource::<Score>().unwrap()
/// # }
/// #
/// # proptest! {
/// #     fn main(score: Score) {
/// #         let mut pure_app = app_setup(score);
/// #         pure_app.add_systems(Update, reset_score_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(score);
/// #         impure_app.add_systems(Update, reset_score_impure);
/// #
/// #         for _ in 0..3 {
/// #              prop_assert_eq!(resource_state(pure_app.world_mut()), resource_state(impure_app.world_mut()));
/// #              pure_app.update();
/// #              impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ResSet<R>
where
    R: Resource + Component<Mutability = Mutable>,
{
    /// The value that the resource will be set to.
    pub value: R,
}

/// Construct a new [`ResSet`] [`Effect`].
pub fn res_set<R>(value: R) -> ResSet<R>
where
    R: Resource + Component<Mutability = Mutable>,
{
    ResSet { value }
}

impl<R> Effect for ResSet<R>
where
    R: Resource + Component<Mutability = Mutable>,
{
    type MutParam = ResMut<'static, R>;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        **param = self.value;
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberResource;
    use crate::system_combinators::affect;

    proptest! {
        #[test]
        fn res_set_overwrites_initial_state(initial: NumberResource, put: NumberResource) {
            let mut app = App::new();

            prop_assume!(initial != put);

            app.insert_resource(initial)
                .add_systems(Update, (move || res_set(put)).pipe(affect));

            prop_assert_eq!(app.world().resource::<NumberResource>(), &initial);

            app.update();

            prop_assert_eq!(app.world().resource::<NumberResource>(), &put);
        }
    }
}
