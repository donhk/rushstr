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
                            return get_selected(&items, &ui_state, &self.store);
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
                                return get_selected(&items, &ui_state, &self.store);
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

/// Returns the raw text of the currently selected item in the list, if any.
/// Also marks the item as "hit" in the store.
///
/// # Arguments
///
/// * `items` - A slice of `HItem` representing all the items.
/// * `ui_state` - The current UI state, which includes the selected index.
/// * `store` - The backing store, used to mark the selected item as accessed.
///
/// # Returns
///
/// * `Ok(Some(String))` if a valid item is selected, containing the item's raw
///   text.
/// * `Ok(None)` if the selection index is out of bounds.
fn get_selected(items: &[HItem], ui_state: &UiState, store: &Store) -> anyhow::Result<Option<String>> {
    if let Some(h_item) = items.get(ui_state.selected) {
        store.mark_hit(h_item.id());
        return Ok(Some(h_item.raw_text()));
    }
    Ok(None)
}

/// Appends a character to the search input, as long as the input is less than
/// 50 characters. Resets the selection and scroll offset to the top.
///
/// # Arguments
///
/// * `ui_state` - Mutable reference to the UI state.
/// * `char` - The character to append to the input.
fn put_char(ui_state: &mut UiState, char: char) {
    if ui_state.search_options.input.len() < 50 {
        ui_state.search_options.input.push(char);
        ui_state.selected = 0;
        ui_state.offset = 0;
    }
}

/// Removes the last character from the search input (backspace behavior).
/// Also resets the selection and scroll offset to the top.
///
/// # Arguments
///
/// * `ui_state` - Mutable reference to the UI state.
fn backspace(ui_state: &mut UiState) {
    ui_state.search_options.input.pop();
    ui_state.selected = 0;
    ui_state.offset = 0;
}

/// Moves the selection up by one, ensuring it does not go below zero.
/// Also adjusts the scroll offset if the selection moves above the visible
/// window.
///
/// # Arguments
///
/// * `ui_state` - Mutable reference to the UI state.
fn key_up(ui_state: &mut UiState) {
    ui_state.selected = ui_state.selected.saturating_sub(1);
    if ui_state.selected < ui_state.offset {
        ui_state.offset = ui_state.selected;
    }
}

/// Moves the selection down by one, unless it's already at the bottom.
/// Adjusts the scroll offset to keep the selected item visible.
///
/// # Arguments
///
/// * `ui_state` - Mutable reference to the UI state.
/// * `list_height` - The number of visible lines in the UI list.
/// * `items` - A slice of `HItem` representing all the items.
///
/// # Returns
///
/// * `Ok(())` on success.
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
