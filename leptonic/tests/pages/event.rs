//! Synthetic DOM events, for what WebDriver can't produce (see `ElementActions::dispatch`).

use std::fmt;

use serde_json::{Map, Value};

/// A DOM event to dispatch from script: its interface (`PointerEvent`, ...), type (`pointerdown`,
/// ...) and init dictionary. It bubbles, is cancelable and composed, like the events of real
/// input; [`Self::with`] overrides any init member.
///
/// ```ignore
/// target
///     .dispatch(SyntheticEvent::pointer("pointerdown").with("pointerType", "touch"))
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct SyntheticEvent {
    interface: &'static str,
    kind: String,
    init: Map<String, Value>,
}

#[allow(dead_code)] // Not every test binary uses every interface.
impl SyntheticEvent {
    fn new(interface: &'static str, kind: &str) -> Self {
        let mut init = Map::new();
        for member in ["bubbles", "cancelable", "composed"] {
            init.insert(member.to_owned(), Value::Bool(true));
        }
        Self {
            interface,
            kind: kind.to_owned(),
            init,
        }
    }

    /// An `Event`, e.g. `beforematch` or `scroll`.
    pub fn plain(kind: &str) -> Self {
        Self::new("Event", kind)
    }

    /// A `PointerEvent`, e.g. `pointerdown` from a touch or pen pointer.
    pub fn pointer(kind: &str) -> Self {
        Self::new("PointerEvent", kind)
    }

    /// A `MouseEvent`, e.g. `contextmenu`.
    pub fn mouse(kind: &str) -> Self {
        Self::new("MouseEvent", kind)
    }

    /// A `KeyboardEvent` for `key`, e.g. a repeated `keydown`.
    pub fn keyboard(kind: &str, key: &str) -> Self {
        Self::new("KeyboardEvent", kind).with("key", key)
    }

    /// A `wheel` event (`WheelEvent`).
    pub fn wheel() -> Self {
        Self::new("WheelEvent", "wheel")
    }

    /// A `DragEvent`, e.g. `dragstart`.
    pub fn drag(kind: &str) -> Self {
        Self::new("DragEvent", kind)
    }

    /// A `CustomEvent` with `detail`.
    pub fn custom(kind: &str, detail: impl Into<Value>) -> Self {
        Self::new("CustomEvent", kind).with("detail", detail)
    }

    /// Set the init member `name` (e.g. `pointerType`, `clientX`, `repeat`, `deltaY`).
    pub fn with(mut self, name: &str, value: impl Into<Value>) -> Self {
        self.init.insert(name.to_owned(), value.into());
        self
    }

    pub(crate) fn into_parts(self) -> (&'static str, String, Value) {
        (self.interface, self.kind, Value::Object(self.init))
    }
}

impl fmt::Display for SyntheticEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {:?} {}",
            self.interface,
            self.kind,
            Value::Object(self.init.clone())
        )
    }
}
