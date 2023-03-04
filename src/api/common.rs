use crate::api::traits::VideoStream as VStream;
use crate::api::traits::AudioStream as AStream;
use std::str;


pub enum Streams {
    VideoStream(Box<dyn VStream>),
    AudioStream(Box<dyn AStream>)
}
pub fn format_duration(duration: i32) -> String {
    let duration = duration.abs();
    let hours = duration / 3600;
    let minutes = (duration - hours * 3600) / 60;
    let seconds = duration - hours * 3600 - minutes * 60;

    match (hours, minutes, seconds) {
        (0, m, s) => format!("{:02}:{:02}", m, s),
        (h, m, s) => format!("{:02}:{:02}:{:02}", h, m, s),
    }
}

pub fn format_numbers(num: i64) -> String{
    num.to_string()
    .as_bytes()
    .rchunks(3)
    .rev()
    .map(str::from_utf8)
    .collect::<Result<Vec<&str>, _>>()
    .unwrap()
    .join(",")
}