use deserialize::from_str_enum;
use std::f64::consts::PI;
use std::str::FromStr;

from_str_enum! {
    /// 曲线
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Curves {
        None = 0,
        Out = 1,
        In = 2,
        InQuad = 3,
        OutQuad = 4,
        InOutQuad = 5,
        InCubic = 6,
        OutCubic = 7,
        InOutCubic = 8,
        InQuart = 9,
        OutQuart = 10,
        InOutQuart = 11,
        InQuint = 12,
        OutQuint = 13,
        InOutQuint = 14,
        InSine = 15,
        OutSine = 16,
        InOutSine = 17,
        InExpo = 18,
        OutExpo = 19,
        InOutExpo = 20,
        InCirc = 21,
        OutCirc = 22,
        InOutCirc = 23,
        InElastic = 24,
        OutElastic = 25,
        OutElasticHalf = 26,
        OutElasticQuarter = 27,
        InOutElastic = 28,
        InBack = 29,
        OutBack = 30,
        InOutBack = 31,
        InBounce = 32,
        OutBounce = 33,
        InOutBounce = 34,
        OutPow10 = 35,
    }
}

const ELASTIC_CONST: f64 = 2.0 * PI / 0.3;
const ELASTIC_CONST2: f64 = 0.3 / 4.0;

const BACK_CONST: f64 = 1.70158;
const BACK_CONST2: f64 = BACK_CONST * 1.525;

const BOUNCE_CONST: f64 = 1.0 / 2.75;

const EXPO_OFFSET: f64 = 2.0f64.powi(-10);
const ELASTIC_OFFSET_FULL: f64 = 2.0f64.powi(-11);
const ELASTIC_OFFSET_HALF: f64 = 2.0f64.powi(-10) * ((0.5 - ELASTIC_CONST2) * ELASTIC_CONST).sin();
const ELASTIC_OFFSET_QUARTER: f64 =
    2.0f64.powi(-10) * ((0.25 - ELASTIC_CONST2) * ELASTIC_CONST).sin();
const IN_OUT_ELASTIC_OFFSET: f64 =
    2.0f64.powi(-10) * ((1.0 - ELASTIC_CONST2 * 1.5) * ELASTIC_CONST / 1.5).sin();

/// 动画曲线
pub struct Curve;

