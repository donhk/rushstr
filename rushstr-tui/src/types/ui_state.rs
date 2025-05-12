use rushstr_core::{HIndex, HLines, SearchOptions};

#[derive(Debug, Clone)]
pub struct UiState {
    /// index within the list of results, inclusive 0..n-1
    pub selected: HIndex,
    /// number of lines to skip, 1..n-(window_height)
    pub offset: HLines,
    /// enable debug mode
    pub debug: bool,
    /// search options
    pub search_options: SearchOptions,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            selected: 0,
            offset: 0,
            debug: false,
            search_options: SearchOptions::default(),
        }
    }
}
