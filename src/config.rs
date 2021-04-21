use serde::Deserialize;
use std::collections::HashSet;
use std::fs;
use structopt::StructOpt;
use url::Url;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    config_file: std::path::PathBuf,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub setting: Setting,
    pub authors: Vec<Author>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Setting {
    pub out_dir: String,
    #[serde(default = "default_preamble_ext")]
    pub preamble_ext: String,
    #[serde(default = "default_tags")]
    pub tags: HashSet<String>,
    #[serde(default = "default_workers")]
    pub workers: i8,
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

fn default_skip_state() -> bool {
    false
}

fn default_tags() -> HashSet<String> {
    HashSet::new()
}

fn default_preamble_ext() -> String {
    "yaml".to_string()
}

fn default_workers() -> i8 {
    10
}

impl Config {
    pub fn new() -> Self {
        let args = Cli::from_args();
        let config_contents =
            fs::read_to_string(args.config_file).expect("Couldn't read the config!");

        let mut config: Config = toml::from_str(&config_contents).unwrap();
        config.pre_populate_author_tags();
        config.dedup_authors();
        config
    }

    // Copy over tags from the base setting only if author tags are unspecified
    fn pre_populate_author_tags(&mut self) {
        if self.setting.tags.is_empty() {
            return;
        }

        for author in self.authors.iter_mut() {
            let default_tags = self.setting.tags.clone();

            if author.tags.is_empty() {
                author.set_tags(default_tags)
            }
        }
    }

    // It was painful to manage a HashSet<Author> under the Config struct
    // While trying to update tags under Author, we need iter_mut() in pre_populate_author_tags()
    // This makes using HashSet<Author> not possible, since HashSet does not implement iter_mut()
    // One way to solve this is to have different structs for serialization and run-time
    // For simplicity, I've instead chosen a Vec<Author> and de-duped it manually, upfront
    fn dedup_authors(&mut self) {
        self.authors.dedup()
    }
}
