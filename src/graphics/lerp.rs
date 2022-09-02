/// 根据t生成过渡对象
pub trait Lerp {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self;
}

/// 适用于实现[Copy]的类型
pub trait LerpCopy {
    fn lerp(a: Self, b: Self, t: f64) -> Self;
}

/// 根据类型生成lerp宏
macro_rules! lerp_impl {
    ($($t:ty)+) => {$(
        impl LerpCopy for $t {
            fn lerp(a: Self, b: Self, t: f64) -> Self {
                lerp!(a, b, t, $t)
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
