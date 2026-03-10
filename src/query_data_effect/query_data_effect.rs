use bevy::ecs::query::{QueryData, QueryFilter};

pub trait QueryDataEffect {
    type MutQueryData: QueryData;
    type Filter: QueryFilter;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>);
}
