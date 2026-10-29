#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FlexDir {
    Row,
    RowReverse,
    #[default]
    Column,
    ColumnReverse,
}

impl FlexDir {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "row" | "horizontal" => Self::Row,
            "row-reverse" | "row_reverse" => Self::RowReverse,
            "column" | "col" | "vertical" => Self::Column,
            "column-reverse" | "column_reverse" | "col-reverse" => Self::ColumnReverse,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Justify {
    #[default]
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Justify {
    pub fn parse(s: &str) -> Option<Self> {
        Some(
            match s.trim().to_ascii_lowercase().replace('_', "-").as_str() {
                "start" | "flex-start" | "left" => Self::Start,
                "center" | "middle" => Self::Center,
                "end" | "flex-end" | "right" => Self::End,
                "space-between" => Self::SpaceBetween,
                "space-around" => Self::SpaceAround,
                "space-evenly" => Self::SpaceEvenly,
                _ => return None,
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    #[default]
    Stretch,
    Start,
    Center,
    End,
}

impl Align {
    pub fn parse(s: &str) -> Option<Self> {
        Some(
            match s.trim().to_ascii_lowercase().replace('_', "-").as_str() {
                "start" | "flex-start" => Self::Start,
                "center" | "middle" => Self::Center,
                "end" | "flex-end" => Self::End,
                "stretch" | "normal" => Self::Stretch,
                _ => return None,
            },
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    #[default]
    Relative,
    Absolute,
}

impl Position {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "relative" | "static" => Self::Relative,
            "absolute" => Self::Absolute,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Flex,
    None,
}

impl Display {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "flex" | "block" | "inline" | "inline-flex" | "true" => Self::Flex,
            "none" | "hidden" | "false" => Self::None,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
}

impl Overflow {
    pub fn parse(s: &str) -> Option<Self> {
        Some(match s.trim().to_ascii_lowercase().as_str() {
            "hidden" | "clip" | "scroll" | "auto" => Self::Hidden,
            "visible" => Self::Visible,
            _ => return None,
        })
    }
}
