use crate::graphics::{Curve, Curves, LerpCopy};

/// 点
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// 点实现
impl Point {
    pub fn zero() -> Self {
        Point::new(0.0, 0.0)
    }

    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// 实现对[Point]的lerp
impl LerpCopy<Point> for Point {
    fn lerp_curve(a: Point, b: Point, t: f64, curve: Curves) -> Point {
        let t = Curve::apply(curve, t);
        Point::new(f64::lerp(a.x, b.x, t), f64::lerp(a.y, b.y, t))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_point_to_point() {
        assert_eq!(
            Point::lerp(Point::new(100.0, 100.0), Point::new(200.0, 200.0), 0.5,),
            Point::new(150.0, 150.0),
        );
        assert_eq!(
            Point::lerp(Point::new(100.0, 100.0), Point::new(200.0, 200.0), -0.5,),
            Point::new(50.0, 50.0),
        );
        assert_eq!(
            Point::lerp(Point::new(100.0, 100.0), Point::new(200.0, 200.0), 1.5,),
            Point::new(250.0, 250.0),
        );
    }
}
