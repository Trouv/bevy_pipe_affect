use std::marker::PhantomData;

use bevy::ecs::query::{QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;

use crate::query_data_effect::QueryDataEffect;
use crate::{Effect, EffectOut};

/// [`Effect`] that applies the given [`QueryDataEffect`] to the given entity.
///
/// Produces an error (handled by `bevy`'s `DefaultErrorHandler`) if the entity does not exist in
/// the [`QueryDataEffect::Filter`] (and the optional `Filter` generic).
///
/// Can be constructed by [`query_entity_affect`].
pub struct QueryEntityAffect<QueryDataE, Filter = ()>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    /// The entity to apply the [`QueryDataEffect`] to.
    pub entity: Entity,
    /// The [`QueryDataEffect`] to apply to the entity.
    pub query_data_effect: QueryDataE,
    filter: PhantomData<Filter>,
}

/// Construct a new [`QueryEntityAffect`] [`Effect`].
pub fn query_entity_affect<QueryDataE, Filter>(
    entity: Entity,
    query_data_effect: QueryDataE,
) -> QueryEntityAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    QueryEntityAffect {
        entity,
        query_data_effect,
        filter: PhantomData,
    }
}

impl<QueryDataE, Filter> Effect for QueryEntityAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect + Clone,
    QueryDataE::MutQueryData: 'static,
    Filter: QueryFilter + 'static,
{
    type MutParam = (
        Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
        <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam,
    );

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let mut query_data = match param.0.get_mut(self.entity) {
            Ok(d) => d,
            Err(e) => {
                Err::<(), _>(e).affect(&mut param.1);
                return;
            }
        };

        self.query_data_effect.clone().affect(&mut query_data);
    }
}

pub struct QueryEntityMap<QueryDataIn, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub entity: Entity,
    pub f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE>,
    filter: PhantomData<Filter>,
}

pub fn query_entity_map<QueryDataIn, QueryDataE, Filter, F>(
    entity: Entity,
    f: F,
) -> QueryEntityMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE + 'static,
{
    QueryEntityMap {
        entity,
        f: Box::new(f),
        filter: PhantomData,
    }
}

impl<QueryDataIn, QueryDataE, Filter> Effect for QueryEntityMap<QueryDataIn, QueryDataE, Filter>
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
            (
                Query<'static, 'static, QueryDataIn, (QueryDataE::Filter, Filter)>,
                <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam,
            ),
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (input_query_param, mut result_param) = param.p0();

        let input = match input_query_param.get(self.entity) {
            Ok(d) => d,
            Err(e) => {
                Err::<(), _>(e).affect(&mut result_param);
                return;
            }
        };

        let query_data_effect = (self.f)(input);

        query_data_effect.affect(&mut param.p1().get_mut(self.entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));
    }
}

pub struct QueryEntityMapAnd<QueryDataIn, E, QueryDataE, Filter = ()>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub entity: Entity,
    pub f: Box<dyn for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE>>,
    filter: PhantomData<Filter>,
}

pub fn query_entity_map_and<QueryDataIn, E, QueryDataE, Filter, F>(
    entity: Entity,
    f: F,
) -> QueryEntityMapAnd<QueryDataIn, E, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    E: Effect,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> EffectOut<E, QueryDataE> + 'static,
{
    QueryEntityMapAnd {
        entity,
        f: Box::new(f),
        filter: PhantomData,
    }
}

impl<QueryDataIn, E, QueryDataE, Filter> Effect
    for QueryEntityMapAnd<QueryDataIn, E, QueryDataE, Filter>
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
            (
                Query<'static, 'static, QueryDataIn, (QueryDataE::Filter, Filter)>,
                <Result<(), bevy::ecs::query::QueryEntityError> as Effect>::MutParam,
            ),
            Query<'static, 'static, QueryDataE::MutQueryData, Filter>,
            <E as Effect>::MutParam,
        ),
    >;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        let (input_query_param, mut result_param) = param.p0();

        let input = match input_query_param.get(self.entity) {
            Ok(d) => d,
            Err(e) => {
                Err::<(), _>(e).affect(&mut result_param);
                return;
            }
        };

        let EffectOut {
            effect,
            out: query_data_effect,
        } = (self.f)(input);

        query_data_effect.affect(&mut param.p1().get_mut(self.entity).expect("The entities in the first query are guaranteed to be a subset of the entities in the second query due to filters"));

        effect.affect(&mut param.p2())
    }
}
