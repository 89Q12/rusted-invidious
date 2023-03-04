use serde_json::Value;
use serde::{Deserialize, Serialize};

use crate::api::{error::{ApiError, Errors}, PartialVideoTrait, common::{Streams, self}};
use super::misc::RelatedStream;
use crate::api::{AudioStreamTrait,VideoBasicInfoTrait,VideoStreamTrait,SubtitleTrait,ChapterTrait};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Video{
    video_streams: Vec<VideoStream>,  // The video streams of the video
    audio_streams: Vec<AudioStream>,  // The audio streams of the video
    subtitles: Vec<Subtitle>, // List of subtitles
    related_streams: Vec<RelatedStream>, // A list of related streams
    chapters: Option<Vec<Chapter>>, // Possible list of chapters
    dash: Option<String>,  // The dash manifest URL, to be used if not null  (for OTF streams)
    hls: Option<String>, // The hls manifest URL, to be used for Livestreams
    description: String,  // The description of the video
    duration: i32,  // The duration of the video in seconds
    likes: i64, // The number of likes the video has
    livestream: bool,  // Whether or not the video is a livestream
    proxy_url: String, // The proxy url to be used for rewrites
    thumbnail_url: String, // The thumbnail of the video
    title: String,  // The title of the video
    upload_date: String, // The date the video was uploaded
    uploader: String,  // The name of the channel of the video
    uploader_url: String, // The URL of the channel of the video
    uploader_verified: bool,  // Whether or not the channel of the video is verified
    uploader_subscriber_count: i64, // Subscribercount of the author(Channel)
    uploader_avatar: String, // The avatar url to the channel
    views: i64, // The number of views the video has
    licence: Option<String>,
    category: String,
    privacy: String,
    age_limit: u8
}
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioStream{
    pub bitrate: i32, // The bitrate of the audio stream in bytes
    pub codec: Option<String>, // The codec of the audio stream
    pub format: String, // The format of the audio stream
    pub index_end: i32, // Useful for creating dash streams
    pub index_start: i32,  // Useful for creating dash streams
    pub init_start: i32,  // Useful for creating dash streams
    pub init_end: i32, // Useful for creating dash streams
    pub mime_type: String, // The mime type of the audio stream
    pub quality: String, // The quality of the audio stream
    pub url: String, // The stream's URL
    pub video_only: bool, // Whether or not the stream is video only
    itag: i32,
}
#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoStream{
    bitrate: i32, // The bitrate of the video stream in bytes
    codec:  Option<String>, // The codec of the video stream
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
    width: i32, // The width of the video stream
    itag: i32,
}
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Subtitle{
    auto_generated: bool, // Whether or not the subtitle was auto-generated
    code: String, // The language code of the subtitle
    mime_type: String, // The mime type of the subtitle
    name: String, // The name of the subtitle
    url: String, // The URL of the subtitle
}
#[derive(Deserialize, Clone)]
pub struct Chapter{
    title: String, // Title of the chapter
    image: String, // Image to display
    start: i32, // Second where the chapter starts
}
 
impl TryFrom<Value> for Video{
    type Error = ApiError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match serde_json::from_value(value){
            Ok(val) => Ok(val),
            Err(err) => Err(ApiError::new(Errors::RequestError, err.to_string())),
        }
    }
}

impl ChapterTrait for Chapter{
    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_image_url(&self) -> String {
        self.image.clone()
    }

    fn get_start_second(&self) -> i32 {
        self.start.clone()
    }
}

impl AudioStreamTrait for AudioStream{
    fn get_bitrate(&self) -> i32 {
        self.bitrate.clone()
    }

    fn get_codec(&self) -> String {
        match &self.codec{
            Some(codec) => codec.clone(),
            None => String::from(""),
        }
    }

    fn get_format(&self) -> String {
        self.format.clone()
    }

    fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }

    fn get_quality(&self) -> String {
        self.quality.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }
    fn get_index_range(&self) -> String {
        format!("{}-{}",self.init_end,self.init_start)
    }

    fn get_itag(&self) -> String {
        self.itag.to_string()
    }
}

impl VideoStreamTrait for VideoStream{
    fn get_bitrate(&self) -> i32 {
        self.bitrate.clone()
    }

    fn get_codec(&self) -> String {
        match &self.codec{
            Some(codec) => codec.clone(),
            None => String::from(""),
        }
    }

    fn get_format(&self) -> String {
        self.format.clone()
    }

    fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }

    fn get_quality(&self) -> String {
        self.quality.clone()
    }

    fn get_url(&self) -> String {
        self.url.clone()
    }

    fn get_fps(&self) -> i32 {
        self.fps.clone()
    }

    fn get_height(&self) -> i32 {
        self.height.clone()
    }

    fn get_width(&self) -> i32 {
        self.width.clone()
    }

    fn get_index_range(&self) -> String {
        format!("{}-{}",self.init_end,self.init_start)
    }

    fn get_itag(&self) -> String {
        self.itag.to_string()
    }
}

