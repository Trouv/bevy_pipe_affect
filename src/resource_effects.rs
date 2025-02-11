use bevy::{ecs::system::SystemParam, prelude::*};

use crate::effect::Effect;

pub struct UpdateRes<R>(pub R)
where
    R: Resource;

impl<R> Effect for UpdateRes<R>
where
    R: Resource,
{
    type MutParam = ResMut<'static, R>;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        **param = self.0;
    }
}
