use crate::SearchOptions;

pub trait StoreTrait {
    fn filter_items(&self, search_options: &SearchOptions) -> Vec<String>;
    fn total(&self) -> usize;
}
