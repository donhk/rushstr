use std::sync::Arc;

use crate::StoreTrait;

pub type Store = Arc<dyn StoreTrait>;
