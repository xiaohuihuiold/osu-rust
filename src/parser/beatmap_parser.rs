use crate::beatmap::BeatmapInfo;
use crate::beatmap::{difficulty, general, metadata};
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// 解析器
pub trait Parser<R, I> {
    fn parse(input: I) -> Result<R, String>;
}

/// 谱面解析
impl Parser<BeatmapInfo, &str> for BeatmapInfo {
    fn parse(input: &str) -> Result<BeatmapInfo, String> {
        let file = match File::open(input) {
            Ok(file) => file,
            Err(_) => return Err("文件打开失败".into()),
        };

        let buf_reader = BufReader::new(file);
        let mut lines = buf_reader.lines();
        let beatmap_info = BeatmapInfo::default();
        let section_regex = Regex::new(r"^\[(.*)\]$").unwrap();

        while let Some(Ok(line)) = lines.next() {
            let mut line = line.trim_end();
            if line.starts_with('[') {
                line = line.trim_start();
            }
            // 跳过空白行
            if line.is_empty() {
                continue;
            }

            match line {
                general::SECTION_NAME => {
                    // 一般信息
                }
                metadata::SECTION_NAME => {
                    // 元数据
                }
                difficulty::SECTION_NAME => {
                    // 难度
                }
                _ => {
                    if section_regex.is_match(line) {
                        // 未知的section
                    } else {
                        // 普通数据
                        todo!();
                    }
                }
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
        assert!(info.is_ok());
    }
}
