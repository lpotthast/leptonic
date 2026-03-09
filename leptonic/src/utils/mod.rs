pub mod aria;
pub mod callback;
pub mod classes;
pub mod color;
pub(crate) mod dom_ext;
pub mod element_capture;
pub mod event_handler;
pub(crate) mod event_listeners;
pub mod event_wrapper;
pub mod focus;
pub mod focus_scope_tree;
pub mod focusability;
pub mod formatters;
pub mod i18n;
pub(crate) mod interaction_rect;
pub mod key;
pub mod live_announcer;
pub mod locale;
pub mod math;
pub mod merge;
pub(crate) mod modifiers;
pub(crate) mod open_link;
pub mod platform;
pub mod pointer_type;
pub mod propagation_control;
pub(crate) mod run_after_transition;
pub mod scroll_behavior;
pub(crate) mod shadow_dom;
pub(crate) mod shadow_tree_walker;
pub mod signals;
pub mod style;
pub mod styles;
pub(crate) mod synthetic_blur;
pub(crate) mod text_selection;
pub mod time;
pub mod use_description;
pub(crate) mod virtual_click;

// Re-exports from dom_ext
pub use dom_ext::{get_owner_document, get_owner_window, ContainsTarget};
pub(crate) use dom_ext::{
    node_contains, set_event_target, ElementExt, EventAccessors, EventTargetExt,
};
pub use element_capture::{CapturedElement, ElementCaptureAttr};
pub use event_handler::EventHandler;
pub use event_listeners::EventListenerOptions;
pub use event_wrapper::EventWrapper;
// Re-exports from interaction_rect
pub use interaction_rect::{is_over, InteractionRect, RectPrecise};
pub use merge::{MergeWith, MergeWithExt};
// Re-exports from modifiers
pub use modifiers::{EventModifiers, Modifiers};
// Re-exports from propagation_control
pub use propagation_control::{Propagation, PropagationControl};
