use std::rc::Rc;

use crate::{HItem, HLines, Key, SearchOptions};

/// Defines the interface for a searchable and interactive item store.
///
/// This trait is designed for implementations that manage a collection of
/// `HItem` entries, such as a command history database, and allows filtering,
/// favoriting, and tracking usage.
pub trait StoreTrait {
    /// Returns a filtered list of items based on the provided search options.
    ///
    /// # Arguments
    ///
    /// * `options` - A `SearchOptions` instance specifying filters like input
    ///   text, search type (e.g., fuzzy, regex), and favorites-only mode.
    ///
    /// # Returns
    ///
    /// A list of `HItem`s matching the given options, or an error if the
    /// operation fails.
    fn items(&self, options: &SearchOptions) -> anyhow::Result<Vec<&Rc<HItem>>>;

    /// Returns the total number of stored items.
    ///
    /// This value is typically used for UI display or pagination logic.
    ///
    /// # Returns
    ///
    /// A count of all items as `HLines`, or an error.
    fn total(&self) -> anyhow::Result<HLines>;

    /// Returns the total number of favorited items.
    ///
    /// # Returns
    ///
    /// A count of items marked as favorites, or an error.
    fn favorites(&self) -> anyhow::Result<usize>;

    /// Marks an item with the given ID as a favorite.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier corresponding to an `HItem`.
    fn mark_favorite(&mut self, id: &Key);

    /// Increments the "hit" count for the item with the given ID.
    ///
    /// Typically used to track item usage frequency for ranking or analytics.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier corresponding to an `HItem`.
    fn mark_hit(&mut self, id: &Key);
}
