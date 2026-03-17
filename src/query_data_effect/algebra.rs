use bevy::ecs::query::QueryData;
use variadics_please::all_tuples;

use crate::query_data_effect::QueryDataEffect;

macro_rules! impl_query_data_effect {
    ($(($QDE:ident, $qde:ident, $qd:ident)),*) => {
        impl<$($QDE),*> QueryDataEffect for ($($QDE,)*)
        where $($QDE: QueryDataEffect,)*
        {
            type MutQueryData = ($(<$QDE as QueryDataEffect>::MutQueryData,)*);
            type Filter = ($(<$QDE as QueryDataEffect>::Filter,)*);

            fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
                let ($($qde,)*) = self;
                let ($($qd,)*) = query_data;

                $($qde.affect($qd));*
            }
        }
    }
}

all_tuples!(impl_query_data_effect, 0, 15, QDE, qde, qd);
