use serde::{Deserialize, Serialize};

use crate::{HLines, hash_string};

/// Represents a multi-line shell command entry.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HItem {
    /// A list of strings, each representing a line of the command.
    command: Vec<String>,
    id: String,
    favorite: bool,
    hits: u64,
}

impl HItem {
    /// Creates a new `HItem` from a list of command lines.
    ///
    /// # Arguments
    ///
    /// * `command` - A vector of strings, where each string is a line of the
    ///   command.
    ///
    /// # Returns
    ///
    /// A new instance of `HItem`.
    pub fn new(command: Vec<String>) -> HItem {
        let id = hash_string(&*command.join("\n"));
        Self {
            command,
            id,
            favorite: false,
            hits: 0,
        }
    }

    /// Returns the number of lines in the command.
    ///
    /// # Returns
    ///
    /// An `HLines` value representing the number of lines.
    pub fn hlines(&self) -> HLines {
        self.command.len()
    }

    /// Returns the command as a single-line string.
    ///
    /// Newlines are replaced with spaces, and each line is normalized by
    /// replacing occurrences of `\s+` with `\s`.
    ///
    /// # Returns
    ///
    /// A single string representing the flattened command.
    pub fn command(&self) -> String {
        self.command
            .iter()
            .map(|line| line.split_whitespace().collect::<Vec<_>>().join(" "))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Returns the raw command text as a multi-line string.
    ///
    /// # Returns
    ///
    /// A single string with lines joined by newline characters.
    pub fn raw_text(&self) -> String {
        self.command.join("\n")
    }

    pub fn command_lines(&self) -> Vec<String> {
        self.command.clone()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn is_fav(&self) -> bool {
        self.favorite
    }

    pub fn flip_fav(&mut self) {
        self.favorite = !self.favorite;
    }

    pub fn inc_hits(&mut self) {
        self.hits += 1
    }

    pub fn hits(&self) -> u64 {
        self.hits
    }
}
