#[derive(Debug, Clone)]
pub enum SearchType {
    MonkeyTyping,
    Exact,
    Regex,
}

impl SearchType {
    pub fn to_str(&self) -> &'static str {
        match self {
            SearchType::MonkeyTyping => "monkey_typing",
            SearchType::Exact => "exact",
            SearchType::Regex => "regex",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            SearchType::MonkeyTyping => SearchType::Exact,
            SearchType::Exact => SearchType::Regex,
            SearchType::Regex => SearchType::MonkeyTyping,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// pattern to search in history
    pub input: String,
    /// type of pattern matching
    pub search_type: SearchType,
    pub favorites: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        SearchOptions {
            input: "".to_string(),
            search_type: SearchType::MonkeyTyping,
            favorites: false,
        }
    }
}

impl SearchOptions {
    pub fn is_case_insensitive(&self) -> bool {
        match self.search_type {
            SearchType::MonkeyTyping => true,
            SearchType::Exact => false,
            SearchType::Regex => true,
        }
    }
}
