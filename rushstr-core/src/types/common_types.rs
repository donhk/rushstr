use std::rc::Rc;
use std::sync::Mutex;

use crate::crawler::scanner::HScanner;
use crate::{HItem, StoreTrait};

pub type Scanner = Rc<dyn HScanner>;

pub type Store = Rc<dyn StoreTrait>;

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
