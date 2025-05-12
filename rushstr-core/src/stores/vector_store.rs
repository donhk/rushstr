use crate::stores::matchers::{filter_items_exact, filter_items_monkey, filter_items_regex};
use crate::{HItem, HLines, Scanner, SearchOptions, SearchType, StoreTrait};

pub struct VectorStore {
    all_items: Vec<HItem>,
}

impl VectorStore {
    pub fn new(scanner: Scanner) -> VectorStore {
        VectorStore {
            all_items: scanner.load(),
        }
    }
}

impl StoreTrait for VectorStore {
    fn items(&self, options: &SearchOptions) -> Vec<HItem> {
        if options.input.is_empty() {
            return self.all_items.clone();
        }
        match options.search_type {
            SearchType::MonkeyTyping => filter_items_monkey(&self.all_items, options),
            SearchType::Exact => filter_items_exact(&self.all_items, options),
            SearchType::Regex => filter_items_regex(&self.all_items, options),
        }
    }

    fn total(&self) -> HLines {
        self.all_items.len()
    }

    fn favorites(&self) -> usize {
        6
    }
}
