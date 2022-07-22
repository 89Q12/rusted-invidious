use youtubei_rs::types::video::{Format, CaptionTrack};

pub struct Player<'a>{
    pub formats: &'a Vec<&'a Format>,
    pub audio_streams: Vec<&'a Format>,
    pub captions: &'a Vec<CaptionTrack>,
    pub preferred_captions: Vec<&'a CaptionTrack>,
    pub hls_manifest_url: Option<String>,
    pub aspect_ratio: &'a str,
}