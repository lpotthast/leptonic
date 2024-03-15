#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBehavior {
    #[default]
    Smooth,
    Instant,
}

impl From<ScrollBehavior> for web_sys::ScrollBehavior {
    fn from(value: ScrollBehavior) -> Self {
        match value {
            ScrollBehavior::Smooth => web_sys::ScrollBehavior::Smooth,
            ScrollBehavior::Instant => web_sys::ScrollBehavior::Instant,
        }
    }
}
