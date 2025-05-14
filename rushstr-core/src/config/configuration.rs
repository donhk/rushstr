use clap::Parser;

/// Represents the different types of files used internally by rushstr.
pub enum RushstrFiles {
    /// The filename for the rushstr command usage database.
    DbName,
    /// The filename for the rushstr settings/configuration file.
    Settings,
}

impl RushstrFiles {
    /// Returns the string name of the file associated with the enum variant.
    pub fn val(self) -> &'static str {
        match self {
            RushstrFiles::DbName => "rushstr.db",
            RushstrFiles::Settings => "settings.hex",
        }
    }

    /// Returns the byte slice of the file name associated with the enum
    /// variant.
    ///
    /// Useful for writing the name directly to byte-based APIs (e.g., Sled,
    /// file system, etc).
    pub fn bytes(self) -> &'static [u8] {
        self.val().as_bytes()
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "rushstr",
    version = env!("CARGO_PKG_VERSION"),
    about = "🚀 rushstr is a blazing-fast⚡, Rust-powered interactive shell history searcher made with ❤️  by https://donhk.dev"
)]
pub struct ConfigOptions {
    /// Reset all saved settings and usage history
    #[arg(long)]
    pub reset_settings: bool,

    /// Show current configuration
    #[arg(long)]
    pub show_settings: bool,

    /// update the zsh config for better integration with rushstr
    #[arg(long)]
    pub zsh_shell_conf: bool,
}
