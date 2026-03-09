use leptos::{
    attr,
    attr::Attr,
    ev,
    ev::{On, SharedEventCallback},
    prelude::*,
};
use uuid::Uuid;
use web_sys::DragEvent;

use crate::{
    hooks::IntoAttrs,
    utils::{
        aria::{AriaGrabbed, AriaRole},
        EventHandler,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dnd/src/useDrag.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Data that can be transferred during a drag operation.
#[derive(Debug, Clone)]
pub struct DragItem {
    /// The MIME type of the data.
    pub kind: String,
    /// The data being dragged.
    pub data: String,
}

impl DragItem {
    /// Creates a new text drag item.
    #[must_use]
    pub fn text(text: impl Into<String>) -> Self {
        Self {
            kind: "text/plain".to_string(),
            data: text.into(),
        }
    }

    /// Creates a new drag item with a custom MIME type.
    #[must_use]
    pub fn custom(kind: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            data: data.into(),
        }
    }

    /// Creates a drag item for JSON data.
    #[must_use]
    pub fn json(data: impl Into<String>) -> Self {
        Self {
            kind: "application/json".to_string(),
            data: data.into(),
        }
    }
}

/// The type of drag preview to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DragPreviewType {
    /// Use the default drag preview (the dragged element).
    #[default]
    Default,
    /// Use no drag preview.
    None,
}

/// Input parameters for the `use_draggable` hook.
#[derive(Clone)]
pub struct UseDraggableInput {
    /// Whether the element is draggable.
    pub is_disabled: Signal<bool>,

    /// The items being dragged.
    pub get_items: Callback<(), Vec<DragItem>>,

    /// The allowed drop effect.
    pub allowed_drop_effect: DropEffect,

    /// The type of drag preview.
    pub preview: DragPreviewType,

    /// Callback when drag starts.
    pub on_drag_start: Option<Callback<DragStartEvent>>,

    /// Callback during drag.
    pub on_drag_move: Option<Callback<DragMoveEvent>>,

    /// Callback when drag ends.
    pub on_drag_end: Option<Callback<DragEndEvent>>,
}

impl Default for UseDraggableInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            get_items: Callback::new(|_| vec![]),
            allowed_drop_effect: DropEffect::All,
            preview: DragPreviewType::Default,
            on_drag_start: None,
            on_drag_move: None,
            on_drag_end: None,
        }
    }
}

/// The allowed drop effects for a drag operation.
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

/// The return value of the `use_draggable` hook.
pub struct UseDraggableReturn {
    /// Props for the draggable element.
    pub drag_props: UseDraggableProps,

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
    pub aria_grabbed: Signal<Option<AriaGrabbed>>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_drag: EventHandler<DragEvent>,
    pub on_dragend: EventHandler<DragEvent>,
}

impl IntoAttrs for UseDraggableProps {
    type Attrs = UseDraggableAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Draggable, self.draggable),
            Attr(attr::Role, self.role),
            Attr(attr::AriaGrabbed, self.aria_grabbed),
            self.on_dragstart.into_on(ev::dragstart),
            self.on_drag.into_on(ev::drag),
            self.on_dragend.into_on(ev::dragend),
        )
    }
}

/// Attributes for the draggable element.
pub type UseDraggableAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Draggable, Signal<&'static str>>,
    Attr<attr::Role, AriaRole>,
    Attr<attr::AriaGrabbed, Signal<Option<AriaGrabbed>>>,
    On<ev::dragstart, SharedEventCallback<DragEvent>>,
    On<ev::drag, SharedEventCallback<DragEvent>>,
    On<ev::dragend, SharedEventCallback<DragEvent>>,
);

/// Provides the behavior and accessibility for a draggable element.
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
///     <div {..draggable.drag_props}>
///         "Drag me"
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_draggable(input: UseDraggableInput) -> UseDraggableReturn {
    let UseDraggableInput {
        is_disabled: disabled,
        get_items,
        allowed_drop_effect,
        preview,
        on_drag_start,
        on_drag_move,
        on_drag_end,
    } = input;

    let draggable_id = format!("draggable-{}", Uuid::new_v4());

    let (is_dragging, set_is_dragging) = signal(false);

    let draggable_attr = Signal::derive(move || if disabled.get() { "false" } else { "true" });

    let aria_grabbed = Signal::derive(move || is_dragging.get().then_some(AriaGrabbed::True));

    let handle_drag_start = move |e: DragEvent| {
        if disabled.get_untracked() {
            e.prevent_default();
            return;
        }

        set_is_dragging.set(true);

        // Set the data transfer
        if let Some(data_transfer) = e.data_transfer() {
            data_transfer.set_effect_allowed(allowed_drop_effect.as_effect_allowed());

            // Add all items to the data transfer
            let items = get_items.run(());
            for item in items {
                let _ = data_transfer.set_data(&item.kind, &item.data);
            }
        }

        if let Some(on_start) = on_drag_start {
            on_start.run(DragStartEvent {
                x: f64::from(e.client_x()),
                y: f64::from(e.client_y()),
            });
        }
    };

    let handle_drag = move |e: DragEvent| {
        if let Some(on_move) = on_drag_move {
            on_move.run(DragMoveEvent {
                x: f64::from(e.client_x()),
                y: f64::from(e.client_y()),
            });
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

    UseDraggableReturn {
        drag_props: UseDraggableProps {
            id: draggable_id.clone(),
            draggable: draggable_attr,
            role: AriaRole::Button,
            aria_grabbed,
            on_dragstart: EventHandler::new(handle_drag_start),
            on_drag: EventHandler::new(handle_drag),
            on_dragend: EventHandler::new(handle_drag_end),
        },
        draggable_id,
        is_dragging: is_dragging.into(),
    }
}
