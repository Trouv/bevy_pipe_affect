use std::marker::PhantomData;

use bevy::asset::AssetPath;
use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that loads an asset, then supplies the asset handle to the provided
/// effect-producing function to cause another effect.
///
/// Can be constructed with [`asset_server_load_and`].
pub struct AssetServerLoadAnd<'a, F, A, E>
where
    F: FnOnce(Handle<A>) -> E,
    A: Asset,
    E: Effect,
{
    /// The path to the asset to load.
    pub path: AssetPath<'a>,
    /// The `Handle<A> -> Effect` function that may cause another effect.
    pub f: F,
    asset: PhantomData<A>,
}

impl<'a, F, A, E> AssetServerLoadAnd<'a, F, A, E>
where
    F: FnOnce(Handle<A>) -> E,
    A: Asset,
    E: Effect,
{
    fn new<P>(path: P, f: F) -> Self
    where
        P: Into<AssetPath<'a>>,
    {
        AssetServerLoadAnd {
            path: path.into(),
            f,
            asset: PhantomData,
        }
    }
}

/// Construct a new [`AssetServerLoadAnd`] [`Effect`], with an extra effect using the `Handle<A>`.
pub fn asset_server_load_and<'a, P, F, A, E>(path: P, f: F) -> AssetServerLoadAnd<'a, F, A, E>
where
    P: Into<AssetPath<'a>>,
    F: FnOnce(Handle<A>) -> E,
    A: Asset,
    E: Effect,
{
    AssetServerLoadAnd::new(path, f)
}

impl<'a, F, A, E> Effect for AssetServerLoadAnd<'a, F, A, E>
where
    A: Asset,
    F: FnOnce(Handle<A>) -> E,
    E: Effect,
{
    type MutParam = (Res<'static, AssetServer>, E::MutParam);

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let handle = param.0.load(self.path);
        (self.f)(handle).affect(&mut param.1);
    }
}

#[cfg(test)]
mod tests {

    use bevy::asset::LoadState;

    use super::*;
    use crate::effects::command_insert_resource;
    use crate::prelude::affect;

    #[derive(Resource)]
    struct PlayerSprite(Handle<Image>);

    #[test]
    fn asset_server_load_loads_asset() {
        let mut app = App::new();

        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            ImagePlugin::default_linear(),
        ))
        .add_systems(
            Startup,
            (|| {
                asset_server_load_and("player.png", |handle| {
                    command_insert_resource(PlayerSprite(handle))
                })
            })
            .pipe(affect),
        );

        app.update();
        let player_sprite_handle = &app.world().resource::<PlayerSprite>().0;

        let asset_server = app.world().resource::<AssetServer>();
        assert!(matches!(
            asset_server.get_load_state(player_sprite_handle),
            Some(LoadState::Loading)
        ));
    }
}
