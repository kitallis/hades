mod config;
mod entry;
mod feed;
mod fetch;
mod file;
mod front_matter;

use config::Config;

fn main() {
    let config = Config::new();
    fetch::generate_feeds(config.clone());
}
