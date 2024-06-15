use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewsArticle {
    pub id: String,
    pub headline: String,
    pub article_body: String,
    pub description: String,
    pub genre: Vec<String>,
    pub images: Vec<String>,
}

impl NewsArticle {
    fn get_filename_from_url(&self, url: &str) -> String {
        let parsed_url = Url::parse(&url).ok().unwrap();
        let segments = parsed_url.path_segments().unwrap();
        segments
            .last()
            .map(|s| s.to_string())
            .get_or_insert("".to_string())
            .to_lowercase()
            .to_string()
    }

    pub fn get_image_path(&self, image_url: &str) -> String {
        format!("{}-{}", self.id, self.get_filename_from_url(image_url))
    }

    pub fn get_image_video_path(&self, image_url: &str) -> String {
        format!("{}.mp4", self.get_image_path(image_url))
    }

    pub fn get_audio_path(&self) -> String {
        format!("{}.mp3", self.id)
    }

    pub fn get_temp_video_path(&self) -> String {
        format!("{}-temp.mp4", self.id)
    }

    pub fn get_final_video_path(&self) -> String {
        format!("{}-final.mp4", self.id)
    }
}
