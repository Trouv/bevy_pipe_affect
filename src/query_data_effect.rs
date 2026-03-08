use bevy::ecs::query::QueryData;

pub trait QueryDataEffect {
    type MutQueryData: QueryData;

    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>);
}
