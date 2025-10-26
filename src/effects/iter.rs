use crate::Effect;

/// [`Effect`] that causes all effects in the provided iterator.
///
/// Using a plain `Vec` or `Option` as an effect works too.
///
/// Can be constructed with [`affect_many`].
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct AffectMany<I>
where
    I: IntoIterator,
    I::Item: Effect,
{
    /// The iterator of effects to affect.
    pub iter: I,
}

/// Construct a new [`AffectMany`] [`Effect`].
pub fn affect_many<I>(iter: I) -> AffectMany<I>
where
    I: IntoIterator,
    I::Item: Effect,
{
    AffectMany { iter }
}

impl<I> Effect for AffectMany<I>
where
    I: IntoIterator,
    I::Item: Effect,
{
    type MutParam = <I::Item as Effect>::MutParam;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        self.iter.into_iter().for_each(|e| {
            e.affect(param);
        });
    }
}

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

    use crate::effects::number_data::NumberComponent;
    use crate::effects::{command_spawn, message_write, res_set};
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
