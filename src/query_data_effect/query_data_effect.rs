use std::marker::PhantomData;

use bevy::ecs::component::Mutable;
use bevy::ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::Effect;

pub trait QueryDataEffect {
    type MutQueryData: QueryData;
    type Filter: QueryFilter;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>);
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct ComponentSet<C>
where
    C: Component<Mutability = Mutable>,
{
    pub component: C,
}

pub fn component_set<C>(component: C) -> ComponentSet<C>
where
    C: Component<Mutability = Mutable>,
{
    ComponentSet { component }
}

impl<C> QueryDataEffect for ComponentSet<C>
where
    C: Component<Mutability = Mutable>,
{
    type MutQueryData = &'static mut C;
    type Filter = With<C>;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        **query_data = self.component;
    }
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct ComponentsSet<Cs> {
    pub components: Cs,
}

pub fn components_set<Cs>(components: Cs) -> ComponentsSet<Cs>
where
    Cs: Component<Mutability = Mutable>,
{
    ComponentsSet { components }
}

macro_rules! impl_query_data_effect_for_components_set {
    ($(($C:ident, $q:ident, $c:ident)),*) => {
        impl<$($C,)*> QueryDataEffect for ComponentsSet<($($C,)*)>
        where
            $($C: Component<Mutability = Mutable>),*
        {
            type MutQueryData = ($(&'static mut $C,)*);
            type Filter = ($(With<$C>,)*);

            fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
                let ($($q,)*) = query_data;
                let ($($c,)*) = self.components;

                $(**$q = $c);*
            }
        }
    }
}

all_tuples!(impl_query_data_effect_for_components_set, 1, 15, C, q, c);

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
