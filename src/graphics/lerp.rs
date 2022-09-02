/// 根据t生成过渡对象
pub trait Lerp {
    fn lerp(a: &Self, b: &Self, t: f64) -> Self;
}

pub trait LerpCopy {
    fn lerp(a: Self, b: Self, t: f64) -> Self;
}

macro_rules! lerp_impl {
    ($($t:ty) +) => {$(
        impl LerpCopy for $t {
            fn lerp(a: Self, b: Self, t: f64) -> Self {
                lerp!(a, b, t, $t)
            }
        }
    )+};
}

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
