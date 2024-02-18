use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
    Keyboard,
    Virtual,
    Other(String),
}

impl std::fmt::Display for PointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mouse => f.write_str("mouse"),
            Self::Pen => f.write_str("pen"),
            Self::Touch => f.write_str("touch"),
            Self::Keyboard => f.write_str("keyboard"),
            Self::Virtual => f.write_str("virtual"),
            Self::Other(other) => f.write_str(other.as_str()),
        }
    }
}

impl From<String> for PointerType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "mouse" => PointerType::Mouse,
            "touch" => PointerType::Touch,
            "pen" => PointerType::Pen,
            _other => PointerType::Other(value),
        }
    }
}

impl FromStr for PointerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mouse" => Ok(Self::Mouse),
            "touch" => Ok(Self::Touch),
            "pen" => Ok(Self::Pen),
            _ => Err(()),
        }
    }
}
