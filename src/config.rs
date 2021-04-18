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
    pub front_matter_ext: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Author {
    pub name: String,
    pub feed: String,
}

const DEFAULT_OUT_DIR: &str = "articles";
const DEFAULT_FRONT_MATTER_EXT: &str = "yaml";

impl Default for Setting {
    fn default() -> Setting {
        Setting {
            out_dir: DEFAULT_OUT_DIR.to_string(),
            front_matter_ext: DEFAULT_FRONT_MATTER_EXT.to_string(),
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let args = Cli::from_args();
        let config_contents = fs::read_to_string(args.config_file).expect("Couldn't read the config!");

        toml::from_str(&config_contents).unwrap()
    }
}

