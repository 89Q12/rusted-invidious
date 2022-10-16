use serde_json::Value;
use serde::Deserialize;

use super::misc::RelatedStream;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video{
    video_streams: Vec<VideoStream>,  // The video streams of the video
    audio_streams: Vec<AudioStream>,  // The audio streams of the video
    subtitles: Vec<Subtitle>, // List of subtitles
    related_streams: Vec<RelatedStream>, // A list of related streams
    chapters: Option<Vec<Chapter>>, // Possible list of chapters
    dash: Option<String>,  // The dash manifest URL, to be used if not null (for OTF streams)
    hls: Option<String>, // The hls manifest URL, to be used for Livestreams
    description: String,  // The description of the video
    duration: i32,  // The duration of the video in seconds
    likes: i32, // The number of likes the video has
    lifestream: bool,  // Whether or not the video is a livestream
    proxy_url: String, // The proxy url to be used for rewrites
    thumbnail_url: String, // The thumbnail of the video
    title: String,  // The title of the video
    uploaded_date: String, // The date the video was uploaded
    uploader: String,  // The name of the channel of the video
    uploader_url: String, // The URL of the channel of the video
    uploader_verified: bool,  // Whether or not the channel of the video is verified
    uploader_subscriber_count: i32, // Subscribercount of the author(Channel)
    views: i32, // The number of views the video has
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream{
    bitrate: i32, // The bitrate of the audio stream in bytes
    codec: String, // The codec of the audio stream
    format: String, // The format of the audio stream
    index_end: i32, // Useful for creating dash streams
    index_start: i32,  // Useful for creating dash streams
    init_start: i32,  // Useful for creating dash streams
    init_end: i32, // Useful for creating dash streams
    mime_type: String, // The mime type of the audio stream
    quality: String, // The quality of the audio stream
    url: String, // The stream's URL
    video_only: bool, // Whether or not the stream is video only
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream{
    bitrate: i32, // The bitrate of the video stream in bytes
    codec: String, // The codec of the video stream
    format: String, // The format of the video stream
    fps: i32, // The frames per second of the video stream
    height: i32, // The height of the video stream
    index_end: i32, // Useful for creating dash streams
    index_start: i32, // Useful for creating dash streams
    init_start: i32, // Useful for creating dash streams
    init_end: i32, // Useful for creating dash streams
    mime_type: String, // The mime type of the video stream
    quality: String, // The quality of the video stream
    url: String, // The stream's URL
    video_only: bool, // Whether or not the stream is video only
    width: i32 // The width of the video stream
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtitle{
    auto_generated: bool, // Whether or not the subtitle was auto-generated
    code: String, // The language code of the subtitle
    mime_type: String, // The mime type of the subtitle
    name: String, // The name of the subtitle
    url: String, // The URL of the subtitle
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