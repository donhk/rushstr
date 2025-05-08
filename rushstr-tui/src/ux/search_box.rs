use ratatui::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind,
};
use ratatui::crossterm::{event, execute};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::prelude::{Color, Line, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use rushstr_core::Store;
use std::io::stdout;

pub struct SearchBox {
    store: Store,
}

impl SearchBox {
    pub fn new(store: &Store) -> SearchBox {
        SearchBox { store: store.clone() }
    }

    pub fn search(&self) -> anyhow::Result<Option<String>> {
        let mut terminal = ratatui::init();
        terminal.clear()?;
        execute!(stdout(), EnableMouseCapture)?;

        let result = self.search_items(&mut terminal);

        execute!(stdout(), DisableMouseCapture)?;
        ratatui::restore();
        result
    }

    fn search_items(&self, terminal: &mut DefaultTerminal) -> anyhow::Result<Option<String>> {
        let mut selected = 0;
        let mut scroll_offset = 0;
        let mut text = String::new();
        loop {
            let items = self.store.filter_items(&text);
            terminal.draw(|frame| self.draw_ui(frame, &items, &text, selected, scroll_offset))?;
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => return Ok(None),
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(None),
                    KeyCode::Up => {
                        selected = selected.saturating_sub(1);
                        if selected < scroll_offset {
                            scroll_offset = selected;
                        }
                    },
                    KeyCode::Down => {
                        self.move_down(&mut selected, &mut scroll_offset, terminal.size()?.height, &items)?;
                    },
                    KeyCode::Enter => {
                        if let Some(cmd) = items.get(selected) {
                            return Ok(Some(cmd.to_string()));
                        }
                    },
                    KeyCode::Char(c) => {
                        if text.len() < 50 {
                            text.push(c);
                            selected = 0;
                            scroll_offset = 0;
                        }
                    },
                    KeyCode::Backspace => {
                        text.pop();
                        selected = 0;
                        scroll_offset = 0;
                    },
                    _ => {},
                },
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        selected = selected.saturating_sub(1);
                        if selected < scroll_offset {
                            scroll_offset = selected;
                        }
                    },
                    MouseEventKind::ScrollDown => {
                        self.move_down(&mut selected, &mut scroll_offset, terminal.size()?.height, &items)?;
                    },
                    MouseEventKind::Down(_button) => {
                        if let Some(cmd) = items.get(selected) {
                            return Ok(Some(cmd.to_string()));
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        }
    }

    fn move_down(
        &self,
        selected: &mut usize,
        scroll_offset: &mut usize,
        height: u16,
        items: &[String],
    ) -> anyhow::Result<()> {
        if *selected + 1 < items.len() {
            *selected += 1;
            let list_height = height as usize - 4;
            if *selected >= *scroll_offset + list_height {
                *scroll_offset = *selected + 1 - list_height;
            }
        }
        Ok(())
    }

    fn draw_ui(&self, frame: &mut Frame, all_items: &[String], text: &str, selected: usize, scroll_offset: usize) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Length(3), // Search box
                Constraint::Length(1), // Help
                Constraint::Min(1),    // List
            ])
            .split(frame.area());

        // ─── Render Search Bar ─────────────────────────────────────
        self.render_search_box(frame, text, &layout);

        let list_height = layout[2].height as usize;
        let items = all_items
            .iter()
            .skip(scroll_offset)
            .take(list_height)
            .cloned()
            .collect::<Vec<_>>();

        // ─── Render Header Line ─────────────────────────────────────\
        frame.render_widget(self.render_help(all_items.len(), frame.area().width), layout[1]);

        // ─── Render Result List ─────────────────────────────────────
        frame.render_widget(self.render_items_list(items, selected - scroll_offset), layout[2]);
    }

    fn render_search_box(&self, frame: &mut Frame, text: &str, layout: &[Rect]) {
        let search = Paragraph::new(Line::from(text)).block(Block::default().borders(Borders::BOTTOM).title("Search"));
        // Show the cursor inside the search bar
        let x_offset = text.chars().count(); // Use byte offset if using Unicode-aware rendering
        let cursor_x = layout[0].x + x_offset as u16 + 1; // +1 to account for left border
        let cursor_y = layout[0].y + 1; // +1 to be inside the border
        Position::new(cursor_x, cursor_y);
        frame.render_widget(search, layout[0]);
    }

    fn render_items_list(&self, items: Vec<String>, selected: usize) -> List {
        let list_items: Vec<ListItem> = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| {
                let style = if i == selected {
                    Style::default()
                        .bg(Color::Gray)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(item).style(style)
            })
            .collect();

        List::new(list_items).block(Block::default().borders(Borders::NONE))
    }

    fn render_help(&self, items: usize, width: u16) -> Paragraph {
        let content = format!(
            " HISTORY - view:ranking (C-/) - match:keywords (C-e) - case:insensitive (C-t) - {}/{}/0",
            items,
            self.store.total()
        );

        // Pad with spaces to fill the width
        let mut padded = content.clone();
        let content_width = content.chars().count() as u16;
        if content_width < width {
            padded.push_str(&" ".repeat((width - content_width) as usize));
        }

        Paragraph::new(
            Line::from(vec![Span::raw(padded)]).style(
                Style::default()
                    .bg(Color::Gray)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ),
        )
    }
}
