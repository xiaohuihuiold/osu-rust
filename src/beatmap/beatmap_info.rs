use crate::beatmap::Difficulty;
use crate::beatmap::General;
use crate::beatmap::Metadata;
use crate::storyboard::StoryboardInfo;

/// 谱面信息
#[derive(Default, Debug)]
pub struct BeatmapInfo {
    pub format: Option<String>,
    pub general: Option<General>,
    pub metadata: Option<Metadata>,
    pub difficulty: Option<Difficulty>,
    pub storyboard: Option<StoryboardInfo>,
}
