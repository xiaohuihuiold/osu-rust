use crate::graphics::{Color, Curves, Point};
use deserialize::from_str_enum;
use std::str::FromStr;

from_str_enum! {
    /// 参数命令类型
    pub enum ParameterCommandType {
        H,
        V,
        A,
    }
}

/// 通用参数
pub struct CommonCommand {
    easing: Curves,
    start_time: i64,
    end_time: i64,
}

/// 透明度命令
pub struct FadeCommand {
    common: CommonCommand,
    start: f64,
    end: f64,
}

/// 移动命令
pub struct MoveCommand {
    common: CommonCommand,
    start: Point,
    end: Point,
}
pub struct MoveXCommand {
    common: CommonCommand,
    start: i64,
    end: i64,
}
pub struct MoveYCommand {
    common: CommonCommand,
    start: i64,
    end: i64,
}

/// 缩放命令
pub struct ScaleCommand {
    common: CommonCommand,
    start: f64,
    end: f64,
}

/// 矢量缩放命令
pub struct ScaleVectorCommand {
    common: CommonCommand,
    start: Point,
    end: Point,
}

/// 旋转命令
pub struct RotateCommand {
    common: CommonCommand,
    start: f64,
    end: f64,
}

/// 颜色命令
pub struct ColorCommand {
    common: CommonCommand,
    start: Color,
    end: Color,
}

/// 参数命令
pub struct ParameterCommand {
    common: CommonCommand,
    parameter: ParameterCommandType,
}

/// 循环命令
pub struct LoopCommand {
    start_time: i64,
    loop_count: i64,
}

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
    ScaleVectorCommand => "V",
    RotateCommand => "R",
    ColorCommand => "C",
    ParameterCommand => "P",
    LoopCommand => "L",
}
