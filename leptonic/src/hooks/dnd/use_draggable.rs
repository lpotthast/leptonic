#![cfg_attr(feature = "ssr", allow(dead_code))]

use std::collections::HashSet;

use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    oco::Oco,
    prelude::*,
};
use uuid::Uuid;
use web_sys::{DragEvent, KeyboardEvent, PointerEvent};

use crate::{
    hooks::IntoAttrs,
    utils::{
        EventHandler,
        aria::{AriaDescribedby, AriaRole},
        use_description::use_description,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dnd/src/useDrag.ts

//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Hook-owned state: `is_dragging` is owned internally and exposed as read-only
//   `Signal<bool>`. React-aria: Uses React state.
// - `get_allowed_drop_operations` is a `Callback` instead of a plain function.
//   React-aria: Plain JavaScript function.
//
// ## API DIFFERENCES
// - `DragItem` uses explicit builder pattern for multi-type items instead of JS
//   object spread. React-aria: `DragItem` is `{[type: string]: string}`.
// - `AllowedDropOperations` is a bitflag struct instead of `DropOperation[]`.
//
// ## OMITTED FEATURES
// - Platform-specific modifier keys for drop operations during virtual drag.
//   React-aria: Uses Alt/Ctrl/Cmd based on OS.
//

/// Custom MIME type used to serialize multiple drag items into a single
/// `DataTransfer` entry when native types are insufficient (multiple items
/// or custom types).
pub const LEPTONIC_DND_ITEMS_TYPE: &str = "application/vnd.leptonic.items+json";

/// Standard MIME types that can be stored directly on `DataTransfer`.
const NATIVE_DRAG_TYPES: [&str; 3] = ["text/plain", "text/uri-list", "text/html"];

/// Represents the set of MIME types available in a drag operation.
///
/// Handles browser quirks: Safari reports `"Files"` in `DataTransfer.types`
/// during file drags but does not expose per-item MIME types during
/// `dragenter`/`dragover`. The [`UnknownFiles`](Self::UnknownFiles) variant
/// makes this quirk explicit in the type system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DragTypes {
    /// A known set of MIME types from `DataTransfer`.
    Known(HashSet<String>),
    /// Files are being dragged but their types are unknown (Safari quirk).
    /// [`contains`](Self::contains) returns `true` for any type query so that
    /// type-acceptance checks are optimistically permissive.
    UnknownFiles,
}

impl DragTypes {
    /// Extracts the available MIME types from a `DataTransfer` object.
    ///
    /// Detects the Safari file-drag quirk: when Safari drags files,
    /// `DataTransfer.types` contains only `"Files"` with no specific MIME
    /// types. In that case, returns [`UnknownFiles`](Self::UnknownFiles).
    #[cfg(not(feature = "ssr"))]
    pub fn from_data_transfer(dt: &web_sys::DataTransfer) -> Self {
        let dt_types = dt.types();
        let mut types = HashSet::new();
        let mut has_files = false;

        for i in 0..dt_types.length() {
            if let Some(t) = dt_types.get(i).as_string() {
                if t == "Files" {
                    has_files = true;
                } else {
                    types.insert(t);
                }
            }
        }

        // Safari quirk: when dragging files, only "Files" is reported
        // with no specific MIME types for the individual files.
        if has_files && types.is_empty() {
            return Self::UnknownFiles;
        }

        Self::Known(types)
    }

    /// Constructs `DragTypes` from a `DragEvent`.
    #[cfg(not(feature = "ssr"))]
    pub fn from_drag_event(e: &DragEvent) -> Self {
        e.data_transfer().map_or(Self::Known(HashSet::new()), |dt| {
            Self::from_data_transfer(&dt)
        })
    }

    /// Returns `true` if the given MIME type is in this set.
    ///
    /// For [`UnknownFiles`](Self::UnknownFiles), always returns `true`.
    #[must_use]
    pub fn contains(&self, kind: &str) -> bool {
        match self {
            Self::Known(types) => types.contains(kind),
            Self::UnknownFiles => true,
        }
    }

    /// Returns `true` if the set is empty.
    ///
    /// [`UnknownFiles`](Self::UnknownFiles) is never empty (files are present,
    /// just not identifiable).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Known(types) => types.is_empty(),
            Self::UnknownFiles => false,
        }
    }

    /// Iterates over the known types. Empty for [`UnknownFiles`](Self::UnknownFiles).
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        match self {
            Self::Known(types) => types.iter().map(String::as_str),
            Self::UnknownFiles => {
                // Return an empty iterator with matching type.
                let empty: &HashSet<String> = &EMPTY_SET;
                empty.iter().map(String::as_str)
            }
        }
    }
}

