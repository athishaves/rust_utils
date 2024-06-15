use serde::Serialize;

#[derive(Serialize)]
pub struct SpeechRequest {
    pub model: String,
    pub input: String,
    pub voice: String,
    pub speed: f32,
}
