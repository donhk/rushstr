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

        let input = &self.ui_state.search_options.input;
        let selected = self.ui_state.selected - self.ui_state.offset;
        let list_items: Vec<ListItem> = items
            .into_iter()
            .enumerate()
            .map(|(index, item)| format_item(index, item, input, selected))
            .collect();

        let list = List::new(list_items).block(Block::default().borders(Borders::NONE));

        frame.render_widget(list, self.layout[2]);
    }
}

pub(crate) fn format_item(index: HIndex, item: HItem, input: &str, selected: HIndex) -> ListItem {
    let style = if index == selected {
        Style::default()
            .bg(Color::LightYellow)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    let mut cmd_lines = Vec::new();
    let in_lines = item.command_lines();
    for line in in_lines {
        let o_line = if !input.is_empty() {
            let c_text = prepare_string(input);
            let spans = match_tokens(&line, &c_text);
            Line::from(spans)
        } else {
            Line::raw(line)
        };
        cmd_lines.push(o_line)
    }
    let text = Text::from(cmd_lines);

    ListItem::new(text).style(style)
}

pub(crate) fn create_tokens(input: &str) -> HashSet<char> {
    let mut tokens = HashSet::new();
    for token in input.split_whitespace().filter(|t| !t.is_empty()) {
        for c in token.chars() {
            tokens.insert(c);
        }
    }
    tokens
}

pub(crate) fn token_finder(item: &str, input: &str) -> Vec<(String, bool)> {
    let tokens = create_tokens(input);
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

pub(crate) fn match_tokens<'a>(item: &str, input: &str) -> Vec<Span<'a>> {
    let mut spans = Vec::new();
    let styled_flags = token_finder(item, input);
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
