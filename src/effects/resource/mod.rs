//! [`Effect`]s that modify resources.
//!
//! [`Effect`]: crate::prelude::Effect

mod res_set;
pub use res_set::{ResSet, res_set};

mod res_set_with;
pub use res_set_with::{ResSetWith, res_set_with};

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
