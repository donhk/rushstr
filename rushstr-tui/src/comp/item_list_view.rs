use crate::ux::search_ui::UiState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::{Block, Borders, List, ListItem};

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

        let text = &self.ui_state.search_options.text;
        let selected = self.ui_state.selected - self.ui_state.scroll_offset;
        let list_items: Vec<ListItem> = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| self.format_item(i, item, text, selected))
            .collect();

        let list = List::new(list_items).block(Block::default().borders(Borders::NONE));

        frame.render_widget(list, self.layout[2]);
    }

    fn format_item(&self, i: usize, item: String, text: &str, selected: usize) -> ListItem {
        let style = if i == selected {
            Style::default()
                .bg(Color::LightYellow)
                .fg(Color::Black)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };

        let line = if !text.is_empty() {
            let mut spans = Vec::new();
            let mut remaining = item.as_str();
            let mut last_offset = 0;

            while let Some(pos) = remaining.find(text) {
                let abs_pos = last_offset + pos;

                if pos > 0 {
                    spans.push(Span::raw(item[last_offset..abs_pos].to_string()));
                }

                let match_end = abs_pos + text.len();
                spans.push(Span::styled(
                    item[abs_pos..match_end].to_string(),
                    Style::default().fg(Color::Red),
                ));

                last_offset = match_end;
                remaining = &item[last_offset..];
            }

            if last_offset < item.len() {
                spans.push(Span::raw(item[last_offset..].to_string()));
            }

            Line::from(spans)
        } else {
            Line::raw(item)
        };

        ListItem::new(line).style(style)
    }
}
