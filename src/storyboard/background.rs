use crate::graphics::Point;
use crate::parser::Builder;

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

impl Background {
    pub const TYPE_IMAGE: &'static str = "0";
    pub const TYPE_VIDEO: &'static str = "1";
    pub const TYPE_VIDEO1: &'static str = "Video";
}

/// 默认值
impl Default for Background {
    fn default() -> Self {
        Self {
            background_type: BackgroundType::Image,
            begin_time: 0,
            file_path: None,
            position: Point::zero(),
        }
    }
}

/// 构造器
pub struct BackgroundBuilder {
    background: Background,
}

impl Builder<Vec<&str>, Background> for BackgroundBuilder {
    fn new() -> Self {
        Self {
            background: Background::default(),
        }
    }

    fn append(&mut self, input: &Vec<&str>) {
        let len = input.len();
        if len != 3 && len != 5 {
            return;
        }
        match input[0] {
            Background::TYPE_IMAGE => {
                self.background.background_type = BackgroundType::Image;
            }
            Background::TYPE_VIDEO | Background::TYPE_VIDEO1 => {
                self.background.background_type = BackgroundType::Video;
            }
            _ => {}
        }
        let time = input[1].parse().unwrap_or(0);
        self.background.begin_time = time;
        let path = input[2].trim_matches('"');
        self.background.file_path = Some(String::from(path));
        if len == 5 {
            let x = input[3].parse().unwrap_or(0.0);
            let y = input[4].parse().unwrap_or(0.0);
            self.background.position = Point::new(x, y);
        }
    }

    fn build(self) -> Background {
        self.background
    }
}
