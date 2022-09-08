use crate::beatmap::Difficulty;
use crate::beatmap::General;
use crate::beatmap::Metadata;

/// 谱面
#[derive(Debug)]
pub struct BeatmapInfo {
    pub format: Option<String>,
    pub general: Option<General>,
    pub metadata: Option<Metadata>,
    pub difficulty: Option<Difficulty>,
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
