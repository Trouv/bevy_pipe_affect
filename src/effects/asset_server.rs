use std::marker::PhantomData;

use bevy::asset::AssetPath;
use bevy::prelude::*;

use crate::Effect;

struct AssetServerLoadAnd<'a, F, A, E>
where
    F: FnOnce(Handle<A>) -> E,
    A: Asset,
    E: Effect,
{
    pub path: AssetPath<'a>,
    pub f: F,
    asset: PhantomData<A>,
}

impl<'a, F, A, E> Effect for AssetServerLoadAnd<'a, F, A, E>
where
    A: Asset,
    F: FnOnce(Handle<A>) -> E,
    E: Effect,
{
    type MutParam = (Res<'static, AssetServer>, E::MutParam);

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let handle = param.0.load(self.path);
        (self.f)(handle).affect(&mut param.1);
    }
}
