use super::misc::{PlaylistVideo, VideoThumbnail};

pub struct ChannelVideo{
    title: String,
    video_id: String,
    author_id: String,
    author_url: String,
    video_thumbnails: Vec<VideoThumbnail>,
    description: String,
    description_html: String,
    view_count: i64,
    published: i64,
    published_text: String,
    length_seconds: i64,
    paid: bool,
    premium: bool
}
pub struct ChannelPlaylist{
    title: String,
    playlist_id: String,
    author: String,
    author_id: String,
    author_url: String,
    video_count: i32,
    videos:Vec<PlaylistVideo>
}

pub struct ChannelPlaylists{
    continuation: Option<String>,
    playlists: Vec<ChannelPlaylist>
}
