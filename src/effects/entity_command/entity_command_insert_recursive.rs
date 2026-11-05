use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

/// [`Effect`] that inserts a component/bundle recursively on an entity and its relationships.
///
/// Can be constructed with [`entity_command_insert_recursive`].
///
/// # Example
/// In this example, a system is written that marks entities and their childern as `Frozen` if
/// their `Temperature` is below zero.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Temperature(i8);
///
/// #[derive(Debug, Default, Copy, Clone, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Frozen;
///
/// /// Pure system using effects.
/// fn freeze_pure(
///     query: Query<(Entity, &Temperature)>,
/// ) -> Vec<EntityCommandInsertRecursive<Children, Frozen>> {
///     query
///         .iter()
///         .filter(|(_, temp)| temp.0 <= 0)
///         .map(|(entity, _)| entity_command_insert_recursive(entity, Frozen))
///         .collect()
/// }
///
/// /// Equivalent impure system.
/// fn freeze_impure(query: Query<(Entity, &Temperature)>, mut commands: Commands) {
///     for (entity, temp) in query.iter() {
///         if temp.0 <= 0 {
///             commands.entity(entity).insert_recursive::<Children>(Frozen);
///         }
///     }
/// }
/// # use proptest::prelude::*;
/// #
/// # #[derive(Debug, Copy, Clone, PartialEq, Eq, proptest_derive::Arbitrary)]
/// # struct ParentIndex(usize);
/// #
/// # fn app_setup(component_table: Vec<(Option<Temperature>, Option<ParentIndex>)>) -> App {
/// #     let mut app = App::new();
/// #
/// #     let _entities = component_table.into_iter().fold(
/// #         vec![app.world_mut().spawn_empty().id()],
/// #         |mut entities, (temp, parent_index)| {
/// #             let mut entity = app.world_mut().spawn_empty();
/// #
/// #             if let Some(temp) = temp {
/// #                 entity.insert(temp);
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
/// #     app
/// # }
/// #
/// # fn test_state(
/// #     world: &mut World,
/// # ) -> Vec<(
/// #     Entity,
/// #     Option<&Temperature>,
/// #     Option<&ChildOf>,
/// #     Option<&Frozen>,
/// # )> {
/// #     let mut query = world.query::<(
/// #         Entity,
/// #         Option<&Temperature>,
/// #         Option<&ChildOf>,
/// #         Option<&Frozen>,
/// #     )>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<(Option<Temperature>, Option<ParentIndex>)>) {
/// #         let mut pure_app = app_setup(component_table.clone());
/// #         pure_app.add_systems(Update, freeze_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table);
/// #         impure_app.add_systems(Update, freeze_impure);
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
/// - A single component is used in this example, but the inserted value is a `Bundle`, so it can
/// be a `Bundle` struct or tuple of components.
/// - A parent/child relationship is used in this example, but any other `RelationshipTarget` would
/// work.
#[doc = include_str!("../defer_command_note.md")]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct EntityCommandInsertRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle + Clone,
{
    /// The entity that is inserted to recursively.
    pub entity: Entity,
    /// The bundle being inserted on relationship targets.
    pub bundle: B,
    relationship_target: PhantomData<RT>,
}

impl<RT, B> EntityCommandInsertRecursive<RT, B>
where
    B: Bundle + Clone,
    RT: RelationshipTarget,
{
    /// Construct a new [`EntityCommandInsertRecursive`].
    pub fn new(entity: Entity, bundle: B) -> Self {
        Self {
            entity,
            bundle,
            relationship_target: PhantomData,
        }
    }
}

/// Construct a new [`EntityCommandInsertRecursive`] [`Effect`].
pub fn entity_command_insert_recursive<RT, B>(
    entity: Entity,
    bundle: B,
) -> EntityCommandInsertRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle + Clone,
{
    EntityCommandInsertRecursive::new(entity, bundle)
}

impl<RT, B> Effect for EntityCommandInsertRecursive<RT, B>
where
    RT: RelationshipTarget,
    B: Bundle + Clone,
{
    type MutParam = Commands<'static, 'static>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param
            .entity(self.entity)
            .insert_recursive::<RT>(self.bundle);
    }
}
