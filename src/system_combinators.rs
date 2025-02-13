use bevy::{ecs::system::StaticSystemParam, prelude::*};

use crate::effect::{Effect, EffectOut};

pub fn affect<I, E, O>(In(into_effect_out): In<I>, param: StaticSystemParam<E::MutParam>) -> O
where
    I: Into<EffectOut<E, O>>,
    E: Effect,
{
    let EffectOut(effect, out) = into_effect_out.into();
    effect.affect(&mut param.into_inner());

    out
}

pub fn and_compose<I1, E1, O1, System, Marker, I2, E2, O2, E3>(
    mut s: System,
    compose_fn: impl Fn(E1, E2) -> E3,
) -> impl FnMut(In<I1>, StaticSystemParam<System::Param>) -> EffectOut<E3, O2>
where
    System: SystemParamFunction<Marker, Out = I2>,
    I1: Into<EffectOut<E1, O1>>,
    I2: Into<EffectOut<E2, O2>>,
    E1: Effect,
    E2: Effect,
    E3: Effect,
    for<'a> System::In: SystemInput<Inner<'a> = O1>,
{
    move |In(into_effect_out), params| {
        let EffectOut(e1, input) = into_effect_out.into();
        let EffectOut(e2, out) = s.run(input, params.into_inner()).into();

        EffectOut(compose_fn(e1, e2), out)
    }
}
