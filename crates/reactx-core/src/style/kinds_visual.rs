#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderKind {
    Single,
    Rounded,
    Double,
    Thick,
    #[default]
    None,
}

impl BorderKind {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "single" | "plain" | "true" | "1" => Self::Single,
            "rounded" | "round" => Self::Rounded,
            "double" => Self::Double,
            "thick" | "bold" => Self::Thick,
            "none" | "false" | "0" | "" => Self::None,
            _ => return None,
        })
    }

    pub fn is_visible(self) -> bool {
        !matches!(self, Self::None)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TitleAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl TitleAlign {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "left" | "start" => Self::Left,
            "center" | "middle" => Self::Center,
            "right" | "end" => Self::Right,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl TextAlign {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "left" | "start" => Self::Left,
            "center" | "middle" => Self::Center,
            "right" | "end" => Self::Right,
            _ => return None,
        })
    }
}

/// A terminal dimension in cells, percent or automatic.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Dim {
    #[default]
    Auto,
    Cells(f32),
    Pct(f32),
}

impl Dim {
    pub fn parse(s: &str) -> Option<Self> {
        let s = s.trim();
        if s.is_empty() || s.eq_ignore_ascii_case("auto") || s.eq_ignore_ascii_case("content") {
            return Some(Self::Auto);
        }
        if let Some(pct) = s.strip_suffix('%') {
            let value = pct.trim().parse::<f32>().ok()?;
            return Some(Self::Pct(value.clamp(0.0, 100.0)));
        }
        let value = s.parse::<f32>().ok()?;
        if value.is_finite() && value >= 0.0 {
            Some(Self::Cells(value))
        } else {
            Some(Self::Auto)
        }
    }
}

/// Four cell edges (top, right, bottom, left).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Edges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

impl Edges {
    pub fn uniform(v: f32) -> Self {
        Self {
            top: v,
            right: v,
            bottom: v,
            left: v,
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let values: Vec<f32> = s
            .split(|c: char| c == ',' || c.is_whitespace())
            .filter(|part| !part.is_empty())
            .filter_map(|part| part.trim().parse::<f32>().ok())
            .collect();
        match values.as_slice() {
            [all] => Some(Self::uniform(*all)),
            [vertical, horizontal] => Some(Self {
                top: *vertical,
                right: *horizontal,
                bottom: *vertical,
                left: *horizontal,
            }),
            [top, horizontal, bottom] => Some(Self {
                top: *top,
                right: *horizontal,
                bottom: *bottom,
                left: *horizontal,
            }),
            [top, right, bottom, left] => Some(Self {
                top: *top,
                right: *right,
                bottom: *bottom,
                left: *left,
            }),
            _ => None,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.top == 0.0 && self.right == 0.0 && self.bottom == 0.0 && self.left == 0.0
    }
}
