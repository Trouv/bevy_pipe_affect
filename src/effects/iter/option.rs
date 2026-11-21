use crate::Effect;
use crate::effects::iter::AffectMany;

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

    use crate::effects::message::message_write;
    use crate::effects::resource::res_set;
    use crate::prelude::affect;

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
