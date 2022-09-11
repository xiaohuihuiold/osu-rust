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
