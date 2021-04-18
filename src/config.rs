use std::fs;
use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
pub struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    config_file: std::path::PathBuf,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub setting: Setting,
    pub authors: Vec<Author>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Setting {
    pub out_dir: String,
    #[serde(default = "default_front_matter_ext")]
    pub front_matter_ext: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Author {
    pub name: String,
    pub feed: String,
    #[serde(default = "default_enabled_state")]
    pub enabled: bool,
}

fn default_enabled_state() -> bool {
    true
}

fn default_front_matter_ext() -> String {
    "yaml".to_string()
}

impl Config {
    pub fn new() -> Self {
        let args = Cli::from_args();
        let config_contents = fs::read_to_string(args.config_file).expect("Couldn't read the config!");

        toml::from_str(&config_contents).unwrap()
    }
}

