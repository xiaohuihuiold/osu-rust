use crate::storyboard::{Background, Layer, Sprite};

/// 故事板信息
#[derive(Default)]
pub struct StoryboardInfo {
    pub background_image: Option<Background>,
    pub background_video: Option<Background>,
    pub layer_background: Vec<Sprite>,
    pub layer_fail: Vec<Sprite>,
    pub layer_pass: Vec<Sprite>,
    pub layer_foreground: Vec<Sprite>,
}

impl StoryboardInfo {
    pub const SECTION_NAME: &'static str = "[Events]";

    pub fn add_sprite(&mut self, sprite: Sprite) {
        let sprites = match sprite.layer {
            Layer::Background => &mut self.layer_background,
            Layer::Fail => &mut self.layer_fail,
            Layer::Pass => &mut self.layer_pass,
            Layer::Foreground => &mut self.layer_foreground,
        };
        sprites.push(sprite);
    }
}

impl ::core::fmt::Debug for StoryboardInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(
            f,
            "StoryboardInfo{{\n    background_image: {:?},\n    background_video: {:?},\n    layer_background: {},\n    layer_fail: {},\n    layer_pass: {},\n    layer_foreground: {},\n}}",
            &&self.background_image,
            &&self.background_video,
            &&self.layer_background.len(),
            &&self.layer_fail.len(),
            &&self.layer_pass.len(),
            &&self.layer_foreground.len(),
        )
    }
}
