use crate::config::Config;
use crate::feed::Feed;
use futures::stream::StreamExt;
use rss::Channel;
use std::error::Error;
use url::Url;

#[tokio::main(worker_threads = 30)]
pub async fn generate_feeds(config: Config) {
    futures::stream::iter(
        config
            .authors
            .clone()
            .into_iter()
            .map(|author| tokio::spawn(Feed::new(author.clone(), config.setting.clone()))),
    )
    .buffer_unordered(config.setting.workers as usize)
    .map(|feed| match feed {
        Ok(Some(feed)) => feed.write(),
        Ok(None) => println!("Failed to fetch feed."),
        Err(_) => println!("Unknown error."),
    })
    .collect::<Vec<_>>()
    .await;
}

pub async fn fetch_feed(url: &Url) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url.as_str()).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}
