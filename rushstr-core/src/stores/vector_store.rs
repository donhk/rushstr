use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

use bincode::config;
use bincode::config::Configuration;
use fuzzy_matcher::skim::SkimMatcherV2;
use sled::Db;

use crate::stores::matchers::{filter_items_exact, filter_items_monkey, filter_items_regex};
use crate::{HItem, HLines, Key, MHItem, Scanner, SearchOptions, SearchType, StoreTrait, create_db};

pub struct VectorStore {
    database: Db,
    items: Vec<MHItem>,
    items_index: HashMap<Key, MHItem>,
    config: Configuration,
    matcher: SkimMatcherV2,
}

impl VectorStore {
    pub fn new(scanner: Scanner) -> anyhow::Result<VectorStore> {
        let config = config::standard();
        let database = create_db()?;
        let elements_in_history = scanner.load()?;

        let mut items = Vec::with_capacity(elements_in_history.len());
        let mut items_index = HashMap::with_capacity(elements_in_history.len());

        for item in elements_in_history {
            let key = item.id();

            // Avoid duplicates: check if we've already processed this item
            if items_index.contains_key(&key) {
                continue;
            }

            let final_item = if let Ok(Some(i_vec)) = database.get(key) {
                let (hitem, _): (HItem, usize) = bincode::decode_from_slice(&i_vec, config)?;
                hitem
            } else {
                let bytes = bincode::encode_to_vec(&item, config)?;
                database.insert(key, bytes)?;
                item
            };

            let rc_item = Rc::new(Mutex::new(final_item));
            items_index.insert(key, rc_item.clone());
            items.push(rc_item);
        }

        // Sort items by number of hits (descending)
        items.sort_by_key(|hitem| {
            let lock = hitem.lock().unwrap();
            std::cmp::Reverse(lock.hits())
        });

        let matcher = SkimMatcherV2::default();
        Ok(VectorStore {
            database,
            items,
            items_index,
            config,
            matcher,
        })
    }
}

impl StoreTrait for VectorStore {
    fn items(&self, options: &SearchOptions) -> anyhow::Result<Vec<HItem>> {
        let mut all_data = Vec::new();
        for item in self.items.iter() {
            if let Ok(g_item) = item.lock() {
                all_data.push(g_item.clone());
            }
        }

        if options.input.is_empty() {
            let mut f_all_data = Vec::new();
            for item in all_data.into_iter() {
                if options.favorites {
                    if item.is_fav() {
                        f_all_data.push(item);
                    }
                } else {
                    f_all_data.push(item);
                }
            }
            return Ok(f_all_data);
        }

        let filtered_results = match options.search_type {
            SearchType::MonkeyTyping => filter_items_monkey(&all_data, options, &self.matcher),
            SearchType::Exact => filter_items_exact(&all_data, options),
            SearchType::Regex => filter_items_regex(&all_data, options),
        };
        Ok(filtered_results)
    }

    fn total(&self) -> anyhow::Result<HLines> {
        Ok(self.items.len())
    }

    fn favorites(&self) -> anyhow::Result<usize> {
        let mut favorites = 0;
        for result in self.items.iter() {
            if let Ok(result) = result.lock() {
                if result.is_fav() {
                    favorites += 1;
                }
            }
        }
        Ok(favorites)
    }

    fn mark_favorite(&self, id: &Key) {
        if let Some(h_item) = self.items_index.get(id) {
            if let Ok(mut h_item) = h_item.lock() {
                h_item.flip_fav();
                if let Ok(bytes) = bincode::encode_to_vec(&*h_item, self.config) {
                    let _ = self.database.insert(h_item.id(), bytes);
                }
            }
        }
    }

    fn mark_hit(&self, id: &Key) {
        if let Some(h_item) = self.items_index.get(id) {
            if let Ok(mut h_item) = h_item.lock() {
                h_item.inc_hits();
                if let Ok(bytes) = bincode::encode_to_vec(&*h_item, self.config) {
                    let _ = self.database.insert(h_item.id(), bytes);
                }
            }
        }
    }
}
