//! Synthetic DOM events, for what WebDriver can't produce (see `ElementActions::dispatch`).

use std::{fmt, marker::PhantomData};

use serde_json::{Map, Value};

/// A DOM event to dispatch from script, built by the constructor of its interface
/// ([`plain`](Self::plain), [`pointer`](Self::pointer), [`mouse`](Self::mouse),
/// [`keyboard`](Self::keyboard), [`wheel`](Self::wheel), [`drag`](Self::drag),
/// [`custom`](Self::custom)) and the typed setters of that interface. Like the events of real
/// input, it bubbles, is cancelable and composed; pointer events come from the primary pointer.
///
/// ```ignore
/// target
///     .dispatch(SyntheticEvent::pointer(PointerKind::Down).pointer_type(PointerType::Touch))
///     .await?;
/// ```
#[derive(Debug, Clone)]
pub struct SyntheticEvent<I> {
    interface: &'static str,
    kind: String,
    init: Map<String, Value>,
    marker: PhantomData<I>,
}

/// The event interfaces: which setters an event has.
pub mod interface {
    /// `Event`.
    #[derive(Debug, Clone, Copy)]
    pub enum Plain {}
    /// `PointerEvent` (a `MouseEvent`).
    #[derive(Debug, Clone, Copy)]
    pub enum Pointer {}
    /// `MouseEvent`.
    #[derive(Debug, Clone, Copy)]
    pub enum Mouse {}
    /// `KeyboardEvent`.
    #[derive(Debug, Clone, Copy)]
    pub enum Keyboard {}
    /// `WheelEvent` (a `MouseEvent`).
    #[derive(Debug, Clone, Copy)]
    pub enum Wheel {}
    /// `DragEvent` (a `MouseEvent`).
    #[derive(Debug, Clone, Copy)]
    pub enum Drag {}
    /// `CustomEvent`.
    #[derive(Debug, Clone, Copy)]
    pub enum Custom {}

    /// Interfaces inheriting `MouseEvent`'s members (position, buttons).
    pub trait MouseLike {}
    impl MouseLike for Pointer {}
    impl MouseLike for Mouse {}
    impl MouseLike for Wheel {}
    impl MouseLike for Drag {}

    /// Interfaces with modifier key members (`altKey`, ...).
    pub trait WithModifiers {}
    impl WithModifiers for Pointer {}
    impl WithModifiers for Mouse {}
    impl WithModifiers for Keyboard {}
    impl WithModifiers for Wheel {}
    impl WithModifiers for Drag {}
}

use interface::{Custom, Drag, Keyboard, Mouse, MouseLike, Plain, Pointer, Wheel, WithModifiers};

/// The types of plain `Event`s the tests dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventKind {
    BeforeMatch,
    BeforeUnload,
    Change,
    Resize,
    Scroll,
}

impl EventKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::BeforeMatch => "beforematch",
            Self::BeforeUnload => "beforeunload",
            Self::Change => "change",
            Self::Resize => "resize",
            Self::Scroll => "scroll",
        }
    }
}

/// The types of `PointerEvent`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "every pointer event type, whether a test uses it yet or not"
)]
pub enum PointerKind {
    Down,
    Move,
    Up,
    Cancel,
    Over,
    Out,
    Enter,
    Leave,
}

impl PointerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Down => "pointerdown",
            Self::Move => "pointermove",
            Self::Up => "pointerup",
            Self::Cancel => "pointercancel",
            Self::Over => "pointerover",
            Self::Out => "pointerout",
            Self::Enter => "pointerenter",
            Self::Leave => "pointerleave",
        }
    }

    /// Whether events of this type bubble (`pointerenter` and `pointerleave` don't).
    fn bubbles(self) -> bool {
        !matches!(self, Self::Enter | Self::Leave)
    }
}

/// A pointer event's `pointerType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerType {
    Mouse,
    Pen,
    Touch,
}

impl PointerType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Mouse => "mouse",
            Self::Pen => "pen",
            Self::Touch => "touch",
        }
    }
}

/// The types of `MouseEvent`s the tests dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseKind {
    Click,
    ContextMenu,
}

impl MouseKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Click => "click",
            Self::ContextMenu => "contextmenu",
        }
    }
}

