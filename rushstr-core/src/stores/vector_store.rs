use sled::{Db, IVec};

use crate::stores::matchers::{filter_items_exact, filter_items_monkey, filter_items_regex};
use crate::{HItem, HLines, Scanner, SearchOptions, SearchType, StoreTrait, create_db};

pub struct VectorStore {
    database: Db,
    items: Vec<HItem>,
}

impl VectorStore {
    pub fn new(scanner: Scanner) -> anyhow::Result<VectorStore> {
        // update the database
        let database = create_db()?;
        for item in scanner.load() {
            if let Ok(None) = database.get(item.id().as_bytes()) {
                // Serialize
                let bytes = bincode::serialize(&item)?;
                // Insert into sled
                database.insert(item.id().as_bytes(), bytes)?;
            }
        }
        let items = Self::convert_to_mem(&database)?;
        Ok(VectorStore { database, items })
    }

    fn convert_to_mem(database: &Db) -> anyhow::Result<Vec<HItem>> {
        let mut all_data = Vec::new();
        for result in database.iter() {
            let (_key, value): (IVec, IVec) = result?;
            let item: HItem = bincode::deserialize(&value)?;
            all_data.push(item);
        }
        all_data.sort_by(|a, b| a.id().cmp(b.id()));
        Ok(all_data)
    }
}

impl StoreTrait for VectorStore {
    fn items(&self, options: &SearchOptions) -> anyhow::Result<Vec<HItem>> {
        if options.input.is_empty() {
            let mut all_data = Vec::new();
            for item in self.items.iter() {
                if options.favorites {
                    if item.is_fav() {
                        all_data.push(item.clone());
                    }
                } else {
                    all_data.push(item.clone());
                }
            }
            return Ok(all_data);
        }
        let filtered_results = match options.search_type {
            SearchType::MonkeyTyping => filter_items_monkey(&self.items, options),
            SearchType::Exact => filter_items_exact(&self.items, options),
            SearchType::Regex => filter_items_regex(&self.items, options),
        };
        Ok(filtered_results)
    }

    fn total(&self) -> anyhow::Result<HLines> {
        Ok(self.database.iter().count())
    }

    fn favorites(&self) -> anyhow::Result<usize> {
        let mut favorites = 0;
        for result in self.database.iter() {
            let (_key, value): (IVec, IVec) = result?;
            let item: HItem = bincode::deserialize(&value)?;
            if item.is_fav() {
                favorites += 1;
            }
        }
        Ok(favorites)
    }

    fn mark_favorite(&self, id: &str) {
        if let Ok(Some(value)) = self.database.get(id.as_bytes()) {
            if let Ok(mut h_item) = bincode::deserialize::<HItem>(&value) {
                // mark the entry is fav
                h_item.flip_fav();
                if let Ok(bytes) = bincode::serialize(&h_item) {
                    let _ = self.database.insert(id.as_bytes(), bytes);
                }
            }
        }
    }

    fn mark_hit(&self, id: &str) {
        if let Ok(Some(value)) = self.database.get(id.as_bytes()) {
            if let Ok(mut h_item) = bincode::deserialize::<HItem>(&value) {
                // increment the history hits
                h_item.inc_hits();
                if let Ok(bytes) = bincode::serialize(&h_item) {
                    let _ = self.database.insert(id.as_bytes(), bytes);
                }
            }
        }
    }
}
