use serde_json::Value;
use serde::Deserialize;

use super::misc::RelatedStream;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video{
    pub video_streams: Vec<VideoStream>,  // The video streams of the video
    pub audio_streams: Vec<AudioStream>,  // The audio streams of the video
    pub subtitles: Vec<Subtitle>, // List of subtitles
    pub related_streams: Vec<RelatedStream>, // A list of related streams
    pub chapters: Option<Vec<Chapter>>, // Possible list of chapters
    pub dash: Option<String>,  // The dash manifest URL, to be used if not null (for OTF streams)
    pub hls: Option<String>, // The hls manifest URL, to be used for Livestreams
    pub description: String,  // The description of the video
    pub duration: i32,  // The duration of the video in seconds
    pub likes: i32, // The number of likes the video has
    pub livestream: bool,  // Whether or not the video is a livestream
    pub proxy_url: String, // The proxy url to be used for rewrites
    pub thumbnail_url: String, // The thumbnail of the video
    pub title: String,  // The title of the video
    pub uploaded_date: String, // The date the video was uploaded
    pub uploader: String,  // The name of the channel of the video
    pub uploader_url: String, // The URL of the channel of the video
    pub uploader_verified: bool,  // Whether or not the channel of the video is verified
    pub uploader_subscriber_count: i32, // Subscribercount of the author(Channel)
    pub uploader_avatar: String, // The avatar url to the channel
    pub views: i32, // The number of views the video has
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream{
    pub bitrate: i32, // The bitrate of the audio stream in bytes
    pub codec: String, // The codec of the audio stream
    pub format: String, // The format of the audio stream
    pub index_end: i32, // Useful for creating dash streams
    pub index_start: i32,  // Useful for creating dash streams
    pub init_start: i32,  // Useful for creating dash streams
    pub init_end: i32, // Useful for creating dash streams
    pub mime_type: String, // The mime type of the audio stream
    pub quality: String, // The quality of the audio stream
    pub url: String, // The stream's URL
    pub video_only: bool, // Whether or not the stream is video only
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream{
    pub bitrate: i32, // The bitrate of the video stream in bytes
    pub codec: String, // The codec of the video stream
    pub format: String, // The format of the video stream
    pub fps: i32, // The frames per second of the video stream
    pub height: i32, // The height of the video stream
    pub index_end: i32, // Useful for creating dash streams
    pub index_start: i32, // Useful for creating dash streams
    pub init_start: i32, // Useful for creating dash streams
    pub init_end: i32, // Useful for creating dash streams
    pub mime_type: String, // The mime type of the video stream
    pub quality: String, // The quality of the video stream
    pub url: String, // The stream's URL
    pub video_only: bool, // Whether or not the stream is video only
    pub width: i32 // The width of the video stream
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitle{
    pub auto_generated: bool, // Whether or not the subtitle was auto-generated
    pub code: String, // The language code of the subtitle
    pub mime_type: String, // The mime type of the subtitle
    pub name: String, // The name of the subtitle
    pub url: String, // The URL of the subtitle
}
#[derive(Deserialize)]
pub struct Chapter{
    title: String, // Title of the chapter
    image: String, // Image to display
    start: i32, // Second where the chapter starts
}

impl From<Value> for Video{
    fn from(value: Value) -> Self {
        serde_json::from_value(value).unwrap()
    }
}
impl Video{
    pub fn get_next_video(&self) -> String{
        match self.related_streams.get(0){
            Some(stream) => stream.get_id(),
            None => String::from(""),
        }
    }
    pub fn get_channel_id(&self) -> String{
        self.uploader_url.split("/").map(|x| x.to_string()).collect::<Vec<String>>()[2]
    }
}