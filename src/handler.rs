use chrono::{DateTime, FixedOffset, TimeZone};
use reqwest;
use serenity::{
    async_trait,
    builder::Timestamp,
    model::{
        channel::Message,
        gateway::Ready,
    },
    prelude::*,
    utils::Color,
};

use crate::logger::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut content = msg.content;
        if has_prefix(&content) && !msg.author.bot {
            let (command, content) = remove_prefix(&content);
            let args = process_args(&content.to_lowercase());
            if command == "ping" {
                // Sending a message can fail, due to a network error, an
                // authentication error, or lack of permissions to post in the
                // channel, so log to stdout when some error happens, with a
                // description of it.
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    log(LogLevel::ERROR, format!("Error sending message: {:?}", why));
                }
            } else if command == "latest" {
                let channel = crate::fetch_channel("https://feeds.bbci.co.uk/news/video_and_audio/world/rss.xml").await.unwrap();
                let articles = crate::process_channel(channel, crate::Publisher {
                    name: "Unknown",
                    profile_link: "https://www.wolflair.com/wp-content/uploads/2017/01/placeholder.jpg",
                }).await.unwrap();

                let article = &articles[0];
                let timestamp_string = DateTime::parse_from_rfc2822(&article.date_published).unwrap().to_string();
                let timestamp_split = timestamp_string.split_whitespace().collect::<Vec<&str>>();
                let timestamp = format!("{}T{}", timestamp_split[0], timestamp_split[1]);

                if args.contains(&"focused".to_string()) || args.contains(&"f".to_string()) {
                    let text = reqwest::get(&article.link).await.unwrap().text().await.unwrap();
                    let image_source = text
                        .split("<meta name=\"twitter:image:src\" content=\"")
                        .collect::<Vec<&str>>()[1]
                        .split("\">")
                        .collect::<Vec<&str>>()[0];

                    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title(&article.headline)
                                .description(&article.preview_text)
                                .color(Color::from_rgb(114, 137, 218))
                                .url(&article.link)
                                .image(&image_source)
                                .footer(|f| {
                                    f.text(format!("Published by {}", &article.publisher.name))
                                        .icon_url(&article.publisher.profile_link);
                                    return f;
                                })
                                .timestamp(timestamp);

                            return e;
                        });

                        return m;
                    }).await {
                        log(LogLevel::ERROR, format!("Error sending message: {:?}", why));
                    }
                } else {
                    if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                        m.embed(|e| {
                            e.title(&article.headline)
                                .description(&article.preview_text)
                                .color(Color::from_rgb(114, 137, 218))
                                .url(&article.link)
                                .footer(|f| {
                                    f.text(format!("Published by {}", &article.publisher.name))
                                        .icon_url(&article.publisher.profile_link);
                                    return f;
                                })
                                .timestamp(timestamp);

                            return e;
                        });

                        return m;
                    }).await {
                        log(LogLevel::ERROR, format!("Error sending message: {:?}", why));
                    }
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        log(LogLevel::OK, format!("{} is connected!", ready.user.name));
    }
}

fn has_prefix(content: &String) -> bool {
    return if content.contains("nb!") {
        true
    } else if content.contains("<@!684330424188534794>") {
        true
    } else {
        false
    };
}

fn remove_prefix(content: &String) -> (String, String) {
    let old_content = content as &str;
    let mut new_content: String = "".to_string();
    if old_content.contains("nb!") {
        new_content = old_content.split("nb!").collect::<Vec<_>>().join("");
    } else if old_content.contains("<@!684330424188534794>") {
        new_content = old_content.split("<@!684330424188534794> ").collect::<Vec<_>>().join("");
    };
    return (new_content.split_whitespace().collect::<Vec<_>>()[0].to_string(), new_content.split_whitespace().collect::<Vec<_>>().join(" "));
}

fn process_args(content: &String) -> Vec<String> {
    let mut args = Vec::<String>::new();
    let splits = content.split_whitespace().collect::<Vec<&str>>();
    let mut counter = 0;
    for x in splits {
        if counter > 0 {
            args.push(String::from(x));
        }
        counter = counter + 1;
    };
    return args;
}