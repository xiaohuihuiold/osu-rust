use crate::beatmap::beatmap_info::BeatmapInfo;
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

        while let Some(Ok(line)) = lines.next() {
            println!("{}", line);
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
