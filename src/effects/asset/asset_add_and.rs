use bevy::prelude::*;

/// [`Effect`] that adds an asset to the asset store, then supplies the asset handle to the provided
/// effect-producing function to cause another effect.
///
/// Can be constructed with [`asset_add_and`].
///
/// *Requires the `asset` feature to be enabled.*
///
/// # Example
/// In this example, we have a custom `AnimationMap` asset for storing animation indices by name,
/// and a system is written that adds a player animation map to the asset store and stores it in a
/// resource.
/// ```
/// use std::collections::HashMap;
/// use std::ops::Range;
///
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Default, Debug, PartialEq, Eq, Reflect, Asset, Clone)]
/// struct AnimationMap(HashMap<String, Range<usize>>);
///
/// #[derive(Debug, PartialEq, Eq, Resource)]
/// struct PlayerAnimationsHandle(Handle<AnimationMap>);
///
/// fn base_player_animation_map() -> AnimationMap {
///     AnimationMap(HashMap::from_iter([
///         ("idle".to_string(), 0..1),
///         ("running".to_string(), 1..5),
///     ]))
/// }
///
/// /// Pure system using effects.
/// fn init_player_animations_pure()
/// -> AssetAddAnd<AnimationMap, CommandInsertResource<PlayerAnimationsHandle>> {
///     asset_add_and(base_player_animation_map(), |handle| {
///         command_insert_resource(PlayerAnimationsHandle(handle))
///     })
/// }
///
/// /// Equivalent impure system.
/// fn init_player_animations_impure(
///     mut assets: ResMut<Assets<AnimationMap>>,
///     mut commands: Commands,
/// ) {
///     let handle = assets.add(base_player_animation_map());
///     commands.insert_resource(PlayerAnimationsHandle(handle));
/// }
/// #
/// # fn app_setup() -> App {
/// #     let mut app = App::new();
/// #
/// #     app.add_plugins(AssetPlugin::default())
/// #         .init_asset::<AnimationMap>();
/// #
/// #     app
/// # }
/// #
/// # fn test_state(
/// #     world: &World,
/// # ) -> (
/// #     Vec<(AssetId<AnimationMap>, &AnimationMap)>,
/// #     Option<&PlayerAnimationsHandle>,
/// # ) {
/// #     let all_assets = world
/// #         .get_resource::<Assets<AnimationMap>>()
/// #         .unwrap()
/// #         .iter()
/// #         .collect();
/// #     let resource = world.get_resource::<PlayerAnimationsHandle>();
/// #
/// #     (all_assets, resource)
/// # }
/// #
/// # fn main() {
/// #     let mut pure_app = app_setup();
/// #     pure_app.add_systems(Update, init_player_animations_pure.pipe(affect));
/// #
/// #     let mut impure_app = app_setup();
/// #     impure_app.add_systems(Update, init_player_animations_impure);
/// #
/// #     for _ in 0..3 {
/// #         assert_eq!(
/// #             test_state(pure_app.world_mut()),
/// #             test_state(impure_app.world_mut())
/// #         );
/// #         pure_app.update();
/// #         impure_app.update();
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - in this example, `CommandInsertResource` is used as the additional [`Effect`], but other
///   [`Effect`]s are available.
#[derive(derive_more::Debug)]
pub struct AssetAddAnd<A, E>
where
    A: Asset,
    E: Effect,
{
    /// The asset to be added to the asset store.
    pub asset: A,
    /// The `Handle<A> -> Effect` function that may cause another effect.
    #[debug("{0} -> {1}", std::any::type_name::<Handle<A>>(), std::any::type_name::<E>())]
    pub f: Box<dyn FnOnce(Handle<A>) -> E>,
}

/// Construct a new [`AssetAddAnd`] [`Effect`].
pub fn asset_add_and<A, E, F>(asset: A, f: F) -> AssetAddAnd<A, E>
where
    A: Asset,
    E: Effect,
    F: FnOnce(Handle<A>) -> E + 'static,
{
    AssetAddAnd {
        asset,
        f: Box::new(f),
    }
}

impl<A, E> Default for AssetAddAnd<A, E>
where
    A: Asset + Default,
    E: Effect + Default,
{
    fn default() -> Self {
        asset_add_and(default(), |_| default())
    }
}

impl<A, E> Effect for AssetAddAnd<A, E>
where
    A: Asset,
    E: Effect,
{
    type MutParam = (ResMut<'static, Assets<A>>, E::MutParam);

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let handle = param.0.add(self.asset);
        (self.f)(handle).affect(&mut param.1);
    }
}

use crate::Effect;

#[cfg(test)]
mod tests {

    use super::*;
    use crate::effects::command::command_insert_resource;
    use crate::prelude::affect;

    #[derive(Resource)]
    struct PlayerSprite(Handle<Image>);

    #[test]
    fn asset_add_and_adds_asset() {
        let mut app = App::new();

        let image = Image::default();

        let image_clone = image.clone();

        app.add_plugins((
            MinimalPlugins,
            AssetPlugin::default(),
            ImagePlugin::default_linear(),
        ))
        .add_systems(
            Startup,
            (move || {
                asset_add_and(image_clone.clone(), |handle| {
                    command_insert_resource(PlayerSprite(handle))
                })
            })
            .pipe(affect),
        );

        app.update();
        let player_sprite_handle = &app.world().resource::<PlayerSprite>().0;

        let assets = app.world().resource::<Assets<Image>>();
        assert_eq!(assets.get(player_sprite_handle), Some(&image));
    }
}