/// The types of `KeyboardEvent`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyKind {
    Down,
    Up,
}

impl KeyKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Down => "keydown",
            Self::Up => "keyup",
        }
    }
}

/// The types of `DragEvent`s.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "every drag event type, whether a test uses it yet or not"
)]
pub enum DragKind {
    Start,
    Drag,
    Enter,
    Over,
    Leave,
    Drop,
    End,
}

impl DragKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Start => "dragstart",
            Self::Drag => "drag",
            Self::Enter => "dragenter",
            Self::Over => "dragover",
            Self::Leave => "dragleave",
            Self::Drop => "drop",
            Self::End => "dragend",
        }
    }
}

/// A mouse button, as a mouse event's `button`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    /// Usually the left button (0).
    Primary,
    /// Usually the wheel (1).
    Auxiliary,
    /// Usually the right button (2).
    Secondary,
}

impl MouseButton {
    fn number(self) -> u8 {
        match self {
            Self::Primary => 0,
            Self::Auxiliary => 1,
            Self::Secondary => 2,
        }
    }
}

/// A modifier key held during an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "every modifier key, whether a test uses it yet or not"
)]
pub enum Modifier {
    Alt,
    Control,
    Meta,
    Shift,
}

impl Modifier {
    /// The event init member saying that the key is held.
    pub fn init_member(self) -> &'static str {
        match self {
            Self::Alt => "altKey",
            Self::Control => "ctrlKey",
            Self::Meta => "metaKey",
            Self::Shift => "shiftKey",
        }
    }
}

impl<I> SyntheticEvent<I> {
    fn new(interface: &'static str, kind: &str) -> Self {
        let mut init = Map::new();
        for member in ["bubbles", "cancelable", "composed"] {
            init.insert(member.to_owned(), Value::Bool(true));
        }
        Self {
            interface,
            kind: kind.to_owned(),
            init,
            marker: PhantomData,
        }
    }

    fn set(mut self, member: &str, value: impl Into<Value>) -> Self {
        self.init.insert(member.to_owned(), value.into());
        self
    }

    /// Whether the event bubbles (by default it does, except where the event type never does).
    #[must_use]
    pub fn bubbles(self, bubbles: bool) -> Self {
        self.set("bubbles", bubbles)
    }

    pub(crate) fn into_parts(self) -> (&'static str, String, Value) {
        (self.interface, self.kind, Value::Object(self.init))
    }
}

impl SyntheticEvent<Plain> {
    /// An `Event`, e.g. `beforematch`.
    pub fn plain(kind: EventKind) -> Self {
        Self::new("Event", kind.as_str())
    }
}

impl SyntheticEvent<Pointer> {
    /// A `PointerEvent` of the primary pointer (a mouse unless [`pointer_type`](Self::pointer_type)
    /// says otherwise): `pointerdown` presses the primary button, `pointerup` releases it.
    pub fn pointer(kind: PointerKind) -> Self {
        let changed_button = matches!(kind, PointerKind::Down | PointerKind::Up);
        Self::new("PointerEvent", kind.as_str())
            .bubbles(kind.bubbles())
            .set("pointerType", PointerType::Mouse.as_str())
            .set("pointerId", 1)
            .set("isPrimary", true)
            .set("width", 1)
            .set("height", 1)
            // `-1`: no button changed.
            .set("button", if changed_button { 0 } else { -1 })
            .set("buttons", u8::from(kind == PointerKind::Down))
    }

    /// The `pointerType`.
    #[must_use]
    pub fn pointer_type(self, pointer_type: PointerType) -> Self {
        self.set("pointerType", pointer_type.as_str())
    }

    /// The contact's size (`width`, `height`; 1×1 by default). VoiceOver on iOS reports less
    /// than 1×1.
    #[must_use]
    pub fn size(self, width: f64, height: f64) -> Self {
        self.set("width", width).set("height", height)
    }

    /// The `pointerId`; another than the primary pointer's (1) is a secondary pointer
    /// (`isPrimary` false).
    #[must_use]
    pub fn pointer_id(self, pointer_id: i32) -> Self {
        self.set("pointerId", pointer_id)
            .set("isPrimary", pointer_id == 1)
    }
}

