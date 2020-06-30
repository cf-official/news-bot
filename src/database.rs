use mysql_async::*;
use mysql_async::prelude::Queryable;

use crate::logger::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinimalGuild {
    id: u64,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Feed {
    pub name: String,
    pub category: String,
    pub feed_url: String,
    pub profile_icon: String,
    pub latest_article: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinimalArticle {
    pub id: u64,
    pub timestamp: String,
    pub headline: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Subscription {
    pub guild: MinimalGuild,
    pub feed: Feed,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MinimalSubscription {
    pub source_id: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Guild {
    pub id: u64,
    pub is_premium: bool,
    pub last_delivery: u32,
    pub subscriptions: Vec<MinimalSubscription>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PartialGuild {
    pub id: u64,
    pub is_premium: bool,
    pub last_delivery: u32,
}

pub async fn create_con(pool: &Pool) -> Conn {
    return pool.get_conn().await.unwrap();
}

pub async fn get_guild(pool: &Pool, id: u64) -> Guild {
    let mut conn = create_con(&pool).await;
    let raw_guild = conn.prep_exec(format!("SELECT * FROM guilds where id = \"{}\"", id), ()).await.unwrap();
    //conn = create_con(pool).await;
    let (_ /* conn */, partial_guild) = raw_guild.map_and_drop(|row: Row| {
        let (id, is_prem, last_delivery): (u64, u8, u32) = mysql_async::from_row(row);
        PartialGuild {
            id,
            is_premium: is_prem == 1,
            last_delivery,
        }
    }).await.unwrap();
    log(LogLevel::DEBUG, format!("{:?}", partial_guild[0]));

    let subscriptions = get_subscriptions(&pool, id).await;

    return Guild {
        id: partial_guild[0].id,
        is_premium: partial_guild[0].is_premium,
        last_delivery: partial_guild[0].last_delivery,
        subscriptions,
    };
}

pub async fn get_subscriptions(pool: &Pool, id: u64) -> Vec<MinimalSubscription> {
    let conn = create_con(&pool).await;
    let raw_subscriptions = conn.prep_exec(format!("SELECT source_id FROM subscriptions where guild_id = \"{}\"", id), ()).await.unwrap();

    let (_ /* conn */, minimal_subscriptions) = raw_subscriptions.map_and_drop(|row: Row| {
        let source_id = mysql_async::from_row(row);
        MinimalSubscription {
            source_id,
        }
    }).await.unwrap();
    let subscriptions = Vec::<Subscription>::new();
    /*for sub in minimal_subscriptions {
        let source = get_sources(&pool, sub.source_id).await;
        println!("{:?}", source[0]);
        //subscriptions.push()
    }*/
    return minimal_subscriptions;
}
//TODO: Fix E0308 on line 102
/*
pub async fn get_sources(pool: &Pool, source_id: u32) -> Vec<Feed> {
    let temp_conn = create_con(&pool).await;
    let raw_sources = temp_conn.prep_exec(format!("SELECT name, category, feed_url, profile_icon, latest_article FROM feeds WHERE id = \"{}\"", source_id), ()).await.unwrap();
    let (_ /* conn */, minimal_subscriptions) = raw_sources.map_and_drop(|row: Row| {
        let (name, category, feed_url, profile_icon, latest_article): (String, String, String, String, u32) = mysql_async::from_row(row);
        Feed {
            name,
            category,
            feed_url,
            profile_icon,
            latest_article,
        }
    });
    return Vec::<Feed>::new();
}*/