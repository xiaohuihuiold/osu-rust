use crate::graphics::{Anchor, Anchors, Point};
use crate::parser::Builder;
use crate::storyboard::Layer;
use std::str::FromStr;

/// Sprite对象
#[derive(Debug)]
pub struct Sprite {
    pub layer: Layer,
    pub anchor: Anchor,
    pub file_path: Option<String>,
    pub position: Point,
}

impl Sprite {
    pub const TYPE: &'static str = "Sprite";
}

/// 默认值
impl Default for Sprite {
    fn default() -> Self {
        Self {
            layer: Layer::Background,
            anchor: Anchor::from(Anchors::TopLeft),
            file_path: None,
            position: Point::zero(),
        }
    }
}

/// 构造器
pub struct SpriteBuilder {
    sprite: Sprite,
}

impl Builder<Vec<&str>, Sprite> for SpriteBuilder {
    fn new() -> Self {
        Self {
            sprite: Sprite::default(),
        }
    }

    fn append(&mut self, input: &Vec<&str>) {
        let len = input.len();
        self.sprite.layer = Layer::from_str(input[1]).unwrap_or(self.sprite.layer);
        let anchor = Anchors::from_str(input[2]).unwrap_or(Anchors::TopLeft);
        self.sprite.anchor = anchor.into();
        let path = input[3].trim_matches('"');
        self.sprite.file_path = Some(String::from(path));
        if len == 6 {
            let x = input[4].parse().unwrap_or(0.0);
            let y = input[5].parse().unwrap_or(0.0);
            self.sprite.position = Point::new(x, y);
        }
    }

    fn build(self) -> Sprite {
        self.sprite
    }
}
