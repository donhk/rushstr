use crate::HLines;

/// Represents a multi-line shell command entry.
#[derive(Clone, Debug)]
pub struct HItem {
    /// A list of strings, each representing a line of the command.
    command: Vec<String>,
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
        Self { command }
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
            .map(|m| m.replace("\\s+", "\\s"))
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
}
