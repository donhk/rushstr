use std::sync::Arc;

use crate::StoreTrait;

pub type Store = Arc<dyn StoreTrait>;

pub type HLines = usize;

pub type HIndex = usize;
