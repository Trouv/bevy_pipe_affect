//! [`Effect`]s that modify resources.
use std::any::type_name;

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
    R: Resource,
{
    /// The value that the resource will be set to.
    pub value: R,
}

/// Construct a new [`ResSet`] [`Effect`].
pub fn res_set<R>(value: R) -> ResSet<R>
where
    R: Resource,
{
    ResSet { value }
}

impl<R> Effect for ResSet<R>
where
    R: Resource,
{
    type MutParam = ResMut<'static, R>;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        **param = self.value;
    }
}

/// [`Effect`] that transforms a `Resource` with the provided `R -> R` function.
///
/// Can be constructed by [`res_set_with`].
///
/// # Example
/// In this example, a system is written that increments the `Updates` by 1.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Resource)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Updates(u32);
///
/// /// Pure system using effects.
/// fn increment_updates_pure() -> ResSetWith<Updates> {
///     res_set_with(|Updates(n)| Updates(n + 1))
/// }
///
/// /// Equivalent impure system.
/// fn increment_updates_impure(mut updates: ResMut<Updates>) {
///     updates.0 += 1;
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(updates: Updates) -> App {
/// #     let mut app = App::new();
/// #     app.insert_resource(updates);
/// #     app
/// # }
/// #
/// # fn resource_state(world: &World) -> &Updates {
/// #     world.get_resource::<Updates>().unwrap()
/// # }
/// #
/// # proptest! {
/// #     fn main(updates: Updates) {
/// #         let mut pure_app = app_setup(updates);
/// #         pure_app.add_systems(Update, increment_updates_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(updates);
/// #         impure_app.add_systems(Update, increment_updates_impure);
/// #
/// #         for _ in 0..20 {
/// #              prop_assert_eq!(resource_state(pure_app.world_mut()), resource_state(impure_app.world_mut()));
/// #              pure_app.update();
/// #              impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
#[derive(derive_more::Debug)]
pub struct ResSetWith<R>
where
    R: Resource,
{
    /// The function that maps the resource to its new value.
    #[debug("{} -> {}", type_name::<&R>(), type_name::<R>())]
    pub f: Box<dyn FnOnce(&R) -> R>,
}

/// Construct a new [`ResSetWith`] [`Effect`].
pub fn res_set_with<F, R>(f: F) -> ResSetWith<R>
where
    F: FnOnce(&R) -> R + 'static,
    R: Resource,
{
    ResSetWith { f: Box::new(f) }
}

impl<R> Default for ResSetWith<R>
where
    R: Resource + Clone,
{
    fn default() -> Self {
        res_set_with(|r: &R| r.clone())
    }
}

impl<R> Effect for ResSetWith<R>
where
    R: Resource,
{
    type MutParam = ResMut<'static, R>;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        **param = (self.f)(param);
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::number_data::NumberResource;
    use crate::effects::one_way_fn::OneWayFn;
    use crate::system_combinators::affect;

    proptest! {
        #[test]
        fn res_put_overwrites_initial_state(initial: NumberResource, put: NumberResource) {
            let mut app = App::new();

            prop_assume!(initial != put);

            app.insert_resource(initial)
                .add_systems(Update, (move || res_set(put)).pipe(affect));

            prop_assert_eq!(app.world().resource::<NumberResource>(), &initial);

            app.update();

            prop_assert_eq!(app.world().resource::<NumberResource>(), &put);
        }

        #[test]
        fn res_with_correctly_executes_one_way_function(initial: NumberResource, f: OneWayFn) {
            let expected = NumberResource(f.call(initial.0));

            let mut app = App::new();

            app.insert_resource(initial.clone()).add_systems(
                Update,
                (move || res_set_with(move |&NumberResource(n)| NumberResource(f.call(n)))).pipe(affect),
            );

            prop_assert_eq!(app.world().resource::<NumberResource>(), &initial);

            app.update();

            prop_assert_eq!(app.world().resource::<NumberResource>(), &expected);
        }
    }
}
