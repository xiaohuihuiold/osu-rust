use crate::graphics::lerp::Lerp;
use sdl2::pixels::Color;

/// 实现对[Color]的lerp
impl Lerp for Color {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Color::RGBA(
            u8::lerp(&a.r, &b.r, t),
            u8::lerp(&a.g, &b.g, t),
            u8::lerp(&a.b, &b.b, t),
            u8::lerp(&a.a, &b.a, t),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::graphics::lerp::Lerp;
    use sdl2::pixels::Color;

    #[test]
    fn blue_to_red() {
        println!("{:?}", Color::lerp(&Color::BLUE, &Color::RED, 0.5));
    }
}
