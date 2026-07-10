use bevy::asset::InvalidGenerationError;
use bevy::prelude::*;

use crate::Effect;

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
