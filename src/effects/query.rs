use std::marker::PhantomData;

use bevy::ecs::query::{QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;

use crate::query_data_effect::QueryDataEffect;
use crate::{Effect, EffectOut, effect_out};

/// [`Effect`] that applies a [`QueryDataEffect`] to all entities in a query.
///
/// The query can be filtered with the `Filter` generic.
///
/// Can be constructed with [`query_affect`].
///
/// # Example
/// In this example, a system is written that sets all `NumberComponent`s in the world to
/// `NumberComponent(2)`.
/// ```
/// use bevy::prelude::*;
/// use bevy_pipe_affect::prelude::*;
///
/// #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Component)]
/// struct NumberComponent(u32);
///
/// // Pure system using effects.
/// fn set_value_to_2_for_all_entities_pure() -> QueryAffect<ComponentSet<NumberComponent>> {
///     query_affect(component_set(NumberComponent(2)))
/// }
///
/// // Equivalent impure system.
/// fn set_value_to_2_for_all_entities_impure(mut query: Query<&mut NumberComponent>) {
///     for (mut number_component) in query.iter_mut() {
///         *number_component = NumberComponent(2);
///     }
/// }
/// # fn app_setup() -> (App, [Entity; 2]) {
/// #     let mut app = App::new();
/// #     let entity_0 = app.world_mut().spawn(NumberComponent(0)).id();
/// #     let entity_1 = app.world_mut().spawn(NumberComponent(1)).id();
/// #     (app, [entity_0, entity_1])
/// # }
/// # fn main() {
/// #     let (mut app_pure, entities_pure) = app_setup();
/// #     app_pure.add_systems(Update, set_value_to_2_for_all_entities_pure.pipe(affect));
/// #     let (mut app_impure, entities_impure) = app_setup();
/// #     app_impure.add_systems(Update, set_value_to_2_for_all_entities_pure.pipe(affect));
/// #     entities_pure
/// #         .iter()
/// #         .zip(&entities_impure)
/// #         .for_each(|(entity_pure, entity_impure)| {
/// #             let component_pure = app_pure
/// #                 .world()
/// #                 .get::<NumberComponent>(*entity_pure)
/// #                 .unwrap();
/// #             let component_impure = app_impure
/// #                 .world()
/// #                 .get::<NumberComponent>(*entity_impure)
/// #                 .unwrap();
/// #             assert_ne!(component_pure.0, 2);
/// #             assert_ne!(component_impure.0, 2);
/// #             assert_eq!(component_pure, component_impure);
/// #         });
/// #     app_pure.update();
/// #     app_impure.update();
/// #     entities_pure
/// #         .iter()
/// #         .zip(&entities_impure)
/// #         .for_each(|(entity_pure, entity_impure)| {
/// #             let component_pure = app_pure
/// #                 .world()
/// #                 .get::<NumberComponent>(*entity_pure)
/// #                 .unwrap();
/// #             let component_impure = app_impure
/// #                 .world()
/// #                 .get::<NumberComponent>(*entity_impure)
/// #                 .unwrap();
/// #             assert_eq!(component_pure.0, 2);
/// #             assert_eq!(component_impure.0, 2);
/// #         });
/// # }
/// ```
///
/// Note that other [`QueryDataEffect`]s are available.
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

impl<QueryDataE, Filter> Effect for QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect + Clone,
    QueryDataE::MutQueryData: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = Query<'static, 'static, QueryDataE::MutQueryData, Filter>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param
            .iter_mut()
            .for_each(|mut query_data| self.query_data_effect.clone().affect(&mut query_data));
    }
}

pub struct QueryMap<QueryDataIn, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE>,
    filter: PhantomData<Filter>,
}

impl<QueryDataIn, QueryDataE, Filter> QueryMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub fn new(f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE>) -> Self {
        QueryMap {
            f,
            filter: PhantomData,
        }
    }
}

pub fn query_map<QueryDataIn, QueryDataE, Filter, F>(
    f: F,
) -> QueryMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE + 'static,
{
    QueryMap::new(Box::new(f))
}

impl<QueryDataIn, QueryDataE, Filter> Effect for QueryMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData + 'static,
    QueryDataE: QueryDataEffect,
    QueryDataE::MutQueryData: 'static,
    QueryDataE::Filter: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = ParamSet<
        'static,
        'static,
        (
            Query<'static, 'static, (Entity, QueryDataIn), (QueryDataE::Filter, Filter)>,
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let query_data_effects = param
            .p0()
            .iter()
            .map(|(entity, data_in)| (entity, (self.f)(data_in)))
            .collect::<Vec<_>>();

        query_data_effects
            .into_iter()
            .for_each(|(entity, query_data_effect)| {
                query_data_effect.affect(&mut param.p1().get_mut(entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));
            })
    }
}

pub struct QueryMapAnd<QueryDataIn, E, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE>>,
    filter: PhantomData<Filter>,
}

impl<QueryDataIn, E, QueryDataE, Filter> QueryMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub fn new(
        f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE>>,
    ) -> Self {
        QueryMapAnd {
            f,
            filter: PhantomData,
        }
    }
}

pub fn query_map_and<QueryDataIn, E, QueryDataE, Filter, F>(
    f: F,
) -> QueryMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE> + 'static,
{
    QueryMapAnd::new(Box::new(f))
}

impl<QueryDataIn, E, QueryDataE, Filter> Effect for QueryMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData + 'static,
    E: Effect,
    QueryDataE: QueryDataEffect,
    QueryDataE::MutQueryData: 'static,
    QueryDataE::Filter: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = ParamSet<
        'static,
        'static,
        (
            Query<'static, 'static, (Entity, QueryDataIn), (QueryDataE::Filter, Filter)>,
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
            <Vec<E> as Effect>::MutParam,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let EffectOut {
            effect: effects,
            out: query_data_effects,
        }: EffectOut<Vec<E>, Vec<(Entity, QueryDataE)>> = param
            .p0()
            .iter()
            .map(|(entity, data_in)| {
                let EffectOut {
                    effect,
                    out: query_data_effect,
                } = (self.f)(data_in);
                effect_out(effect, (entity, query_data_effect))
            })
            .collect();

        query_data_effects
            .into_iter()
            .for_each(|(entity, query_data_effect)| {
                query_data_effect.affect(&mut param.p1().get_mut(entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));
            });

        effects.affect(&mut param.p2())
    }
}
