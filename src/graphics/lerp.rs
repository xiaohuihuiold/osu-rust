use crate::graphics::{Curve, Curves};

/// 根据t生成过渡对象
pub trait Lerp<T> {
    fn lerp(a: &T, b: &T, t: f64) -> T {
        Self::lerp_curve(a, b, t, Curves::Linear)
    }

    fn lerp_curve(a: &T, b: &T, t: f64, curve: Curves) -> T;
}

pub trait LerpCopy<T>
where
    T: Copy,
{
    fn lerp(a: T, b: T, t: f64) -> T {
        Self::lerp_curve(a, b, t, Curves::Linear)
    }

    fn lerp_curve(a: T, b: T, t: f64, curve: Curves) -> T;
}

/// 根据类型生成lerp宏
macro_rules! lerp_impl {
    ($($t:ty)+) => {$(
        impl LerpCopy<$t> for $t{
            fn lerp_curve(a: $t, b: $t, t: f64, curve: Curves) -> $t {
                lerp!(a, b, Curve::apply(curve, t), $t)
            }
        }
    )+};
}

/// lerp宏
macro_rules! lerp {
    ($a:expr,$b:expr,$t:expr) => {
        lerp!($a, $b, $t, f64)
    };

    ($a:expr,$b:expr,$t:expr,$result_type:ty) => {{
        let a = $a as f64;
        let b = $b as f64;
        let t = $t as f64;
        (a + (b - a) * t) as $result_type
    }};
}

lerp_impl! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 f32 f64 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lerp_number() {
        assert_eq!(u8::lerp(4, 9, 0.5), 6);
        assert_eq!(u8::lerp(4, 9, -0.5), 1);
        assert_eq!(u8::lerp(4, 9, 1.5), 11);
        assert_eq!(u8::lerp(4, 255, 1.5), 255);
        assert_eq!(f64::lerp(4.0, 9.0, 0.5), 6.5);
        assert_eq!(f64::lerp(4.0, 9.0, -0.5), 1.5);
    }
}
