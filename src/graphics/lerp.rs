/// 根据t生成过渡对象
pub trait Lerp {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

/// 针对8位无符号整型进行lerp
impl Lerp for u8 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        todo!()
    }
}

/// 针对32位浮点进行lerp
impl Lerp for f32 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

/// 针对64位浮点进行lerp
impl Lerp for f64 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        a + (b - a) * (t as f64)
    }
}
