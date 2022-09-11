use deserialize::from_str_enum;
use std::str::FromStr;

from_str_enum! {
    /// 锚点
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    pub enum Anchors {
        TopLeft,
        Centre,
        CentreLeft,
        TopRight,
        BottomCentre,
        TopCentre,
        CentreRight,
        BottomLeft,
        BottomRight,
    }
}

/// 锚点
#[derive(Copy, Clone, Debug)]
pub struct Anchor {
    x: f64,
    y: f64,
}

impl Anchor {
    const TOP_LEFT: Anchor = Anchor { x: 0.0, y: 0.0 };
    const TOP_CENTER: Anchor = Anchor { x: 0.5, y: 0.0 };
    const TOP_RIGHT: Anchor = Anchor { x: 1.0, y: 0.0 };
    const CENTER_LEFT: Anchor = Anchor { x: 0.0, y: 0.5 };
    const CENTER: Anchor = Anchor { x: 0.5, y: 0.5 };
    const CENTER_RIGHT: Anchor = Anchor { x: 1.0, y: 0.5 };
    const BOTTOM_LEFT: Anchor = Anchor { x: 0.0, y: 1.0 };
    const BOTTOM_CENTER: Anchor = Anchor { x: 0.5, y: 1.0 };
    const BOTTOM_RIGHT: Anchor = Anchor { x: 1.0, y: 1.0 };
}

impl From<Anchors> for Anchor {
    fn from(anchor: Anchors) -> Self {
        match anchor {
            Anchors::TopLeft => Anchor::TOP_LEFT,
            Anchors::Centre => Anchor::CENTER,
            Anchors::CentreLeft => Anchor::CENTER_LEFT,
            Anchors::TopRight => Anchor::TOP_RIGHT,
            Anchors::BottomCentre => Anchor::BOTTOM_CENTER,
            Anchors::TopCentre => Anchor::TOP_CENTER,
            Anchors::CentreRight => Anchor::CENTER_RIGHT,
            Anchors::BottomLeft => Anchor::BOTTOM_LEFT,
            Anchors::BottomRight => Anchor::BOTTOM_RIGHT,
        }
    }
}
