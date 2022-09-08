use deserialize::{from_str_enum, from_str_enum_value, DeserializeJson};
use deserialize_derive::DeserializeJson;
use std::collections::HashMap;
use std::str::FromStr;

pub const SECTION_NAME: &str = "[General]";

from_str_enum_value! {
    /// 倒计时速度
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Countdown {
        None = 0,
        Normal = 1,
        Half = 2,
        Double = 3,
    }
}

from_str_enum! {
    /// 默认音效组
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum SampleSet {
        Normal,
        Soft,
        Drum,
    }
}

from_str_enum_value! {
    /// 游戏模式
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum GameMode {
        Osu = 0,
        Taiko = 1,
        Catch = 2,
        Mania = 3,
    }
}

from_str_enum! {
    /// 皮肤覆盖层与数字层的关系
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum OverlayPosition {
        NoChange,
        Below,
        Above,
    }
}

/// 谱面信息
#[derive(DeserializeJson, Debug)]
pub struct General {
    #[json_value(name = "AudioFilename")]
    pub audio_file_name: Option<String>,

    #[json_value(name = "AudioLeadIn")]
    pub audio_lead_in: i64,

    #[json_value(name = "PreviewTime")]
    pub preview_time: i64,

    #[json_value(name = "Countdown")]
    pub countdown: Countdown,

    #[json_value(name = "SampleSet")]
    pub sample_set: SampleSet,

    #[json_value(name = "StackLeniency")]
    pub stack_leniency: f64,

    #[json_value(name = "Mode")]
    pub mode: GameMode,

    #[json_value(name = "LetterboxInBreaks")]
    pub letterbox_in_breaks: bool,

    #[json_value(name = "UseSkinSprites")]
    pub use_skin_sprites: bool,

    #[json_value(name = "OverlayPosition")]
    pub overlay_position: OverlayPosition,

    #[json_value(name = "SkinPreference")]
    pub skin_preference: Option<String>,

    #[json_value(name = "EpilepsyWarning")]
    pub epilepsy_warning: bool,

    #[json_value(name = "CountdownOffset")]
    pub countdown_offset: i64,

    #[json_value(name = "SpecialStyle")]
    pub spacial_style: bool,

    #[json_value(name = "WidescreenStoryboard")]
    pub widescreen_storyboard: bool,

    #[json_value(name = "SamplesMatchPlaybackRate")]
    pub samples_match_playback_rate: bool,
}

/// 默认值填充
impl Default for General {
    fn default() -> Self {
        Self {
            audio_file_name: None,
            audio_lead_in: 0,
            preview_time: -1,
            countdown: Countdown::None,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: GameMode::Osu,
            letterbox_in_breaks: false,
            use_skin_sprites: false,
            overlay_position: OverlayPosition::NoChange,
            skin_preference: None,
            epilepsy_warning: false,
            countdown_offset: 0,
            spacial_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn deserialize_json_general() {
        let mut map = HashMap::<String, String>::new();
        map.insert("AudioFilename".into(), "测试音频文件".into());
        map.insert("AudioLeadIn".into(), "0".into());
        map.insert("PreviewTime".into(), "12345".into());
        map.insert("Countdown".into(), "0".into());
        map.insert("SampleSet".into(), "Soft".into());
        map.insert("StackLeniency".into(), "0.5".into());
        map.insert("Mode".into(), "0".into());
        map.insert("LetterboxInBreaks".into(), "1".into());
        map.insert("WidescreenStoryboard".into(), "1".into());
        let general = General::from_json(&map);
        assert_eq!(general.mode, GameMode::Osu);
    }
}
