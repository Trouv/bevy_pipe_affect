use bevy::ecs::query::{QueryData, QueryFilter};

/// Define a state transition for the `QueryData` of individual entities.
///
/// Used in the `Query-` effects.
pub trait QueryDataEffect {
    /// The `QueryData` this effect mutates.
    type MutQueryData: QueryData;
    /// `QueryFilter` that selects the same entities that [`Self::MutQueryData`] would.
    type Filter: QueryFilter;

    /// Perform the state transition on the query data.
    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>);
}
