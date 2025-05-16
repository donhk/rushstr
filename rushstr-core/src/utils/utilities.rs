use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs};

use regex::Regex;
use sha2::{Digest, Sha256};
use sled::Db;

use crate::{RushstrFiles, Shell};

/// Detects the current user's shell based on the `SHELL` environment variable.
///
/// This function is intended to work on Unix-like systems (e.g., Linux and
/// macOS). It reads the value of the `SHELL` environment variable and returns a
/// corresponding variant of the `Shell` enum.
///
/// # Returns
///
/// - `Shell::Zsh` if the shell path contains "zsh"
/// - `Shell::Bash` if the shell path contains "bash"
/// - `Shell::Csh` if the shell path contains "csh"
/// - `Shell::Unknown` if the `SHELL` variable is not set or the shell is
///   unrecognized
pub fn detect_shell() -> Shell {
    if let Ok(shell_path) = env::var("SHELL") {
        if shell_path.contains("zsh") {
            Shell::Zsh
        } else if shell_path.contains("bash") {
            Shell::Bash
        } else if shell_path.contains("csh") {
            Shell::Csh
        } else {
            Shell::Unknown
        }
    } else {
        Shell::Unknown
    }
}

/// Reads the shell history file based on the given `Shell` type.
///
/// This function returns the history as a list of strings, one per line.
/// If the history file cannot be found or read, an empty list is returned.
///
/// # Arguments
///
/// * `shell` - A variant of the `Shell` enum indicating the user's shell.
///
/// # Returns
///
/// A vector of strings representing each line in the history file.
///
/// # Notes
///
/// - For `Zsh`, it reads from `~/.zsh_history`
/// - For `Bash`, it reads from `~/.bash_history`
/// - For `Csh`, it reads from `~/.history` (common default)
pub fn read_history(shell: Shell) -> Vec<String> {
    let home = match env::var("HOME") {
        Ok(path) => PathBuf::from(path),
        Err(_) => return vec!["default1".to_string()],
    };

    let history_file = match shell {
        Shell::Zsh => home.join(".zsh_history"),
        Shell::Bash => home.join(".bash_history"),
        Shell::Csh => home.join(".history"),
        Shell::Unknown => return vec!["default2".to_string()],
    };

    let contents = match fs::read(&history_file) {
        Ok(data) => String::from_utf8_lossy(&data).into_owned(),
        Err(_) => return vec!["failed to read history".to_string()],
    };
    let mut commands = Vec::new();
    let mut current_command = String::new();
    let mut in_multiline = false;

    for line in contents.lines() {
        let line = &clean_history_line(line.trim());
        if line.is_empty() {
            continue;
        }
        // Skip corrupted or binary-looking lines
        if looks_corrupted(line) {
            continue;
        }

        if line.starts_with(": ") {
            // Zsh extended history entry
            if !current_command.is_empty() {
                commands.push(unescape_zsh(&current_command));
                current_command.clear();
            }

            if let Some(cmd) = line.split_once(';').map(|x| x.1) {
                let trimmed = cmd.trim_end_matches('\\');
                current_command.push_str(trimmed);
                in_multiline = cmd.trim_end().ends_with('\\');
                if in_multiline {
                    current_command.push('\n');
                }
            }
        } else if in_multiline {
            let trimmed = line.trim_end_matches('\\');
            current_command.push_str(trimmed);
            in_multiline = line.trim_end().ends_with('\\');
            if in_multiline {
                current_command.push('\n');
            }
        } else {
            // Bash or Csh single-line history
            if !looks_corrupted(line) {
                commands.push(line.to_string());
            }
        }
    }

    if !current_command.is_empty() {
        commands.push(unescape_zsh(&current_command));
    }

    commands
}

/// Removes Zsh escaping (e.g., `\\` becomes `\`)
fn unescape_zsh(command: &str) -> String {
    command.replace("\\\\", "\\")
}

/// Detects whether a line looks corrupted or is filled with ANSI/binary garbage
fn looks_corrupted(line: &str) -> bool {
    // ANSI escape codes like \x1b[ or ^[[
    line.contains("\x1b[") || line.contains("^[[")

        // Or: if the line contains too many non-printable/control characters
        || line.chars().filter(|c| c.is_control() && !c.is_ascii_whitespace()).count() > 5

        // Or: if it's very long and suspiciously dense
        || line.len() > 1000
}

fn remove_control_chars(input: &str) -> String {
    input.chars().filter(|c| !c.is_control()).collect()
}

