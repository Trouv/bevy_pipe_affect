//! [`Effect`]s that queue entity-specific `Commands`.
use std::marker::PhantomData;

use bevy::prelude::*;

use crate::Effect;

mod entity_command_queue;
pub use entity_command_queue::{EntityCommandQueue, entity_command_queue};

mod entity_command_insert;
pub use entity_command_insert::{EntityCommandInsert, entity_command_insert};

mod entity_command_remove;
pub use entity_command_remove::{EntityCommandRemove, entity_command_remove};

mod entity_command_despawn;
pub use entity_command_despawn::{EntityCommandDespawn, entity_command_despawn};

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

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::effects::command::{command_insert_resource, command_spawn_and};
    use crate::effects::number_data::NumberComponent;
    use crate::prelude::affect;

    proptest! {
        #[test]
        fn entity_command_queue_can_insert_component_non_exclusively(component in any::<NumberComponent<0>>()) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(()).id();

            let actual_component = app.world().get_entity(entity).unwrap().get_components::<&NumberComponent<0>>();

            assert!(actual_component.is_err());

            let insert_component_system = move || {
                EntityCommandQueue::new(entity, move |mut entity_world: EntityWorldMut<'_>| {
                    entity_world.insert(component.clone());
                })
            };

            assert!(!IntoSystem::into_system(insert_component_system.pipe(affect)).initialize(app.world_mut()).is_exclusive());

            app.add_systems(
                Update,
                insert_component_system.pipe(affect),
            );

            app.update();

            let actual_component = app.world().get_entity(entity).unwrap().get_components::<&NumberComponent<0>>().unwrap();

            assert_eq!(actual_component, &component);
        }

        #[test]
        fn bundle_commands_correctly_insert_and_remove(component_0 in any::<NumberComponent<0>>(), component_1 in any::<NumberComponent<1>>()) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(()).id();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert!(actual_component_0.is_none());
            assert!(actual_component_1.is_none());

            #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
            struct InsertSystem;

            app.add_systems(
                Update,
                (move || entity_command_insert(entity,  (component_0, component_1))).pipe(affect).in_set(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert_eq!(actual_component_0, Some(&component_0));
            assert_eq!(actual_component_1, Some(&component_1));

            app.add_systems(
                Update,
                (move || entity_command_remove::<(NumberComponent<1>, NumberComponent<2>)>(entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert_eq!(actual_component_0, Some(&component_0));
            assert!(actual_component_1.is_none());

            app.add_systems(
                Update,
                (move || entity_command_remove::<(NumberComponent<0>, NumberComponent<1>)>(entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            let actual_component_0 = app.world().entity(entity).get::<NumberComponent<0>>();
            let actual_component_1 = app.world().entity(entity).get::<NumberComponent<1>>();

            assert!(actual_component_0.is_none());
            assert!(actual_component_1.is_none());
        }

        #[test]
        fn bundle_recursively_commands_correctly_insert_and_remove(component_0 in any::<NumberComponent<0>>(), component_1 in any::<NumberComponent<1>>()) {
            let mut app = App::new();

            let parent_entity = app.world_mut().spawn(()).id();
            let child_entity_a = app.world_mut().spawn(ChildOf(parent_entity)).id();
            let child_entity_b = app.world_mut().spawn(ChildOf(parent_entity)).id();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), None);

            #[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
            struct InsertSystem;

            app.add_systems(
                Update,
                (move || entity_command_insert_recursive::<Children, _>(parent_entity,  (component_0, component_1))).pipe(affect).in_set(InsertSystem),
            );

            app.update();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), Some(&component_1));

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), Some(&component_1));

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), Some(&component_1));

            app.add_systems(
                Update,
                (move || entity_command_remove_recursive::<Children, (NumberComponent<1>, NumberComponent<2>)>(parent_entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), Some(&component_0));
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), None);

            app.add_systems(
                Update,
                (move || entity_command_remove_recursive::<Children, (NumberComponent<0>, NumberComponent<1>)>(parent_entity)).pipe(affect).after(InsertSystem),
            );

            app.update();

            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(parent_entity).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_a).get::<NumberComponent<1>>(), None);

            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<0>>(), None);
            assert_eq!(app.world().entity(child_entity_b).get::<NumberComponent<1>>(), None);
        }
    }

    #[test]
    fn commands_can_spawn_and_despawn_entities() {
        #[derive(Resource, Clone)]
        struct EntityHolder(Entity);

        let mut app = App::new();

        app.add_systems(
            Update,
            (move || command_spawn_and((), |entity| command_insert_resource(EntityHolder(entity))))
                .pipe(affect)
                .run_if(not(resource_exists::<EntityHolder>)),
        );

        app.update();

        let EntityHolder(entity) = app.world().resource::<EntityHolder>().clone();

        assert!(app.world().get_entity(entity).is_ok());

        app.add_systems(
            Update,
            (move |entity_holder: Res<EntityHolder>| entity_command_despawn(entity_holder.0))
                .pipe(affect),
        );

        app.update();

        assert!(app.world().get_entity(entity).is_err());
    }
}
