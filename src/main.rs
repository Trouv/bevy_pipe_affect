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

fn and_compose<E1, O1, System, Marker, E2, O2, E3>(
    mut s: System,
    compose_fn: impl Fn(E1, E2) -> E3,
) -> impl FnMut(In<EffectOut<E1, O1>>, StaticSystemParam<System::Param>) -> EffectOut<E3, O2>
where
    System: SystemParamFunction<Marker, Out = EffectOut<E2, O2>>,
    E1: Effect,
    E2: Effect,
    E3: Effect,
    for<'a> System::In: SystemInput<Inner<'a> = O1>,
{
    move |In(EffectOut(e1, input)), params| {
        let EffectOut(e2, out) = s.run(input, params.into_inner());

        EffectOut(compose_fn(e1, e2), out)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<NumUpdates>()
        .add_systems(
            Update,
            sample_system_with_effect_and_output
                .pipe(and_compose(
                    sample_system_with_effect_and_input,
                    |e1, e2| (e1, e2),
                ))
                .pipe(affect),
        )
        .run();
}

fn sample_system_with_effect_and_input(
    In(theta): In<f32>,
    current: Res<ClearColor>,
) -> EffectOut<UpdateRes<ClearColor>, ()> {
    EffectOut(UpdateRes(ClearColor(current.0.rotate_hue(theta))), ())
}

#[derive(Resource, Default)]
struct NumUpdates(u32);

fn sample_system_with_effect_and_output(
    num_updates: Res<NumUpdates>,
) -> EffectOut<UpdateRes<NumUpdates>, f32> {
    EffectOut(
        UpdateRes(NumUpdates(num_updates.0 + 1)),
        (num_updates.0 % 10) as f32 / 10.,
    )
}
