const SECTION_NAME: &str = "[Metadata]";

/// 谱面元数据
#[derive(Debug)]
pub struct Metadata {
    title: Option<String>,
    title_unicode: Option<String>,
    artist: Option<String>,
    artist_unicode: Option<String>,
    creator: Option<String>,
    version: Option<String>,
    source: Option<String>,
    tags: Option<Vec<String>>,
    beatmap_id: i32,
    beatmap_set_id: i32,
}

/// 默认值填充
impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            title: None,
            title_unicode: None,
            artist: None,
            artist_unicode: None,
            creator: None,
            version: None,
            source: None,
            tags: None,
            beatmap_id: 0,
            beatmap_set_id: 0,
        }
    }
}
