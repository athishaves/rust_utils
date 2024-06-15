use std::process::{Command, ExitStatus};
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{copy, Write};
use std::str;

use reqwest::blocking::get;

use crate::models::{gpt_transcribe::TranscribeSegment, news_article::NewsArticle};

fn download_image(url: &str, output_path: &Path) {
    let response = get(url).unwrap();
    let mut file = File::create(output_path).unwrap();
    let content = response.bytes().unwrap();
    copy(&mut content.as_ref(), &mut file).unwrap();
}

fn create_video(article: NewsArticle, running_time: f32) {
    let mut image_paths: Vec<String> = vec![];
    for image_url in &article.images {
        let image_path = article.get_image_path(&image_url);
        download_image(&image_url, Path::new(&image_path));
        println!("Image saved at {}", image_path);
        image_paths.push(image_path);
    }

    let time_per_image = running_time / (image_paths.len() as f32);
    println!("Total time : {} - Time / image {}", running_time, time_per_image);

    let temp_file = "temp.txt";
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(temp_file).unwrap();

    for image in image_paths {
        let command = format!(
            "ffmpeg -loop 1 -i {image} -loop 1 -i {image} -filter_complex \"\
            [0:v]scale=iw*max(1080/iw\\,1920/ih):ih*max(1080/iw\\,1920/ih),\
            crop=1080:1920,avgblur={blur},setsar=1[bg]; \
            [1:v]scale='if(gt(a,1080/1920),1080,-2)':'if(gt(a,1080/1920),-2,1920)',\
            pad=1080:1920:(1080-iw)/2:(1920-ih)/2:color=black@0,setsar=1[fg]; \
            [bg][fg]overlay=W/2-overlay_w/2:H/2-overlay_h/2[out]\" -map \"[out]\" -t {time} -c:v libx264 -crf 18 -pix_fmt yuv420p -y {output}.mp4",
            image = image,
            blur = BLUR,
            time = time_per_image,
            output = image,
        );
        let status = execute_command(&command);
        if status.success() {
            println!("Generated {}", image);
            writeln!(file, "file '{}.mp4'", image).unwrap();
        }
    }

    let command = format!(
       "ffmpeg -f concat -safe 0 -i {} -c copy -y {}",
       temp_file,
       article.get_temp_video_path(),
    );
    let status = execute_command(&command);
    if status.success() {
        println!("Created video {}", &article.get_temp_video_path());
    }
}

fn get_audio_running_time(audio_path: &str) -> f32 {
    let command = format!(
        "ffprobe -i {} -show_entries format=duration -v quiet -of csv=\"p=0\"",
        audio_path,
    );

    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output().unwrap();
    let stdout = str::from_utf8(&output.stdout).unwrap();
    stdout.trim().parse().unwrap()
}

pub fn generate_video(article: NewsArticle, subtitles: Vec<TranscribeSegment>) {
    let total_time = get_audio_running_time(&article.get_audio_path());
    create_video(article.clone(), total_time);
    add_subtitle(
        &article.get_temp_video_path(), 
        &article.get_audio_path(), 
        &article.get_final_video_path(), 
        subtitles
    );
}

const FONT: &str = "KOMIKAX_.ttf";
const FONT_SIZE: usize = 75;
const LINE_PADDING: usize = 16 + FONT_SIZE;
const SHADOW_OFFSET: usize = 8;
const BORDER_WIDTH: usize = 6;
const BLUR: usize = 25;
const MAX_CHAR_IN_LINE: usize = 36;

fn get_text_format(segment_text: &str, segment_index: usize, segment: &TranscribeSegment, is_shadow: bool) -> String {
  let binding = segment_text.replace("\"", "'");
  let binding = binding.replace("'", "'\\\\\\\\\\\\''");
  let text = binding.trim();
  let color = if is_shadow {"black"} else {"white"};
  let offset = if is_shadow {SHADOW_OFFSET} else {0};
  
  format!(
    "drawtext=fontfile={}:text='{}':fontcolor={}:fontsize={}:x=(w-text_w)/2+{}:y=75*h/100+{}:borderw={}:bordercolor=black:enable='between(t,{},{})'", 
    FONT,
    text.to_uppercase(), 
    color,
    FONT_SIZE, 
    offset,
    offset + (LINE_PADDING * segment_index), 
    BORDER_WIDTH,
    segment.start, 
    segment.end
  )
}

fn get_text_command(segment: &TranscribeSegment) -> Vec<String> {
  let mut result = Vec::new();
  let mut current_line = String::new();
  let mut segment_count: usize = 0;

  for word in segment.text.split_whitespace() {
      if word.is_empty() {
          continue;
      }

      if current_line.len() + word.len() + 1 > MAX_CHAR_IN_LINE {
          result.push(
              get_text_format(&current_line, segment_count, segment, true)
          );
          result.push(
              get_text_format(&current_line, segment_count, segment, false)
          );
          segment_count += 1;
          current_line = String::new();
      }

      if !current_line.is_empty() {
          current_line.push(' ');
      }

      current_line.push_str(word);
  }

  if !current_line.is_empty() {
      result.push(
          get_text_format(&current_line, segment_count, segment, true)
      );
      result.push(
          get_text_format(&current_line, segment_count, segment, false)
      );
  }

  result
}

fn get_draw_text(subtitles: Vec<TranscribeSegment>) -> String {
    let sub_commands: Vec<String> = subtitles
      .iter()
      .flat_map(|segment| get_text_command(segment))
      .collect();

    sub_commands.join(", ")
}

fn add_subtitle(input_file: &str, audio_file: &str, output_file: &str, subtitles: Vec<TranscribeSegment>) {
    let command = format!(
        "ffmpeg -i {} -i {} -vf \"{}\" -map 0:v -map 1:a -c:v libx264 -c:a aac -strict experimental -y {}",
        input_file,
        audio_file,
        get_draw_text(subtitles),
        output_file
    );

    let status = execute_command(&command);
    if status.success() {
        println!("Successfully added subtitles");
    } else {
        eprintln!("Couldn't add subtitle. Failed with status: {}", status);
    }
}

pub fn resize_video(video_path: &str, logo_path: &str, output_video_path: &str) {
    let command = format!(
        "ffmpeg -i {video_path} -i {logo_path} -filter_complex \"\
        [0:v]scale='if(gt(a,1080/1920),1080,-1)':'if(gt(a,1080/1920),-1,1920)',pad=1080:1920:(1080-iw)/2:(1920-ih)/2:white[bg];\
        [1:v]scale={width}:{height}[logo];\
        [bg][logo]overlay=x=(W-w)/2:y=(H-h)/2\" -c:a copy -y {output_video_path}",
        video_path = video_path,
        logo_path = logo_path,
        width = 250,
        height = 156,
        output_video_path = output_video_path,
    );
    let status = execute_command(&command);
    if status.success() {
        println!("Successfully resized video");
    } else {
        eprintln!("Couldn't resize video. Failed with status: {}", status);
    }
}


fn execute_command(command: &str) -> ExitStatus {
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("failed to execute process");
    child.wait().expect("failed to wait on child")
}
