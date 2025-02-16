use std::marker::PhantomData;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::effect::Effect;

/// [`Effect`] that sets a `Resource` to the provided value.
pub struct ResPut<R>(pub R)
where
    R: Resource;

impl<R> Effect for ResPut<R>
where
    R: Resource,
{
    type MutParam = ResMut<'static, R>;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        **param = self.0;
    }
}

/// [`Effect`] that transforms a `Resource` with the provided function.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct ResWith<F, R>
where
    F: FnOnce(R) -> R,
    R: Resource + Clone,
{
    f: F,
    phantom: PhantomData<R>,
}

impl<F, R> ResWith<F, R>
where
    F: FnOnce(R) -> R,
    R: Resource + Clone,
{
    /// Construct a new [`ResWith`].
    pub fn new(f: F) -> Self {
        ResWith {
            f,
            phantom: PhantomData,
        }
    }
}

impl<F, R> Effect for ResWith<F, R>
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
    use std::hash::Hash;

    use blake2::digest::consts::{U32, U4};
    use blake2::{Blake2b, Digest};
    use proptest::prelude::*;
    use proptest_derive::Arbitrary;

    use super::*;
    use crate::system_combinators::affect;

    #[derive(Copy, Clone, Debug, PartialEq, Eq, Resource, Arbitrary)]
    struct NumberResource(i32);

    fn one_way_number_fn_fn(salt: Vec<u8>) -> impl FnOnce(NumberResource) -> NumberResource {
        move |NumberResource(input)| {
            let data = salt
                .into_iter()
                .chain(input.to_be_bytes())
                .collect::<Vec<_>>();

            let mut hasher = Blake2b::<U4>::new();
            hasher.update(data);
            let res = hasher.finalize();

            NumberResource(i32::from_be_bytes(res.into()))
        }
    }

    proptest! {
        #[test]
        fn res_put_overwrites_initial_state(initial: NumberResource, put: NumberResource) {
            let mut app = App::new();

            prop_assume!(initial != put);

            app.insert_resource(initial).add_systems(Update, (move || ResPut(put)).pipe(affect));

            prop_assert_eq!(app.world().resource::<NumberResource>(), &initial);

            app.update();

            prop_assert_eq!(app.world().resource::<NumberResource>(), &put);

        }

        #[test]
        fn res_with_correctly_executes_one_way_function(initial: NumberResource, salt: Vec<u8>) {
            let mut app = App::new();

            let expected = one_way_number_fn_fn(salt.clone())(initial);

            app.insert_resource(initial.clone()).add_systems(Update, (move || {
            ResWith::new(one_way_number_fn_fn(salt.clone()))
            }).pipe(affect));

            prop_assert_eq!(app.world().resource::<NumberResource>(), &initial);

            app.update();

            prop_assert_eq!(app.world().resource::<NumberResource>(), &expected);
        }
    }
}
