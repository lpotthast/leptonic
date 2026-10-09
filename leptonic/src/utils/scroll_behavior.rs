// No upstream: leptonic's typed `ScrollBehavior` for anchor links (react-aria's `useLink` has no
// scrolling); the DOM's `ScrollBehavior` without its `auto` variant.
/// How an anchor link scrolls to its target.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollBehavior {
    /// An animated scroll.
    #[default]
    Smooth,
    /// A jump.
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
