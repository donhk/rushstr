use std::sync::Arc;

use crate::StoreTrait;
use crate::crawler::scanner::HScanner;

pub type Scanner = Arc<dyn HScanner>;

pub type Store = Arc<dyn StoreTrait>;

/// Number of Height Lines
pub type HLines = usize;

/// Index within the history
pub type HIndex = usize;
