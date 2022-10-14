use super::related_streams::RelatedStream;

pub struct Playlist {
    banner_url: String, // The banner of the playlist
    name: String, // The name of the playlist
    nextpage: String, // A JSON encoded page, which is used for the nextpage endpoint.
    related_streams: Vec<RelatedStream>, // A list of videos from the playlist
    thumbnail_url: String, // The thumbnail of the playlist
    uploader: String, // The name of the creator of the playlist
    uploader_svatar: String, // The avatar of the creator of the playlist
    uploader_url: String, // The URL of the creator of the playlist
    videos: i32 // The number of videos in the playlist
}
pub struct PlaylistNext {
    nextpage: String,
    related_streams: Vec<RelatedStream>,
}