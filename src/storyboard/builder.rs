use crate::parser::Builder;
use crate::storyboard::StoryboardInfo;

/// 构造器
pub struct StoryboardBuilder {}

/// 实现构造器
impl StoryboardBuilder {
    pub fn new() -> StoryboardBuilder {
        Self {}
    }
}

impl Builder<String, StoryboardInfo> for StoryboardBuilder {
    fn append(&mut self, input: &String) {
        todo!()
    }

    fn build(&self) -> StoryboardInfo {
        StoryboardInfo::default()
    }
}
