use crate::Effect;
use crate::effects::iter::AffectMany;

impl<E> Effect for Vec<E>
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
    use crate::effects::number_data::NumberComponent;
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
}
