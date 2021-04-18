// As I user, I would like to be able to:
// - specify the blogs that I want to aggregate
// - spit markdown with useful author-related metadata
// - allow tags that I want to specifically pick up
// - notify me about blogs that it can't read tags for
// - see attribution to an aggregated post as a footer or w/e
// - specify which blogs are enabled/disabled
// - not regen posts that are already aggregated (?)

// Non-goals:
// - designing a static blog platform
// - styling pages, the markdown/markup should be blog generator agnostic

// Implementation goals (v1):
// - fast
// - idempotent
// - parallel (fetching blogs)
// - CLI, not library
// - simple

#![allow(dead_code)]

mod config;
mod feed;
mod entry;
mod fetch;

use config::Config;
use feed::Feed;

fn main() {
    let config = Config::new();
    let setting = config.setting.clone();

    println!("Spitting posts to: {}", setting.out_dir);

    for author in config.authors.iter() {
        let author = author.clone(); // TODO: see if we can avoid cloning here
        let setting = config.setting.clone(); // TODO: see if we can avoid cloning here
        println!("Fetching for {} from {}", author.name, author.feed);
        Feed::new(author, setting).write();
    }
}
