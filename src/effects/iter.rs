//! [`Effect`] implementations for generic iterators.
//!
//! On top of the types shown here, this implements [`Effect`] for...
//! - `Vec<T>` where `T: Effect`
//! - `Option<T>` where `T: Effect`
use crate::Effect;

/// [`Effect`] that causes all effects in the provided iterator.
///
/// Using a plain `Vec` or `Option` as an effect works too.
///
/// Can be constructed with [`affect_many`].
///
/// # Example
/// In this example, a system is written that spawns 20 empty entities.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// /// Pure system using effects.
/// fn spawn_20_pure() -> AffectMany<std::iter::RepeatN<CommandSpawn<()>>> {
///     affect_many(std::iter::repeat_n(command_spawn(()), 20))
/// }
///
/// /// Equivalent impure system.
/// fn spawn_20_impure(mut commands: Commands) {
///     for _ in 0..20 {
///         commands.spawn_empty();
///     }
/// }
/// #
/// # fn app_setup() -> App {
/// #     App::new()
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<Entity> {
/// #     let mut query = world.query::<Entity>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # fn main() {
/// #     let mut pure_app = app_setup();
/// #     pure_app.add_systems(Update, spawn_20_pure.pipe(affect));
/// #
/// #     let mut impure_app = app_setup();
/// #     impure_app.add_systems(Update, spawn_20_impure);
/// #
/// #     for _ in 0..3 {
/// #         assert_eq!(
/// #             test_state(pure_app.world_mut()),
/// #             test_state(impure_app.world_mut())
/// #         );
/// #         pure_app.update();
/// #         impure_app.update();
/// #     }
/// # }
/// ```
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

impl<I> IntoIterator for AffectMany<I>
where
    I: IntoIterator,
    I::Item: Effect,
{
    type Item = I::Item;
    type IntoIter = I::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter.into_iter()
    }
}

impl<I, E> Extend<E> for AffectMany<I>
where
    I: IntoIterator<Item = E> + Extend<E>,
    E: Effect,
{
    fn extend<T: IntoIterator<Item = E>>(&mut self, iter: T) {
        self.iter.extend(iter)
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
