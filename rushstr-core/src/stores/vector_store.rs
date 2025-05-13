use sled::{Db, IVec};

use crate::stores::matchers::{filter_items_exact, filter_items_monkey, filter_items_regex};
use crate::{HItem, HLines, Scanner, SearchOptions, SearchType, StoreTrait, create_db};

pub struct VectorStore {
    database: Db,
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
        Ok(VectorStore { database })
    }
}

impl StoreTrait for VectorStore {
    fn items(&self, options: &SearchOptions) -> anyhow::Result<Vec<HItem>> {
        if options.input.is_empty() {
            let mut all_data = Vec::new();
            for result in self.database.iter() {
                let (_key, value): (IVec, IVec) = result?;
                let item: HItem = bincode::deserialize(&value)?;
                all_data.push(item);
            }
            return Ok(all_data);
        }
        Ok(match options.search_type {
            SearchType::MonkeyTyping => filter_items_monkey(&self.database, options),
            SearchType::Exact => filter_items_exact(&self.database, options),
            SearchType::Regex => filter_items_regex(&self.database, options),
        })
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
}
