use dash_mpd::{fetch::DashDownloader, DashMpdError};
use reqwest::{header::HeaderMap, Client};
use std::path::{Path, PathBuf};

use crate::{
    models::{
        reddit_auth::RedditAuth,
        reddit_post::{RedditPost, RedditPostResponse},
    },
    CONFIG,
};

const REDDIT_AUTH_URL: &str = "https://www.reddit.com/api/v1/access_token";

pub async fn download_reddit_video(post: &RedditPost) -> Result<PathBuf, DashMpdError> {
    let video_path = post.data.get_video_path();
    let url = post.data.media.reddit_video.dash_url.clone();
    let result = DashDownloader::new(&url)
        .best_quality()
        .download_to(Path::new(&video_path))
        .await;
    println!("Result {:?}", result);
    result
}

async fn reddit_auth() -> String {
    let client = reqwest::Client::builder().build().unwrap();

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        "application/x-www-form-urlencoded".parse().unwrap(),
    );
    headers.insert(
        "User-Agent",
        "Doge-Devotee-Reddit-Scraper-Script".parse().unwrap(),
    );

    let mut params = std::collections::HashMap::new();
    params.insert("grant_type", "password");
    params.insert("username", &CONFIG.reddit_username);
    params.insert("password", &CONFIG.reddit_password);

    let request = client
        .post(REDDIT_AUTH_URL)
        .basic_auth(&CONFIG.reddit_app_id, Some(&CONFIG.reddit_app_secret))
        .headers(headers)
        .form(&params);

    let response = request.send().await.unwrap();
    let body = response.text().await.unwrap();

    let auth_response: RedditAuth = serde_json::from_str(&body).unwrap();
    auth_response.access_token
}

fn get_top_posts_url(subreddit: &str, timeframe: &str, limit: usize) -> String {
    format!(
        "https://oauth.reddit.com/r/{}/top.json?t={}?limit={}",
        subreddit, timeframe, limit,
    )
    .to_string()
}

pub async fn get_top_posts(subreddit: &str, timeframe: &str) -> Vec<RedditPost> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert(
        "User-Agent",
        "Doge-Devotee-Reddit-Scraper-Script".parse().unwrap(),
    );
    headers.insert("Authorization", reddit_auth().await.parse().unwrap());

    const POST_LIMIT: usize = 100;
    let response = client
        .get(get_top_posts_url(subreddit, timeframe, POST_LIMIT))
        .headers(headers)
        .send()
        .await
        .unwrap();
    let response_str = response.text().await.unwrap();
    let posts_response: RedditPostResponse = serde_json::from_str(&response_str).unwrap();

    posts_response.data.children
}
