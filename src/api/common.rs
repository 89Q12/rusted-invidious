use crate::api::traits::VideoStream as VStream;
use crate::api::traits::AudioStream as AStream;


pub enum Streams {
    VideoStream(Box<dyn VStream>),
    AudioStream(Box<dyn AStream>)
}