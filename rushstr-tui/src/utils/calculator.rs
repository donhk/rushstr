use rushstr_core::{HIndex, HItem, HLines};

/// Converts a history index (`hindex`) into the total number of visual lines
/// (`hlines`) occupied by the history items up to and including that index.
///
/// This is useful when rendering or navigating history entries where each item
/// may span multiple terminal lines (e.g., due to multi-line commands).
///
/// # Arguments
///
/// * `items` - A slice of `HItem` instances representing history entries.
/// * `hindex` - The index of the last item (inclusive) whose `hlines` should be
///   included in the total.
///
/// # Returns
///
/// The total number of `hlines` (visual terminal lines) for all items from
/// index 0 to `hindex`, inclusive.
///
/// If `hindex` is out of bounds or the slice is empty, the function returns 0.
pub fn hindex_to_hlines(items: &[HItem], hindex: HIndex) -> HLines {
    if items.is_empty() || hindex >= items.len() {
        return 0;
    }
    items[..=hindex].iter().map(|item| item.hlines()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hitem_with_lines(lines: &[&str]) -> HItem {
        HItem::new(lines.iter().map(|s| s.to_string()).collect()).unwrap()
    }

    #[test]
    fn test_empty_list() {
        let items: Vec<HItem> = vec![];
        assert_eq!(hindex_to_hlines(&items, 0), 0);
    }

    #[test]
    fn test_hindex_out_of_bounds() {
        let items = vec![hitem_with_lines(&["echo hi"]), hitem_with_lines(&["ls", "-la"])];
        assert_eq!(hindex_to_hlines(&items, 5), 0);
    }

    #[test]
    fn test_single_item() {
        let items = vec![hitem_with_lines(&["echo hello", "echo again"])];
        assert_eq!(hindex_to_hlines(&items, 0), 2);
    }

    #[test]
    fn test_multiple_items() {
        let items = vec![
            hitem_with_lines(&["line1"]),
            hitem_with_lines(&["line2a", "line2b"]),
            hitem_with_lines(&["line3a", "line3b", "line3c"]),
        ];
        assert_eq!(hindex_to_hlines(&items, 0), 1);
        assert_eq!(hindex_to_hlines(&items, 1), 3); // 1 + 2
        assert_eq!(hindex_to_hlines(&items, 2), 6); // 1 + 2 + 3
    }

    #[test]
    fn test_hindex_exactly_last() {
        let items = vec![
            hitem_with_lines(&["a"]),
            hitem_with_lines(&["b"]),
            hitem_with_lines(&["c"]),
        ];
        assert_eq!(hindex_to_hlines(&items, items.len() - 1), 3);
    }
}
