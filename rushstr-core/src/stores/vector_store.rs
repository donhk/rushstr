use std::collections::HashMap;
use std::rc::Rc;

use bincode::config;
use bincode::config::Configuration;
use fuzzy_matcher::skim::SkimMatcherV2;
use sled::Db;

use crate::stores::matchers::{filter_items_exact, filter_items_monkey, filter_items_regex};
use crate::{HItem, HLines, Key, Scanner, SearchOptions, SearchType, StoreTrait, create_db};

pub struct VectorStore {
    database: Db,
    items: Vec<Rc<HItem>>,
    items_index: HashMap<Key, Rc<HItem>>,
    config: Configuration,
    matcher: SkimMatcherV2,
}

impl VectorStore {
    pub fn new(scanner: Scanner) -> anyhow::Result<VectorStore> {
        // update the database
        let config = config::standard();
        let database = create_db()?;
        let elements_in_history = scanner.load()?;
        let mut items = Vec::with_capacity(elements_in_history.len());
        let mut items_index = HashMap::with_capacity(elements_in_history.len());
        for item in elements_in_history {
            // if the elements already exists in our history
            let final_item = if let Ok(Some(i_vec)) = database.get(item.id()) {
                let (hitem, _): (HItem, usize) = bincode::decode_from_slice(&i_vec, config)?;
                hitem
            } else {
                let bytes = bincode::encode_to_vec(&item, config)?;
                database.insert(item.id(), bytes)?;
                item
            };

            let final_item = Rc::new(final_item);

            items_index.insert(final_item.id(), final_item.clone());
            items.push(final_item);
        }

        items.sort_by(|a, b| b.hits().cmp(&a.hits()));

        let matcher = SkimMatcherV2::default();
        Ok(VectorStore {
            database,
            items,
            config,
            items_index,
            matcher,
        })
    }
}

impl StoreTrait for VectorStore {
    fn items(&self, options: &SearchOptions) -> anyhow::Result<Vec<&Rc<HItem>>> {
        if options.input.is_empty() {
            let mut all_data = Vec::new();
            for item in self.items.iter() {
                if options.favorites {
                    if item.is_fav() {
                        all_data.push(item);
                    }
                } else {
                    all_data.push(item);
                }
            }
            return Ok(all_data);
        }
        let filtered_results = match options.search_type {
            SearchType::MonkeyTyping => filter_items_monkey(&self.items, options, &self.matcher),
            SearchType::Exact => filter_items_exact(&self.items, options),
            SearchType::Regex => filter_items_regex(&self.items, options),
        };
        Ok(filtered_results)
    }

    fn total(&self) -> anyhow::Result<HLines> {
        Ok(self.items.len())
    }

    fn favorites(&self) -> anyhow::Result<usize> {
        let mut favorites = 0;
        for result in self.items.iter() {
            if result.is_fav() {
                favorites += 1;
            }
        }
        Ok(favorites)
    }

    fn mark_favorite(&mut self, id: &Key) {
        if let Some(h_item) = self.items_index.get_mut(id) {
            let mut_h_item = Rc::make_mut(h_item);
            mut_h_item.flip_fav();
            if let Ok(bytes) = bincode::encode_to_vec(&*h_item, self.config) {
                let _ = self.database.insert(h_item.id(), bytes);
            }
        }
    }

    fn mark_hit(&mut self, id: &Key) {
        if let Some(h_item) = self.items_index.get_mut(id) {
            let mut_h_item = Rc::make_mut(h_item);
            mut_h_item.inc_hits();
            if let Ok(bytes) = bincode::encode_to_vec(&*h_item, self.config) {
                let _ = self.database.insert(h_item.id(), bytes);
            }
        }
    }
}
