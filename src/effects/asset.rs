//! [`Effect`]s that operate on `Assets` stores and the `AssetServer`.
use bevy::asset::{AssetPath, InvalidGenerationError};
use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that loads an asset, then supplies the asset handle to the provided
/// effect-producing function to cause another effect.
///
/// Can be constructed with [`asset_server_load_and`].
///
/// *Requires the `asset` feature to be enabled.*
///
/// # Example
/// In this example, a system is written that spawns a sprite with the "player.png" image.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// fn spawn_player_pure() -> AssetServerLoadAnd<'static, Image, CommandSpawn<Sprite>> {
///     asset_server_load_and("player.png", |handle| {
///         command_spawn(Sprite::from_image(handle))
///     })
/// }
///
/// fn spawn_player_impure(asset_server: Res<AssetServer>, mut commands: Commands) {
///     let handle = asset_server.load("player.png");
///     commands.spawn(Sprite::from_image(handle));
/// }
/// #
/// # fn app_setup() -> App {
/// #     let mut app = App::new();
/// #     app.add_plugins((
/// #         MinimalPlugins,
/// #         AssetPlugin::default(),
/// #         ImagePlugin::default_linear(),
/// #     ));
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<Handle<Image>>)> {
/// #     let mut query = world.query::<(Entity, Option<&Sprite>)>();
/// #     query
/// #         .iter(world)
/// #         .map(|(entity, sprite)| (entity, sprite.map(|sprite| sprite.image.clone())))
/// #         .collect()
/// # }
/// #
/// # fn main() {
/// #     let mut pure_app = app_setup();
/// #     pure_app.add_systems(Update, spawn_player_pure.pipe(affect));
/// #
/// #     let mut impure_app = app_setup();
/// #     impure_app.add_systems(Update, spawn_player_impure);
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
/// - in this example, a `CommandSpawn` is used as the additional [`Effect`], but other
///   [`Effect`]s are available.
#[derive(derive_more::Debug)]
pub struct AssetServerLoadAnd<'a, A, E>
where
    A: Asset,
    E: Effect,
{
    /// The path to the asset to load.
    pub path: AssetPath<'a>,
    /// The `Handle<A> -> Effect` function that may cause another effect.
    #[debug("{0} -> {1}", std::any::type_name::<Handle<A>>(), std::any::type_name::<E>())]
    pub f: Box<dyn FnOnce(Handle<A>) -> E>,
}

/// Construct a new [`AssetServerLoadAnd`] [`Effect`], with an extra effect using the `Handle<A>`.
///
/// *Requires the `asset` feature to be enabled.*
pub fn asset_server_load_and<'a, P, F, A, E>(path: P, f: F) -> AssetServerLoadAnd<'a, A, E>
where
    P: Into<AssetPath<'a>>,
    F: FnOnce(Handle<A>) -> E + 'static,
    A: Asset,
    E: Effect,
{
    AssetServerLoadAnd {
        path: path.into(),
        f: Box::new(f),
    }
}

impl<'a, A, E> Effect for AssetServerLoadAnd<'a, A, E>
where
    A: Asset,
    E: Effect,
{
    type MutParam = (Res<'static, AssetServer>, E::MutParam);

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let handle = param.0.load(self.path);
        (self.f)(handle).affect(&mut param.1);
    }
}

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

/// [`Effect`] that inserts an `Asset` to the asset store with the given `AssetId` (overwriting any
/// existing asset at that id).
///
/// Can be constructed with [`asset_insert`], which can conveniently convert from a `&Handle<A>`.
///
/// *Requires the `asset` feature to be enabled.*
///
/// # Example
/// In this example, we have a custom `AnimationMap` asset for storing animation indices by name,
/// and a system is written that resets the player's animation map asset to a basic setting
/// (defined by `base_player_animation_map`).
/// ```
/// use std::collections::HashMap;
/// use std::ops::Range;
///
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Default, Debug, PartialEq, Eq, Reflect, Asset, Clone)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct AnimationMap(HashMap<String, Range<usize>>);
///
/// #[derive(Resource)]
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
/// fn reset_player_animations_pure(
///     player_animations_handle: Res<PlayerAnimationsHandle>,
/// ) -> AssetInsert<AnimationMap> {
///     asset_insert(&player_animations_handle.0, base_player_animation_map())
/// }
///
/// /// Equivalent impure system.
/// fn reset_player_animations_impure(
///     player_animations_handle: Res<PlayerAnimationsHandle>,
///     mut assets: ResMut<Assets<AnimationMap>>,
/// ) -> Result<(), BevyError> {
///     Ok(assets.insert(
///         &player_animations_handle.0,
///         base_player_animation_map()
///     )?)
/// }
/// #
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(assets: Vec<AnimationMap>, player_handle_index: usize) -> App {
/// #     let mut app = App::new();
/// #
/// #     app.add_plugins(AssetPlugin::default())
/// #         .init_asset::<AnimationMap>();
/// #
/// #     let handles =
/// #             assets
/// #                 .into_iter()
/// #                 .map(|asset| app.world_mut().add_asset(asset))
/// #         .collect::<Vec<_>>();
/// #
/// #     app.insert_resource(PlayerAnimationsHandle(
/// #         handles[player_handle_index % handles.len()].clone(),
/// #     ));
/// #
/// #     app
/// # }
/// #
/// # fn asset_state(world: &World) -> Vec<(AssetId<AnimationMap>, &AnimationMap)> {
/// #     world
/// #         .get_resource::<Assets<AnimationMap>>()
/// #         .unwrap()
/// #         .iter()
/// #         .collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(assets in proptest::collection::vec(any::<AnimationMap>(), 1..=10), player_handle_index: usize) {
/// #         let mut pure_app = app_setup(assets.clone(), player_handle_index);
/// #         pure_app.add_systems(Update, reset_player_animations_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(assets.clone(), player_handle_index);
/// #         impure_app.add_systems(Update, reset_player_animations_impure);
/// #
/// #         for _ in 0..3 {
/// #              assert_eq!(asset_state(pure_app.world_mut()), asset_state(impure_app.world_mut()));
/// #              pure_app.update();
/// #              impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub struct AssetInsert<A>
where
    A: Asset,
{
    /// The id to assign the `Asset` in the asset store.
    pub id: AssetId<A>,
    /// The `Asset` to insert at the id.
    pub asset: A,
}

/// Construct a new [`AssetInsert`] [`Effect`].
pub fn asset_insert<IntoAssetId, A>(id: IntoAssetId, asset: A) -> AssetInsert<A>
where
    IntoAssetId: Into<AssetId<A>>,
    A: Asset,
{
    AssetInsert {
        id: id.into(),
        asset,
    }
}

impl<A> Effect for AssetInsert<A>
where
    A: Asset,
{
    type MutParam = (
        ResMut<'static, Assets<A>>,
        <Result<(), InvalidGenerationError> as Effect>::MutParam,
    );

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        match param.0.insert(self.id, self.asset) {
            Ok(()) => (),
            e => e.affect(&mut param.1),
        }
    }
}

#[cfg(test)]
mod tests {

    use bevy::asset::LoadState;

    use super::*;
    use crate::effects::command::command_insert_resource;
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
