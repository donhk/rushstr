use crate::StoreTrait;
use std::sync::Arc;

pub type Store = Arc<dyn StoreTrait>;
