use reqwest::blocking::Client;
use scraper::{Html, Selector};
use serde_json::Value;
use std::collections::HashSet;

use crate::models::news_article::NewsArticle;

pub fn scrape_cnn_news(news_url: &str) -> Option<NewsArticle> {
    let client = Client::new();

    let response = client.get(news_url).send().unwrap().text().unwrap();

    let document = Html::parse_document(&response);

    let selector = Selector::parse(r#"script[type="application/ld+json"]"#).unwrap();

    for element in document.select(&selector) {
        let script_content = element.inner_html();
        let json_value: Value = serde_json::from_str(&script_content).unwrap();

        // println!("{:#}", json_value);
        let headline = json_value.get("headline")?.as_str()?.to_string();
        let article_body = json_value.get("articleBody")?.as_str()?.to_string();
        let description = json_value.get("description")?.as_str()?.to_string();
        let images = json_value.get("image").and_then(|v| v.as_array()).unwrap();
        let genre = json_value.get("articleSection")?.as_array()?;

        let content_urls: HashSet<String> = images
            .iter()
            .filter_map(|image| image.get("contentUrl")?.as_str().map(|url| url.to_string()))
            .collect();

        let genre_sections: HashSet<String> = genre
            .iter()
            .filter_map(|genre| genre.as_str().map(|s| s.to_string()))
            .collect();

        return Some(NewsArticle {
            id: news_url.replace("https://t.co/", ""),
            headline,
            article_body,
            description,
            genre: genre_sections.into_iter().collect(),
            images: content_urls.into_iter().collect(),
        });
    }
    return None;
}
