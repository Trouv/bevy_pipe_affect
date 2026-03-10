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

pub struct QueryDataMap<QueryDataIn, QueryDataE>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
{
    pub f: Box<dyn for<'w, 's> Fn(&QueryDataIn::Item<'w, 's>) -> QueryDataE>,
}

pub fn query_data_map<F, QueryDataIn, QueryDataE>(f: F) -> QueryDataMap<QueryDataIn, QueryDataE>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
    F: for<'w, 's> Fn(&QueryDataIn::Item<'w, 's>) -> QueryDataE + 'static,
{
    QueryDataMap { f: Box::new(f) }
}

impl<QueryDataIn, QueryDataE> QueryDataEffect for QueryDataMap<QueryDataIn, QueryDataE>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
{
    type MutQueryData = (QueryDataIn, QueryDataE::MutQueryData);
    type Filter = QueryDataE::Filter;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        (self.f)(&query_data.0).affect(&mut query_data.1);
    }
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
