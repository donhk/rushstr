use std::sync::Arc;

use anyhow::Result;
use rushstr_core::{HistoryCrawler, Scanner, Store, VectorStore};
use rushstr_tui::SearchUI;

fn main() -> Result<()> {
    let scanner: Scanner = Arc::new(HistoryCrawler::new());
    let store: Store = Arc::new(VectorStore::new(scanner)?);
    let search_box = SearchUI::new(&store);
    if let Some(text) = search_box.search()? {
        println!("{}", text);
    }
    Ok(())
}
