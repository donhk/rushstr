use rushstr_core::{HIndex, HLines, SearchOptions};

/// Represents the current state of the UI, including selection,
/// scrolling offset, debug mode, and search input configuration.
#[derive(Debug, Clone)]
pub struct UiState {
    /// The index of the currently selected item in the visible list.
    ///
    /// This is a 0-based index (i.e., in the range `0..n-1`) referring to
    /// the item within the filtered or visible items.
    pub selected: HIndex,

    /// The number of lines to skip from the top when rendering the view.
    ///
    /// This allows for vertical scrolling of the list. The offset determines
    /// how many lines (not items) are skipped and typically ranges from `1` up
    /// to `n - window_height`, where `n` is the total number of lines.
    pub offset: HLines,

    /// Whether the UI is in debug mode.
    ///
    /// When enabled, additional debug information may be rendered or logged.
    pub debug: bool,

    /// The search options used for filtering and matching UI entries.
    ///
    /// This includes search text and case sensitivity preferences.
    pub search_options: SearchOptions,
}

impl Default for UiState {
    /// Returns a default `UiState`:
    /// - `selected`: 0 (first item)
    /// - `offset`: 0 (top of the list)
    /// - `debug`: false
    /// - `search_options`: empty search input and default config
    fn default() -> Self {
        UiState {
            selected: 0,
            offset: 0,
            debug: false,
            search_options: SearchOptions::default(),
        }
    }
}