fn strip_ansi_sequences(s: &str) -> String {
    // Matches ANSI escape codes like \x1B[31m or \x1B[0m
    let ansi_regex = Regex::new(r"\x1B\[[0-9;]*[mK]").unwrap();
    ansi_regex.replace_all(s, "").to_string()
}

fn strip_non_printable(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii_graphic() || *c == ' ').collect()
}

fn clean_history_line(input: &str) -> String {
    let cleaned = strip_ansi_sequences(&strip_non_printable(input));
    remove_control_chars(&cleaned)
}

/// Computes the SHA-256 hash of the given string and returns it as a lowercase
/// hexadecimal string.
///
/// This can be used to deduplicate history entries efficiently by comparing
/// their hashes.
///
/// # Arguments
///
/// * `line` - The input string to hash.
///
/// # Returns
///
/// A `String` containing the SHA-256 hash in hexadecimal format.
pub fn hash_string(line: &str) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(line.as_bytes());
    hasher.finalize().to_vec()
}

pub fn prepare_string(text: &str) -> String {
    text.trim().replace("\\s+", "\\s")
}

/// Returns the path to the ~/.config/rushstr directory, creating it if needed.
pub fn get_home_directory() -> anyhow::Result<String> {
    let home = env::var("HOME")?;
    let target: PathBuf = Path::new(&home).join(".config").join("rushstr");
    fs::create_dir_all(&target)?;
    Ok(target.to_string_lossy().into_owned())
}

pub fn create_db() -> anyhow::Result<Db> {
    let home = get_home_directory()?;
    let db_name = RushstrFiles::DbName.val();
    let target = format!("{home}/{db_name}");
    let db: Db = sled::open(target)?;
    Ok(db)
}

const ZSHRC_SNIPPET: &str = r#"
# RUSHSTR configuration - add this to ~/.zshrc
rushstr_no_tiocsti() {
    zle -I
    { RUSHSTR_OUT="$( { </dev/tty rushstr ${BUFFER}; } 2>&1 1>&3 3>&- )"; } 3>&1;
    BUFFER="${RUSHSTR_OUT}"
    CURSOR=${#BUFFER}
    zle redisplay
}
zle -N rushstr_no_tiocsti
bindkey '\C-r' rushstr_no_tiocsti
"#;

/// Appends the RUSHSTR Zsh integration config to ~/.zshrc if not already
/// present
pub fn configure_zsh_profile() -> anyhow::Result<()> {
    let home = env::var("HOME")?;
    let zshrc_path = PathBuf::from(home).join(".zshrc");

    let existing_content = read_to_string(&zshrc_path).unwrap_or_default();

    if !existing_content.contains("rushstr_widget") {
        let mut file = OpenOptions::new().create(true).append(true).open(&zshrc_path)?;
        writeln!(file, "\n{ZSHRC_SNIPPET}")?;
    }

    Ok(())
}

pub fn print_settings() -> anyhow::Result<()> {
    let home = get_home_directory()?;
    let db_name = RushstrFiles::DbName.val();
    let target = format!("{home}/{db_name}");
    println!("settings dir: {target}");
    Ok(())
}

pub fn delete_db() -> anyhow::Result<()> {
    let home = get_home_directory()?;
    let db_name = RushstrFiles::DbName.val();
    let target = format!("{home}/{db_name}");
    fs::remove_dir_all(target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use serial_test::serial;
    use temp_env::with_var;

    use super::*;

    #[test]
    #[serial]
    fn test_detect_zsh() {
        with_var("SHELL", Some("/bin/zsh"), || {
            assert_eq!(detect_shell(), Shell::Zsh);
        });
    }

    #[test]
    #[serial]
    fn test_detect_bash() {
        with_var("SHELL", Some("/usr/bin/bash"), || {
            assert_eq!(detect_shell(), Shell::Bash);
        });
    }

    #[test]
    #[serial]
    fn test_detect_csh() {
        with_var("SHELL", Some("/bin/csh"), || {
            assert_eq!(detect_shell(), Shell::Csh);
        });
    }

    #[test]
    #[serial]
    fn test_detect_unknown() {
        with_var("SHELL", Some("/bin/fish"), || {
            assert_eq!(detect_shell(), Shell::Unknown);
        });
    }

    #[test]
    #[serial]
    fn test_shell_not_set() {
        with_var("SHELL", None::<&str>, || {
            assert_eq!(detect_shell(), Shell::Unknown);
        });
    }
}
