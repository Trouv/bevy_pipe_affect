//! [`Effect`] implementations for generic iterators.
//!
//! On top of the types shown here, this implements [`Effect`] for...
//! - `Vec<T>` where `T: Effect`
//! - `Option<T>` where `T: Effect`

use crate::Effect;

mod affect_many;
pub use affect_many::{AffectMany, affect_many};

impl<E> Effect for Vec<E>
where
    E: Effect,
{
    type MutParam = E::MutParam;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        AffectMany { iter: self }.affect(param);
    }
}

impl<E> Effect for Option<E>
where
    E: Effect,
{
    type MutParam = E::MutParam;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        AffectMany { iter: self }.affect(param);
    }
}

#[cfg(test)]
mod tests {

    use bevy::prelude::*;
    use proptest::prelude::*;

    use crate::effects::command::command_spawn;
    use crate::effects::message::message_write;
    use crate::effects::number_data::NumberComponent;
    use crate::effects::resource::res_set;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn vecs_can_spawn_many_entities(mut components in proptest::collection::vec(any::<NumberComponent<0>>(), 1..64)) {
            let mut app = App::new();

            let components_clone = components.clone();
            app.add_systems(Update, (move || components_clone.clone().into_iter().map(|c| command_spawn(c)).collect::<Vec<_>>()).pipe(affect));

            app.update();

            for num in app.world_mut().query::<&NumberComponent<0>>().iter(app.world()) {
                let index = components.iter().position(|c| c == num).unwrap();
                components.remove(index);
            }

            assert!(components.is_empty())
        }
    }

    #[test]
    fn option_can_sometimes_cause_effect() {
        #[derive(Message)]
        struct UpdateIsEven;

        #[derive(Resource)]
        struct NumUpdates(usize);

        let mut app = App::new();

        app.add_message::<UpdateIsEven>()
            .insert_resource(NumUpdates(0))
            .add_systems(
                Update,
                (|num_updates: Res<NumUpdates>| {
                    (
                        res_set(NumUpdates(num_updates.0 + 1)),
                        (num_updates.0 % 2 == 0).then_some(message_write(UpdateIsEven)),
                    )
                })
                .pipe(affect),
            );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Messages<UpdateIsEven>>()
                .iter_current_update_messages()
                .count(),
            1
        );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Messages<UpdateIsEven>>()
                .iter_current_update_messages()
                .count(),
            0
        );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Messages<UpdateIsEven>>()
                .iter_current_update_messages()
                .count(),
            1
        );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Messages<UpdateIsEven>>()
                .iter_current_update_messages()
                .count(),
            0
        );
    }
}
