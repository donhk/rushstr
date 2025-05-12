use rushstr_core::{HIndex, HItem, HLines};

/// convert an hindex to the hlines number
pub fn hindex_to_hlines(items: &[HItem], hindex: HIndex) -> HLines {
    let mut hlines = 0;
    for item in &items[..=hindex] {
        hlines += item.hlines();
    }
    hlines
}
