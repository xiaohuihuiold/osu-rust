use deserialize::from_str_enum;
use std::str::FromStr;

from_str_enum! {
    /// 层
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Layer {
        Background,
        Fail,
        Pass,
        Foreground,
    }
}
