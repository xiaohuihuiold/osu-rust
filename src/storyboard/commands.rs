use deserialize::from_str_enum;
use std::str::FromStr;

/// 通用参数
pub struct CommonCommand {}

/// 透明度命令
pub struct FadeCommand {}

/// 移动命令
pub struct MoveCommand {}
pub struct MoveXCommand {}
pub struct MoveYCommand {}

/// 缩放命令
pub struct ScaleCommand {}

/// 旋转命令
pub struct RotateCommand {}

/// 颜色命令
pub struct ColorCommand {}

from_str_enum! {
    /// 参数命令类型
    pub enum ParameterCommandType {
        H,
        V,
        A,
    }
}

/// 参数命令
pub struct ParameterCommand {}

/// 循环命令
pub struct LoopCommand {}

macro_rules! bind_type {
    ($($name:ident => $type_name:expr,)+) => {$(
        impl $name {
            pub const TYPE: &'static str = $type_name;
        })+
    };
}

bind_type! {
    FadeCommand => "F",
    MoveCommand => "M",
    MoveXCommand => "MX",
    MoveYCommand => "MY",
    ScaleCommand => "S",
    RotateCommand => "R",
    ColorCommand => "C",
    ParameterCommand => "P",
    LoopCommand => "L",
}
