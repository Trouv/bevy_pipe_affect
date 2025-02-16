use std::marker::PhantomData;

use bevy::ecs::query::QueryFilter;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::utils::all_tuples;

use crate::Effect;

/// [`Effect`] that sets `Component`s of all entities in a query to the provided `Component` tuple.
///
/// Can be parameterized by a `QueryFilter` to narrow down the components updated.
pub struct ComponentsPut<C, F = ()>
where
    C: Clone,
    F: QueryFilter,
{
    components: C,
    filter: PhantomData<F>,
}

impl<C, F> ComponentsPut<C, F>
where
    C: Clone,
    F: QueryFilter,
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
        impl<$($C,)* F> Effect for ComponentsPut<($($C,)*), F>
        where
            $($C: Component + Clone),*,
            F: QueryFilter + 'static,
        {
            type MutParam = Query<'static, 'static, ($(&'static mut $C,)*), F>;

            fn affect(self, param: &mut <Self::MutParam as SystemParam>::Item<'_, '_>) {
                let ($($c,)*) = self.components;
                param.iter_mut().for_each(|($(mut $r,)*)| {
                    $(*$r = $c.clone());*
                });
            }
        }
    }
}

all_tuples!(impl_effect_for_components_put, 1, 15, C, c, r);
