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

use std::env;
use std::fs;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use rss::{Channel, Item};
use chrono::prelude::*;
use futures::future::Future;
use futures::executor::block_on;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use slug::slugify;

const ENTRY_EXT: &str = "md";

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

            if entry_metadata.validate() {
                entries.push(entry_metadata)
            } else {
                println!("Skipping this entry...")
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
        // TODO: can we use a multiline string or an actual template
        format!("---\ntitle: {}\nauthor: {}\ncreated_at: {}\n---", self.entry.title().unwrap(), self.default_author(), self.entry.pub_date().unwrap())
    }

    fn body(&self) -> String {
        format!("{}\n{}", self.preamble(), self.default_content())
    }

    fn name(&self) -> PathBuf {
        let time = DateTime::parse_from_rfc2822(self.entry.pub_date().unwrap()).unwrap().format("%Y-%m-%d");
        let directory = "out";
        let title = slugify(self.entry.title().unwrap());
        let file_name = format!("{}-{}", time, title);

        Path::new(directory)
            .join(&file_name)
            .with_extension(ENTRY_EXT)
    }

    fn validate(&self) -> bool {
        true
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
        let file_path = &self.name();
        let display = file_path.display();

        let mut file = match File::create(&file_path) {
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
    front_matter_ext: String,
}

impl Default for Setting {
    fn default() -> Setting {
        Setting { out_dir: "articles".to_owned(), front_matter_ext: "toml".to_owned() }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    config_file: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let config_contents = fs::read_to_string(args.config_file).expect("Couldn't read the config!");
    let package_info: Config = toml::from_str(&config_contents).unwrap();

    println!("Spitting posts to: {}", package_info.setting.out_dir);
    println!("{}", package_info.authors.len());

    for author in package_info.authors.iter() {
        let author = author.clone(); // TODO: see if we can avoid cloning here
        println!("Fetching for {} from {}", author.name, author.feed);

        match fetch_feed(&author.feed) {
            Ok(channel) => {
                let feed = Feed { channel };
                for entry in feed.parse(author) {
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
