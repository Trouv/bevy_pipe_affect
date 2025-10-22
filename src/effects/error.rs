use bevy::ecs::error::{DefaultErrorHandler, ErrorContext};
use bevy::ecs::system::{SystemChangeTick, SystemName};
use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that causes the `Ok` effect, or handles the `Err` with a custom handler.
///
/// This handler can be parameterized by any of the `bevy`-provided error handlers.
///
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// fn zero_red_clear_color_srgba(clear_color: Res<ClearColor>) -> impl Effect {
///     let result = match clear_color.0 {
///         Color::Srgba(srgba) => {
///             let color = Color::Srgba(Srgba { red: 0., ..srgba });
///             Ok(ResSet {
///                 value: ClearColor(color),
///             })
///         }
///         _ => Err("color is not srgba"),
///     };
///
///     AffectOrHandle {
///         result,
///         handler: bevy::ecs::error::warn,
///     }
/// }
///
/// bevy::ecs::system::assert_is_system(zero_red_clear_color_srgba.pipe(affect))
/// ```
///
/// Using a plain `Result` as an effect works too, but uses `bevy`'s `DefaultErrorHandler`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AffectOrHandle<Ef, Er, Handler>
where
    Ef: Effect,
    Er: Into<BevyError>,
    Handler: FnOnce(BevyError, ErrorContext),
{
    /// The result to be affected or handled.
    pub result: Result<Ef, Er>,
    /// The handler to use in the `Err` case.
    pub handler: Handler,
}

impl<Ef, Er, Handler> Effect for AffectOrHandle<Ef, Er, Handler>
where
    Ef: Effect,
    Er: Into<BevyError>,
    Handler: FnOnce(BevyError, ErrorContext),
{
    type MutParam = (Ef::MutParam, SystemName, SystemChangeTick);

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        match self.result {
            Ok(ef) => ef.affect(&mut param.0),
            Err(er) => (self.handler)(
                er.into(),
                ErrorContext::System {
                    name: param.1.name(),
                    last_run: param.2.last_run(),
                },
            ),
        }
    }
}

impl<Ef, Er> Effect for Result<Ef, Er>
where
    Ef: Effect,
    Er: Into<BevyError>,
{
    type MutParam = (
        Option<Res<'static, DefaultErrorHandler>>,
        (Ef::MutParam, SystemName, SystemChangeTick),
    );

    fn affect(
        self,
        (default_error_handler, param): &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) {
        AffectOrHandle {
            result: self,
            handler: default_error_handler
                .as_deref()
                .copied()
                .unwrap_or_default()
                .0,
        }
        .affect(param);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use bevy::ecs::query::QuerySingleError;

    use super::*;
    use crate::effects::{CommandSpawnAnd, EntityCommandInsert, EntityCommandRemove};
    use crate::prelude::affect;

    #[derive(Component)]
    struct Blueprint;

    #[derive(Component)]
    struct ProcessedBlueprint;

    fn spawn_blueprint_component(
        processed_blueprints: Query<(), Or<(With<Blueprint>, With<ProcessedBlueprint>)>>,
    ) -> impl Effect {
        processed_blueprints.is_empty().then_some(CommandSpawnAnd {
            bundle: Blueprint,
            f: |_| (),
        })
    }

    fn process_blueprint_component<F>(
        error_handler: F,
    ) -> impl Fn(
        Query<Entity, With<Blueprint>>,
    ) -> AffectOrHandle<
        (
            EntityCommandRemove<Blueprint>,
            EntityCommandInsert<ProcessedBlueprint>,
        ),
        QuerySingleError,
        F,
    >
    where
        F: Fn(BevyError, ErrorContext) + Clone,
    {
        move |blueprints| AffectOrHandle {
            result: blueprints.single().map(|entity| {
                (
                    EntityCommandRemove::<Blueprint>::new(entity),
                    EntityCommandInsert {
                        entity,
                        bundle: ProcessedBlueprint,
                    },
                )
            }),
            handler: error_handler.clone(),
        }
    }

    fn logs_and_error_handler() -> (
        Arc<Mutex<Vec<(BevyError, ErrorContext)>>>,
        impl Fn(BevyError, ErrorContext) + Clone,
    ) {
        let errors = Arc::new(Mutex::new(Vec::new()));

        let handler = {
            let errors = errors.clone();
            move |bevy_error, error_context| {
                errors.lock().unwrap().push((bevy_error, error_context));
            }
        };

        (errors, handler)
    }

    #[test]
    fn affect_or_handle_can_process_blueprint_or_log_error() {
        let mut app = App::new();

        let (logs, error_handler) = logs_and_error_handler();

        app.add_systems(
            Update,
            (
                spawn_blueprint_component.pipe(affect),
                process_blueprint_component(error_handler).pipe(affect),
            ),
        );

        let mut update_and_assert_counts =
            |blueprint_count: usize, processed_blueprint_count: usize, error_count: usize| {
                app.update();

                assert_eq!(
                    app.world_mut()
                        .query::<&Blueprint>()
                        .iter(app.world())
                        .count(),
                    blueprint_count
                );
                assert_eq!(
                    app.world_mut()
                        .query::<&ProcessedBlueprint>()
                        .iter(app.world())
                        .count(),
                    processed_blueprint_count
                );
                assert_eq!(logs.lock().unwrap().len(), error_count);
            };

        update_and_assert_counts(1, 0, 1);

        update_and_assert_counts(0, 1, 1);

        update_and_assert_counts(0, 1, 2);
    }
}
