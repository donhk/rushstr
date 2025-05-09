use ratatui::Frame;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use rushstr_core::Store;

use crate::comp::info_bar::InfoBar;
use crate::comp::item_list_view::ItemListView;
use crate::comp::search_box::SearchBox;
use crate::ux::search_ui::UiState;

pub struct UiRenderEngine<'f> {
    items: &'f [String],
    store: &'f Store,
    search_options: &'f UiState,
}

impl<'f> UiRenderEngine<'f> {
    pub fn new(items: &'f [String], search_options: &'f UiState, store: &'f Store) -> Self {
        Self {
            items,
            search_options,
            store,
        }
    }

    pub fn render(&self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints([
                Constraint::Length(1), // Search box
                Constraint::Length(1), // InfoBar
                Constraint::Min(1),    // List
            ])
            .split(frame.area());

        // ─── Render Search Bar ─────────────────────────────────────
        SearchBox::new(self.search_options, &layout).render(frame);

        // ─── Render Header Line ─────────────────────────────────────
        InfoBar::new(self.items, self.search_options, self.store, &layout).render(frame);

        // ─── Render Result List ─────────────────────────────────────
        ItemListView::new(self.items, self.search_options, &layout).render(frame);
    }
}
