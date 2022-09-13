use crate::storyboard::{Background, Sprite};

/// 故事板信息
#[derive(Default, Debug)]
pub struct StoryboardInfo {
    pub background_image: Option<Background>,
    pub background_video: Option<Background>,
    pub layer_background: Vec<Sprite>,
    pub layer_fail: Vec<Sprite>,
    pub layer_pass: Vec<Sprite>,
    pub layer_foreground: Vec<Sprite>,
}

impl StoryboardInfo {
    pub const SECTION_NAME: &'static str = "SECTION_NAME";
}
