use std::marker::PhantomData;

use bevy::ecs::query::{QueryFilter, ReadOnlyQueryData, WorldQuery};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::all_tuples;

use crate::Effect;

/// [`Effect`] that sets `Component`s of all entities in a query to the provided `Component` tuple.
///
/// Can be parameterized by a `QueryFilter` to narrow down the components updated.
pub struct ComponentsPut<C, Filter = ()>
where
    C: Clone,
    Filter: QueryFilter,
{
    components: C,
    filter: PhantomData<Filter>,
}

impl<C, Filter> ComponentsPut<C, Filter>
where
    C: Clone,
    Filter: QueryFilter,
{
    /// Construct a new [`ComponentsPut`].
    pub fn new(components: C) -> Self {
        ComponentsPut {
            components,
            filter: PhantomData,
        }
    }
}

macro_rules! impl_effect_for_components_put {
    ($(($C:ident, $c:ident, $r:ident)),*) => {
        impl<$($C,)* Filter> Effect for ComponentsPut<($($C,)*), Filter>
        where
            $($C: Component + Clone),*,
            Filter: QueryFilter + 'static,
        {
            type MutParam = Query<'static, 'static, ($(&'static mut $C,)*), Filter>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let ($($c,)*) = self.components;
                param.iter_mut().for_each(|($(mut $r,)*)| {
                    $(*$r = $c.clone();)*
                });
            }
        }
    }
}

all_tuples!(impl_effect_for_components_put, 1, 15, C, c, r);

/// [`Effect`] that transforms `Component`s of all entities in a query with the provided function.
///
/// Can be parameterized by a `ReadOnlyQueryData` to access additional query data in the function.
///
/// Can be parameterized by a `QueryFilter` to narrow down the components updated.
pub struct ComponentsWith<F, C, Data = (), Filter = ()>
where
    F: for<'a> FnMut(C, <Data as WorldQuery>::Item<'a>) -> C,
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    f: F,
    components: PhantomData<C>,
    data: PhantomData<Data>,
    filter: PhantomData<Filter>,
}

impl<F, C, Data, Filter> ComponentsWith<F, C, Data, Filter>
where
    F: for<'a> FnMut(C, <Data as WorldQuery>::Item<'a>) -> C,
    C: Clone,
    Data: ReadOnlyQueryData,
    Filter: QueryFilter,
{
    /// Construct a new [`ComponentsWith`].
    pub fn new(f: F) -> Self {
        ComponentsWith {
            f,
            components: PhantomData,
            data: PhantomData,
            filter: PhantomData,
        }
    }
}
