use serenity::{
    async_trait,
    model::{
        channel::{
            Embed,
            Message,
        },
        gateway::Ready,
    },
    prelude::*,
};
use serenity::utils::Color;

use crate::logger::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut content = msg.content;
        if has_prefix(&content) && !msg.author.bot {
            content = remove_prefix(&content);
            if content == "ping" {
                // Sending a message can fail, due to a network error, an
                // authentication error, or lack of permissions to post in the
                // channel, so log to stdout when some error happens, with a
                // description of it.
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    log(LogLevel::ERROR, format!("Error sending message: {:?}", why));
                }
            } else if content == "latest" {
                let channel = crate::fetch_channel().await.unwrap();
                let articles = crate::process_channel(channel).await.unwrap();
                let article = &articles[0];


                if let Err(why) = msg.channel_id.send_message(&ctx.http, |m| {
                    m.embed(|e| {
                        e.title(&article.headline)
                            .description(format!("{}\n\nAuthor: {}\nPublished: {}\n[Link]({})", &article.preview_text, &article.author, &article.date_published, &article.link))
                            .color(Color::from_rgb(114, 137, 218))
                            .url(&article.link);

                        return e;
                    });

                    return m;
                }).await {
                    log(LogLevel::ERROR, format!("Error sending message: {:?}", why));
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

fn remove_prefix(content: &String) -> String {
    let old_content = content as &str;
    let mut new_content: String = "".to_string();
    if old_content.contains("nb!") {
        new_content = old_content.split("nb!").collect::<Vec<_>>().join("");
    } else if old_content.contains("<@!684330424188534794>") {
        new_content = old_content.split("<@!684330424188534794> ").collect::<Vec<_>>().join("");
    };
    return new_content;
}