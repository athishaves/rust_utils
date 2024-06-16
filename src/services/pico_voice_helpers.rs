use leopard::{Leopard, LeopardBuilder};

use crate::{models::gpt_transcribe::TranscribeSegment, CONFIG};

pub fn get_subtitles(audio_path: &str) -> Vec<TranscribeSegment> {
    let leopard: Leopard = LeopardBuilder::new()
        .access_key(CONFIG.get().unwrap().pv_token.clone())
        .init()
        .unwrap();
    let transcript = leopard.process_file(audio_path).unwrap();
    transcript
        .words
        .iter()
        .map(|word| TranscribeSegment {
            id: 0,
            seek: 0.0,
            start: word.start_sec,
            end: word.end_sec,
            text: word.word.to_string(),
        })
        .collect()
}
