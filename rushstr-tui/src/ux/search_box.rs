use ratatui::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use ratatui::crossterm::{event, execute};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::prelude::{Color, Line, Modifier, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use rushstr_core::Store;
use std::io::stdout;

struct SearchOptions {
    pub selected: usize,
    pub scroll_offset: usize,
    pub text: String,
    pub case_insensitive: bool,
    pub whole_words: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            selected: 0,
            scroll_offset: 0,
            text: "".to_string(),
            case_insensitive: true,
            whole_words: false,
        }
    }
}

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
        let mut search_options = SearchOptions::default();
        loop {
            let items = self
                .store
                .filter_items(&search_options.text, search_options.case_insensitive);
            terminal.draw(|frame| self.draw_ui(frame, &items, &search_options))?;
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                    KeyCode::Esc => return Ok(None),
                    KeyCode::Char('e') if key.modifiers == KeyModifiers::CONTROL => {
                        search_options.whole_words = !search_options.whole_words
                    },
                    KeyCode::Char('t') if key.modifiers == KeyModifiers::CONTROL => {
                        search_options.case_insensitive = !search_options.case_insensitive
                    },
                    KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => return Ok(None),
                    KeyCode::Up => {
                        search_options.selected = search_options.selected.saturating_sub(1);
                        if search_options.selected < search_options.scroll_offset {
                            search_options.scroll_offset = search_options.selected;
                        }
                    },
                    KeyCode::Down => {
                        self.move_down(&mut search_options, terminal.size()?.height, &items)?;
                    },
                    KeyCode::Enter => {
                        if let Some(cmd) = items.get(search_options.selected) {
                            return Ok(Some(cmd.to_string()));
                        }
                    },
                    KeyCode::Char(c) => {
                        if search_options.text.len() < 50 {
                            search_options.text.push(c);
                            search_options.selected = 0;
                            search_options.scroll_offset = 0;
                        }
                    },
                    KeyCode::Backspace => {
                        search_options.text.pop();
                        search_options.selected = 0;
                        search_options.scroll_offset = 0;
                    },
                    _ => {},
                },
                Event::Mouse(mouse_event) => match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        search_options.selected = search_options.selected.saturating_sub(1);
                        if search_options.selected < search_options.scroll_offset {
                            search_options.scroll_offset = search_options.selected;
                        }
                    },
                    MouseEventKind::ScrollDown => {
                        self.move_down(&mut search_options, terminal.size()?.height, &items)?;
                    },
                    MouseEventKind::Down(button) => {
                        if button == MouseButton::Middle {
                            if let Some(cmd) = items.get(search_options.selected) {
                                return Ok(Some(cmd.to_string()));
                            }
                        }
                    },
                    _ => {},
                },
                _ => {},
            }
        }
    }

    fn move_down(&self, search_options: &mut SearchOptions, height: u16, items: &[String]) -> anyhow::Result<()> {
        if search_options.selected + 1 < items.len() {
            search_options.selected += 1;
            let list_height = height as usize - 2;
            if search_options.selected >= search_options.scroll_offset + list_height {
                search_options.scroll_offset = search_options.selected + 1 - list_height;
            }
        }
        Ok(())
    }

    fn draw_ui(&self, frame: &mut Frame, all_items: &[String], search_options: &SearchOptions) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Length(1), // Search box
                Constraint::Length(1), // Help
                Constraint::Min(1),    // List
            ])
            .split(frame.area());

        // ─── Render Search Bar ─────────────────────────────────────
        self.render_search_box(frame, &search_options.text, &layout);

        let list_height = layout[2].height as usize;
        let items = all_items
            .iter()
            .skip(search_options.scroll_offset)
            .take(list_height)
            .cloned()
            .collect::<Vec<_>>();

        // ─── Render Header Line ─────────────────────────────────────\
        frame.render_widget(self.render_help(all_items.len(), frame.area().width, search_options), layout[1]);

        // ─── Render Result List ─────────────────────────────────────
        frame.render_widget(self.render_items_list(items, search_options), layout[2]);
    }

    fn render_search_box(&self, frame: &mut Frame, text: &str, layout: &[Rect]) {
        let spans = vec![
            Span::raw("🦀🔍"),
            Span::styled(" > ", Style::default().fg(Color::Cyan)),
            Span::styled(text, Style::default().fg(Color::Green)),
        ];
        let line = Line::from(spans.clone());

        let search = Paragraph::new(line).block(Block::default().borders(Borders::NONE));
        // Show the cursor inside the search bar
        let x_offset: usize = spans.iter().map(|s| s.content.chars().count()).sum();
        let cursor_x = layout[0].x + x_offset as u16 + 2; // +2 to account for left border
        let cursor_y = layout[0].y;
        let position = Position::new(cursor_x, cursor_y);
        frame.render_widget(search, layout[0]);
        frame.set_cursor_position(position);
    }

    fn render_items_list(&self, items: Vec<String>, search_options: &SearchOptions) -> List {
        let text = &search_options.text;
        let selected = search_options.selected - search_options.scroll_offset;
        let list_items: Vec<ListItem> = items
            .into_iter()
            .enumerate()
            .map(|(i, item)| self.format_item(i, item, text, selected))
            .collect();

        List::new(list_items).block(Block::default().borders(Borders::NONE))
    }

    fn format_item(&self, i: usize, item: String, text: &str, selected: usize) -> ListItem {
        let style = if i == selected {
            Style::default()
                .bg(Color::Gray)
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

    fn render_help(&self, items: usize, width: u16, search_options: &SearchOptions) -> Paragraph {
        let case = if search_options.case_insensitive {
            "insensitive"
        } else {
            "sensitive"
        };

        let matching = if search_options.whole_words {
            "exact"
        } else {
            "keywords"
        };

        let content = format!(
            " HISTORY - match:{} (C-e) - case:{} (C-t) - {}/{}/0",
            matching,
            case,
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
