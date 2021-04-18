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
mod entry;
mod feed;
mod fetch;
mod file;

use crate::config::{Author, Setting};
use config::Config;
use feed::Feed;

fn main() {
    let config = Config::new();
    println!("Spitting posts to: {}", &config.setting.out_dir);
    config
        .authors
        .iter()
        .for_each(|author| generate(author.clone(), config.setting.clone()))
}

fn generate(author: Author, setting: Setting) {
    println!("Fetching for {} from {}", author.name, author.feed);
    match Feed::new(author, setting) {
        Some(feed) => feed.write(),
        None => println!("Could not fetch feed."),
    };
}
