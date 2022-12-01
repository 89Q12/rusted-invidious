use super::piped::SearchItem;

pub trait VideoBasicInfo{
    fn get_id(&self) -> String;
    fn get_dash(&self) -> String;
    fn get_hls(&self) -> Option<String>;
    fn get_description(&self) -> String;
    fn get_duration(&self) -> i32;
    fn get_likes(&self) -> i32;
    fn is_live(&self) -> bool;
    fn get_title(&self) -> String;
    fn get_upload_date(&self) -> String;
    fn get_uploader_name(&self) -> String;
    fn is_uploader_verified(&self) -> bool;
    fn get_uploader_avatar_url(&self) -> String;
    fn get_uploader_url(&self) -> String;
    fn get_uploader_id(&self) -> String;
    fn get_uploader_subscriber_count(&self) -> i32;
    fn get_views(&self) -> i32;
    fn get_thumbnail_url(&self) -> String;
    fn get_video_streams(&self) -> Vec<Box<dyn VideoStream>>;
    fn get_audio_streams(&self) -> Vec<Box<dyn AudioStream>>;
    fn get_chapters(&self) -> Option<Vec<Box<dyn Chapter>>>;
    fn get_subtitles(&self) -> Vec<Box<dyn Subtitle>>;
    fn get_channel_id(&self) -> String;
    fn get_next_video(&self) -> String;
    fn get_licence(&self) -> String;
    fn get_category(&self) -> String;
    fn get_privacy(&self) -> String;
    fn get_age_limit(&self) -> u8;
    fn has_related_streams(&self) -> bool;
    fn get_related_streams(&self) -> Vec<Box<dyn PartialVideo>>;
}
pub trait AudioStream{
    fn get_bitrate(&self) -> i32;
    fn get_codec(&self) -> String;
    fn get_format(&self) -> String;
    fn get_mime_type(&self) -> String;
    fn get_quality(&self) -> String;
    fn get_url(&self) -> String;
    fn get_index_range(&self) -> String;
    fn get_itag(&self) -> String;
}

pub trait VideoStream{
    fn get_bitrate(&self) -> i32;
    fn get_codec(&self) -> String;
    fn get_format(&self) -> String;
    fn get_mime_type(&self) -> String;
    fn get_quality(&self) -> String;
    fn get_url(&self) -> String;
    fn get_fps(&self) -> i32;
    fn get_height(&self) -> i32;
    fn get_width(&self) -> i32;
    fn get_index_range(&self) -> String;
    fn get_itag(&self) -> String;
}

pub trait Subtitle{
    fn is_auto_generated(&self) -> bool;
    fn get_lang_code(&self) -> String;
    fn get_mime_type(&self) -> String;
    fn get_name(&self) -> String;
    fn get_api_url(&self) -> String;
}

pub trait Chapter{
    fn get_title(&self) -> String;
    fn get_image_url(&self) -> String;
    fn get_start_second(&self) -> i32;
}

pub trait Playlist{
    fn get_banner_url(&self) -> Option<String>;
    fn get_name(&self) -> String;
    fn get_nextpage(&self) -> String;
    fn get_thumbnail_url(&self) -> String;
    fn get_uploader_name(&self) -> String;
    fn get_uploader_avatar_url(&self) -> String;
    fn get_uploader_url(&self) -> String;
    fn get_video_count(&self) -> i32;
    fn get_videos(&self) -> Vec<Box<dyn PartialVideo>>;
    fn get_id(&self) -> String;
}
pub trait PartialVideo{
    fn get_id(&self) -> String;
    fn get_short_description(&self) -> String;
    fn is_live(&self) -> bool;
    fn get_title(&self) -> String;
    fn get_upload_date(&self) -> String;
    fn get_uploader_name(&self) -> String;
    fn is_uploader_verified(&self) -> bool;
    fn get_uploader_avatar_url(&self) -> String;
    fn get_url(&self) -> String;
    fn get_thumbnail_url(&self) -> String;
    fn get_duration(&self) -> i32;
    fn get_uploader_url(&self) -> String;
    fn get_views(&self) -> i32;
}

pub trait SearchResult{
    fn get_items(&self) -> Vec<SearchItem>;
    fn get_suggestion(&self) -> Option<String>;
    fn is_corrected(&self) -> bool;
    fn get_nextpage(&self) -> String;
}

pub trait Channel{
    fn get_avatar_url(&self) -> String;
    fn get_banner_url(&self) -> String;
    fn get_description(&self) -> String;
    fn get_id(&self) -> String;
    fn get_name(&self) -> String;
    fn get_nextpage(&self) -> String;
    fn get_related_streams(&self) -> Vec<Box<dyn PartialVideo>>;
    fn get_subscriber_count(&self) -> i32;
    fn is_verified(&self) -> bool;
}
pub trait Comments{
    fn is_disabled(&self) -> bool;
    fn get_comments(&self) -> Vec<Box<dyn Comment>>;
    fn get_nextpage(&self) -> String;
}

pub trait Comment{
    fn get_author_name(&self) -> String;
    fn get_author_url(&self) -> String;
    fn get_author_avatar_url(&self) -> String;
    fn get_id(&self) -> String;
    fn is_hearted(&self) -> bool;
    fn is_pineed(&self) -> bool;
    fn get_like_count(&self) -> i32;
    fn is_author_verified(&self) -> bool;
    fn get_comment_text(&self) -> String;
    fn get_posted_date(&self) -> String;
}

pub trait Trending{
    fn get_videos(self) -> Vec<Box<dyn PartialVideo>>;
}

pub trait NextResult{
    fn get_items(&self) -> Vec<Box<dyn PartialVideo>>;
    fn get_nextpage(&self) -> String;
}