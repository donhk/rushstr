mod config;
mod crawler;
mod stores;
mod types;
mod utils;

#[doc(inline)]
pub use config::args_handler::ArgsHandler;
#[doc(inline)]
pub use config::configuration::{ConfigOptions, RushstrFiles};
#[doc(inline)]
pub use config::search_options::SearchOptions;
#[doc(inline)]
pub use config::search_options::SearchType;
#[doc(inline)]
pub use crawler::history_crawler::HistoryCrawler;
#[doc(inline)]
pub use crawler::mem_crawler::MemCrawler;
#[doc(inline)]
pub use crawler::scanner::HScanner;
#[doc(inline)]
pub use stores::store_trait::StoreTrait;
#[doc(inline)]
pub use stores::vector_store::VectorStore;
#[doc(inline)]
pub use types::common_types::{HIndex, HLines, Key, MHItem, Scanner, Shell, Store};
#[doc(inline)]
pub use types::history_item::HItem;
#[doc(inline)]
pub use utils::utilities::{
    configure_zsh_profile, create_db, delete_db, detect_shell, get_home_directory, hash_string, prepare_string,
    print_settings, read_history,
};