/// Empty set used as a source for the empty iterator in `DragTypes::iter()`.
static EMPTY_SET: std::sync::LazyLock<HashSet<String>> = std::sync::LazyLock::new(HashSet::new);

/// ARIA description for keyboard-draggable elements.
const DRAG_DESCRIPTION_KEYBOARD: &str = "Press Enter to start dragging. Press Tab to navigate to a drop target, then press Enter to drop, or press Escape to cancel.";

/// ARIA description for elements with a primary action where Alt+Enter is
/// required to avoid conflicting with the item's default Enter behavior.
const DRAG_DESCRIPTION_KEYBOARD_ALT: &str = "Press Alt+Enter to start dragging. Press Tab to navigate to a drop target, then press Enter to drop, or press Escape to cancel.";

/// ARIA description for elements with a separate drag button.
const DRAG_DESCRIPTION_DRAG_BUTTON: &str =
    "Has a drag handle. Press Enter on the drag handle to start dragging.";

/// Data that can be transferred during a drag operation.
///
/// A `DragItem` represents a single draggable object with one or more MIME type
/// representations. For example, a rich text item might have both `text/plain`
/// and `text/html` representations of the same content.
///
/// # Example
///
/// ```
/// use leptonic::hooks::DragItem;
///
/// // Single representation
/// let item = DragItem::text("Hello, world!");
///
/// // Multiple representations of the same content
/// let item = DragItem::text("Hello")
///     .with_type("text/html", "<b>Hello</b>");
/// ```
#[derive(Debug, Clone)]
pub struct DragItem {
    /// Ordered list of (MIME type, data) pairs. No duplicate types.
    types: Vec<(String, String)>,
}

impl Default for DragItem {
    fn default() -> Self {
        Self::new()
    }
}

impl DragItem {
    /// Creates a new empty drag item.
    #[must_use]
    pub fn new() -> Self {
        Self { types: Vec::new() }
    }

    /// Creates a new text drag item with `text/plain` type.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            types: vec![("text/plain".to_string(), text.into())],
        }
    }

    /// Creates a new drag item with a custom MIME type.
    #[must_use]
    pub fn custom(kind: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            types: vec![(kind.into(), data.into())],
        }
    }

    /// Creates a drag item for JSON data.
    #[must_use]
    pub fn json(data: impl Into<String>) -> Self {
        Self {
            types: vec![("application/json".to_string(), data.into())],
        }
    }

    /// Adds a type representation to this drag item. If the type already exists,
    /// its data is replaced.
    #[must_use]
    pub fn with_type(mut self, kind: impl Into<String>, data: impl Into<String>) -> Self {
        let kind = kind.into();
        let data = data.into();
        if let Some(entry) = self.types.iter_mut().find(|(k, _)| *k == kind) {
            entry.1 = data;
        } else {
            self.types.push((kind, data));
        }
        self
    }

    /// Returns an iterator over the MIME types in this item.
    pub fn types(&self) -> impl Iterator<Item = &str> {
        self.types.iter().map(|(k, _)| k.as_str())
    }

    /// Returns an iterator over (type, data) pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.types.iter().map(|(k, d)| (k.as_str(), d.as_str()))
    }

    /// Returns the data for a specific MIME type, if present.
    #[must_use]
    pub fn get_data(&self, kind: &str) -> Option<&str> {
        self.types
            .iter()
            .find(|(k, _)| k == kind)
            .map(|(_, d)| d.as_str())
    }

    /// Returns `true` if this item has the given MIME type.
    #[must_use]
    pub fn has_type(&self, kind: &str) -> bool {
        self.types.iter().any(|(k, _)| k == kind)
    }

    /// Returns the first MIME type, or an empty string if empty.
    ///
    /// Convenient for single-type items.
    #[must_use]
    pub fn kind(&self) -> &str {
        self.types.first().map_or("", |(k, _)| k.as_str())
    }

    /// Returns the first data value, or an empty string if empty.
    ///
    /// Convenient for single-type items.
    #[must_use]
    pub fn data(&self) -> &str {
        self.types.first().map_or("", |(_, d)| d.as_str())
    }

    /// Returns the number of type representations.
    #[must_use]
    pub fn len(&self) -> usize {
        self.types.len()
    }

    /// Returns `true` if the item has no type representations.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.types.is_empty()
    }
}

