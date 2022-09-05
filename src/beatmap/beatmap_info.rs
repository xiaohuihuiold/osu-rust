use crate::beatmap::Difficulty;
use crate::beatmap::General;
use crate::beatmap::Metadata;

/// 谱面
#[derive(Debug)]
pub struct BeatmapInfo {
    format: Option<String>,
    general: Option<General>,
    metadata: Option<Metadata>,
    difficulty: Option<Difficulty>,
}

/// 默认信息
impl Default for BeatmapInfo {
    fn default() -> Self {
        Self {
            format: None,
            general: None,
            metadata: None,
            difficulty: None,
        }
    }
}
