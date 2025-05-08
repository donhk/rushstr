pub trait StoreTrait {
    fn filter_items(&self, input: &str, case_insensitive:bool) -> Vec<String>;
    fn total(&self) -> usize;
}
