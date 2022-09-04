use crate::beatmap::difficulty::Difficulty;
use crate::beatmap::general::General;
use crate::beatmap::metadata::Metadata;

/// 谱面
#[derive(Debug)]
pub struct BeatmapInfo {
    general: Option<General>,
    metadata: Option<Metadata>,
    difficulty: Option<Difficulty>,
}

/// 默认信息
impl Default for BeatmapInfo {
    fn default() -> Self {
        Self {
            general: None,
            metadata: None,
            difficulty: None,
        }
    }
}
