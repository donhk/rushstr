pub enum Config {
    DbName,
}

impl Config {
    pub fn val(self) -> &'static str {
        match self {
            Config::DbName => "rushstr.db",
        }
    }
}
