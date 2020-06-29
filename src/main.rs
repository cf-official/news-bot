use std::error::Error;

use reqwest;
use rss::Channel;
use serenity::client::Client;

use crate::logger::{log, LogLevel};

mod logger;
mod handler;
mod news_types;
mod structures;
mod config;

#[tokio::main]
async fn main() {
    log(LogLevel::OK, "News is starting");

    let channel = fetch_channel().await.unwrap();
    let articles = process_channel(channel).await.unwrap();
    log(LogLevel::OK, format!("Buffered {} articles into memory at startup", articles.len()));
    log(LogLevel::OK, "Building Client");
    let mut client = Client::new(config::TOKEN)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}


pub async fn fetch_channel() -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get("https://feeds.bbci.co.uk/news/video_and_audio/world/rss.xml")
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn process_channel(channel: Channel) -> Result<Vec<structures::article::Article>, Box<dyn Error>> {
    let mut articles = Vec::<structures::article::Article>::new();
    let items = channel.into_items();

    for item in items {
        let headline = item.title().unwrap_or_default();
        let description = item.description().unwrap_or_default();
        let content = item.content().unwrap_or_default();
        let author = item.author().unwrap_or_default();
        let url = item.link().unwrap_or_default();
        let pub_date = item.pub_date().unwrap_or_default();
        articles.push(structures::article::Article::new(headline, description, content, author, url, pub_date));
    }
    Ok(articles)
}