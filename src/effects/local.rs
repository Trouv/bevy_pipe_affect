use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::{Effect, EffectOut};

#[derive(derive_more::Debug)]
pub struct LocalSetAnd<T, E>
where
    T: FromWorld + Send + 'static,
    E: Effect,
{
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
