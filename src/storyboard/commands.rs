use deserialize::from_str_enum;
use std::str::FromStr;

/// 透明度命令
pub struct FadeCommand {}

/// 移动命令
pub struct MoveCommand {}

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
