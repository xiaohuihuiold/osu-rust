use crate::graphics::Anchor;
use crate::storyboard::Layer;
use sdl2::rect::Point;

/// Sprite对象
#[derive(Debug)]
pub struct Sprite {
    layer: Layer,
    anchor: Anchor,
    path: String,
    position: Point,
}