/// A set of allowed drop operations, represented as a bitflag.
///
/// Use the associated constants (`COPY`, `MOVE`, `LINK`, `ALL`, `NONE`) or
/// combine operations with the `|` operator.
///
/// # Example
///
/// ```
/// use leptonic::hooks::AllowedDropOperations;
///
/// let ops = AllowedDropOperations::COPY | AllowedDropOperations::MOVE;
/// assert!(ops.contains_effect(leptonic::hooks::DropEffect::Copy));
/// assert!(ops.contains_effect(leptonic::hooks::DropEffect::Move));
/// assert!(!ops.contains_effect(leptonic::hooks::DropEffect::Link));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AllowedDropOperations(u8);

impl AllowedDropOperations {
    const COPY_BIT: u8 = 0b001;
    const MOVE_BIT: u8 = 0b010;
    const LINK_BIT: u8 = 0b100;

    /// No operations allowed.
    pub const NONE: Self = Self(0);
    /// Only copy is allowed.
    pub const COPY: Self = Self(Self::COPY_BIT);
    /// Only move is allowed.
    pub const MOVE: Self = Self(Self::MOVE_BIT);
    /// Only link is allowed.
    pub const LINK: Self = Self(Self::LINK_BIT);
    /// Copy and move are allowed.
    pub const COPY_MOVE: Self = Self(Self::COPY_BIT | Self::MOVE_BIT);
    /// Copy and link are allowed.
    pub const COPY_LINK: Self = Self(Self::COPY_BIT | Self::LINK_BIT);
    /// Move and link are allowed.
    pub const MOVE_LINK: Self = Self(Self::MOVE_BIT | Self::LINK_BIT);
    /// All operations are allowed.
    pub const ALL: Self = Self(Self::COPY_BIT | Self::MOVE_BIT | Self::LINK_BIT);

    /// Returns true if the given drop effect is in this set.
    #[must_use]
    pub fn contains_effect(self, op: DropEffect) -> bool {
        match op {
            DropEffect::Copy => self.0 & Self::COPY_BIT != 0,
            DropEffect::Move => self.0 & Self::MOVE_BIT != 0,
            DropEffect::Link => self.0 & Self::LINK_BIT != 0,
            DropEffect::All => self == Self::ALL,
            DropEffect::None => true,
        }
    }

    /// Returns the `effectAllowed` string for the `DataTransfer` API.
    #[must_use]
    pub fn as_effect_allowed(self) -> &'static str {
        match self.0 {
            0b001 => "copy",
            0b010 => "move",
            0b100 => "link",
            0b011 => "copyMove",
            0b101 => "copyLink",
            0b110 => "linkMove",
            0b111 => "all",
            _ => "none",
        }
    }

    /// Parses an `effectAllowed` string from the `DataTransfer` API.
    #[must_use]
    pub fn from_effect_allowed(s: &str) -> Self {
        match s {
            "copy" => Self::COPY,
            "move" => Self::MOVE,
            "link" => Self::LINK,
            "copyMove" => Self::COPY_MOVE,
            "copyLink" => Self::COPY_LINK,
            "linkMove" => Self::MOVE_LINK,
            "all" | "uninitialized" => Self::ALL,
            _ => Self::NONE,
        }
    }

    /// Returns true if no operations are allowed.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }
}

impl Default for AllowedDropOperations {
    fn default() -> Self {
        Self::ALL
    }
}

impl std::ops::BitOr for AllowedDropOperations {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for AllowedDropOperations {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

impl From<DropEffect> for AllowedDropOperations {
    fn from(effect: DropEffect) -> Self {
        match effect {
            DropEffect::Copy => Self::COPY,
            DropEffect::Move => Self::MOVE,
            DropEffect::Link => Self::LINK,
            DropEffect::All => Self::ALL,
            DropEffect::None => Self::NONE,
        }
    }
}

/// The drop effect that was performed or allowed (single value).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DropEffect {
    /// Copy the item.
    Copy,
    /// Move the item.
    Move,
    /// Create a link to the item.
    Link,
    /// All effects are allowed.
    #[default]
    All,
    /// No drop effect.
    None,
}

impl DropEffect {
    /// Returns the `effectAllowed` value for the `DataTransfer` API.
    #[must_use]
    pub fn as_effect_allowed(&self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Move => "move",
            Self::Link => "link",
            Self::All => "all",
            Self::None => "none",
        }
    }
}

