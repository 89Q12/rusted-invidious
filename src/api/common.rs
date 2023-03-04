use crate::api::traits::VideoStream as VStream;
use crate::api::traits::AudioStream as AStream;
use std::str;


pub enum Streams {
    VideoStream(Box<dyn VStream>),
    AudioStream(Box<dyn AStream>)
}
pub fn format_duration(duration: i32) -> String {
    fn pad(num: f64, size: usize) -> String{
        let time_string =("000".to_string() + &num.to_string()).to_string();
        let real_index = time_string.len() - size;
        time_string[real_index..].to_string()
    }
    let time: f64 = duration.into();
    let hours = (time as i32/ 60 /60).abs();
    let minutes = (time as i32/ 60).abs() % 60;
    let seconds = (time - minutes as f64 * 60_f64).abs();
    let mut time_str = "".to_owned();
    if hours > 0 {
        time_str = hours.to_string() + &":";
    }
    format!("{}{}:{}", time_str,pad(minutes.into(), 2), pad(seconds, 2))
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