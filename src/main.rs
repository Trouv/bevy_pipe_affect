use std::marker::PhantomData;

use bevy::{
    ecs::system::{StaticSystemInput, StaticSystemParam, SystemParam},
    prelude::*,
};

trait SideEffect {
    type MutParam<'w, 's>: SystemParam;

    fn affect(self, param: <Self::MutParam<'_, '_> as SystemParam>::Item<'_, '_>);
}

struct SetRes<R>(R)
where
    R: Resource;

impl<R> SideEffect for SetRes<R>
where
    R: Resource,
{
    type MutParam<'w, 's> = ResMut<'w, R>;

    fn affect(self, mut param: <Self::MutParam<'_, '_> as SystemParam>::Item<'_, '_>) {
        *param = self.0;
    }
}

fn affect_system<'w, 's, S>(In(effect): In<S>, param: StaticSystemParam<S::MutParam<'w, 's>>)
where
    S: SideEffect + 'w,
{
    effect.affect(param.into_inner())
}

fn pipe_side_effect<'w, 's, F, FMarker>(
    mut f: F,
) -> impl FnMut(
    StaticSystemInput<F::In>,
    ParamSet<
        'w,
        's,
        (
            F::Param,
            StaticSystemParam<'w, 's, <F::Out as SideEffect>::MutParam<'w, 's>>,
        ),
    >,
)
where
    F: SystemParamFunction<FMarker>,
    F::Out: SideEffect + 'w,
{
    move |StaticSystemInput(input), mut params| {
        let side_effect = f.run(input, params.p0());

        affect_system.run(side_effect, (params.p1(),));
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}
