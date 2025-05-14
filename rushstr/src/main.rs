use std::sync::Arc;

use anyhow::Result;
use clap::Parser;
use rushstr_core::{ConfigOptions, HistoryCrawler, Scanner, Store, VectorStore};
use rushstr_tui::SearchUI;

fn main() -> Result<()> {
    let args = ConfigOptions::parse();
    if args.zsh_shell_conf {
        println!("Magic");
        return Ok(());
    }
    let scanner: Scanner = Arc::new(HistoryCrawler::new());
    let store: Store = Arc::new(VectorStore::new(scanner)?);
    if let Some(text) = SearchUI::new(&store).search()? {
        println!("{}", text);
    }
    Ok(())
}
