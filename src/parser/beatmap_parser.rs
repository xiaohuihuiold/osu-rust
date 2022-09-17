use crate::beatmap::{BeatmapInfo, Difficulty, General, Metadata};
use crate::storyboard::{StoryboardBuilder, StoryboardInfo};
use deserialize::DeserializeJson;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// 解析器
pub trait Parser<I, R> {
    type Err;
    fn parse(input: I) -> Result<R, Self::Err>;
}

/// 构造器
pub trait Builder<I, R> {
    fn new() -> Self;
    fn append(&mut self, input: &I);
    fn build(self) -> R;
}

/// builder默认实现宏
macro_rules! impl_builder {
    ($($name:ident => $builder_name:ident,)+) => {$(
        struct $builder_name {
            data: HashMap<String, String>,
        }
        impl Builder<String, $name> for $builder_name {

            fn new() -> Self {
                Self {
                    data: HashMap::new(),
                }
            }

            fn append(&mut self, input: &String) {
                if let Some(index) = input.find(':') {
                    let left = &input[..index];
                    let right = &input[index + 1..];
                    self.data.insert(
                        String::from(left).trim().into(),
                        String::from(right).trim().into(),
                    );
                }
            }

            fn build(self) -> $name {
                <$name>::from_json(&self.data)
            }
        })+
    };
}

impl_builder! {
    General => GeneralBuilder,
    Metadata => MetadataBuilder,
    Difficulty => DifficultyBuilder,
}

/// 解析构造枚举
enum InfoBuilder {
    None,
    First,
    General(GeneralBuilder),
    Metadata(MetadataBuilder),
    Difficulty(DifficultyBuilder),
    Storyboard(StoryboardBuilder),
}

/// 谱面解析
impl Parser<&str, BeatmapInfo> for BeatmapInfo {
    type Err = String;

    fn parse(input: &str) -> Result<BeatmapInfo, Self::Err> {
        let path = Path::new(input);
        // 查找osb文件
        let parent = match path.parent() {
            Some(parent) => parent,
            None => return Err("父目录查找失败".into()),
        };
        let mut dirs = match parent.read_dir() {
            Ok(dirs) => dirs,
            Err(_) => return Err("文件查找失败".into()),
        };
        let osb = dirs.find_map(|e| match e {
            Ok(dir) => {
                let name = dir.file_name();
                let name = name.to_str().unwrap_or("");
                if name.to_lowercase().ends_with(".osb") {
                    Some(File::open(dir.path()))
                } else {
                    None
                }
            }
            Err(_) => None,
        });
        let osb_file = match osb {
            None => Err(()),
            Some(osb) => match osb {
                Ok(file) => Ok(file),
                Err(_) => Err(()),
            },
        };

        // 打开osu文件
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return Err("文件打开失败".into()),
        };
        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();

        // 打开osb文件
        let osb_lines = &mut match osb_file {
            Ok(osb_file) => {
                let buf_reader = BufReader::new(osb_file);
                Some(buf_reader.lines())
            }
            Err(_) => None,
        };

        let mut beatmap_info = BeatmapInfo::default();
        let section_regex = Regex::new(r"^\[(.*)\]$").unwrap();
        let mut builder = InfoBuilder::First;

        loop {
            let line = if let Some(Ok(line)) = lines.next() {
                line
            } else if let Some(lines) = osb_lines {
                if let Some(Ok(line)) = lines.next() {
                    line
                } else {
                    break;
                }
            } else {
                break;
            };
            let mut line = line.trim_end();
            if line.starts_with('[') {
                line = line.trim_start();
            }
            // 跳过空白行
            if line.is_empty() {
                continue;
            }
            // 遇到新的section时保存上一个
            if section_regex.is_match(line) {
                build(&mut beatmap_info, builder);
                builder = InfoBuilder::None;
            }

            // 生成构造器
            match line {
                General::SECTION_NAME => {
                    // 一般信息
                    builder = InfoBuilder::General(GeneralBuilder::new());
                }
                Metadata::SECTION_NAME => {
                    // 元数据
                    builder = InfoBuilder::Metadata(MetadataBuilder::new());
                }
                Difficulty::SECTION_NAME => {
                    // 难度
                    builder = InfoBuilder::Difficulty(DifficultyBuilder::new());
                }
                StoryboardInfo::SECTION_NAME => {
                    // 事件
                    let storyboard = beatmap_info.storyboard;
                    beatmap_info.storyboard = None;
                    if let Some(storyboard) = storyboard {
                        builder = InfoBuilder::Storyboard(StoryboardBuilder::from(storyboard));
                    } else {
                        builder = InfoBuilder::Storyboard(StoryboardBuilder::new());
                    }
                }
                _ => {
                    // 跳过未知section
                    if section_regex.is_match(line) {
                        continue;
                    }
                    // 为当前builder追加数据
                    match &mut builder {
                        InfoBuilder::General(builder) => {
                            builder.append(&String::from(line));
                        }
                        InfoBuilder::Metadata(builder) => {
                            builder.append(&String::from(line));
                        }
                        InfoBuilder::Difficulty(builder) => {
                            builder.append(&String::from(line));
                        }
                        InfoBuilder::Storyboard(builder) => {
                            builder.append(&String::from(line));
                        }
                        InfoBuilder::First => {
                            beatmap_info.format = Some(String::from(line));
                        }
                        InfoBuilder::None => {}
                    }
                }
            }
        }

        // 生成最后一次数据
        build(&mut beatmap_info, builder);

        Ok(beatmap_info)
    }
}

fn build(beatmap_info: &mut BeatmapInfo, builder: InfoBuilder) {
    match builder {
        InfoBuilder::General(builder) => {
            beatmap_info.general = Some(builder.build());
        }
        InfoBuilder::Metadata(builder) => {
            beatmap_info.metadata = Some(builder.build());
        }
        InfoBuilder::Difficulty(builder) => {
            beatmap_info.difficulty = Some(builder.build());
        }
        InfoBuilder::Storyboard(builder) => {
            beatmap_info.storyboard = Some(builder.build());
        }
        InfoBuilder::None | InfoBuilder::First => {}
    }
}

#[cfg(test)]
mod tests {
    const TEST_PATH: &str = "temp/large/temp.osu";

    use super::*;

    #[test]
    fn parse_from_path() {
        let info = BeatmapInfo::parse(TEST_PATH);
        println!("{:#?}", info);
        assert!(info.is_ok());
    }
}
