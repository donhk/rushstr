use std::io::stdout;

use arboard::Clipboard;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyModifiers, MouseButton, MouseEventKind,
};
use ratatui::crossterm::{event, execute};
use rushstr_core::{HItem, Store};

use crate::UiState;
use crate::ux::ui_render_engine::UiRenderEngine;

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
            let items = self.store.items(&ui_state.search_options).ok().unwrap_or(vec![]);
            let height = (terminal.size()?.height - 2) as usize;
            terminal.draw(|frame| UiRenderEngine::new(&items, &ui_state, &self.store).render(frame))?;

            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Esc => return Ok(None),
                        KeyCode::Char('x') if key.modifiers == KeyModifiers::CONTROL => {
                            if let Some(item) = items.get(ui_state.selected) {
                                self.store.mark_favorite(item.id());
                            }
                        },
                        KeyCode::Char('f') if key.modifiers == KeyModifiers::CONTROL => {
                            ui_state.search_options.favorites = !ui_state.search_options.favorites;
                        },
                        KeyCode::Char('d') if key.modifiers == KeyModifiers::CONTROL => {
                            ui_state.debug = !ui_state.debug
                        },
                        KeyCode::Char('t') if key.modifiers == KeyModifiers::CONTROL => {
                            ui_state.search_options.search_type = ui_state.search_options.search_type.next()
                        },
                        KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                            if let Some(cmd) = items.get(ui_state.selected) {
                                Clipboard::new()?.set_text(cmd.raw_text())?;
                            }
                            return Ok(None);
                        },
                        KeyCode::Up => key_up(&mut ui_state),
                        KeyCode::Down => {
                            key_down(&mut ui_state, height, &items)?;
                        },
                        KeyCode::Enter => {
                            if let Some(cmd) = items.get(ui_state.selected) {
                                return Ok(Some(cmd.raw_text()));
                            }
                        },
                        KeyCode::Char(c) => put_char(&mut ui_state, c),
                        KeyCode::Backspace => backspace(&mut ui_state),
                        _ => {},
                    }
                },
                Event::Mouse(mouse_event) => {
                    match mouse_event.kind {
                        MouseEventKind::ScrollUp => key_up(&mut ui_state),
                        MouseEventKind::ScrollDown => {
                            key_down(&mut ui_state, height, &items)?;
                        },
                        MouseEventKind::Down(button) => {
                            if button == MouseButton::Middle {
                                if let Some(cmd) = items.get(ui_state.selected) {
                                    return Ok(Some(cmd.raw_text()));
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
}

fn put_char(ui_state: &mut UiState, char: char) {
    if ui_state.search_options.input.len() < 50 {
        ui_state.search_options.input.push(char);
        ui_state.selected = 0;
        ui_state.offset = 0;
    }
}

fn backspace(ui_state: &mut UiState) {
    ui_state.search_options.input.pop();
    ui_state.selected = 0;
    ui_state.offset = 0;
}

fn key_up(ui_state: &mut UiState) {
    ui_state.selected = ui_state.selected.saturating_sub(1);
    if ui_state.selected < ui_state.offset {
        ui_state.offset = ui_state.selected;
    }
}

fn key_down(ui_state: &mut UiState, list_height: usize, items: &[HItem]) -> anyhow::Result<()> {
    if ui_state.selected + 1 >= items.len() {
        return Ok(());
    }
    ui_state.selected += 1;
    if ui_state.selected >= ui_state.offset + list_height {
        ui_state.offset = ui_state.selected + 1 - list_height;
    }
    Ok(())
}
