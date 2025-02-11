use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
};

trait Effect {
    type MutParam: SystemParam;

    fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>);
}

struct UpdateRes<R>(R)
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

fn affect<S>(In(effect): In<S>, param: StaticSystemParam<S::MutParam>)
where
    S: Effect,
{
    effect.affect(param.into_inner())
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, sample_system_with_effect.pipe(affect))
        .run();
}

fn sample_system_with_effect(current: Res<ClearColor>) -> UpdateRes<ClearColor> {
    UpdateRes(ClearColor(current.0.rotate_hue(1.)))
}
