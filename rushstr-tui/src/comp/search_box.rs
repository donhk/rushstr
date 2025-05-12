use ratatui::Frame;
use ratatui::layout::{Position, Rect};
use ratatui::prelude::{Color, Line, Span, Style};
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::UiState;

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
            Span::styled("🦀:search:", Style::default().fg(Color::Green)),
            Span::styled("> ", Style::default().fg(Color::Cyan)),
            Span::styled(&self.search_options.search_options.input, Style::default().fg(Color::Green)),
        ];
        let line = Line::from(spans.clone());

        let search = Paragraph::new(line).block(Block::default().borders(Borders::NONE));
        // Show the cursor inside the search bar
        let x_offset: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        let cursor_x = self.layout[0].x + x_offset as u16 + 1; // +1 to account for left border
        let cursor_y = self.layout[0].y;
        let position = Position::new(cursor_x, cursor_y);
        frame.render_widget(search, self.layout[0]);
        frame.set_cursor_position(position);
    }
}
