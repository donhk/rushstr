use std::collections::HashSet;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, List, ListItem};
use rushstr_core::{HIndex, HItem, prepare_string};

use crate::UiState;

pub struct ItemListView<'f> {
    items: &'f [HItem],
    ui_state: &'f UiState,
    layout: &'f [Rect],
}

impl<'f> ItemListView<'f> {
    pub fn new(items: &'f [HItem], ui_state: &'f UiState, layout: &'f [Rect]) -> Self {
        Self {
            items,
            ui_state,
            layout,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let height = self.layout[2].height as usize;
        let items = self
            .items
            .iter()
            .skip(self.ui_state.offset)
            .take(height)
            .cloned()
            .collect::<Vec<_>>();

        let list_items: Vec<ListItem> = items
            .into_iter()
            .enumerate()
            .map(|(index, item)| format_item(index, item, &self.ui_state))
            .collect();

        let list = List::new(list_items).block(Block::default().borders(Borders::NONE));

        frame.render_widget(list, self.layout[2]);
    }
}

/// Formats an `HItem` into a `ListItem` for display in a TUI list,
/// applying highlighting and selection styling.
///
/// This function handles:
/// - Highlighting the currently selected item.
/// - Applying text highlighting for search matches on each line of the command.
/// - Building a styled `ListItem` from multi-line commands.
///
/// # Arguments
///
/// * `index` - The index of the item in the visible list.
/// * `item` - The `HItem` to format, containing the command lines.
/// * `ui_state` - The current UI state, including selection and search options.
///
/// # Returns
///
/// A `ListItem` that is:
/// - Highlighted if selected.
/// - Styled with `Span` highlights for matched tokens if a search input exists.
/// - Composed of potentially multiple lines, preserving command formatting.
pub(crate) fn format_item(index: HIndex, item: HItem, ui_state: &UiState) -> ListItem {
    let selected = ui_state.selected - ui_state.offset;
    let style = if index == selected {
        Style::default()
            .bg(Color::LightYellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let input = &ui_state.search_options.input;
    let mut cmd_lines = Vec::new();
    let in_lines = item.command_lines();
    for line in in_lines {
        let o_line = if !input.is_empty() {
            let case_insensitive = ui_state.search_options.is_case_insensitive();
            let c_text = prepare_string(input);
            let spans = match_tokens(&line, &c_text, case_insensitive);
            Line::from(spans)
        } else {
            Line::raw(line)
        };
        cmd_lines.push(o_line)
    }
    let text = Text::from(cmd_lines);

    ListItem::new(text).style(style)
}

/// Creates a set of unique characters (tokens) from the input string.
///
/// If `case_insensitive` is true, both lowercase and uppercase variants
/// of each character are included.
///
/// # Arguments
///
/// * `input` - The input string to tokenize.
/// * `case_insensitive` - Whether to treat characters case-insensitively.
///
/// # Returns
///
/// A `HashSet<char>` containing the unique token characters.
pub(crate) fn create_tokens(input: &str, case_insensitive: bool) -> HashSet<char> {
    let mut tokens = HashSet::new();

    for token in input.split_whitespace().filter(|t| !t.is_empty()) {
        for c in token.chars() {
            if case_insensitive {
                tokens.extend(c.to_lowercase());
                tokens.extend(c.to_uppercase());
            } else {
                tokens.insert(c);
            }
        }
    }

    tokens
}

/// Splits an input string (`item`) into segments, tagging each segment with a
/// boolean that indicates whether it contains characters present in the search
/// input.
///
/// The segmentation is based on whether each character is part of the search
/// tokens generated from `input`, considering case sensitivity.
///
/// # Arguments
///
/// * `item` - The string to analyze and split into segments.
/// * `input` - The user-provided search input used to generate matching tokens.
/// * `case_insensitive` - If `true`, the matching ignores character casing.
///
/// # Returns
///
/// A `Vec<(String, bool)>` where each tuple contains:
/// - a substring of `item`
/// - a boolean indicating whether that substring matches any of the search
///   tokens
pub(crate) fn token_finder(item: &str, input: &str, case_insensitive: bool) -> Vec<(String, bool)> {
    let tokens = create_tokens(input, case_insensitive);
    let mut result = Vec::new();
    let mut current = String::new();
    let mut current_flag: Option<bool> = None;

    for char in item.chars() {
        let flag = tokens.contains(&char);
        match current_flag {
            Some(f) if f != flag => {
                result.push((current.clone(), f));
                current.clear();
            },
            None => {},
            _ => {},
        }
        current.push(char);
        current_flag = Some(flag);
    }

    if let Some(flag) = current_flag {
        result.push((current, flag));
    }

    result
}

/// Converts a string (`item`) into a vector of styled spans, where each span
/// highlights whether its characters match the search input.
///
/// This is typically used to visually highlight matching characters (e.g., in a
/// TUI), with matching segments rendered in bold red text and non-matching
/// segments unstyled.
///
/// # Arguments
///
/// * `item` - The string to be analyzed and segmented into styled spans.
/// * `input` - The user input string used to determine which characters to
///   highlight.
/// * `case_insensitive` - Whether to perform case-insensitive matching.
///
/// # Returns
///
/// A `Vec<Span>` representing the original string broken into styled and
/// unstyled parts, depending on character matches against the input.
pub(crate) fn match_tokens<'a>(item: &str, input: &str, case_insensitive: bool) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let styled_flags = token_finder(item, input, case_insensitive);
    for (str, red) in styled_flags {
        let span = if red {
            Span::styled(str, Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        } else {
            Span::raw(str)
        };
        spans.push(span);
    }
    spans
}
