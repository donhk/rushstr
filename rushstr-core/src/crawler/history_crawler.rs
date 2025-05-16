use crate::{HItem, HScanner, detect_shell, read_history};

pub struct HistoryCrawler {}

impl Default for HistoryCrawler {
    fn default() -> Self {
        Self::new()
    }
}

impl HistoryCrawler {
    pub fn new() -> Self {
        Self {}
    }
}

impl HScanner for HistoryCrawler {
    fn load(&self) -> anyhow::Result<Vec<HItem>> {
        let detect_shell = detect_shell();
        let history_entries = read_history(detect_shell);
        let mut h_items = Vec::new();
        for entry in history_entries {
            let cmds = entry.split("\n").map(|m| m.to_string()).collect::<Vec<_>>();
            h_items.push(HItem::new(cmds)?);
        }
        // reverse to start by the most recent first
        h_items.reverse();
        Ok(h_items)
    }
}
