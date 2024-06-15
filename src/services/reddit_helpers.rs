use dash_mpd::{fetch::DashDownloader, DashMpdError};
use reqwest::{header::HeaderMap, Client};
use std::path::{Path, PathBuf};

use crate::models::reddit_post::{RedditPost, RedditPostResponse};

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

fn get_top_posts_url(subreddit: &str, timeframe: &str) -> String {
    format!(
        "https://www.reddit.com/r/{}/top.json?t={}",
        subreddit, timeframe
    )
    .to_string()
}

pub async fn get_top_posts(subreddit: &str, timeframe: &str) -> Vec<RedditPost> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("User-Agent", "PostmanRuntime/7.39.0".parse().unwrap());

    let response = client
        .get(get_top_posts_url(subreddit, timeframe))
        .headers(headers)
        .send()
        .await
        .unwrap();
    let response_str = response.text().await.unwrap();
    let posts_response: RedditPostResponse = serde_json::from_str(&response_str).unwrap();

    posts_response.data.children
}
