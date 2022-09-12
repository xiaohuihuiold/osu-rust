use crate::graphics::{Curve, Curves, LerpCopy};
use sdl2::pixels::Color;

/// 实现对[Color]的lerp
impl LerpCopy<Color> for Color {
    fn lerp_curve(a: Color, b: Color, t: f64, curve: Curves) -> Color {
        let t = Curve::apply(curve, t);
        Color::RGBA(
            u8::lerp(a.r, b.r, t),
            u8::lerp(a.g, b.g, t),
            u8::lerp(a.b, b.b, t),
            u8::lerp(a.a, b.a, t),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_blue_to_red() {
        assert_eq!(
            Color::lerp(Color::BLUE, Color::RED, 0.5),
            Color::RGB(127, 0, 127)
        );
        assert_eq!(
            Color::lerp(Color::BLUE, Color::RED, -0.5),
            Color::RGB(0, 0, 255)
        );
        assert_eq!(
            Color::lerp(Color::BLUE, Color::RED, 1.1),
            Color::RGB(255, 0, 0)
        );
    }
}
