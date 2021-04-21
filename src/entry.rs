use crate::config::{Author, Setting};
use crate::file::write;
use crate::front_matter::FrontMatter;
use chrono::{DateTime, FixedOffset};
use nanoid::nanoid;
use rss::Item;
use slug::slugify;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

const ENTRY_EXT: &str = "md";

#[derive(Debug, Clone)]
pub struct Entry {
    pub entry: Item,
    pub author: Author,
    pub setting: Setting,
}

impl Entry {
    pub fn new(entry: Item, author: Author, setting: Setting) -> Option<Self> {
        let entry = Self {
            entry,
            author,
            setting,
        };

        if entry.validate() {
            return Some(entry);
        }

        None
    }

    pub fn write(&self) {
        write(&self.name(), &self.body().as_bytes())
    }

    fn preamble(&self) -> String {
        let preamble = FrontMatter {
            title: self.entry.title().unwrap_or("").to_string(),
            author: self.default_author(),
            date: self.default_pub_date().unwrap(),
            link: self.entry.link().unwrap().to_string(),
        };

        match self.setting.preamble_ext.as_str() {
            "yaml" => preamble.yaml(),
            "zola-toml" => preamble.zola_toml(),
            _ => unreachable!(),
        }
    }

    fn body(&self) -> String {
        format!("{}\n{}", self.preamble(), self.default_content())
    }

    fn name(&self) -> PathBuf {
        let directory = self.setting.out_dir.to_string();
        let title = slugify(self.entry.title().unwrap_or(""));
        let pub_date = self
            .default_pub_date()
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
        let file_name = format!("{}-{}-{}", pub_date, title, nanoid!());

        Path::new(&directory)
            .join(&file_name)
            .with_extension(ENTRY_EXT)
    }

    fn default_author(&self) -> String {
        match self.entry.author() {
            Some(author) => author.to_owned(),
            None => self.author.name.to_owned(),
        }
    }

    fn default_content(&self) -> String {
        match (self.entry.content(), self.entry.description()) {
            (Some(content), None) => content.to_owned(),
            (None, Some(description)) => description.to_owned(),
            _ => String::new(),
        }
    }

    fn default_pub_date(&self) -> Option<DateTime<FixedOffset>> {
        let pub_date = self.entry.pub_date()?;

        match DateTime::parse_from_rfc2822(pub_date) {
            Ok(time) => Some(time),
            Err(_e) => match DateTime::parse_from_rfc3339(pub_date) {
                Ok(time) => Some(time),
                Err(_e) => None,
            },
        }
    }

    fn validate(&self) -> bool {
        self.are_tags_valid() && self.is_pub_date_present()
    }

    fn are_tags_valid(&self) -> bool {
        // if there are no tags in config, assume the entry is valid
        if self.author.tags.is_empty() {
            return true;
        }

        // if tags in config are specified, but there are no tags in the entry, don't fetch
        let category_names = self.category_names();
        if category_names.is_empty() {
            return false;
        }

        // if there are tags in config and in the entry, only fetch if they intersect
        let allowed_tags: HashSet<_> = self.author.tags.intersection(&category_names).collect();
        if allowed_tags.is_empty() {
            return false;
        }

        true
    }

    fn is_pub_date_present(&self) -> bool {
        self.default_pub_date().is_some()
    }

    fn category_names(&self) -> HashSet<String> {
        HashSet::from_iter(
            self.entry
                .categories()
                .iter()
                .map(|category| category.name.to_string()),
        )
    }
}
