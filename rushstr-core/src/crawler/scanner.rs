use crate::HItem;

pub trait HScanner {
    fn load(&self) -> anyhow::Result<Vec<HItem>>;
}
