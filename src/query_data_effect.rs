use bevy::ecs::component::Mutable;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use variadics_please::all_tuples;

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
