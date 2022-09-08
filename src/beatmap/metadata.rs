use deserialize::{DeserializeJson, ParseList};
use deserialize_derive::DeserializeJson;
use std::collections::HashMap;

pub const SECTION_NAME: &str = "[Metadata]";

/// 谱面元数据
#[derive(DeserializeJson, Debug)]
pub struct Metadata {
    #[json_value(name = "Title")]
    title: Option<String>,

    #[json_value(name = "TitleUnicode")]
    title_unicode: Option<String>,

    #[json_value(name = "Artist")]
    artist: Option<String>,

    #[json_value(name = "ArtistUnicode")]
    artist_unicode: Option<String>,

    #[json_value(name = "Creator")]
    creator: Option<String>,

    #[json_value(name = "Version")]
    version: Option<String>,

    #[json_value(name = "Source")]
    source: Option<String>,

    #[json_value(name = "Tags")]
    tags: Vec<String>,

    #[json_value(name = "BeatmapID")]
    beatmap_id: i64,

    #[json_value(name = "BeatmapSetID")]
    beatmap_set_id: i64,
}

/// 默认值填充
impl Default for Metadata {
    fn default() -> Self {
        Self {
            title: None,
            title_unicode: None,
            artist: None,
            artist_unicode: None,
            creator: None,
            version: None,
            source: None,
            tags: vec![],
            beatmap_id: 0,
            beatmap_set_id: 0,
        }
    }
}
