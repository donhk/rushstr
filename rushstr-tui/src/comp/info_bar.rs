use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::Paragraph;
use rushstr_core::{HLines, Store};

use crate::UiState;

pub struct InfoBar<'f> {
    items: &'f [String],
    store: &'f Store,
    ui_state: &'f UiState,
    layout: &'f [Rect],
}

impl<'f> InfoBar<'f> {
    pub fn new(items: &'f [String], ui_state: &'f UiState, store: &'f Store, layout: &'f [Rect]) -> Self {
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
        let height = frame.area().height as usize;
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
                    .bg(Color::LightGreen)
                    .fg(Color::Black)
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
        vec![
            Span::raw("HISTORY - (C-f) ⭐ (C-t) match:"),
            Span::styled(format!("{:<15}", matching), Style::default().fg(Color::Blue)),
            Span::raw(" case:"),
            Span::styled(format!("{:<15}", case), Style::default().fg(Color::Blue)),
            Span::raw(" - "),
            Span::styled(self.items.len().to_string(), Style::default().fg(Color::Blue)),
            Span::raw("/"),
            Span::styled(self.store.total().to_string(), Style::default().fg(Color::Blue)),
            Span::raw("/"),
            Span::styled(self.store.favorites().to_string(), Style::default().fg(Color::Blue)),
        ]
    }

    fn build_debug_info_bar(&self, case: &str, matching: &str, height: HLines) -> Vec<Span> {
        let mut parts = self.base_info_base(case, matching);
        let debug = vec![
            Span::raw(" 🐛 height:"),
            Span::styled(format!("{:<3}", height), Style::default().fg(Color::Yellow)),
            Span::raw("selected:"),
            Span::styled(format!("{:<3}", self.ui_state.selected), Style::default().fg(Color::Yellow)),
            Span::raw("offset:"),
            Span::styled(format!("{:<3}", self.ui_state.offset), Style::default().fg(Color::Yellow)),
        ];
        parts.extend(debug);
        parts
    }
}
