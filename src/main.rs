use std::error::Error;
use std::panic::*;
use std::panic;

use mysql_async::*;
use reqwest;
use rss::Channel;
use serenity::client::Client;

use crate::logger::{log, LogLevel};
use crate::news_publishers::Publisher;

mod news_publishers;
mod logger;
mod handler;
mod news_types;
mod structures;
mod config;
mod database;

#[tokio::main]
async fn main() {
    log(LogLevel::WARNING, "Testing Database module");
    let pool = Pool::new(crate::config::DB_URI);
    let guild = database::get_guild(&pool, 275377268728135680).await;
    log(LogLevel::OK, "News is starting");

    let channel = crate::fetch_channel("https://feeds.bbci.co.uk/news/video_and_audio/world/rss.xml").await.unwrap();
    let articles = crate::process_channel(channel, Publisher {
        name: "Unknown",
        profile_link: "https://www.wolflair.com/wp-content/uploads/2017/01/placeholder.jpg",
    }).await.unwrap();

    log(LogLevel::OK, format!("Buffered {} articles into memory at startup", articles.len()));
    log(LogLevel::OK, "Building Client");
    let mut client = Client::new(config::TOKEN)
        .event_handler(handler::Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        log(LogLevel::FATAL, format!("Client error: {:?}", why));
    }
}


pub async fn fetch_channel(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url)
        .await?
        .bytes()
        .await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

pub async fn process_channel(channel: Channel, source: news_publishers::Publisher) -> Result<Vec<structures::article::Article>, Box<dyn Error>> {
    let mut articles = Vec::<structures::article::Article>::new();
    let items = channel.into_items();

    for item in items {
        let headline = item.title().unwrap_or_default();
        let description = item.description().unwrap_or_default();
        let content = item.content().unwrap_or_default();
        let author = item.author().unwrap_or_default();
        let url = item.link().unwrap_or_default();
        let pub_date = item.pub_date().unwrap_or_default();
        articles.push(structures::article::Article::new(headline, description, content, author, url, pub_date, source));
    }
    Ok(articles)
}

fn panic_handler(info: &PanicInfo) -> ! {
    log(LogLevel::FATAL, format!("{}", info));
    std::process::exit(1);
}