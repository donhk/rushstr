use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::Paragraph;
use rushstr_core::{HItem, HLines, Store};

use crate::{UiState, hindex_to_hlines};

pub struct InfoBar<'f> {
    items: &'f [HItem],
    store: &'f Store,
    ui_state: &'f UiState,
    layout: &'f [Rect],
}

impl<'f> InfoBar<'f> {
    pub fn new(items: &'f [HItem], ui_state: &'f UiState, store: &'f Store, layout: &'f [Rect]) -> Self {
        Self {
            items,
            ui_state,
            store,
            layout,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let case = if self.ui_state.search_options.is_case_insensitive() {
            "insensitive"
        } else {
            "sensitive"
        };
        let height = frame.area().height as usize - 2;
        let matching = self.ui_state.search_options.search_type.to_str();
        let mut spans = self.build_info_bar(case, matching, height);

        let width = frame.area().width;
        // Pad with spaces to fill the width
        let content_width = Line::from(spans.clone()).to_string().chars().count() as u16;
        let mut padded = Vec::new();
        if content_width < width {
            for _ in 0..(width - content_width) {
                padded.push(Span::raw(" "));
            }
        }
        spans.extend(padded);

        let paragraph = Paragraph::new(
            Line::from(spans).style(
                Style::default()
                    .bg(Color::Black)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        );

        frame.render_widget(paragraph, self.layout[1]);
    }

    fn build_info_bar(&self, case: &str, matching: &str, height: HLines) -> Vec<Span> {
        if self.ui_state.debug {
            return self.build_debug_info_bar(case, matching, height);
        }
        self.base_info_base(case, matching)
    }

    fn base_info_base(&self, case: &str, matching: &str) -> Vec<Span> {
        let store_total = self.store.total().ok().unwrap_or(0).to_string();
        let store_favorites = self.store.favorites().ok().unwrap_or(0).to_string();
        let favs_style = if self.ui_state.search_options.favorites {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };
        vec![
            Span::raw("HISTORY -"),
            Span::raw(" (⭐C-x)"),
            Span::styled(" (C-f)", favs_style),
            Span::raw(" (C-t)"),
            Span::raw(" match:"),
            Span::styled(format!("{matching:<15}"), Style::default().fg(Color::Blue)),
            Span::raw("case:"),
            Span::styled(format!("{case:<15}"), Style::default().fg(Color::Blue)),
            Span::raw(" - "),
            Span::styled(self.items.len().to_string(), Style::default().fg(Color::Blue)),
            Span::raw("/"),
            Span::styled(store_total, Style::default().fg(Color::Blue)),
            Span::raw("/"),
            Span::styled(store_favorites, Style::default().fg(Color::Blue)),
        ]
    }

    fn build_debug_info_bar(&self, case: &str, matching: &str, height: HLines) -> Vec<Span> {
        let h_to_selected = hindex_to_hlines(self.items, self.ui_state.selected);
        let mut parts = self.base_info_base(case, matching);
        let debug = vec![
            Span::raw(" 🐛 height:"),
            Span::styled(format!("{height:<4}"), Style::default().fg(Color::Yellow)),
            Span::raw("selected:"),
            Span::styled(format!("{:<4}", self.ui_state.selected), Style::default().fg(Color::Yellow)),
            Span::raw("offset:"),
            Span::styled(format!("{:<4}", self.ui_state.offset), Style::default().fg(Color::Yellow)),
            Span::raw("h_to_selected:"),
            Span::styled(format!("{h_to_selected:<4}"), Style::default().fg(Color::Yellow)),
        ];
        parts.extend(debug);
        parts
    }
}
