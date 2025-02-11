use bevy::{ecs::system::StaticSystemParam, prelude::*};
use effect::{Effect, EffectOut};

pub mod effect;
pub mod resource_effects;

pub fn affect<E, O>(
    In(EffectOut(effect, out)): In<EffectOut<E, O>>,
    param: StaticSystemParam<E::MutParam>,
) -> O
where
    E: Effect,
{
    effect.affect(&mut param.into_inner());

    out
}

pub fn and_compose<E1, O1, System, Marker, E2, O2, E3>(
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
