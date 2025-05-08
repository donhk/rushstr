pub trait StoreTrait {
    fn filter_items(&self, input: &str) -> Vec<String>;
    fn total(&self) -> usize;
}
