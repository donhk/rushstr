use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::crawler::scanner::HScanner;
use crate::{HItem, StoreTrait};

pub type Scanner = Arc<dyn HScanner>;

pub type Store = Arc<dyn StoreTrait>;

/// Number of Height Lines
pub type HLines = usize;

/// Index within the history
pub type HIndex = usize;

/// h item key
pub type Key = [u8; 32];

/// mutable HItem
pub type MHItem = Rc<Mutex<HItem>>;

#[derive(Debug, PartialEq)]
pub enum Shell {
    Unknown,
    Zsh,
    Bash,
    Csh,
}
