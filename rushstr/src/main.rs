use anyhow::Result;
use rushstr_core::{Store, VectorStore};
use rushstr_tui::SearchBox;
use std::sync::Arc;

fn main() -> Result<()> {
    let store: Store = Arc::new(VectorStore::new());
    let search_box = SearchBox::new(&store);
    if let Some(text) = search_box.search()? {
        println!("{}", text);
    }
    Ok(())
}
