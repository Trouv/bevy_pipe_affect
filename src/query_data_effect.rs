use std::marker::PhantomData;

use bevy::ecs::component::Mutable;
use bevy::ecs::query::{QueryData, QueryFilter, ReadOnlyQueryData};
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::Effect;

pub trait QueryDataEffect {
    type MutQueryData: QueryData;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>);
}

struct ComponentSet<C>
where
    C: Component<Mutability = Mutable>,
{
    component: C,
}

impl<C> QueryDataEffect for ComponentSet<C>
where
    C: Component<Mutability = Mutable>,
{
    type MutQueryData = &'static mut C;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        **query_data = self.component;
    }
}

struct ComponentsSet<Cs> {
    components: Cs,
}

macro_rules! impl_query_data_effect_for_components_set {
    ($(($C:ident, $q:ident, $c:ident)),*) => {
        impl<$($C,)*> QueryDataEffect for ComponentsSet<($($C,)*)>
        where
            $($C: Component<Mutability = Mutable>),*
        {
            type MutQueryData = ($(&'static mut $C,)*);

            fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
                let ($($q,)*) = query_data;
                let ($($c,)*) = self.components;

                $(**$q = $c);*
            }
        }
    }
}

all_tuples!(impl_query_data_effect_for_components_set, 1, 15, C, q, c);

struct QueryDataMap<QueryDataIn, QueryDataE>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
{
    f: Box<dyn for<'w, 's> FnOnce(&QueryDataIn::Item<'w, 's>) -> QueryDataE>,
}

impl<QueryDataIn, QueryDataE> QueryDataEffect for QueryDataMap<QueryDataIn, QueryDataE>
where
    QueryDataIn: ReadOnlyQueryData,
    QueryDataE: QueryDataEffect,
{
    type MutQueryData = (QueryDataIn, QueryDataE::MutQueryData);

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        (self.f)(&query_data.0).affect(&mut query_data.1);
    }
}

struct QueryAffect<QueryDataE, Filter = ()>
where
    QueryDataE: QueryDataEffect,
    Filter: QueryFilter,
{
    query_data_effect: QueryDataE,
    filter: PhantomData<Filter>,
}

impl<QueryDataE, Filter> Effect for QueryAffect<QueryDataE, Filter>
where
    QueryDataE: QueryDataEffect + Clone,
    QueryDataE::MutQueryData: 'static,
    Filter: QueryFilter,
{
    type MutParam = Query<'static, 'static, QueryDataE::MutQueryData>;

    fn affect(self, param: &mut <Self::MutParam as bevy::ecs::system::SystemParam>::Item<'_, '_>) {
        param
            .iter_mut()
            .for_each(|mut query_data| self.query_data_effect.clone().affect(&mut query_data));
    }
}
