pub mod aria;
pub(crate) mod aria_hide_outside;
pub mod callback;
pub use leptos_classes as classes;
pub mod color;
pub mod css;
pub mod data_attributes;
pub mod date_time_formatter;
pub(crate) mod dom_ext;
pub mod event_handler;
pub(crate) mod event_listeners;
pub mod event_wrapper;
pub mod filter;
pub mod focus;
pub mod focus_scope_tree;
pub mod focusability;
pub(crate) mod focusable_tree_walker;
pub mod i18n;
pub(crate) mod interaction_rect;
pub mod key;
pub mod list_formatter;
pub mod live_announcer;
pub mod locale;
pub mod math;
pub mod merge;
pub(crate) mod modifiers;
pub mod number_formatter;
pub mod number_parser;
pub(crate) mod open_link;
pub mod platform;
pub mod plurals;
pub mod pointer_type;
pub mod propagation_control;
pub(crate) mod run_after_transition;
pub(crate) mod scroll;
pub mod scroll_behavior;
pub(crate) mod shadow_dom;
pub(crate) mod shadow_tree_walker;
pub mod signals;
pub mod slot_id;
pub mod style;
pub use leptos_styles as styles;
pub(crate) mod synthetic_blur;
pub(crate) mod text_selection;
pub mod time;
pub mod use_description;
pub(crate) mod virtual_click;

#[cfg(all(feature = "syntax-highlight", not(feature = "ssr")))]
pub(crate) mod syntax_highlight;

// Re-exports from aria_hide_outside
pub use aria_hide_outside::{AriaHideOutsideOptions, aria_hide_outside, keep_visible};
// Re-exports from dom_ext
pub use dom_ext::{ContainsTarget, get_owner_document, get_owner_window};
pub(crate) use dom_ext::{
    ElementExt, EventAccessors, EventTargetExt, node_contains, set_event_target,
};
pub use event_handler::EventHandler;
pub use event_listeners::EventListenerOptions;
pub use event_wrapper::EventWrapper;
pub use focusability::will_open_keyboard;
pub use leptos_element_capture as element_capture;
pub use leptos_element_capture::{CapturedElement, ElementCaptureAttr, ElementCaptureCallback};
// Re-exports from interaction_rect
pub use interaction_rect::{InteractionRect, RectPrecise, is_over};
pub use merge::{MergeWith, MergeWithExt};
// Re-exports from modifiers
pub use modifiers::{EventModifiers, Modifiers};
// Re-exports from propagation_control
pub use propagation_control::{Propagation, PropagationControl};
pub use slot_id::{join_slot_ids, use_slot_id};
