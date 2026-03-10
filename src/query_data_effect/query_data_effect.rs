use std::marker::PhantomData;

use bevy::ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;

use crate::Effect;

pub trait QueryDataEffect {
    type MutQueryData: QueryData;
    type Filter: QueryFilter;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>);
}

pub struct QueryAffect<QueryDataE, Filter = ()>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    pub query_data_effect: QueryDataE,
    pub filter: PhantomData<Filter>,
}

pub fn query_affect<QueryDataE, Filter>(
    query_data_effect: QueryDataE,
) -> QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    QueryAffect {
        query_data_effect,
        filter: PhantomData,
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

pub fn query_map<QueryDataIn, QueryDataE, Filter, F>(
    f: F,
) -> QueryMap<QueryDataIn, QueryDataE, Filter>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
    F: for<'w, 's> Fn(QueryDataIn::Item<'w, 's>) -> QueryDataE + 'static,
{
    QueryMap {
        f: Box::new(f),
        filter: PhantomData,
    }
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
