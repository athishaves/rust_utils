use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TranscribeSegment {
    pub id: i32,
    pub seek: f32,
    pub start: f32,
    pub end: f32,
    pub text: String,
}

#[derive(Deserialize)]
pub struct TranscribeResponse {
    pub language: String,
    pub duration: f32,
    pub text: String,
    pub segments: Vec<TranscribeSegment>,
}
