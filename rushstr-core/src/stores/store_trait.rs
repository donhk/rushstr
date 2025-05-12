use crate::{HItem, HLines, SearchOptions};

pub trait StoreTrait {
    fn items(&self, options: &SearchOptions) -> Vec<HItem>;

    fn total(&self) -> HLines;

    fn favorites(&self) -> usize;
}
