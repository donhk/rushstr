use crate::ux::search_ui::UiState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Color, Line, Modifier, Span, Style};
use ratatui::widgets::Paragraph;
use rushstr_core::Store;

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

        let matching = self.ui_state.search_options.search_type.to_str();

        let content = format!(
            " HISTORY - match:{} (C-e) - case:{} (C-t) - {}/{}/0",
            matching,
            case,
            self.items.len(),
            self.store.total()
        );

        let width = frame.area().width;
        // Pad with spaces to fill the width
        let mut padded = content.clone();
        let content_width = content.chars().count() as u16;
        if content_width < width {
            padded.push_str(&" ".repeat((width - content_width) as usize));
        }

        let paragraph = Paragraph::new(
            Line::from(vec![Span::raw(padded)]).style(
                Style::default()
                    .bg(Color::LightGreen)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
        );

        frame.render_widget(paragraph, self.layout[1]);
    }
}
