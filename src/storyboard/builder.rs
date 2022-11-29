use crate::parser::Builder;
use crate::storyboard::sprite::SpriteBuilder;
use crate::storyboard::{
    Background, BackgroundBuilder, BackgroundType, ColorCommand, FadeCommand, LoopCommand,
    MoveCommand, MoveXCommand, MoveYCommand, ParameterCommand, RotateCommand, ScaleCommand, Sprite,
    StoryboardInfo,
};

/// 构造器
pub struct StoryboardBuilder {
    storyboard: StoryboardInfo,
}

/// 实现构造器
impl From<StoryboardInfo> for StoryboardBuilder {
    fn from(storyboard: StoryboardInfo) -> Self {
        Self { storyboard }
    }
}

impl Builder<String, StoryboardInfo> for StoryboardBuilder {
    fn new() -> Self {
        Self {
            storyboard: StoryboardInfo::default(),
        }
    }

    fn append(&mut self, input: &String) {
        if input.is_empty() || input.starts_with("//") {
            return;
        }
        let params: Vec<&str> = input.split(',').collect();
        let ty = params[0];
        if ty.is_empty() {
            return;
        }
        match ty {
            Sprite::TYPE => {
                let mut builder = SpriteBuilder::new();
                builder.append(&params);
                self.storyboard.add_sprite(builder.build());
            }
            FadeCommand::TYPE => {}
            MoveCommand::TYPE => {}
            MoveXCommand::TYPE => {}
            MoveYCommand::TYPE => {}
            ScaleCommand::TYPE => {}
            RotateCommand::TYPE => {}
            ColorCommand::TYPE => {}
            ParameterCommand::TYPE => {}
            LoopCommand::TYPE => {}
            Background::TYPE_IMAGE | Background::TYPE_VIDEO | Background::TYPE_VIDEO1 => {
                // 背景图和视频
                let mut builder = BackgroundBuilder::new();
                builder.append(&params);
                let background = builder.build();
                match background.background_type {
                    BackgroundType::Image => {
                        self.storyboard.background_image = Some(background);
                    }
                    BackgroundType::Video => {
                        self.storyboard.background_video = Some(background);
                    }
                }
            }
            _ => {}
        }
    }

    fn build(self) -> StoryboardInfo {
        self.storyboard
    }
}
