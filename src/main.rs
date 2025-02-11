use bevy::{
    ecs::system::{StaticSystemParam, SystemParam},
    prelude::*,
    utils::all_tuples,
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

macro_rules! impl_effect {
    ($(($E:ident, $e:ident, $p:ident)),*) => {
        impl<$($E),*> Effect for ($($E,)*)
        where $($E: Effect,)* {
            type MutParam = ParamSet<'static, 'static, ($(<$E as Effect>::MutParam,)*)>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let ($($e,)*) = self;
                $($e.affect(&mut param.$p());)*
            }
        }
    };
}

all_tuples!(impl_effect, 1, 8, E, e, p);

struct EffectOut<E, O>(E, O)
where
    E: Effect;

fn affect<E, O>(
    In(EffectOut(effect, out)): In<EffectOut<E, O>>,
    param: StaticSystemParam<E::MutParam>,
) -> O
where
    E: Effect,
{
    effect.affect(&mut param.into_inner());

    out
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
