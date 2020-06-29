use crate::news_types::NewsType;

pub struct Article {
    pub headline: String,
    pub author: String,
    pub article_type: NewsType,
    pub link: String,
    pub preview_text: String,
    pub content: String,
    pub date_published: String,
}

impl Article {
    pub fn new(headline: &str, description: &str, content: &str, author: &str, url: &str, date_published: &str) -> Article {
        Article {
            headline: String::from(headline),
            author: String::from(author),
            article_type: NewsType::WORLD,
            link: String::from(url),
            preview_text: String::from(description),
            content: String::from(content),
            date_published: String::from(date_published),
        }
    }
}