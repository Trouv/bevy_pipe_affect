use crate::Effect;

/// [`Effect`] that causes all effects in the provided iterator.
///
/// Using a plain `Vec` or `Option` as an effect works too.
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
pub struct AffectMany<I>(pub I)
where
    I: IntoIterator,
    I::Item: Effect;

impl<I> Effect for AffectMany<I>
where
    I: IntoIterator,
    I::Item: Effect,
{
    type MutParam = <I::Item as Effect>::MutParam;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        self.0.into_iter().for_each(|e| {
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
        AffectMany(self).affect(param);
    }
}

impl<E> Effect for Option<E>
where
    E: Effect,
{
    type MutParam = E::MutParam;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        AffectMany(self).affect(param);
    }
}

#[cfg(test)]
mod tests {

    use bevy::prelude::*;
    use proptest::prelude::*;

    use crate::effects::number_data::NumberComponent;
    use crate::effects::{CommandSpawnAnd, EventWrite, ResPut};
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn vecs_can_spawn_many_entities(mut components in proptest::collection::vec(any::<NumberComponent<0>>(), 1..64)) {
            let mut app = App::new();

            let components_clone = components.clone();
            app.add_systems(Update, (move || components_clone.clone().into_iter().map(|c| CommandSpawnAnd(c, |_| ())).collect::<Vec<_>>()).pipe(affect));

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
        #[derive(Event)]
        struct UpdateIsEven;

        #[derive(Resource)]
        struct NumUpdates(usize);

        let mut app = App::new();

        app.add_event::<UpdateIsEven>()
            .insert_resource(NumUpdates(0))
            .add_systems(
                Update,
                (|num_updates: Res<NumUpdates>| {
                    (
                        ResPut(NumUpdates(num_updates.0 + 1)),
                        (num_updates.0 % 2 == 0).then_some(EventWrite(UpdateIsEven)),
                    )
                })
                .pipe(affect),
            );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Events<UpdateIsEven>>()
                .iter_current_update_events()
                .count(),
            1
        );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Events<UpdateIsEven>>()
                .iter_current_update_events()
                .count(),
            0
        );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Events<UpdateIsEven>>()
                .iter_current_update_events()
                .count(),
            1
        );

        app.update();

        assert_eq!(
            app.world()
                .resource::<Events<UpdateIsEven>>()
                .iter_current_update_events()
                .count(),
            0
        );
    }
}
