use std::process::Command;

use crate::{models::reddit_post::RedditPost, CONFIG};

pub async fn upload_to_instagram(post: &RedditPost) {
    let hashtags = vec!["#_flip__da__script_", "#unexpected", "#twist"];
    let title = format!("{}\n{}", post.data.title, hashtags.join(" "));
    let title = format!("\"{}\"", title.replace("\"", "\\\""));
    let command = format!(
        "node {} {} {} {} {} {}",
        CONFIG.get().unwrap().insta_scraper_path,
        CONFIG.get().unwrap().insta_username,
        CONFIG.get().unwrap().insta_password,
        title,
        post.data.get_converted_video_path(),
        post.data.get_video_cover_path(),
    );
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute process");
    let status = child.wait().expect("failed to wait on child");
    if status.success() {
        println!("Successfully uploaded to instagram");
    } else {
        eprintln!("Couldn't upload to instagram {}", status);
    }
}
