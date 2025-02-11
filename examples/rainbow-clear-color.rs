use bevy::prelude::*;
use bevy_pipe_affect::{
    effect::EffectOut,
    resource_effects::ResPut,
    system_combinators::{affect, and_compose},
};

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
) -> EffectOut<ResPut<ClearColor>, ()> {
    EffectOut(ResPut(ClearColor(current.0.rotate_hue(theta))), ())
}

#[derive(Resource, Default)]
struct NumUpdates(u32);

fn sample_system_with_effect_and_output(
    num_updates: Res<NumUpdates>,
) -> EffectOut<ResPut<NumUpdates>, f32> {
    EffectOut(
        ResPut(NumUpdates(num_updates.0 + 1)),
        (num_updates.0 % 10) as f32 / 10.,
    )
}
