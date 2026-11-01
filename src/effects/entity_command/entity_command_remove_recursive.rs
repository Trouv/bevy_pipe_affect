use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that removes a component/bundle recursively from an entity and its relationships.
///
/// Can be constructed with [`entity_command_remove_recursive`].
///
/// # Example
/// In this example, a system is written that removes the `Curse` from the `BlessedEntity` and its
/// children.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Copy, Clone, PartialEq, Resource)]
/// struct BlessedEntity(Entity);
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Curse;
///
/// /// Pure system using effects.
/// fn bless_pure(blessed_entity: Res<BlessedEntity>) -> EntityCommandRemoveRecursive<Children, Curse> {
///     entity_command_remove_recursive(blessed_entity.0)
/// }
///
/// /// Equivalent impure system.
/// fn bless_impure(blessed_entity: Res<BlessedEntity>, mut commands: Commands) {
///     commands
///         .entity(blessed_entity.0)
///         .remove_recursive::<Children, Curse>();
/// }
/// # use proptest::prelude::*;
/// #
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, proptest_derive::Arbitrary)]
/// # struct ParentIndex(usize);
/// #
/// # fn app_setup(
/// #     component_table: Vec<(Option<Curse>, Option<ParentIndex>)>,
/// #     cured_index: usize,
/// # ) -> App {
/// #     let mut app = App::new();
/// #
/// #     let entities = component_table.into_iter().fold(
/// #         vec![app.world_mut().spawn_empty().id()],
/// #         |mut entities, (curse, parent_index)| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #
/// #             if let Some(curse) = curse {
/// #                 entity.insert(curse);
/// #             }
/// #
/// #             if let Some(parent_index) = parent_index {
/// #                 let parent = entities[parent_index.0 % entities.len()];
/// #                 entity.insert(ChildOf(parent));
/// #             }
/// #
/// #             entities.push(entity.id());
/// #
/// #             entities
/// #         },
/// #     );
/// #
/// #     let cured_entity = entities[cured_index % entities.len()];
/// #
/// #     app.insert_resource(BlessedEntity(cured_entity));
/// #
/// #     app
/// # }
/// #
/// # fn test_state(world: &mut World) -> Vec<(Entity, Option<&Curse>, Option<&ChildOf>)> {
/// #     let mut query = world.query::<(Entity, Option<&Curse>, Option<&ChildOf>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<(Option<Curse>, Option<ParentIndex>)>, cured_index: usize) {
/// #         let mut pure_app = app_setup(component_table.clone(), cured_index);
/// #         pure_app.add_systems(Update, bless_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table, cured_index);
/// #         impure_app.add_systems(Update, bless_impure);
/// #
/// #         for _ in 0..3 {
/// #             prop_assert_eq!(test_state(pure_app.world_mut()), test_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - A single component is used in this example, but the removed type is a `Bundle`, so it can be
/// a `Bundle` struct or tuple of components.
/// - A parent/child relationship is used in this example, but any other `RelationshipTarget` would
/// work.
#[doc = include_str!("../defer_command_note.md")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct EntityCommandRemoveRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle,
{
    /// The entity that the bundle is removed from recursively.
    pub entity: Entity,
    bundle: PhantomData<B>,
    relationship_target: PhantomData<RT>,
}

impl<RT, B> EntityCommandRemoveRecursive<RT, B>
where
    B: Bundle,
    RT: RelationshipTarget,
{
    /// Construct a new [`EntityCommandRemoveRecursive`].
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            bundle: PhantomData,
            relationship_target: PhantomData,
        }
    }
}

/// Construct a new [`EntityCommandRemoveRecursive`] [`Effect`].
pub fn entity_command_remove_recursive<RT, B>(entity: Entity) -> EntityCommandRemoveRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle,
{
    EntityCommandRemoveRecursive::new(entity)
}

impl<RT, B> Effect for EntityCommandRemoveRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param.entity(self.entity).remove_recursive::<RT, B>();
    }
}
