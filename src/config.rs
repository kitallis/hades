use serde::Deserialize;
use std::fs;
use structopt::StructOpt;
use std::collections::HashSet;
use std::cell::Cell;
use url::{Url, ParseError};
use std::hash::{Hash, Hasher};

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    config_file: std::path::PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub setting: Setting,
    pub authors: HashSet<Author>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Setting {
    pub out_dir: String,
    #[serde(default = "default_front_matter_ext")]
    pub front_matter_ext: String,
    #[serde(default = "default_tags")]
    pub tags: HashSet<String>,
}

#[derive(Deserialize, Clone, Debug, Eq)]
pub struct Author {
    pub name: String,
    pub feed: Url,
    #[serde(default = "default_skip_state")]
    pub skip: bool,
    #[serde(default = "default_tags")]
    pub tags: HashSet<String>,
}

impl Author {
    pub fn set_tags(&mut self, tags: HashSet<String>) {
        self.tags = tags
    }
}

impl PartialEq for Author {
    fn eq(&self, other: &Self) -> bool {
        self.feed == other.feed
    }
}

impl Hash for Author {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        self.feed.hash(hasher);
    }
}

fn default_skip_state() -> bool {
    false
}

fn default_tags() -> HashSet<String> {
    HashSet::new()
}

fn default_front_matter_ext() -> String {
    "yaml".to_string()
}

impl Config {
    pub fn new() -> Self {
        let args = Cli::from_args();
        let config_contents =
            fs::read_to_string(args.config_file).expect("Couldn't read the config!");

        let mut config: Config = toml::from_str(&config_contents).unwrap();
        config.pre_populate_author_tags();
        config
    }

    // Copy over tags from the base setting only if author tags are unspecified
    fn pre_populate_author_tags(&mut self) {
        let default_tags = self.setting.tags.clone();
        if default_tags.is_empty() { return; }

        for author in self.authors.iter_mut() {
            if author.tags.is_empty() {
                author.set_tags(default_tags)
            }
        }
    }
}
