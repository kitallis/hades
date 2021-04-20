use crate::config::{Author, Setting};
use crate::file::write;
use crate::preamble::Preamble;
use chrono::DateTime;
use rss::Item;
use slug::slugify;
use std::path::{Path, PathBuf};
use futures::{FutureExt, StreamExt};
use std::collections::HashSet;
use std::iter::FromIterator;

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

        if entry.skip() { return None; }
        Some(entry)
    }

    pub fn preamble(&self) -> String {
        let preamble = Preamble {
            title: self.entry.title().unwrap().to_string(),
            author: self.default_author(),
            created_at: self.entry.pub_date().unwrap().to_string(),
        };

        match self.setting.preamble_ext.as_str() {
            "yaml" => preamble.yaml(),
            "toml" => preamble.toml(),
            _ => unreachable!()
        }
    }

    pub fn body(&self) -> String {
        format!("{}\n{}", self.preamble(), self.default_content())
    }

    pub fn name(&self) -> PathBuf {
        let time = DateTime::parse_from_rfc2822(self.entry.pub_date().unwrap())
            .unwrap()
            .format("%Y-%m-%d");
        let directory = self.setting.out_dir.to_string();
        let title = slugify(self.entry.title().unwrap());
        let file_name = format!("{}-{}", time, title);

        Path::new(&directory)
            .join(&file_name)
            .with_extension(ENTRY_EXT)
    }

    pub fn skip(&self) -> bool {
        let category_names = self.category_names();
        if category_names.is_empty() {
            return true;
        }

        let allowed_tags: HashSet<_> = self.author.tags.intersection(&category_names).collect();
        if allowed_tags.is_empty() {
            return true;
        }

        false
    }

    pub fn default_author(&self) -> String {
        match self.entry.author() {
            Some(author) => author.to_owned(),
            None => self.author.name.to_owned(),
        }
    }

    pub fn default_content(&self) -> String {
        match (self.entry.content(), self.entry.description()) {
            (Some(content), None) => content.to_owned(),
            (None, Some(description)) => description.to_owned(),
            _ => String::new(),
        }
    }

    fn category_names(&self) -> HashSet<String> {
        HashSet::from_iter(self.entry.categories().iter().map(|category| category.name.to_string()))
    }

    pub fn write(&self) {
        write(&self.name(), &self.body().as_bytes())
    }
}
