use std::rc::Rc;

use anyhow::Result;
use clap::Parser;
use rushstr_core::{ArgsHandler, ConfigOptions, HistoryCrawler, Scanner, Store, VectorStore};
use rushstr_tui::SearchUI;

fn main() -> Result<()> {
    let args = ConfigOptions::parse();
    if ArgsHandler::new(args).execute()? {
        return Ok(());
    }
    let scanner: Scanner = Rc::new(HistoryCrawler::new());
    let store: Store = Rc::new(VectorStore::new(scanner)?);
    if let Some(text) = SearchUI::new(&store).search()? {
        eprint!("{}", text.trim_end());
    }
    Ok(())
}