/// A custom drag preview image to display during drag.
///
/// Returned by the `render_drag_preview` callback. The element should be
/// pre-rendered (e.g. an offscreen `<div>` or `<canvas>`).
#[derive(Debug, Clone)]
pub struct DragPreviewImage {
    /// The element to use as the drag image.
    pub element: web_sys::Element,
    /// The x offset from the cursor to the left edge of the preview.
    pub x_offset: i32,
    /// The y offset from the cursor to the top edge of the preview.
    pub y_offset: i32,
}

/// Event fired when a drag starts.
#[derive(Debug, Clone)]
pub struct DragStartEvent {
    /// The x coordinate of the drag start.
    pub x: f64,
    /// The y coordinate of the drag start.
    pub y: f64,
}

/// Event fired during a drag operation.
#[derive(Debug, Clone)]
pub struct DragMoveEvent {
    /// The current x coordinate.
    pub x: f64,
    /// The current y coordinate.
    pub y: f64,
}

/// Event fired when a drag ends.
#[derive(Debug, Clone)]
pub struct DragEndEvent {
    /// The final x coordinate.
    pub x: f64,
    /// The final y coordinate.
    pub y: f64,
    /// The drop effect that was performed.
    pub drop_effect: DropEffect,
}

/// Input parameters for the `use_draggable` hook.
#[derive(Clone)]
pub struct UseDraggableInput {
    /// Whether the element is disabled (not draggable).
    pub is_disabled: Signal<bool>,

    /// Returns the items to be dragged.
    pub get_items: Callback<(), Vec<DragItem>>,

    /// Returns the allowed drop operations. Called at drag start to determine
    /// which operations the drag source permits. This is a callback (not a
    /// static value) so it can vary based on context (e.g. modifier keys).
    pub get_allowed_drop_operations: Callback<(), AllowedDropOperations>,

    /// Optional callback to provide a custom drag preview image.
    /// Called at drag start with the items being dragged.
    /// Return `Some(DragPreviewImage)` for a custom image, or `None` for default.
    pub render_drag_preview: Option<Callback<Vec<DragItem>, Option<DragPreviewImage>>>,

    /// Whether this draggable has a separate drag button (handle).
    /// When true, `drag_button_props` in the return value will contain props
    /// for an accessible drag handle button. Keyboard activation shifts from
    /// the element itself to the drag button.
    pub has_drag_button: bool,

    /// Whether this draggable item has a primary action (e.g. navigation,
    /// selection). When true, keyboard drag requires Alt+Enter instead of
    /// Enter to avoid conflicting with the item's primary action.
    pub has_action: bool,

    /// Callback when drag starts.
    pub on_drag_start: Option<Callback<DragStartEvent>>,

    /// Callback during drag. Deduplicated: only fires when coordinates change.
    pub on_drag_move: Option<Callback<DragMoveEvent>>,

    /// Callback when drag ends.
    pub on_drag_end: Option<Callback<DragEndEvent>>,
}

impl Default for UseDraggableInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            get_items: Callback::new(|_| vec![]),
            get_allowed_drop_operations: Callback::new(|_| AllowedDropOperations::ALL),
            render_drag_preview: None,
            has_drag_button: false,
            has_action: false,
            on_drag_start: None,
            on_drag_move: None,
            on_drag_end: None,
        }
    }
}

/// The return value of the `use_draggable` hook.
pub struct UseDraggableReturn {
    /// Props for the draggable element.
    pub drag_props: UseDraggableProps,

    /// Props for an accessible drag button (handle). Only present when
    /// `has_drag_button` was set to `true` in the input.
    pub drag_button_props: Option<UseDragButtonProps>,

    /// The ID of the draggable element.
    pub draggable_id: String,

    /// Whether a drag is currently in progress.
    pub is_dragging: Signal<bool>,
}

/// Props from `use_draggable` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseDraggableProps {
    pub id: String,
    pub draggable: Signal<&'static str>,
    pub role: AriaRole,
    pub tabindex: Signal<i32>,
    pub aria_describedby: AriaDescribedby,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_drag: EventHandler<DragEvent>,
    pub on_dragend: EventHandler<DragEvent>,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
}

impl IntoAttrs for UseDraggableProps {
    type Attrs = UseDraggableAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Draggable, self.draggable),
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_drag.into_on(ev::drag),
            self.on_dragend.into_on(ev::dragend),
            self.on_keydown.into_on(ev::keydown),
            self.on_pointerdown.into_on(ev::pointerdown),
        )
    }
}

/// Attributes for the draggable element.
pub type UseDraggableAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Draggable, Signal<&'static str>>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, Signal<i32>>,
    Attr<attr::AriaDescribedby, AriaDescribedby>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::drag, SharedEventCallback<DragEvent>>,
    On<ev::dragend, SharedEventCallback<DragEvent>>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
);

