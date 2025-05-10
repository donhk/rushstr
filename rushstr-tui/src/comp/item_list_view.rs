use std::collections::HashSet;

use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Color;
use ratatui::prelude::Line;
use ratatui::prelude::Modifier;
use ratatui::prelude::Span;
use ratatui::prelude::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use rushstr_core::prepare_string;

use crate::ux::search_ui::UiState;

pub struct ItemListView<'f> {
    items: &'f [String],
    ui_state: &'f UiState,
    layout: &'f [Rect],
}

impl<'f> ItemListView<'f> {
    pub fn new(items: &'f [String], ui_state: &'f UiState, layout: &'f [Rect]) -> Self {
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
            .skip(self.ui_state.scroll_offset)
            .take(height)
            .cloned()
            .collect::<Vec<_>>();

        let text = &self.ui_state.search_options.input;
        let selected = self.ui_state.selected - self.ui_state.scroll_offset;
        let list_items: Vec<ListItem> = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| format_item(i, item, text, selected))
            .collect();

        let list = List::new(list_items).block(Block::default().borders(Borders::NONE));

        frame.render_widget(list, self.layout[2]);
    }
}

pub(crate) fn format_item(i: usize, item: String, text: &str, selected: usize) -> ListItem {
    let style = if i == selected {
        Style::default()
            .bg(Color::LightYellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let line = if !text.is_empty() {
        let c_text = prepare_string(text);
        let spans = match_tokens(&item, &c_text);
        Line::from(spans)
    } else {
        Line::raw(item)
    };

    ListItem::new(line).style(style)
}

pub(crate) fn create_tokens(text: &str) -> HashSet<char> {
    let mut tokens = HashSet::new();
    for token in text.split_whitespace().filter(|t| !t.is_empty()) {
        for c in token.chars() {
            tokens.insert(c);
        }
    }
    tokens
}

pub(crate) fn token_finder(item: &str, text: &str) -> Vec<(String, bool)> {
    let tokens = create_tokens(text);
    let mut result = Vec::new();
    let mut current = String::new();
    let mut current_flag: Option<bool> = None;

    for c in item.chars() {
        let flag = tokens.contains(&c);
        match current_flag {
            Some(f) if f != flag => {
                result.push((current.clone(), f));
                current.clear();
            },
            None => {},
            _ => {},
        }
        current.push(c);
        current_flag = Some(flag);
    }

    if let Some(flag) = current_flag {
        result.push((current, flag));
    }

    result
}

pub(crate) fn match_tokens<'a>(item: &str, text: &str) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let styled_flags = token_finder(item, text);
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
