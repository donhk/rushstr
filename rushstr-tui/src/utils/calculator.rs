use rushstr_core::{HIndex, HLines};

/// convert an hindex to the hlines number
pub fn hindex_to_hlines(items: &[String], hindex: HIndex) -> HLines {
    let mut hlines = 0;
    for item in &items[..=hindex] {
        hlines += item.split("\n").count();
    }
    hlines
}