/// Props for an accessible drag button (handle).
#[derive(Debug)]
pub struct UseDragButtonProps {
    pub role: AriaRole,
    pub tabindex: Signal<i32>,
    pub aria_describedby: AriaDescribedby,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl IntoAttrs for UseDragButtonProps {
    type Attrs = UseDragButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::Tabindex, self.tabindex),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// Attributes for a drag button element.
pub type UseDragButtonAttrs = (
    Attr<attr::Role, AriaRole>,
    Attr<attr::Tabindex, Signal<i32>>,
    Attr<attr::AriaDescribedby, AriaDescribedby>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Writes drag items to a `DataTransfer`, using JSON serialization when needed.
pub(crate) fn write_to_data_transfer(dt: &web_sys::DataTransfer, items: &[DragItem]) {
    // Clear existing data before writing (#8 fix).
    let _ = dt.clear_data();

    // Determine if we need JSON serialization (multiple items, or items with
    // non-native types that could collide).
    let needs_serialization = items.len() > 1
        || items
            .iter()
            .any(|item| item.types().any(|t| !NATIVE_DRAG_TYPES.contains(&t)));

    if needs_serialization {
        let json = serialize_drag_items(items);
        let _ = dt.set_data(LEPTONIC_DND_ITEMS_TYPE, &json);

        // Also set native types from the first item for interop with
        // non-leptonic drop targets.
        if let Some(first) = items.first() {
            for (kind, data) in &first.types {
                if NATIVE_DRAG_TYPES.contains(&kind.as_str()) {
                    let _ = dt.set_data(kind, data);
                }
            }
        }
    } else if let Some(item) = items.first() {
        // Single item with only native types — set directly.
        for (kind, data) in &item.types {
            let _ = dt.set_data(kind, data);
        }
    }
}

/// Reads drag items from a `DataTransfer`, deserializing JSON if present.
pub(crate) fn read_from_data_transfer(dt: &web_sys::DataTransfer) -> Vec<DragItem> {
    // Check for serialized items first.
    if let Ok(json) = dt.get_data(LEPTONIC_DND_ITEMS_TYPE) {
        if !json.is_empty() {
            if let Some(items) = deserialize_drag_items(&json) {
                return items;
            }
        }
    }

    // Fall back: read all native types as a single DragItem.
    let types = dt.types();
    let mut item = DragItem::new();
    for i in 0..types.length() {
        if let Some(kind) = types.get(i).as_string() {
            if let Ok(data) = dt.get_data(&kind) {
                item = item.with_type(kind, data);
            }
        }
    }

    if item.is_empty() { vec![] } else { vec![item] }
}

/// Serializes drag items to JSON.
///
/// Format: `[{"type1":"data1","type2":"data2"}, ...]`
fn serialize_drag_items(items: &[DragItem]) -> String {
    let array: Vec<serde_json::Map<String, serde_json::Value>> = items
        .iter()
        .map(|item| {
            item.types
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect()
        })
        .collect();
    serde_json::to_string(&array).unwrap_or_else(|_| "[]".to_string())
}

/// Deserializes drag items from JSON.
fn deserialize_drag_items(json: &str) -> Option<Vec<DragItem>> {
    let array: Vec<serde_json::Map<String, serde_json::Value>> = serde_json::from_str(json).ok()?;
    let items = array
        .into_iter()
        .map(|obj| {
            let types: Vec<(String, String)> = obj
                .into_iter()
                .filter_map(|(k, v)| {
                    if let serde_json::Value::String(s) = v {
                        Some((k, s))
                    } else {
                        None
                    }
                })
                .collect();
            DragItem { types }
        })
        .collect();
    Some(items)
}

/// Provides the behavior and accessibility for a draggable element.
///
/// Supports native mouse/touch drag via HTML5 drag events, as well as keyboard
/// drag via the virtual `DragManager`. Includes ARIA descriptions for screen
/// reader users.
///
/// # Example
///
/// ```ignore
/// let draggable = use_draggable(UseDraggableInput {
///     get_items: Callback::new(|_| vec![DragItem::text("Hello")]),
///     on_drag_end: Some(Callback::new(|e| {
///         tracing::info!("Drag ended: {:?}", e.drop_effect);
///     })),
///     ..Default::default()
/// });
///
/// view! {
///     <div {..draggable.drag_props.into_attrs()}>
///         "Drag me"
///     </div>
/// }
/// ```
#[allow(
    clippy::needless_pass_by_value,
    clippy::too_many_lines,
    clippy::useless_conversion
)]
pub fn use_draggable(input: UseDraggableInput) -> UseDraggableReturn {
    cfg_if::cfg_if! {
        if #[cfg(feature = "ssr")] {
            let UseDraggableInput {
                is_disabled: disabled,
                has_drag_button,
                ..
            } = input;

            let draggable_id = format!("draggable-{}", Uuid::new_v4());
            let (is_dragging, _) = signal(false);
            let description = if has_drag_button {
                DRAG_DESCRIPTION_DRAG_BUTTON
            } else {
                DRAG_DESCRIPTION_KEYBOARD
            };
            let aria_describedby = use_description(Oco::Borrowed(description));
            let drag_button_props = if has_drag_button {
                Some(UseDragButtonProps {
                    role: AriaRole::Button,
                    tabindex: Signal::derive(move || if disabled.get() { -1_i32 } else { 0 }),
                    aria_describedby: use_description(Oco::Borrowed(DRAG_DESCRIPTION_KEYBOARD)),
                    on_keydown: EventHandler::new(|_: KeyboardEvent| {}),
                })
            } else {
                None
            };

            UseDraggableReturn {
                drag_props: UseDraggableProps {
                    id: draggable_id.clone(),
                    draggable: Signal::derive(move || if disabled.get() { "false" } else { "true" }),
                    role: AriaRole::Button,
                    tabindex: Signal::derive(move || if disabled.get() { -1_i32 } else { 0 }),
                    aria_describedby,
                    on_dragstart: EventHandler::new(|_: DragEvent| {}),
                    on_drag: EventHandler::new(|_: DragEvent| {}),
                    on_dragend: EventHandler::new(|_: DragEvent| {}),
                    on_keydown: EventHandler::new(|_: KeyboardEvent| {}),
                    on_pointerdown: EventHandler::new(|_: PointerEvent| {}),
                },
                drag_button_props,
                draggable_id,
                is_dragging: is_dragging.into(),
            }
        } else {
            let UseDraggableInput {
                is_disabled: disabled,
                get_items,
                get_allowed_drop_operations,
                render_drag_preview,
                has_drag_button,
                has_action,
                on_drag_start,
                on_drag_move,
                on_drag_end,
            } = input;

    let draggable_id = format!("draggable-{}", Uuid::new_v4());

    let (is_dragging, set_is_dragging) = signal(false);

    // --- ARIA description (#2) ---
    // Replaces deprecated `aria-grabbed` with `aria-describedby` pointing to
    // instructions for screen reader users. The description varies by
    // interaction modality and whether the item has a primary action.
    let description = if has_drag_button {
        DRAG_DESCRIPTION_DRAG_BUTTON
    } else if has_action {
        DRAG_DESCRIPTION_KEYBOARD_ALT
    } else {
        DRAG_DESCRIPTION_KEYBOARD
    };
    let aria_describedby = use_description(Oco::Borrowed(description));

    // --- Coordinate dedup state (#7) ---
    // Track last coordinates to avoid firing `on_drag_move` when the cursor
    // hasn't actually moved. Browsers fire the `drag` event continuously.
    let last_x: StoredValue<f64> = StoredValue::new(f64::NAN);
    let last_y: StoredValue<f64> = StoredValue::new(f64::NAN);

    // --- Derived attributes ---
    let draggable_attr = Signal::derive(move || if disabled.get() { "false" } else { "true" });
    let tabindex = Signal::derive(move || if disabled.get() { -1_i32 } else { 0 });

    // --- Native drag handlers ---

    let handle_drag_start = move |e: DragEvent| {
        if disabled.get_untracked() {
            e.prevent_default();
            return;
        }

        set_is_dragging.set(true);

        if let Some(data_transfer) = e.data_transfer() {
            let allowed = get_allowed_drop_operations.run(());
            data_transfer.set_effect_allowed(allowed.as_effect_allowed());

            let items = get_items.run(());

            // Write items to DataTransfer (clears first — #8 fix).
            write_to_data_transfer(&data_transfer, &items);

            // Custom drag preview (#6).
            if let Some(render_preview) = render_drag_preview {
                if let Some(preview) = render_preview.run(items.clone()) {
                    data_transfer.set_drag_image(
                        &preview.element,
                        preview.x_offset,
                        preview.y_offset,
                    );
                }
            }

            if let Some(on_start) = on_drag_start {
                on_start.run(DragStartEvent {
                    x: f64::from(e.client_x()),
                    y: f64::from(e.client_y()),
                });
            }
        }
    };

    let handle_drag = move |e: DragEvent| {
        // Coordinate dedup (#7): only fire when position actually changes.
        let x = f64::from(e.client_x());
        let y = f64::from(e.client_y());

        #[allow(clippy::float_cmp)]
        if x == last_x.get_value() && y == last_y.get_value() {
            return;
        }
        last_x.set_value(x);
        last_y.set_value(y);

        if let Some(on_move) = on_drag_move {
            on_move.run(DragMoveEvent { x, y });
        }
    };

    let handle_drag_end = move |e: DragEvent| {
        set_is_dragging.set(false);

        let drop_effect =
            e.data_transfer()
                .map_or(DropEffect::None, |dt| match dt.drop_effect().as_str() {
                    "copy" => DropEffect::Copy,
                    "move" => DropEffect::Move,
                    "link" => DropEffect::Link,
                    _ => DropEffect::None,
                });

        if let Some(on_end) = on_drag_end {
            on_end.run(DragEndEvent {
                x: f64::from(e.client_x()),
                y: f64::from(e.client_y()),
                drop_effect,
            });
        }
    };

    // --- Keyboard handler for virtual drag (#1) ---
    let make_keyboard_handler = move || {
        EventHandler::new(move |e: KeyboardEvent| {
            if disabled.get_untracked() {
                return;
            }
            // When has_action is true, require Alt+Enter to avoid conflicting
            // with the item's primary action (e.g. navigation, selection).
            if has_action {
                if !(e.key() == "Enter" && e.alt_key()) {
                    return;
                }
            } else if e.key() != "Enter" {
                return;
            }
            e.prevent_default();

            #[cfg(not(feature = "ssr"))]
            {
                use wasm_bindgen::JsCast;
                if let Some(element) = e
                    .current_target()
                    .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                {
                    let items = get_items.run(());
                    let allowed = get_allowed_drop_operations.run(());

                    set_is_dragging.set(true);

                    if let Some(on_start) = on_drag_start {
                        on_start.run(DragStartEvent { x: 0.0, y: 0.0 });
                    }

                    super::drag_manager::begin_dragging(super::drag_manager::DragSource {
                        element,
                        items,
                        allowed_operations: allowed,
                        on_end: Box::new(move |end_event: DragEndEvent| {
                            set_is_dragging.set(false);
                            if let Some(on_end) = on_drag_end {
                                on_end.run(end_event);
                            }
                        }),
                    });
                }
            }
        })
    };

    let main_keydown = if has_drag_button {
        EventHandler::empty()
    } else {
        make_keyboard_handler()
    };

    // --- Drag button props (#3) ---
    let drag_button_props = if has_drag_button {
        let button_describedby = use_description(Oco::Borrowed(DRAG_DESCRIPTION_KEYBOARD));
        Some(UseDragButtonProps {
            role: AriaRole::Button,
            tabindex: Signal::derive(move || if disabled.get() { -1_i32 } else { 0 }),
            aria_describedby: button_describedby,
            on_keydown: make_keyboard_handler(),
        })
    } else {
        None
    };

    // --- Virtual pointer handler for screen readers (#10) ---
    // Detects VoiceOver/TalkBack virtual clicks and routes them through
    // the DragManager's keyboard drag flow instead of native drag.
    let handle_pointer_down = move |e: PointerEvent| {
        #[cfg(feature = "ssr")]
        let _ = e;

        if disabled.get_untracked() || has_drag_button {
            return;
        }

        #[cfg(not(feature = "ssr"))]
        if crate::utils::virtual_click::is_virtual_pointer_event(&e) {
            #[cfg(not(feature = "ssr"))]
            use wasm_bindgen::JsCast;

            e.prevent_default();
            e.stop_propagation();

            if let Some(element) = e
                .current_target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            {
                let items = get_items.run(());
                let allowed = get_allowed_drop_operations.run(());

                set_is_dragging.set(true);

                if let Some(on_start) = on_drag_start {
                    on_start.run(DragStartEvent {
                        x: e.client_x(),
                        y: e.client_y(),
                    });
                }

                super::drag_manager::begin_dragging(super::drag_manager::DragSource {
                    element,
                    items,
                    allowed_operations: allowed,
                    on_end: Box::new(move |end_event: DragEndEvent| {
                        set_is_dragging.set(false);
                        if let Some(on_end) = on_drag_end {
                            on_end.run(end_event);
                        }
                    }),
                });
            }
        }
    };

    // --- Unmount cleanup (#9) ---
    on_cleanup(move || {
        if is_dragging.get_untracked() {
            set_is_dragging.set(false);
            if let Some(on_end) = on_drag_end {
                on_end.run(DragEndEvent {
                    x: 0.0,
                    y: 0.0,
                    drop_effect: DropEffect::None,
                });
            }
        }
    });

            UseDraggableReturn {
                drag_props: UseDraggableProps {
                    id: draggable_id.clone(),
                    draggable: draggable_attr,
                    role: AriaRole::Button,
                    tabindex,
                    aria_describedby,
                    on_dragstart: EventHandler::new(handle_drag_start),
                    on_drag: EventHandler::new(handle_drag),
                    on_dragend: EventHandler::new(handle_drag_end),
                    on_keydown: main_keydown,
                    on_pointerdown: EventHandler::new(handle_pointer_down),
                },
                drag_button_props,
                draggable_id,
                is_dragging: is_dragging.into(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drag_item_single_type() {
        let item = DragItem::text("hello");
        assert_eq!(item.kind(), "text/plain");
        assert_eq!(item.data(), "hello");
        assert_eq!(item.len(), 1);
        assert!(item.has_type("text/plain"));
        assert!(!item.has_type("text/html"));
    }

    #[test]
    fn test_drag_item_multi_type() {
        let item = DragItem::text("hello").with_type("text/html", "<b>hello</b>");
        assert_eq!(item.len(), 2);
        assert_eq!(item.get_data("text/plain"), Some("hello"));
        assert_eq!(item.get_data("text/html"), Some("<b>hello</b>"));
        assert_eq!(item.kind(), "text/plain");
    }

    #[test]
    fn test_drag_item_replace_type() {
        let item = DragItem::text("hello").with_type("text/plain", "world");
        assert_eq!(item.len(), 1);
        assert_eq!(item.data(), "world");
    }

    #[test]
    fn test_allowed_drop_operations() {
        let ops = AllowedDropOperations::COPY | AllowedDropOperations::MOVE;
        assert!(ops.contains_effect(DropEffect::Copy));
        assert!(ops.contains_effect(DropEffect::Move));
        assert!(!ops.contains_effect(DropEffect::Link));
        assert_eq!(ops.as_effect_allowed(), "copyMove");
    }

    #[test]
    fn test_allowed_drop_operations_from_effect_allowed() {
        assert_eq!(
            AllowedDropOperations::from_effect_allowed("copyMove"),
            AllowedDropOperations::COPY_MOVE
        );
        assert_eq!(
            AllowedDropOperations::from_effect_allowed("all"),
            AllowedDropOperations::ALL
        );
        assert_eq!(
            AllowedDropOperations::from_effect_allowed("none"),
            AllowedDropOperations::NONE
        );
    }

    #[test]
    fn test_serialize_deserialize_drag_items() {
        let items = vec![
            DragItem::text("hello").with_type("text/html", "<b>hello</b>"),
            DragItem::text("world"),
        ];
        let json = serialize_drag_items(&items);
        let deserialized = deserialize_drag_items(&json).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].get_data("text/plain"), Some("hello"));
        assert_eq!(deserialized[0].get_data("text/html"), Some("<b>hello</b>"));
        assert_eq!(deserialized[1].get_data("text/plain"), Some("world"));
    }

    #[test]
    fn test_drag_types_known_contains() {
        let types = DragTypes::Known(HashSet::from([
            "text/plain".to_string(),
            "text/html".to_string(),
        ]));
        assert!(types.contains("text/plain"));
        assert!(types.contains("text/html"));
        assert!(!types.contains("image/png"));
        assert!(!types.is_empty());
    }

    #[test]
    fn test_drag_types_unknown_files_contains_any() {
        let types = DragTypes::UnknownFiles;
        assert!(types.contains("text/plain"));
        assert!(types.contains("image/png"));
        assert!(types.contains("anything"));
        assert!(!types.is_empty());
    }

    #[test]
    fn test_drag_types_known_empty() {
        let types = DragTypes::Known(HashSet::new());
        assert!(!types.contains("text/plain"));
        assert!(types.is_empty());
    }

    #[test]
    fn test_drag_types_iter() {
        let types = DragTypes::Known(HashSet::from(["text/plain".to_string()]));
        let collected: Vec<&str> = types.iter().collect();
        assert_eq!(collected, vec!["text/plain"]);

        let unknown = DragTypes::UnknownFiles;
        let collected: Vec<&str> = unknown.iter().collect();
        assert!(collected.is_empty());
    }
}
