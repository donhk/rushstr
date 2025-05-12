use crate::{HLines, SearchOptions};

pub trait StoreTrait {
    fn filter_items(&self, search_options: &SearchOptions) -> Vec<String>;

    fn total(&self) -> HLines;

    fn favorites(&self) -> usize;
}
