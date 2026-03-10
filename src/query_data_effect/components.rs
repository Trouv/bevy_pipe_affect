use bevy::ecs::component::Mutable;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::query_data_effect::QueryDataEffect;

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
