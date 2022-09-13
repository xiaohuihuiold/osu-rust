use crate::graphics::{Anchor, Point};
use crate::storyboard::Layer;

/// Sprite对象
#[derive(Debug)]
pub struct Sprite {
    layer: Layer,
    anchor: Anchor,
    file_path: Option<String>,
    position: Point,
}
