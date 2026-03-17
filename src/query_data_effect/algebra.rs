use bevy::ecs::query::QueryData;
use either::Either;
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

impl<QDE0, QDE1> QueryDataEffect for Either<QDE0, QDE1>
where
    QDE0: QueryDataEffect,
    QDE1: QueryDataEffect,
{
    type MutQueryData = (
        <QDE0 as QueryDataEffect>::MutQueryData,
        <QDE1 as QueryDataEffect>::MutQueryData,
    );
    type Filter = (
        <QDE0 as QueryDataEffect>::Filter,
        <QDE1 as QueryDataEffect>::Filter,
    );

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        match self {
            Either::Left(query_data_effect) => query_data_effect.affect(&mut query_data.0),
            Either::Right(query_data_effect) => query_data_effect.affect(&mut query_data.1),
        }
    }
}

impl<QDE> QueryDataEffect for Option<QDE>
where
    QDE: QueryDataEffect,
{
    type MutQueryData = <Either<QDE, ()> as QueryDataEffect>::MutQueryData;
    type Filter = <Either<QDE, ()> as QueryDataEffect>::Filter;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>) {
        let as_either = match self {
            Some(query_data_effect) => Either::Left(query_data_effect),
            None => Either::Right(()),
        };

        as_either.affect(query_data);
    }
}
