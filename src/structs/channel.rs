use youtubei_rs::types::{channel::{ChannelAboutFullMetadataRenderer, C4TabbedHeaderRenderer, ChannelMetadataRenderer}, thumbnail::Thumbnail, enums::ItemSectionRendererContents};

pub struct Channel {
    pub about: ChannelAboutFullMetadataRenderer,
    pub banner: Option<String>,
    pub header: C4TabbedHeaderRenderer,
    pub metadata:  ChannelMetadataRenderer,
    pub thumbnail: String,
    pub sort_options: Vec<String>,
    pub sorted_by: String,
    pub auto_generated: bool,
    pub items: Vec<ItemSectionRendererContents>,
    pub has_community_enabled: bool,
}