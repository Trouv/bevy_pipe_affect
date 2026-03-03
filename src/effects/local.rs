use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::{Effect, EffectOut};

/// [`Effect`] that transforms a `Local<T>` parameter with the provided function, and can
/// potentially produce another effect `E`.
///
/// Note: the local parameter read/written to in this effect is local to this effect. It cannot be
/// read/written anywhere outside of `f`. E.g., the system that produces this effect cannot read it
/// with its own `Local<T>`, it will instead be an additional, independent local parameter.
#[derive(derive_more::Debug)]
pub struct LocalSetAnd<T, E>
where
    T: FromWorld + Send + 'static,
    E: Effect,
{
    /// The function taking the current value of the parameter and returning its new value and
    /// another effect `E`.
    #[expect(clippy::type_complexity)]
    #[debug("{0} -> {1}", std::any::type_name::<&T>(), std::any::type_name::<EffectOut<E, T>>())]
    pub f: Box<dyn FnOnce(&T) -> EffectOut<E, T>>,
}

/// Construct a new [`LocalSetAnd`] [`Effect`].
pub fn local_set_and<T, E, F>(f: F) -> LocalSetAnd<T, E>
where
    T: FromWorld + Send + 'static,
    E: Effect,
    F: FnOnce(&T) -> EffectOut<E, T> + 'static,
{
    LocalSetAnd { f: Box::new(f) }
}

impl<T, E> Default for LocalSetAnd<T, E>
where
    T: FromWorld + Send + Clone + 'static,
    E: Effect + Default,
{
    fn default() -> Self {
        local_set_and(|t: &T| EffectOut::from_out(t.clone()))
    }
}

impl<T, E> Effect for LocalSetAnd<T, E>
where
    T: FromWorld + Send + 'static,
    E: Effect,
{
    type MutParam = (Local<'static, T>, <E as Effect>::MutParam);

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        let EffectOut {
            effect,
            out: new_local,
        } = (self.f)(&param.0);

        *param.0 = new_local;

        effect.affect(&mut param.1);
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effect_out;
    use crate::effects::number_data::NumberResource;
    use crate::effects::one_way_fn::OneWayFn;
    use crate::effects::res_set;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn local_set_and_sets_value_and_produces_effect(one_way_fn: OneWayFn) {
            let mut app = App::new();

            app.init_resource::<NumberResource>().add_systems(
                Update,
                (move |non_effect_local: Local<u128>| {
                    // locals outside the effect are not affected.
                    assert_eq!(*non_effect_local, 0);

                    local_set_and(move |x: &u128| {
                        let new_value = x + 1;

                        // the resource value is a one way fn of the local value.
                        // so, asserting against it proves that it wasn't merely updated as a fn of
                        // its own state.
                        let resource_value = one_way_fn.call(new_value);
                        effect_out(res_set(NumberResource(resource_value)), new_value)
                    })
                })
                .pipe(affect),
            );

            assert_eq!(app.world().resource::<NumberResource>().0, 0);

            for i in 1..10u128 {
                app.update();
                assert_eq!(
                    app.world().resource::<NumberResource>().0,
                    one_way_fn.call(i)
                );
            }
        }
    }
}
