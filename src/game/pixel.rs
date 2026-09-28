use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Pixel {
    Apple,
    Head,
    Skin,
    Empty,
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Apple => "🍎",
            Self::Head => "🟥",
            Self::Skin => "🟩",
            Self::Empty => "  ",
        };
        f.write_str(s)
    }
}