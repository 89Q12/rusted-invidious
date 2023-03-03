pub mod dash {
    use std::time::Duration;

    use serde::ser::Serialize;
    use quick_xml::writer::Writer;
    use quick_xml::se::Serializer;
    use dash_mpd::{MPD, Period, AdaptationSet, Representation, BaseURL, AudioChannelConfiguration, SegmentBase, Initialization};
    use base64::encode;

    use crate::api::common::Streams;

    pub fn generate_dash_file_from_formats(vstreams: Vec<Streams>, video_length: i32) -> String {
        let mut buffer = Vec::new();
        let writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let mut ser = Serializer::with_root(writer, Some("MPD"));
        let mpd = MPD {
            mpdtype: Some("static".into()),
            xmlns: Some("urn:mpeg:dash:schema:mpd:2011".into()),
            profiles: Some("urn:mpeg:dash:profile:full:2011".to_string()),
            minBufferTime: Some(Duration::from_millis(1500)),
            mediaPresentationDuration: Some(Duration::from_secs(video_length.try_into().unwrap())),
            periods: [gen_period(vstreams, video_length)].to_vec(),
            ..Default::default()
        };
        mpd.serialize(&mut ser)
        .expect("serializing MPD struct");
        encode(String::from_utf8(buffer.clone()).unwrap())
    }
    fn gen_period(vstreams: Vec<Streams>, video_length: i32) -> Period{
        Period{
            adaptations: gen_adaptations(vstreams, video_length),
            ..Default::default()
        }
    }

    fn gen_adaptations(vstreams: Vec<Streams>, video_length: i32) -> Vec<AdaptationSet> {
        let mut adaptations = Vec::new();
        let mut audio_mime_types: Vec<String> = Vec::new();
        let mut video_mime_types: Vec<String> = Vec::new();
        for i in 0..vstreams.len(){
            match &vstreams[i]{
                Streams::VideoStream(stream) => video_mime_types.push(stream.get_mime_type()),
                Streams::AudioStream(stream) => audio_mime_types.push(stream.get_mime_type()),
            }
        }
        audio_mime_types.sort();
        audio_mime_types.dedup();
        video_mime_types.sort();
        video_mime_types.dedup();
        for i in 0..audio_mime_types.len(){
            let set = AdaptationSet{
                //Should not ever panic
                id: Some(i.try_into().unwrap()),
                subsegmentAlignment: Some(true),
                mimeType: Some(audio_mime_types[i].clone()),
                representations: gen_representations(&vstreams, video_length,false),
                ..Default::default()
            };
            adaptations.push(set);
        };
        for i in 0..video_mime_types.len(){
            let set = AdaptationSet{
                //Should not ever panic
                id: Some(i.try_into().unwrap()),
                subsegmentAlignment: Some(true),
                mimeType: Some(audio_mime_types[i].clone()),
                representations: gen_representations(&vstreams, video_length, true),
                ..Default::default()
            };
            adaptations.push(set);
        };
        adaptations
    }

    fn gen_representations(vstreams: &Vec<Streams>, video_length: i32,match_video: bool) -> Vec<Representation> {
        let mut representations: Vec<Representation> = Vec::new();
        for stream in vstreams{
            if match_video{
                match stream{
                    Streams::VideoStream(video) => representations.push(Representation{
                        id: Some(video.get_itag()),
                        mimeType: Some(video.get_mime_type()),
                        codecs:Some(video.get_codec()),
                        startWithSAP: Some(1),
                        BaseURL: [BaseURL{
                            base: video.get_url(),
                            serviceLocation: None,
                        }].to_vec(),
                        bandwidth: Some(video.get_bitrate().try_into().unwrap()),
                        SegmentBase: Some(SegmentBase{
                            initialization: Some(Initialization{
                                sourceURL: None,
                                range: Some(video.get_index_range()),
                            }),
                            indexRange: Some(video.get_index_range()),
                            ..Default::default()
                        }),
                        width: Some(video.get_width().try_into().unwrap()),
                        height: Some(video.get_height().try_into().unwrap()),
                        frameRate: Some(video.get_fps().to_string()),
                        ..Default::default()
                    }),
                    _ => continue,
                }
            }
            else{
                match stream{
                    Streams::AudioStream(audio) => representations.push(Representation{
                        id: Some(audio.get_itag()),
                        mimeType: Some(audio.get_mime_type()),
                        codecs:Some(audio.get_codec()),
                        scanType: Some("Progressive".to_string()),
                        startWithSAP: Some(1),
                        BaseURL: [BaseURL{
                            base: audio.get_url(),
                            serviceLocation: None,
                        }].to_vec(),
                        bandwidth: Some(audio.get_bitrate().try_into().unwrap()),
                        AudioChannelConfiguration: Some(AudioChannelConfiguration{
                            id: None,
                            schemeIdUri: Some("urn:mpeg:dash:23003:3:audio_channel_configuration:2011".to_string()),
                            value: Some("2".to_string()),
                        }),
                        SegmentBase: Some(SegmentBase{
                            initialization: Some(Initialization{
                                sourceURL: None,
                                range: Some(audio.get_index_range()),
                            }),
                            indexRange: Some(audio.get_index_range()),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    _ => continue,
                } 
            }
        }
        todo!()
    }
}
