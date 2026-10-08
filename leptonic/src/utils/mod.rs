pub mod aria;
pub(crate) mod aria_hide_outside;
pub mod callback;
#[cfg(feature = "clipboard")]
pub mod clipboard;
pub use leptos_classes as classes;
pub mod color;
pub mod css;
pub mod data_attributes;
pub mod date;
pub mod date_time_formatter;
pub mod default_class;
pub(crate) mod dom_ext;
pub mod event_handler;
pub(crate) mod event_listeners;
pub mod event_wrapper;
pub mod filter;
pub mod focus;
pub mod focus_scope_tree;
pub mod focusability;
pub(crate) mod focusable_tree_walker;
pub mod fraction;
pub mod heading_level;
pub mod i18n;
pub mod id;
pub mod intl_strings;
pub mod key;
pub mod keyboard_shortcut;
pub mod list_formatter;
pub mod live_announcer;
pub mod locale;
pub mod math;
pub mod merge;
pub(crate) mod modifiers;
pub mod number_formatter;
pub mod number_parser;
pub mod number_value;
pub(crate) mod open_link;
pub mod orientation;
pub(crate) mod owner_alive;
pub mod platform;
pub mod plurals;
pub mod point;
pub mod pointer_type;
pub(crate) mod prevent_focus;
pub mod propagation_control;
#[cfg(not(feature = "ssr"))]
pub(crate) mod run_after_transition;
pub(crate) mod scoped_context;
pub mod scroll;
pub mod scroll_behavior;
pub(crate) mod shadow_dom;
pub(crate) mod shadow_tree_walker;
pub mod slot_id;
pub mod style;
pub use leptos_styles as styles;
#[cfg(not(feature = "ssr"))]
pub(crate) mod synthetic_blur;
#[cfg(not(feature = "ssr"))]
pub(crate) mod text_selection;
pub mod use_description;
pub mod use_viewport_size;
pub mod value_binding;
pub(crate) mod virtual_click;
pub mod virtual_focus;
pub mod visually_hidden;

#[cfg(feature = "syntax-highlight")]
pub mod syntax_highlight;

// Re-exports from aria_hide_outside
pub use aria_hide_outside::{AriaHideOutsideOptions, HideMode, aria_hide_outside, keep_visible};
// Re-exports from dom_ext
pub use dom_ext::ContainsTarget;
#[cfg(not(feature = "ssr"))]
pub(crate) use dom_ext::{ElementExt, set_event_target};
pub(crate) use dom_ext::{EventAccessors, EventTargetExt, node_contains};
pub use event_handler::EventHandler;
pub use event_wrapper::EventWrapper;
pub use focusability::will_open_keyboard;
pub use leptos_element_capture as element_capture;
pub use leptos_element_capture::{CapturedElement, ElementCaptureAttr, ElementCaptureCallback};
pub use merge::{MergeWith, MergeWithExt};
// Re-exports from modifiers
pub use modifiers::{EventModifiers, Modifiers};
pub use number_value::NumberValue;
pub use point::Point;
// Re-exports from propagation_control
pub use propagation_control::{Propagation, PropagationControl};
pub use slot_id::{Slot, SlotAttrs, SlotProps, join_slot_ids, use_slot};
pub use value_binding::ValueBinding;

/// A warning about API misuse for developers, as react-aria's `NODE_ENV !== 'production'`
/// warnings: logged in debug builds only.
macro_rules! dev_warn {
    ($($arg:tt)*) => {
        if cfg!(debug_assertions) {
            ::leptos::logging::warn!($($arg)*);
        }
    };
}
pub(crate) use dev_warn;
