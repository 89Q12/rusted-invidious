pub struct VideoThumbnail{
    quality: String,
    url: String,
    width: i32,
    height: i32
}

pub struct PlaylistVideo{
    title: String,
    video_id: String,
    length_seconds: i32,
    video_thumbnails: Vec<VideoThumbnail>
}