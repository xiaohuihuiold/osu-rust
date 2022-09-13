use crate::graphics::Point;

/// 背景类型
#[derive(Copy, Clone, Debug)]
pub enum BackgroundType {
    Image = 0,
    Video = 1,
}

/// 背景
#[derive(Debug)]
pub struct Background {
    pub background_type: BackgroundType,
    pub begin_time: i64,
    pub file_path: Option<String>,
    pub position: Point,
}
