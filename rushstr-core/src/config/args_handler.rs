use crate::{ConfigOptions, configure_zsh_profile, delete_db, print_settings};

pub struct ArgsHandler {
    options: ConfigOptions,
}

impl ArgsHandler {
    pub fn new(options: ConfigOptions) -> Self {
        Self { options }
    }

    /// returns true if we should exit the execution
    pub fn execute(&self) -> anyhow::Result<bool> {
        if self.options.reset_settings {
            delete_db()?;
        }
        if self.options.show_settings {
            print_settings()?;
        }
        if self.options.zsh_shell_conf {
            configure_zsh_profile()?;
        }
        Ok(true)
    }
}
