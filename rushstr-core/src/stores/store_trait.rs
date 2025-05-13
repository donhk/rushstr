use crate::{HItem, HLines, SearchOptions};

pub trait StoreTrait {
    fn items(&self, options: &SearchOptions) -> anyhow::Result<Vec<HItem>>;

    fn total(&self) -> anyhow::Result<HLines>;

    fn favorites(&self) -> anyhow::Result<usize>;
}
