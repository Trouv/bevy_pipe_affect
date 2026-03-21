use bevy::ecs::component::Mutable;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use variadics_please::all_tuples;

use crate::QueryDataEffect;

/// [`QueryDataEffect`] that sets a component to the given value.
///
/// If you want to set multiple components, see [`ComponentsSet`].
///
/// Can be constructed by [`component_set`].
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct ComponentSet<C>
where
    C: Component<Mutability = Mutable>,
{
    /// The value to set the component to.
    pub component: C,
}

/// Constructs a [`ComponentSet`] [`QueryDataEffect`].
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

/// [`QueryDataEffect`] that sets multiple (up to 15) components to the given values.
///
/// If you want to set single component, see [`ComponentSet`].
///
/// Can be constructed by [`components_set`].
#[derive(Debug, Default, PartialEq, Eq, Copy, Clone)]
pub struct ComponentsSet<Cs> {
    /// The values to set the components to.
    pub components: Cs,
}

/// Constructs a [`ComponentsSet`] [`QueryDataEffect`].
pub fn components_set<Cs>(components: Cs) -> ComponentsSet<Cs> {
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

#[cfg(test)]
mod tests {
    use proptest::*;

    use super::*;
    use crate::effects::number_data::NumberComponent;

    proptest! {
        fn component_set_affect_updates_query_data(initial: NumberComponent<0>, component: NumberComponent<0>) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(initial).id();

            let component_set = component_set(component);

            app.world_mut()
                .query::<<ComponentSet<NumberComponent<0>> as QueryDataEffect>::MutQueryData>()
                .iter_mut(app.world_mut())
                .for_each(|mut query_data| component_set.affect(&mut query_data));

            assert_eq!(
                app.world().get::<NumberComponent<0>>(entity).unwrap(),
                &component
            );
        }

        fn components_set_affect_updates_query_data(initial: (NumberComponent<0>, NumberComponent<1>), components: (NumberComponent<0>, NumberComponent<1>)) {
            let mut app = App::new();

            let entity = app.world_mut().spawn(initial).id();

            let components_set = components_set(components);

            app.world_mut()
                .query::<<ComponentsSet<(NumberComponent<0>, NumberComponent<1>)> as QueryDataEffect>::MutQueryData>()
                .iter_mut(app.world_mut())
                .for_each(|mut query_data| components_set.affect(&mut query_data));

            assert_eq!(
                app.world().get::<NumberComponent<0>>(entity).unwrap(),
                &components.0
            );

            assert_eq!(
                app.world().get::<NumberComponent<1>>(entity).unwrap(),
                &components.1
            );
        }
    }
}
