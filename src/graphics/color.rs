use crate::graphics::{Curve, Curves, LerpCopy};

/// 颜色
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// 颜色实现
impl Color {
    pub const RED: Color = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };
    pub const GREEN: Color = Color {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };
    pub const BLUE: Color = Color {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
}

/// 实现对[Color]的lerp
impl LerpCopy<Color> for Color {
    fn lerp_curve(a: Color, b: Color, t: f64, curve: Curves) -> Color {
        let t = Curve::apply(curve, t);
        Self::rgba(
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
            Color::rgb(127, 0, 127)
        );
        assert_eq!(
            Color::lerp(Color::BLUE, Color::RED, -0.5),
            Color::rgb(0, 0, 255)
        );
        assert_eq!(
            Color::lerp(Color::BLUE, Color::RED, 1.1),
            Color::rgb(255, 0, 0)
        );
    }
}
