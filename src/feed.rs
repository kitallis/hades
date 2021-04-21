use crate::config::{Author, Setting};
use crate::entry::Entry;
use crate::fetch::fetch_feed;
use rss::Channel;

#[derive(Debug)]
pub struct Feed {
    author: Author,
    setting: Setting,
    channel: Channel,
}

impl Feed {
    pub async fn new(author: Author, setting: Setting) -> Option<Self> {
        if author.skip {
            return None;
        }

        match fetch_feed(&author.feed).await {
            Ok(channel) => Some(Self {
                channel,
                author,
                setting,
            }),
            Err(_) => None,
        }
    }

    pub fn write(&self) {
        self.parse().iter().for_each(|entry| entry.write())
    }

    fn parse(&self) -> Vec<Entry> {
        self.channel
            .items()
            .iter()
            .fold(Vec::new(), |mut entries, item| {
                match Entry::new(item.clone(), self.author.clone(), self.setting.clone()) {
                    Some(entry) => {
                        entries.push(entry);
                    }
                    None => println!("Parse error, skipping..."),
                }

                entries
            })
    }
}
