use crate::beatmap::{difficulty, general, metadata, Difficulty, Metadata};
use crate::beatmap::{BeatmapInfo, General};
use deserialize::DeserializeJson;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// 解析器
pub trait Parser<I, R> {
    type Err;
    fn parse(input: I) -> Result<R, Self::Err>;
}

/// 构造器
pub trait Builder<I, R> {
    fn append(&mut self, input: &I);
    fn build(&self) -> R;
}

struct GeneralBuilder {
    data: HashMap<String, String>,
}

impl GeneralBuilder {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
struct MetadataBuilder {
    data: HashMap<String, String>,
}

impl MetadataBuilder {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
struct DifficultyBuilder {
    data: HashMap<String, String>,
}

impl DifficultyBuilder {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Builder<String, General> for GeneralBuilder {
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

    fn build(&self) -> General {
        General::from_json(&self.data)
    }
}
impl Builder<String, Metadata> for MetadataBuilder {
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

    fn build(&self) -> Metadata {
        Metadata::from_json(&self.data)
    }
}
impl Builder<String, Difficulty> for DifficultyBuilder {
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

    fn build(&self) -> Difficulty {
        Difficulty::from_json(&self.data)
    }
}

/// 解析构造枚举
enum InfoBuilder {
    None,
    General(GeneralBuilder),
    Metadata(MetadataBuilder),
    Difficulty(DifficultyBuilder),
}

/// 谱面解析
impl Parser<&str, BeatmapInfo> for BeatmapInfo {
    type Err = String;

    fn parse(input: &str) -> Result<BeatmapInfo, Self::Err> {
        let file = match File::open(input) {
            Ok(file) => file,
            Err(_) => return Err("文件打开失败".into()),
        };

        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();
        let mut beatmap_info = BeatmapInfo::default();
        let section_regex = Regex::new(r"^\[(.*)\]$").unwrap();

        let mut builder = InfoBuilder::None;

        while let Some(Ok(line)) = lines.next() {
            let mut line = line.trim_end();
            if line.starts_with('[') {
                line = line.trim_start();
            }
            // 跳过空白行
            if line.is_empty() {
                continue;
            }
            if section_regex.is_match(line) {
                match &builder {
                    InfoBuilder::None => {}
                    InfoBuilder::General(builder) => {
                        beatmap_info.general = Some(builder.build());
                    }
                    InfoBuilder::Metadata(builder) => {
                        beatmap_info.metadata = Some(builder.build());
                    }
                    InfoBuilder::Difficulty(builder) => {
                        beatmap_info.difficulty = Some(builder.build());
                    }
                }
                // 未知的section
                builder = InfoBuilder::None;
            }
            match line {
                general::SECTION_NAME => {
                    // 一般信息
                    builder = InfoBuilder::General(GeneralBuilder::new());
                }
                metadata::SECTION_NAME => {
                    // 元数据
                    builder = InfoBuilder::Metadata(MetadataBuilder::new());
                }
                difficulty::SECTION_NAME => {
                    // 难度
                    builder = InfoBuilder::Difficulty(DifficultyBuilder::new());
                }
                _ => {
                    if !section_regex.is_match(line) {
                        // 普通数据
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
                            InfoBuilder::None => {}
                        }
                    }
                }
            }
        }
        match &builder {
            InfoBuilder::None => {}
            InfoBuilder::General(builder) => {
                beatmap_info.general = Some(builder.build());
            }
            InfoBuilder::Metadata(builder) => {
                beatmap_info.metadata = Some(builder.build());
            }
            InfoBuilder::Difficulty(builder) => {
                beatmap_info.difficulty = Some(builder.build());
            }
        }
        Ok(beatmap_info)
    }
}

#[cfg(test)]
mod tests {
    const TEST_PATH: &str = "temp/temp/temp.osu";

    use super::*;

    #[test]
    fn parse_from_path() {
        let info = BeatmapInfo::parse(TEST_PATH);
        println!("{:#?}", info);
        assert!(info.is_ok());
    }
}
