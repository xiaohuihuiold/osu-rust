use deserialize::DeserializeJson;
use deserialize_derive::DeserializeJson;
use std::collections::HashMap;

/// 谱面难度
#[derive(DeserializeJson, Debug)]
pub struct Difficulty {
    #[json_value(name = "HPDrainRate")]
    pub hp_drain_rate: f64,

    #[json_value(name = "CircleSize")]
    pub circle_size: f64,

    #[json_value(name = "OverallDifficulty")]
    pub overall_difficulty: f64,

    #[json_value(name = "ApproachRate")]
    pub approach_rate: f64,

    #[json_value(name = "SliderMultiplier")]
    pub slider_multiplier: f64,

    #[json_value(name = "SliderTickRate")]
    pub slider_tick_rate: f64,
}

impl Difficulty {
    pub const SECTION_NAME: &'static str = "[Difficulty]";
}

/// 默认值
impl Default for Difficulty {
    fn default() -> Self {
        Self {
            hp_drain_rate: 0.0,
            circle_size: 0.0,
            overall_difficulty: 0.0,
            approach_rate: 0.0,
            slider_multiplier: 0.0,
            slider_tick_rate: 0.0,
        }
    }
}
