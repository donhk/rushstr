use std::sync::Arc;

use crate::StoreTrait;
use crate::crawler::scanner::HScanner;

pub type Scanner = Arc<dyn HScanner>;

pub type Store = Arc<dyn StoreTrait>;

/// Number of Height Lines
pub type HLines = usize;

/// Index within the history
pub type HIndex = usize;

/// h item key
pub type Key = [u8; 32];

#[derive(Debug, PartialEq)]
pub enum Shell {
    Unknown,
    Zsh,
    Bash,
    Csh,
}
