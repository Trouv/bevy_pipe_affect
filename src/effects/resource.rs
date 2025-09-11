use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::effect::Effect;

/// [`Effect`] that sets a `Resource` to the provided value.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ResSet<R>
where
    R: Resource,
{
    /// The value that the resource will be set to.
    pub value: R,
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
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ResSetWith<F, R>
where
    F: FnOnce(R) -> R,
    R: Resource + Clone,
{
    f: F,
    phantom: PhantomData<R>,
}

impl<F, R> ResSetWith<F, R>
where
    F: FnOnce(R) -> R,
    R: Resource + Clone,
{
    /// Construct a new [`ResSetWith`].
    pub fn new(f: F) -> Self {
        ResSetWith {
            f,
            phantom: PhantomData,
        }
    }
}

impl<F, R> Effect for ResSetWith<F, R>
where
    F: FnOnce(R) -> R,
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
                .add_systems(Update, (move || ResSet { value: put }).pipe(affect));

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
                (move || ResSetWith::new(move |NumberResource(n)| NumberResource(f.call(n)))).pipe(affect),
            );

            prop_assert_eq!(app.world().resource::<NumberResource>(), &initial);

            app.update();

            prop_assert_eq!(app.world().resource::<NumberResource>(), &expected);
        }
    }
}
