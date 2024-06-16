use reqwest::{blocking::{multipart::{Form, Part}, Client}, header::HeaderMap};

use crate::{models::{gpt_completion::{CompletionBody, CompletionContent, CompletionRequestMessage, CompletionResponse}, gpt_speech::SpeechRequest, gpt_transcribe::{TranscribeResponse, TranscribeSegment}}, CONFIG};

use std::{fs::File, io::Read};
use std::io::copy;

const COMPLETIONS_URL: &str = "https://api.openai.com/v1/chat/completions";
const SPEECH_URL: &str = "https://api.openai.com/v1/audio/speech";
const TRANSCRIBE_URL: &str = "https://api.openai.com/v1/audio/transcriptions";

pub fn get_summary(content: &str) -> String {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", CONFIG.get().unwrap().gpt_token.parse().unwrap());

    let data = CompletionBody {
        model: "gpt-4o".to_string(),
        messages: vec![
            CompletionRequestMessage {
                role: "system".to_string(),
                content: vec![
                    CompletionContent {
                        r#type: "text".to_string(),
                        text: "summarise the given article within 120 words. result should be creative and interesting. use short statements".to_string(),
                    }
                ]
            },
            CompletionRequestMessage {
                role: "user".to_string(),
                content: vec![
                    CompletionContent {
                        r#type: "text".to_string(),
                        text: content.to_string(),
                    }
                ]
            }
        ],
        temperature: 1, 
        max_tokens: 1024, 
        top_p: 1, 
        frequency_penalty: 0, 
        presence_penalty: 0,
    };

    let response = client.post(COMPLETIONS_URL).headers(headers).json(&data).send().unwrap();
    let response_str = response.text().unwrap();
    let completion_response: CompletionResponse = serde_json::from_str(&response_str).unwrap();
    
    completion_response.choices[0].message.content.to_string()
}

pub fn get_audio(content: &str, output_path: &str) {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("Authorization", CONFIG.get().unwrap().gpt_token.parse().unwrap());

    let data = SpeechRequest {
        model: "tts-1".to_string(),
        input: content.to_string(),
        voice: "echo".to_string(),
        speed: 1.2,
    };

    let response = client.post(SPEECH_URL).headers(headers).json(&data).send().unwrap();

    let response_bytes = response.bytes().unwrap();

    let mut file = File::create(output_path).unwrap();
    copy(&mut response_bytes.as_ref(), &mut file).unwrap();
}

pub fn get_subtitles(audio_path: &str) -> Vec<TranscribeSegment> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", CONFIG.get().unwrap().gpt_token.parse().unwrap());

    let mut file = File::open(audio_path).expect("File not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let form = Form::new()
        .part("file", Part::bytes(buffer).file_name("file.mp3"))
        .text("model", "whisper-1")
        .text("response_format", "verbose_json");

    let response = client.post(TRANSCRIBE_URL).headers(headers).multipart(form).send().unwrap();
    let response_str = response.text().unwrap();

    let transcribe_response: TranscribeResponse = serde_json::from_str(&response_str).unwrap();
    transcribe_response.segments
}
