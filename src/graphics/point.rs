use crate::graphics::lerp::LerpCopy;
use sdl2::rect::Point;

/// 实现对[Point]的lerp
impl LerpCopy for Point {
    fn lerp(a: Self, b: Self, t: f64) -> Self {
        Point::new(i32::lerp(a.x(), b.x(), t), i32::lerp(a.y(), b.y(), t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_point_to_point() {
        assert_eq!(
            Point::lerp(Point::new(100, 100), Point::new(200, 200), 0.5,),
            Point::new(150, 150),
        );
        assert_eq!(
            Point::lerp(Point::new(100, 100), Point::new(200, 200), -0.5,),
            Point::new(50, 50),
        );
        assert_eq!(
            Point::lerp(Point::new(100, 100), Point::new(200, 200), 1.5,),
            Point::new(250, 250),
        );
    }
}