impl SubtitleTrait for  Subtitle{
    fn is_auto_generated(&self) -> bool {
        self.auto_generated.clone()
    }

    fn get_lang_code(&self) -> String {
        self.code.clone()
    }

    fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_api_url(&self) -> String {
        self.url.clone()
    }
}

impl VideoBasicInfoTrait for Video{
    fn get_id(&self) -> String {
        String::from("")
    }
    fn get_dash(&self) -> Option<String> {
        self.dash.to_owned()
    }

    fn get_hls(&self) -> Option<String> {
        self.hls.clone()
    }

    fn get_description(&self) -> String {
        self.description.clone()
    }

    fn get_duration(&self) -> String {
        common::format_duration(self.duration)
    }

    fn get_likes(&self) -> i64 {
        self.likes.clone()
    }

    fn is_live(&self) -> bool {
        self.livestream.clone()
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_upload_date(&self) -> String {
        self.upload_date.clone()
    }

    fn get_uploader_name(&self) -> String {
        self.uploader.clone()
    }

    fn is_uploader_verified(&self) -> bool {
        self.uploader_verified.clone()
    }

    fn get_uploader_avatar_url(&self) -> String {
        self.uploader_avatar.clone()
    }

    fn get_uploader_url(&self) -> String {
        self.uploader_url.clone()
    }

    fn get_uploader_id(&self) -> String {
        self.uploader_url.split("/").map(|x| x.to_string()).collect::<Vec<String>>()[2].clone()    
    }

    fn get_uploader_subscriber_count(&self) -> i64 {
        self.uploader_subscriber_count.clone()
    }

    fn get_views(&self) -> String {
        common::format_numbers(self.views.clone())
    }

    fn get_thumbnail_url(&self) -> String {
        self.thumbnail_url.clone()
    }

    fn get_video_streams(&self) -> Vec<Box<dyn VideoStreamTrait>> {
        let mut ret_vec = Vec::new();
        for stream in self.video_streams.to_owned(){
            ret_vec.push(Box::new(stream) as Box<dyn VideoStreamTrait>);
        };
        ret_vec
    }

    fn get_audio_streams(&self) -> Vec<Box<dyn AudioStreamTrait>> {
        let mut ret_vec = Vec::new();
        for stream in self.audio_streams.to_owned(){
            ret_vec.push(Box::new(stream) as Box<dyn AudioStreamTrait>);
        };
        ret_vec
    }

    fn get_chapters(&self) -> Option<Vec<Box<dyn ChapterTrait>>> {
        match self.chapters.as_ref(){
            Some(chapters) => {
                let mut ret_vec = Vec::new();
                for stream in chapters.to_owned(){
                    ret_vec.push(Box::new(stream) as Box<dyn ChapterTrait>);
                };
                Some(ret_vec)
            },
            None => None
        }
    }

    fn get_subtitles(&self) -> Vec<Box<dyn SubtitleTrait>> {
        let mut ret_vec = Vec::new();
        for stream in self.subtitles.to_owned(){
            ret_vec.push(Box::new(stream) as Box<dyn SubtitleTrait>);
        };
        ret_vec
    }

    fn get_channel_id(&self) -> String {
        self.title.clone()
    }
    fn get_next_video(&self) -> String {
        match self.related_streams.get(0){
            Some(stream) => stream.get_id(),
            None => String::from(""),
        }
    }

    fn get_licence(&self) -> String {
        match &self.licence{
            Some(licence) => licence.clone(),
            None => String::from("No licence"),
        }
    }

    fn get_category(&self) -> String {
        self.category.clone()
    }

    fn get_privacy(&self) -> String {
        self.privacy.clone()
    }

    fn get_age_limit(&self) -> u8 {
        self.age_limit.clone()
    }

    fn has_related_streams(&self) -> bool {
        self.related_streams.is_empty().clone()
    }

    fn get_related_streams(&self) -> Vec<Box<dyn crate::api::PartialVideoTrait>> {
        let mut ret_vec = Vec::new();
        for stream in self.related_streams.to_owned(){
            ret_vec.push(Box::new(stream) as Box<dyn PartialVideoTrait>);
        };
        ret_vec
    }

}

impl Video{
    pub(super) fn set_dash(&mut self, dash: String){
        self.dash = Some(dash)
    }
    pub(super) fn has_dash(&self) -> bool {
        println!("{}", self.dash.is_some());
        match  self.dash {
            Some(_) => true,
            None => false,
        }
    }
    pub(super) fn get_audio(&self) -> Vec<AudioStream>{
        self.audio_streams.clone()
    }
    pub(super) fn get_video(&self) -> Vec<VideoStream>{
        self.video_streams.clone()
    }
    pub(super) fn get_duration_seconds(&self) -> i32{
        self.duration.clone()
    }
}