use bevy::ecs::query::{QueryData, QueryFilter};

/// Define a state transition for the `QueryData` of individual entities.
///
/// Used in the `Query-` effects.
///
/// # Algebra
/// [`QueryDataEffect`] is implemented for some algebraic types too:
/// - `Option<T>` (where `T` is [`QueryDataEffect`])
/// - `Either<L, R>` where (`L` and `R` are [`QueryDataEffect`]s)
/// - `()` (no-op effect)
/// - Tuples of [`QueryDataEffect`]s up to size 15.
///
/// When combining effets with `Either` or tuples, you will find that bevy's typical borrowing
/// rules for query data will still apply. I.e., you can't use something like:
/// ```ignore
/// (ComponentSet<T>, ComponentsSet<(T, U, V)>)
/// ```
/// as it mutably borrows the component `T` twice.
pub trait QueryDataEffect {
    /// The `QueryData` this effect mutates.
    type MutQueryData: QueryData;
    /// `QueryFilter` that selects the same entities that [`Self::MutQueryData`] would.
    type Filter: QueryFilter;

    /// Perform the state transition on the query data.
    fn affect(self, query_data: &mut <Self::MutQueryData as QueryData>::Item<'_, '_>);
}