impl Curve {
    /// 参考来源: [DefaultEasingFunction.cs](https://github.com/ppy/osu-framework/blob/master/osu.Framework/Graphics/Transforms/DefaultEasingFunction.cs)
    pub fn apply(curve: Curves, mut t: f64) -> f64 {
        match curve {
            Curves::In | Curves::InQuad => t * t,
            Curves::Out | Curves::OutQuad => t * (2.0 - t),
            Curves::InOutQuad => {
                if t < 0.5 {
                    t * t * 2.0
                } else {
                    t -= 1.0;
                    t * t * -2.0 + 1.0
                }
            }
            Curves::InCubic => t * t * t,
            Curves::OutCubic => {
                t -= 1.0;
                t * t * t + 1.0
            }
            Curves::InOutCubic => {
                if t < 0.5 {
                    t * t * t * 4.0
                } else {
                    t -= 1.0;
                    t * t * t * 4.0 + 1.0
                }
            }
            Curves::InQuart => t * t * t * t,
            Curves::OutQuart => {
                t -= 1.0;
                1.0 - t * t * t * t
            }
            Curves::InOutQuart => {
                if t < 0.5 {
                    t * t * t * t * 8.0
                } else {
                    t -= 1.0;
                    t * t * t * t * -8.0 + 1.0
                }
            }
            Curves::InQuint => t * t * t * t * t,
            Curves::OutQuint => {
                t -= 1.0;
                t * t * t * t * t + 1.0
            }
            Curves::InOutQuint => {
                if t < 0.5 {
                    t * t * t * t * t * 16.0
                } else {
                    t -= 1.0;
                    t * t * t * t * t * 16.0 + 1.0
                }
            }
            Curves::InSine => 1.0 - (t * PI * 0.5).cos(),
            Curves::OutSine => (t * PI * 0.5).sin(),
            Curves::InOutSine => 0.5 - 0.5 * (PI * t).cos(),
            Curves::InExpo => 2.0f64.powf(10.0 * (t - 1.0)) + EXPO_OFFSET * (t - 1.0),
            Curves::OutExpo => -(2.0f64.powf(-10.0 * t)) + 1.0 + EXPO_OFFSET * t,
            Curves::InOutExpo => {
                if t < 0.5 {
                    0.5 * (2.0f64.powf(20.0 * t - 10.0) + EXPO_OFFSET * (2.0 * t - 1.0))
                } else {
                    1.0 - 0.5 * (2.0f64.powf(-20.0 * t + 10.0) + EXPO_OFFSET * (-2.0 * t + 1.0))
                }
            }
            Curves::InCirc => 1.0 - (1.0 - t * t).sqrt(),
            Curves::OutCirc => {
                t -= 1.0;
                (1.0 - t * t).sqrt()
            }
            Curves::InOutCirc => {
                t *= 2.0;
                if t < 1.0 {
                    0.5 - 0.5 * (1.0 - t * t).sqrt()
                } else {
                    t -= 2.0;
                    0.5 * (1.0 - t * t).sqrt() + 0.5
                }
            }
            Curves::InElastic => {
                -(2.0f64.powf(-10.0 + 10.0 * t))
                    * ((1.0 - ELASTIC_CONST2 - t) * ELASTIC_CONST).sin()
                    + ELASTIC_OFFSET_FULL * (1.0 - t)
            }
            Curves::OutElastic => {
                2.0f64.powf(-10.0 * t) * ((t - ELASTIC_CONST2) * ELASTIC_CONST).sin() + 1.0
                    - ELASTIC_OFFSET_FULL * t
            }
            Curves::OutElasticHalf => {
                2.0f64.powf(-10.0 * t) * ((0.5 * t - ELASTIC_CONST2) * ELASTIC_CONST).sin() + 1.0
                    - ELASTIC_OFFSET_HALF * t
            }
            Curves::OutElasticQuarter => {
                2.0f64.powf(-10.0 * t) * ((0.25 * t - ELASTIC_CONST2) * ELASTIC_CONST).sin() + 1.0
                    - ELASTIC_OFFSET_QUARTER * t
            }
            Curves::InOutElastic => {
                t *= 2.0;
                if t < 1.0 {
                    -0.5 * (2.0f64.powf(-10.0 + 10.0 * t)
                        * ((1.0 - ELASTIC_CONST2 * 1.5 - t) * ELASTIC_CONST / 1.5).sin()
                        - IN_OUT_ELASTIC_OFFSET * (1.0 - t))
                } else {
                    t -= 1.0;
                    0.5 * (2.0f64.powf(-10.0 * t)
                        * ((t - ELASTIC_CONST2 * 1.5) * ELASTIC_CONST / 1.5).sin()
                        - IN_OUT_ELASTIC_OFFSET * t)
                        + 1.0
                }
            }
            Curves::InBack => t * t * ((BACK_CONST + 1.0) * t - BACK_CONST),
            Curves::OutBack => {
                t -= 1.0;
                t * t * ((BACK_CONST + 1.0) * t + BACK_CONST) + 1.0
            }
            Curves::InOutBack => {
                t *= 2.0;
                if t < 1.0 {
                    0.5 * t * t * ((BACK_CONST2 + 1.0) * t - BACK_CONST2)
                } else {
                    t -= 2.0;
                    0.5 * (t * t * ((BACK_CONST2 + 1.0) * t + BACK_CONST2) + 2.0)
                }
            }
            Curves::InBounce => {
                t = 1.0 - t;
                if t < BOUNCE_CONST {
                    1.0 - 7.5625 * t * t
                } else if t < 2.0 * BOUNCE_CONST {
                    t -= 1.5 * BOUNCE_CONST;
                    1.0 - (7.5625 * t * t + 0.75)
                } else if t < 2.5 * BOUNCE_CONST {
                    t -= 2.25 * BOUNCE_CONST;
                    1.0 - (7.5625 * t * t + 0.9375)
                } else {
                    t -= 2.625 * BOUNCE_CONST;
                    1.0 - (7.5625 * t * t + 0.984375)
                }
            }
            Curves::OutBounce => {
                if t < BOUNCE_CONST {
                    7.5625 * t * t
                } else if t < 2.0 * BOUNCE_CONST {
                    t -= 1.5 * BOUNCE_CONST;
                    7.5625 * t * t + 0.75
                } else if t < 2.5 * BOUNCE_CONST {
                    t -= 2.25 * BOUNCE_CONST;
                    7.5625 * t * t + 0.9375
                } else {
                    t -= 2.625 * BOUNCE_CONST;
                    7.5625 * t * t + 0.984375
                }
            }
            Curves::InOutBounce => {
                if t < 0.5 {
                    0.5 - 0.5 * Self::apply(Curves::OutBounce, 1.0 - t * 2.0)
                } else {
                    Self::apply(Curves::OutBounce, (t - 0.5) * 2.0) * 0.5 + 0.5
                }
            }
            Curves::OutPow10 => {
                t -= 1.0;
                t * t.powf(10.0) + 1.0
            }
            _ => t,
        }
    }
}
