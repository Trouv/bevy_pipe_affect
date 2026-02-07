use std::convert::identity;
use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::effect::Effect;

/// [`Effect`] that sets a `Resource` to the provided value.
///
/// Can be constructed by [`res_set`].
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

/// [`Effect`] that transforms a `Resource` with the provided function.
///
/// Can be constructed by [`res_set_with`].
#[derive(derive_more::Debug)]
pub struct ResSetWith<R>
where
    R: Resource + Clone,
{
    #[debug("R -> R")]
    f: Box<dyn FnOnce(R) -> R>,
    phantom: PhantomData<R>,
}

impl<R> ResSetWith<R>
where
    R: Resource + Clone,
{
    /// Construct a new [`ResSetWith`].
    pub fn new(f: Box<dyn FnOnce(R) -> R>) -> Self {
        ResSetWith {
            f,
            phantom: PhantomData,
        }
    }
}

/// Construct a new [`ResSetWith`] [`Effect`].
pub fn res_set_with<F, R>(f: F) -> ResSetWith<R>
where
    F: FnOnce(R) -> R + 'static,
    R: Resource + Clone,
{
    ResSetWith::new(Box::new(f))
}

impl<R> Default for ResSetWith<R>
where
    R: Resource + Clone,
{
    fn default() -> Self {
        res_set_with(identity)
    }
}

impl<R> Effect for ResSetWith<R>
where
    R: Resource + Clone,
{
    type MutParam = ResMut<'static, R>;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        **param = (self.f)(param.clone());
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
                (move || res_set_with(move |NumberResource(n)| NumberResource(f.call(n)))).pipe(affect),
            );

            prop_assert_eq!(app.world().resource::<NumberResource>(), &initial);

            app.update();

            prop_assert_eq!(app.world().resource::<NumberResource>(), &expected);
        }
    }
}
