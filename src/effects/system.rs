use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::prelude::*;

pub type BoxedSystemEffect<P, E> =
    Box<dyn SystemParamFunction<fn(P) -> E, In = (), Param = (P,), Out = E>>;

impl<P, E> Effect for BoxedSystemEffect<P, E>
where
    P: SystemParam + 'static,
    E: Effect + 'static,
{
    type MutParam = ParamSet<'static, 'static, ((P,), E::MutParam)>;

    fn affect(mut self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        let effect = self.run((), param.p0());

        effect.affect(&mut param.p1());
    }
}
