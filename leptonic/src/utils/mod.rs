use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub mod aria;
pub mod callback;
pub mod classes;
pub mod color;
pub(crate) mod dom_ext;
pub mod element_capture;
pub mod event_handler;
pub(crate) mod event_listeners;
pub mod focus;
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
pub(crate) mod run_after_transition;
pub mod scroll_behavior;
pub mod signals;
pub mod style;
pub mod styles;
pub(crate) mod text_selection;
pub mod time;
pub mod use_description;
pub(crate) mod virtual_click;

pub use element_capture::{CapturedElement, ElementCaptureAttr};
pub use event_handler::EventHandler;
pub use merge::{MergeWith, MergeWithExt};

// Re-exports from dom_ext
pub use dom_ext::{get_owner_document, get_owner_window, ContainsTarget};
pub(crate) use dom_ext::{node_contains, ElementExt, EventAccessors, EventTargetExt};

// Re-exports from event_listeners
pub use event_listeners::EventListenerOptions;
pub(crate) use event_listeners::ListenExt;

// Re-exports from modifiers
pub use modifiers::{EventModifiers, Modifiers};

// Re-exports from interaction_rect
pub use interaction_rect::{is_over, InteractionRect, RectPrecise};

pub(crate) fn use_continue_propagation() -> (Arc<AtomicBool>, Arc<dyn Fn() + Send + Sync + 'static>)
{
    let continue_propagation_state = Arc::new(AtomicBool::new(false));
    let state = continue_propagation_state.clone();
    let continue_propagation = Arc::new(move || {
        state.store(true, Ordering::Release);
    });
    (continue_propagation_state, continue_propagation)
}
