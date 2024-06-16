use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditVideo {
    pub bitrate_kbps: i32,
    pub fallback_url: String,
    pub dash_url: String,
    pub hls_url: String,
    pub has_audio: bool,
    pub height: f32,
    pub width: f32,
    pub duration: f32,
    pub is_gif: bool,
    pub transcoding_status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditMedia {
    pub reddit_video: RedditVideo,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditPostData {
    pub subreddit: String,
    pub author_fullname: String,
    pub title: String,
    pub downs: i32,
    pub upvote_ratio: f32,
    pub ups: i32,
    pub total_awards_received: i32,
    pub score: i32,
    pub created: f32,
    pub url_overridden_by_dest: String,
    pub over_18: bool,
    pub num_comments: i32,
    pub url: String,
    pub media: RedditMedia,
    pub is_video: bool,
}

impl RedditPostData {
    pub fn get_post_id(&self) -> String {
        self.url.split('/').last().unwrap().to_string()
    }

    pub fn get_video_path(&self) -> String {
        format!("{}/{}.mp4", "reddit_videos", self.get_post_id())
    }

    pub fn get_converted_video_path(&self) -> String {
        format!(
            "{}/{}-converted.mp4",
            "converted_videos",
            self.get_post_id()
        )
    }

    pub fn get_video_cover_path(&self) -> String {
        format!("{}/{}-cover.jpg", "converted_videos", self.get_post_id())
    }

    pub fn is_eligible_for_insta(&self) -> bool {
        !self.over_18
            && self.is_video
            && self.ups > 1000
            && !Path::new(&self.get_converted_video_path()).exists()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditPost {
    pub data: RedditPostData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditData {
    pub dist: usize,
    pub children: Vec<RedditPost>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RedditPostResponse {
    pub data: RedditData,
}
