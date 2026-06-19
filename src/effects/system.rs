use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

use crate::prelude::*;

#[derive(derive_more::Debug)]
pub struct RunFnSystem<P, E>
where
    P: SystemParam + 'static,
    E: Effect + 'static,
{
    #[debug("{0} -> {1}", std::any::type_name::<P>(), std::any::type_name::<E>())]
    pub system: Box<dyn SystemParamFunction<fn(P) -> E, In = (), Param = (P,), Out = E>>,
}

pub fn run_fn_system<S, P, E>(system: S) -> RunFnSystem<P, E>
where
    S: SystemParamFunction<fn(P) -> E, In = (), Param = (P,), Out = E>,
    P: SystemParam + 'static,
    E: Effect + 'static,
{
    RunFnSystem {
        system: Box::new(system),
    }
}

impl<P, E> Effect for RunFnSystem<P, E>
where
    P: SystemParam + 'static,
    E: Effect + 'static,
{
    type MutParam = ParamSet<'static, 'static, ((P,), E::MutParam)>;

    fn affect(mut self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
        let effect = self.system.run((), param.p0());

        effect.affect(&mut param.p1());
    }
}
