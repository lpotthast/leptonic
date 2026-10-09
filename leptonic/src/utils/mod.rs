pub mod aria;
pub(crate) mod aria_hide_outside;
#[cfg(feature = "clipboard")]
pub mod clipboard;
pub mod color;
pub mod data_attributes;
pub mod date;
pub mod date_time_formatter;
#[cfg(any(feature = "atoms", test))]
pub mod default_class;
pub(crate) mod dom_ext;
pub mod event_handler;
pub(crate) mod event_listeners;
pub mod filter;
pub mod focus;
#[cfg(any(all(feature = "atoms", not(feature = "ssr")), test))]
pub(crate) mod focus_scope_tree;
pub mod focusability;
pub(crate) mod focusable_tree_walker;
pub mod fraction;
pub mod heading_level;
pub mod i18n;
pub mod id;
pub mod id_refs;
pub mod intl_strings;
pub mod key;
pub mod keyboard_shortcut;
pub mod labels;
pub mod list_formatter;
pub mod live_announcer;
pub mod math;
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
pub mod styles;
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
