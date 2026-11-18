//! [`Effect`]s for error handling.
//!
//! On top of the types shown here, this implements [`Effect`] for...
//! - `Result<T, E>` where `T: Effect` and `E: Into<BevyError>`
use bevy::ecs::error::FallbackErrorHandler;
use bevy::ecs::system::{SystemChangeTick, SystemName};
use bevy::prelude::*;

use crate::Effect;

mod affect_or_handle;
pub use affect_or_handle::{AffectOrHandle, affect_or_handle};

impl<Ef, Er> Effect for Result<Ef, Er>
where
    Ef: Effect,
    Er: Into<BevyError>,
{
    type MutParam = (
        Option<Res<'static, FallbackErrorHandler>>,
        (Ef::MutParam, SystemName, SystemChangeTick),
    );

    fn affect(
        self,
        (default_error_handler, param): &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>,
    ) {
        affect_or_handle(
            self,
            default_error_handler
                .as_deref()
                .copied()
                .unwrap_or_default()
                .0,
        )
        .affect(param);
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use bevy::ecs::query::QuerySingleError;

    use super::*;
    use crate::effects::command::{CommandSpawn, command_spawn};
    use crate::effects::entity_command::{EntityCommandInsert, EntityCommandRemove};
    use crate::prelude::affect;

    #[derive(Component)]
    struct Blueprint;

    #[derive(Component)]
    struct ProcessedBlueprint;

    fn spawn_blueprint_component(
        processed_blueprints: Query<(), Or<(With<Blueprint>, With<ProcessedBlueprint>)>>,
    ) -> Option<CommandSpawn<Blueprint>> {
        processed_blueprints
            .is_empty()
            .then_some(command_spawn(Blueprint))
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
    >
    where
        F: Fn(BevyError, ErrorContext) + Clone + 'static,
    {
        move |blueprints| {
            affect_or_handle(
                blueprints.single().map(|entity| {
                    (
                        EntityCommandRemove::<Blueprint>::new(entity),
                        EntityCommandInsert {
                            entity,
                            bundle: ProcessedBlueprint,
                        },
                    )
                }),
                error_handler.clone(),
            )
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
