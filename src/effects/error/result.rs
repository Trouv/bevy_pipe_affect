use bevy::ecs::error::FallbackErrorHandler;
use bevy::ecs::system::{SystemChangeTick, SystemName};
use bevy::prelude::*;

use crate::Effect;
use crate::effects::error::affect_or_handle;

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
