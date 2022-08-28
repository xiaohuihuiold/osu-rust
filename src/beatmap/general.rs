/// 倒计时速度
#[derive(Debug)]
pub enum Countdown {
    None = 0,
    Normal = 1,
    Half = 2,
    Double = 3,
}

/// 默认音效组
#[derive(Debug)]
pub enum SampleSet {
    Normal,
    Soft,
    Drum,
}

/// 游戏模式
#[derive(Debug)]
pub enum GameMode {
    Osu = 0,
    Taiko = 1,
    Catch = 2,
    Mania = 3,
}

/// 皮肤覆盖层与数字层的关系
#[derive(Debug)]
pub enum OverlayPosition {
    NoChange,
    Below,
    Above,
}

/// 谱面信息
#[derive(Debug)]
pub struct General {
    pub audio_file_name: String,
    pub audio_lead_in: i32,
    pub preview_time: i32,
    pub countdown: Countdown,
    pub sample_set: SampleSet,
    pub stack_leniency: f32,
    pub mode: GameMode,
    pub letterbox_in_breaks: bool,
    pub use_skin_sprites: bool,
    pub overlay_position: OverlayPosition,
    pub skin_preference: Option<String>,
    pub epilepsy_warning: bool,
    pub countdown_offset: i32,
    pub spacial_style: bool,
    pub widescreen_storyboard: bool,
    pub samples_match_playback_rate: bool,
}

/// 默认值填充
impl Default for General {
    fn default() -> Self {
        General {
            audio_file_name: String::new(),
            audio_lead_in: 0,
            preview_time: -1,
            countdown: Countdown::None,
            sample_set: SampleSet::Normal,
            stack_leniency: 0.7,
            mode: GameMode::Osu,
            letterbox_in_breaks: false,
            use_skin_sprites: false,
            overlay_position: OverlayPosition::NoChange,
            skin_preference: None,
            epilepsy_warning: false,
            countdown_offset: 0,
            spacial_style: false,
            widescreen_storyboard: false,
            samples_match_playback_rate: false,
        }
    }
}
