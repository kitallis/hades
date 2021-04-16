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
// -

// Implementation goals (v1):
// - fast
// - idempotent
// - parallel (fetching blogs)
// - CLI, not library
// - simple

#![allow(unused_imports)]
#![allow(dead_code)]

use std::error::Error;
use rss::{Channel, Item};
use chrono::prelude::*;
use futures::future::Future;
use futures::executor::block_on;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Clone)]
struct Entry {
    entry: Item,
    author: Author,
}

#[derive(Debug)]
struct Feed {
    channel: Channel
}

impl Feed {
    fn parse(&self, author: Author) -> Vec<Entry> {
        let mut entries = Vec::new();

        // TODO: use a reduce
        for entry in self.channel.items() {
            let entry_metadata = Entry {
                entry: entry.clone(),
                author: author.clone(),
            };

            match entry_metadata.validate() {
                Some(_) => {
                    entries.push(entry_metadata)
                }

                None => println!("Skipping this entry...")
            }
        }

        entries
    }
}

impl Entry {
    // ---
    // title: "Devcamp Bangalore, Barcamp Pune!"
    // kind: article
    // created_at: 2008-02-06 16:58:00 UTC
    // author: Steven Deobald
    // layout: post
    // ---
    fn preamble(&self) -> String {
        format!("---\ntitle: {}\n---", self.entry.title().unwrap().to_owned())
    }

    fn body(&self) -> String {
        format!("{}\n{}", self.preamble(), self.default_content())
    }

    fn name(&self) -> String {
        format!("out/{}-{}.md", Utc::now(), self.entry.title().unwrap().to_owned())
    }

    fn validate(&self) -> Option<bool> {
        Some(true)
    }

    fn default_author(&self) -> String {
        match self.entry.author() {
            Some(author) => author.to_owned(),
            None => self.author.name.to_owned()
        }
    }

    fn default_content(&self) -> String {
        match (self.entry.content(), self.entry.description()) {
            (Some(content), None) => content.to_owned(),
            (None, Some(description)) => description.to_owned(),
            _ => String::new()
        }
    }

    fn write(&self) {
        let file_name = &self.name();
        let path = Path::new(file_name);
        let display = path.display();

        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't create {}: {}", display, why),
        };

        match file.write_all(self.body().as_bytes()) {
            Ok(_) => println!("Successfully wrote to: {}", display),
            Err(why) => panic!("Couldn't write to {}: {}", display, why),
        }
    }
}

#[derive(Deserialize, Clone, Debug)]
struct Author {
    name: String,
    feed: String,
}

#[derive(Deserialize)]
struct Config {
    setting: Setting,
    authors: Vec<Author>,
}

#[derive(Deserialize)]
struct Setting {
    out_dir: String,
    front_matter: String,
}

// take a blog
// read an entry
// spit markdown post
fn main() {
    let toml_content = r#"
          [setting]
          out_dir = "articles"
          front_matter = "yaml"

          [[authors]]
          name = "Steven Deobald"
          feed = "https://www.deobald.ca/index.xml"

          [[authors]]
          name = "Nivedita Priyadarshini"
          feed = "https://medium.com/@nid90/feed"
          "#;

    let package_info: Config = toml::from_str(toml_content).unwrap();
    println!("Spitting posts to: {}", package_info.setting.out_dir);

    for author in package_info.authors.iter() {
        let author = author.clone();
        println!("Fetching for {} from {}", author.name, author.feed);

        match fetch_feed(&author.feed) {
            Ok(channel) => {
                let feed = Feed { channel };
                let entries = feed.parse(author);
                for entry in entries {
                    println!("Writing entry...");
                    entry.write()
                }
            }

            Err(_) => println!("Failed to fetch for author: {}", author.name),
        }
    }
}

#[tokio::main]
async fn fetch_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
