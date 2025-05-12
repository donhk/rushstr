use crate::HItem;

pub trait HScanner {
    fn load(&self) -> Vec<HItem>;
}