impl SyntheticEvent<Mouse> {
    /// A `MouseEvent`, e.g. `contextmenu`.
    pub fn mouse(kind: MouseKind) -> Self {
        Self::new("MouseEvent", kind.as_str())
    }
}

impl SyntheticEvent<Keyboard> {
    /// A `KeyboardEvent` for the key value `key` (`"a"`, `" "`, `"ArrowDown"`).
    pub fn keyboard(kind: KeyKind, key: &str) -> Self {
        Self::new("KeyboardEvent", kind.as_str()).set("key", key)
    }

    /// Whether the key is held down long enough to repeat (`repeat`).
    #[must_use]
    pub fn repeat(self, repeat: bool) -> Self {
        self.set("repeat", repeat)
    }

    /// Whether the event is part of a composition session (`isComposing`, an IME).
    #[must_use]
    pub fn composing(self, composing: bool) -> Self {
        self.set("isComposing", composing)
    }
}

impl SyntheticEvent<Wheel> {
    /// A `wheel` event (`WheelEvent`).
    pub fn wheel() -> Self {
        Self::new("WheelEvent", "wheel")
    }

    /// The vertical scroll amount (`deltaY`, in the [`delta_mode`](Self::delta_mode)'s unit,
    /// pixels by default; positive: down).
    #[must_use]
    pub fn delta_y(self, delta_y: f64) -> Self {
        self.set("deltaY", delta_y)
    }

    /// The unit of the deltas (`deltaMode`).
    #[must_use]
    pub fn delta_mode(self, mode: DeltaMode) -> Self {
        self.set(
            "deltaMode",
            match mode {
                DeltaMode::Pixel => 0,
                DeltaMode::Line => 1,
                DeltaMode::Page => 2,
            },
        )
    }
}

/// The unit of a wheel event's deltas (`deltaMode`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(
    dead_code,
    reason = "every delta mode, whether a test uses it yet or not"
)]
pub enum DeltaMode {
    Pixel,
    /// Lines (Firefox with a mouse wheel).
    Line,
    Page,
}

impl SyntheticEvent<Drag> {
    /// A `DragEvent`, e.g. `dragstart`, without a `DataTransfer` (`DndActions` fires drags
    /// with one).
    pub fn drag(kind: DragKind) -> Self {
        Self::new("DragEvent", kind.as_str())
    }
}

impl SyntheticEvent<Custom> {
    /// A `CustomEvent` of the app-defined type `name`, with `detail`.
    pub fn custom(name: &str, detail: impl Into<Value>) -> Self {
        Self::new("CustomEvent", name).set("detail", detail)
    }
}

impl<I: MouseLike> SyntheticEvent<I> {
    /// The position in the viewport (`clientX`, `clientY`), CSS pixels.
    #[must_use]
    pub fn at(self, x: f64, y: f64) -> Self {
        self.set("clientX", x).set("clientY", y)
    }

    /// The button that changed (`button`).
    #[must_use]
    pub fn button(self, button: MouseButton) -> Self {
        self.set("button", button.number())
    }

    /// The buttons held (`buttons`).
    #[must_use]
    pub fn buttons(self, buttons: &[MouseButton]) -> Self {
        // The bits of `buttons`: primary 1, secondary 2, auxiliary 4.
        let bits = buttons.iter().fold(0_u8, |bits, button| {
            bits | match button {
                MouseButton::Primary => 1,
                MouseButton::Secondary => 2,
                MouseButton::Auxiliary => 4,
            }
        });
        self.set("buttons", bits)
    }

    /// The click count (`detail`): `0` for clicks of assistive technology, `1` for a pointer's
    /// (and TalkBack's on Android).
    #[must_use]
    pub fn detail(self, detail: i32) -> Self {
        self.set("detail", detail)
    }
}

impl<I: WithModifiers> SyntheticEvent<I> {
    /// The modifier keys held.
    #[must_use]
    pub fn modifiers(mut self, modifiers: &[Modifier]) -> Self {
        for modifier in modifiers {
            self = self.set(modifier.init_member(), true);
        }
        self
    }
}

impl<I> fmt::Display for SyntheticEvent<I> {
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
