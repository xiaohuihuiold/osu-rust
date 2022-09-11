use deserialize::from_str_enum_value;
use std::str::FromStr;

from_str_enum_value! {
    /// 曲线
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Curves {
        Linear=0,
    }
}

/// 动画曲线
pub struct Curve;

impl Curve {
    fn apply(curve: Curves, t: f64) -> f64 {
        match curve {
            Curves::Linear => t,
        }
    }
}
