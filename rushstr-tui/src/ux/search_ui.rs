use std::io::stdout;

use arboard::Clipboard;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event;
use ratatui::crossterm::event::DisableMouseCapture;
use ratatui::crossterm::event::EnableMouseCapture;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::crossterm::event::MouseButton;
use ratatui::crossterm::event::MouseEventKind;
use ratatui::crossterm::execute;
use rushstr_core::SearchOptions;
use rushstr_core::Store;

use crate::ux::ui_render_engine::UiRenderEngine;

#[derive(Debug, Clone)]
pub(crate) struct UiState {
    pub selected: usize,
    pub scroll_offset: usize,
    pub search_options: SearchOptions,
}

impl Default for UiState {
    fn default() -> Self {
        UiState {
            selected: 0,
            scroll_offset: 0,
            search_options: SearchOptions::default(),
        }
    }
}

pub struct SearchUI {
    store: Store,
}

impl SearchUI {
    pub fn new(store: &Store) -> SearchUI {
        SearchUI { store: store.clone() }
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
        let mut ui_state = UiState::default();
        loop {
            let items = self.store.filter_items(&ui_state.search_options);

            terminal.draw(|frame| UiRenderEngine::new(&items, &ui_state, &self.store).render(frame))?;

            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Esc => return Ok(None),
                        KeyCode::Char('f') if key.modifiers == KeyModifiers::CONTROL => {
                            // mark as favorite
                        },
                        KeyCode::Char('t') if key.modifiers == KeyModifiers::CONTROL => {
                            ui_state.search_options.search_type = ui_state.search_options.search_type.next()
                        },
                        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                            if let Some(cmd) = items.get(ui_state.selected) {
                                let mut clipboard = Clipboard::new()?;
                                clipboard.set_text(cmd)?;
                            }
                            return Ok(None);
                        },
                        KeyCode::Up => {
                            ui_state.selected = ui_state.selected.saturating_sub(1);
                            if ui_state.selected < ui_state.scroll_offset {
                                ui_state.scroll_offset = ui_state.selected;
                            }
                        },
                        KeyCode::Down => {
                            self.move_down(&mut ui_state, terminal.size()?.height, &items)?;
                        },
                        KeyCode::Enter => {
                            if let Some(cmd) = items.get(ui_state.selected) {
                                return Ok(Some(cmd.to_string()));
                            }
                        },
                        KeyCode::Char(c) => {
                            if ui_state.search_options.text.len() < 50 {
                                ui_state.search_options.text.push(c);
                                ui_state.selected = 0;
                                ui_state.scroll_offset = 0;
                            }
                        },
                        KeyCode::Backspace => {
                            ui_state.search_options.text.pop();
                            ui_state.selected = 0;
                            ui_state.scroll_offset = 0;
                        },
                        _ => {},
                    }
                },
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => {
                            ui_state.selected = ui_state.selected.saturating_sub(1);
                            if ui_state.selected < ui_state.scroll_offset {
                                ui_state.scroll_offset = ui_state.selected;
                            }
                        },
                        MouseEventKind::ScrollDown => {
                            self.move_down(&mut ui_state, terminal.size()?.height, &items)?;
                        },
                        MouseEventKind::Down(button) => {
                            if button == MouseButton::Middle {
                                if let Some(cmd) = items.get(ui_state.selected) {
                                    return Ok(Some(cmd.to_string()));
                                }
                            }
                        },
                        _ => {},
                    }
                },
                _ => {},
            }
        }
    }

    fn move_down(&self, search_options: &mut UiState, height: u16, items: &[String]) -> anyhow::Result<()> {
        if search_options.selected + 1 < items.len() {
            search_options.selected += 1;
            let list_height = height as usize - 2;
            if search_options.selected >= search_options.scroll_offset + list_height {
                search_options.scroll_offset = search_options.selected + 1 - list_height;
            }
        }
        Ok(())
    }
}
