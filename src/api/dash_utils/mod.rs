pub mod types;
pub mod dash {
    use crate::api::traits::AudioStream;
    use crate::api::traits::VideoStream;

    use super::super::common::Streams;
    use super::types::AdaptionSet;
    use super::types::AdaptionSetAttributes;
    use super::types::Attributes;
    use super::types::Attributes2;
    use super::types::Attributes4;
    use super::types::Attributes5;
    use super::types::Attributes8;
    use super::types::Declaration;
    use super::types::Element;
    use super::types::Element2;
    use super::types::Element4;
    use super::types::Element5;
    use super::types::Element6;
    use super::types::Root;
    use quick_xml::se::Serializer;
    use quick_xml::writer::Writer;
    use serde::Serialize;
    use base64::encode;
    pub fn generate_dash_file_from_formats(vstreams: Vec<Streams>, length: i32) -> String {
        let generated_json = generate_xmljs_json_from_data(vstreams, length);
        let mut buffer = Vec::new();
        let writer = Writer::new_with_indent(&mut buffer, b' ', 2);
        let mut ser = Serializer::with_root(writer, Some("root"));
        generated_json.serialize(&mut ser).unwrap();
        println!("{}",String::from_utf8(buffer.clone()).unwrap());
        println!("{}", length);
        "data:application/dash+xml;charset=utf-8;base64,".to_owned()+ &encode(String::from_utf8(buffer.clone()).unwrap())
    }

    fn generate_xmljs_json_from_data(streams: Vec<Streams>, length: i32) -> Root {
        Root {
            declaration: Declaration {
                attributes: Attributes {
                    version: "1.0".to_string(),
                    encoding: "utf-8".to_string(),
                },
            },
            elements: [Element {
                type_field: "element".to_string(),
                name: "MPD".to_string(),
                attributes: Attributes2 {
                    xmlns: "urn:mpeg:dash:schema:mpd:2011".to_string(),
                    profiles: "urn:mpeg:dash:profile:full:2011".to_string(),
                    min_buffer_time: "PT1.5S".to_string(),
                    type_field: "static".to_string(),
                    media_presentation_duration: format!("PT{}S", length),
                },
                elements: generate_adaptation_set(streams),
            }]
            .to_vec(),
        }
    }

    fn generate_adaptation_set(streams: Vec<Streams>) -> Vec<Element2> {
        let mut elements = Vec::<Element2>::new();
        let mut mime_types = Vec::<String>::new();
        let mut root = Element2{
            type_field: "element".to_string(),
            name: "Period".to_string(),
            elements: Vec::new(),
        };
        for stream in &streams {
            match stream {
                Streams::VideoStream(stream) => mime_types.push(stream.get_mime_type()),
                Streams::AudioStream(stream) => mime_types.push(stream.get_mime_type()),
            }
        }
        mime_types.sort();
        println!("{:?}", mime_types);
        mime_types.dedup();
        println!("{:?}", mime_types);
        for (i, mime_type) in mime_types.iter().enumerate() {
            println!("{}", mime_type);
            let mut set = AdaptionSet {
                type_field: "element".to_string(),
                name: "AdaptionSet".to_string(),
                attributes: AdaptionSetAttributes {
                    id: i,
                    mime_type: mime_type.to_owned(),
                    start_with_sap: "1".to_string(),
                    subsegment_alignment: "true".to_string(),
                    scan_type: scan_type(mime_type),
                },
                elements: Vec::new(),
            };
            for i in 0..streams.len() {
                match &streams[i] {
                    Streams::VideoStream(stream) => {
                        set.elements.push(generate_representation_video(&stream))
                    }
                    Streams::AudioStream(stream) => {
                        set.elements.push(generate_representation_audio(&stream))
                    }
                }
            };
            root.elements.push(set);
        }
        elements.push(root);
        println!("{:?}", elements);
        return elements;
    }

    fn generate_representation_audio(stream: &Box<dyn AudioStream>) -> Element4 {
        Element4 {
            type_field: "Element".to_string(),
            name: "Representation".to_string(),
            attributes: Attributes4 {
                id: stream.get_itag(),
                codecs: stream.get_codec(),
                bandwidth: stream.get_bitrate().to_string(),
                width: None,
                height: None,
                max_playout_rate: None,
                frame_rate: None,
            },
            elements: [
                Element5 {
                    type_field: "element".to_string(),
                    name: "AudioChannelConfiguration".to_string(),
                    elements: None,
                    attributes: Some(Attributes8 {
                        index_range: None,
                        scheme_id_uri: Some(
                            "urn:mpeg:dash:23003:3:audio_channel_configuration:2011".to_string(),
                        ),
                        value: Some("2".to_string()),
                    }),
                },
                Element5 {
                    type_field: "element".to_string(),
                    name: "BaseURL".to_string(),
                    elements: Some(
                        [Element6 {
                            type_field: "text".to_string(),
                            text: Some(stream.get_url()),
                            name: None,
                            attributes: None,
                            elements: None,
                        }]
                        .to_vec(),
                    ),
                    attributes: None,
                },
                Element5 {
                    type_field: "element".to_string(),
                    name: "SeelementgmentBase".to_string(),
                    elements: Some(
                        [Element6 {
                            type_field: "element".to_string(),
                            text: None,
                            name: Some("Initialization".to_string()),
                            attributes: Some(Attributes5 {
                                range: Some(stream.get_index_range()),
                                id: None,
                                codecs: None,
                                bandwidth: None,
                            }),
                            elements: None,
                        }]
                        .to_vec(),
                    ),
                    attributes: Some(Attributes8 {
                        index_range: Some(stream.get_index_range()),
                        scheme_id_uri: None,
                        value: None,
                    }),
                },
            ]
            .to_vec(),
        }
    }
    fn generate_representation_video(stream: &Box<dyn VideoStream>) -> Element4 {
        Element4 {
            type_field: "Element".to_string(),
            name: "Representation".to_string(),
            attributes: Attributes4 {
                id: stream.get_itag(),
                codecs: stream.get_codec(),
                bandwidth: stream.get_bitrate().to_string(),
                width: Some(stream.get_width().into()),
                height: Some(stream.get_height().into()),
                max_playout_rate: Some("1".to_string()),
                frame_rate: Some(stream.get_fps().into()),
            },
            elements: [
                Element5 {
                    type_field: "element".to_string(),
                    name: "BaseURL".to_string(),
                    elements: Some(
                        [Element6 {
                            type_field: "text".to_string(),
                            text: Some(stream.get_url()),
                            name: None,
                            attributes: None,
                            elements: None,
                        }]
                        .to_vec(),
                    ),
                    attributes: None,
                },
                Element5 {
                    type_field: "element".to_string(),
                    name: "SegmentBase".to_string(),
                    elements: Some(
                        [Element6 {
                            type_field: "element".to_string(),
                            text: None,
                            name: Some("Initialization".to_string()),
                            attributes: Some(Attributes5 {
                                range: Some(stream.get_index_range()),
                                id: None,
                                codecs: None,
                                bandwidth: None,
                            }),
                            elements: None,
                        }]
                        .to_vec(),
                    ),
                    attributes: Some(Attributes8 {
                        index_range: Some(stream.get_index_range()),
                        scheme_id_uri: None,
                        value: None,
                    }),
                },
            ]
            .to_vec(),
        }
    }
    fn scan_type(mimetype: &String) -> Option<String> {
        if !mimetype.contains("audio") {
            return Some(String::from("progressive"));
        }
        None
    }
}
