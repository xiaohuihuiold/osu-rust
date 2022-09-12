use deserialize::from_str_enum;
use std::str::FromStr;

from_str_enum! {
    /// 曲线
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Curves {
        Linear = 0,
    }
}

/// 动画曲线
pub struct Curve;

impl Curve {
    pub fn apply(curve: Curves, t: f64) -> f64 {
        match curve {
            Curves::Linear => t,
        }
    }
}
