use ratatui::Frame;
use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::prelude::Color;
use ratatui::prelude::Line;
use ratatui::prelude::Span;
use ratatui::prelude::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;

use crate::ux::search_ui::UiState;

pub struct SearchBox<'f> {
    search_options: &'f UiState,
    layout: &'f [Rect],
}

impl<'f> SearchBox<'f> {
    pub fn new(search_options: &'f UiState, layout: &'f [Rect]) -> Self {
        Self { search_options, layout }
    }

    pub fn render(&self, frame: &mut Frame) {
        let spans = vec![
            Span::raw("🦀🔍"),
            Span::styled(" > ", Style::default().fg(Color::Cyan)),
            Span::styled(&self.search_options.search_options.text, Style::default().fg(Color::Green)),
        ];
        let line = Line::from(spans.clone());

        let search = Paragraph::new(line).block(Block::default().borders(Borders::NONE));
        // Show the cursor inside the search bar
        let x_offset: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        let cursor_x = self.layout[0].x + x_offset as u16 + 2; // +2 to account for left border
        let cursor_y = self.layout[0].y;
        let position = Position::new(cursor_x, cursor_y);
        frame.render_widget(search, self.layout[0]);
        frame.set_cursor_position(position);
    }
}
