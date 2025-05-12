use std::sync::Arc;

use anyhow::Result;
use rushstr_core::{MemCrawler, Store, VectorStore};
use rushstr_tui::SearchUI;

fn main() -> Result<()> {
    let store: Store = Arc::new(VectorStore::new(Arc::new(MemCrawler::new())));
    let search_box = SearchUI::new(&store);
    if let Some(text) = search_box.search()? {
        println!("{}", text);
    }
    Ok(())
}
