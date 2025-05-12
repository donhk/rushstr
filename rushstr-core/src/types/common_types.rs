use std::sync::Arc;

use crate::StoreTrait;

pub type Store = Arc<dyn StoreTrait>;

/// Number of Height Lines
pub type HLines = usize;

/// Index within the history
pub type HIndex = usize;
