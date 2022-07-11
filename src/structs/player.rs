use youtubei_rs::types::misc::{Format, CaptionTrack};

pub struct Player<'a>{
    pub formats: Vec<Format>,
    pub audio_streams: Vec<Format>,
    pub captions: Vec<CaptionTrack>,
    pub preferred_captions: Vec<&'a CaptionTrack>,
    pub hls_manifest_url: Option<String>,
    pub aspect_ratio: String,
}