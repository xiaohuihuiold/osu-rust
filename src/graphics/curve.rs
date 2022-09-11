/// 动画曲线
pub trait Curve {
    const LINEAR: Linear = Linear {};

    fn apply(&self, t: f64) -> f64;
}

#[derive(Copy, Clone)]
pub struct Linear;

impl Curve for Linear {
    fn apply(&self, t: f64) -> f64 {
        t
    }
}
