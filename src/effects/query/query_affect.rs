use std::marker::PhantomData;

use bevy::ecs::query::QueryFilter;
use bevy::prelude::*;

use crate::Effect;
use crate::query_data_effect::QueryDataEffect;

/// [`Effect`] that applies a [`QueryDataEffect`] to all entities in a query.
///
/// The query can be filtered with the `Filter` generic.
///
/// Can be constructed with [`query_affect`].
///
/// # Example
/// In this example, a system is written that sets all entities' `Speed` to 0 if they have a
/// `Brake` component.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Speed(f32);
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Component)]
/// # #[derive(proptest_derive::Arbitrary)]
/// struct Brake;
///
/// /// Pure system using effects.
/// fn stop_all_pure() -> QueryAffect<ComponentSet<Speed>, With<Brake>> {
///     query_affect(component_set(Speed(0.0)))
/// }
///
/// /// Equivalent impure system.
/// fn stop_all_impure(mut query: Query<&mut Speed, With<Brake>>) {
///     for (mut number_component) in query.iter_mut() {
///         *number_component = Speed(0.0);
///     }
/// }
/// # use proptest::prelude::*;
/// #
/// # fn app_setup(component_table: Vec<(Option<Speed>, Option<Brake>)>) -> App {
/// #     let mut app = App::new();
/// #     component_table.into_iter().for_each(|(speed, brake)| {
/// #         let mut entity = app.world_mut().spawn_empty();
/// #         if let Some(speed) = speed {
/// #             entity.insert(speed);
/// #         }
/// #         if let Some(brake) = brake {
/// #             entity.insert(brake);
/// #         }
/// #     });
/// #
/// #     app
/// # }
/// #
/// # fn query_state(world: &mut World) -> Vec<(Entity, Option<&Speed>, Option<&Brake>)> {
/// #     let mut query = world.query::<(Entity, Option<&Speed>, Option<&Brake>)>();
/// #     query.iter(world).collect()
/// # }
/// #
/// # proptest! {
/// #     fn main(component_table: Vec<(Option<Speed>, Option<Brake>)>) {
/// #         let mut pure_app = app_setup(component_table.clone());
/// #         pure_app.add_systems(Update, stop_all_pure.pipe(affect));
/// #
/// #         let mut impure_app = app_setup(component_table.clone());
/// #         impure_app.add_systems(Update, stop_all_impure);
/// #
/// #         for _ in 0..3 {
/// #             assert_eq!(query_state(pure_app.world_mut()), query_state(impure_app.world_mut()));
/// #             pure_app.update();
/// #             impure_app.update();
/// #         }
/// #     }
/// # }
/// ```
///
/// Not shown...
/// - other [`QueryDataEffect`]s are available
/// - the `Filter` parameter can be omitted
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct QueryAffect<QueryDataE, Filter = ()>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// The [`QueryDataEffect`] that is applied to all entities in the query.
    pub query_data_effect: QueryDataE,
    filter: PhantomData<Filter>,
}

impl<QueryDataE, Filter> QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// Construct a new [`QueryAffect`].
    pub fn new(query_data_effect: QueryDataE) -> Self {
        QueryAffect {
            query_data_effect,
            filter: PhantomData,
        }
    }
}

/// Construct a new [`QueryAffect`] [`Effect`].
pub fn query_affect<QueryDataE, Filter>(
    query_data_effect: QueryDataE,
) -> QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    QueryAffect::new(query_data_effect)
}

impl<QueryDataE, Filter> Default for QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect + Default,
    Filter: QueryFilter,
{
    fn default() -> Self {
        query_affect(default())
    }
}

impl<QueryDataE, Filter> Effect for QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect + Clone,
    QueryDataE::MutQueryData: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = Query<'static, 'static, QueryDataE::MutQueryData, Filter>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let mut iter = param.iter_mut();
        while let Some(mut query_data) = iter.fetch_next() {
            self.query_data_effect.clone().affect(&mut query_data);
        }
    }
}
