use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;

use crate::hooks::interactions::use_prevent_scroll::{use_prevent_scroll, UsePreventScrollInput};
use crate::utils::EventHandler;

// This is based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useModalOverlay.ts
// React Aria calls this "useModalOverlay" with "underlay" props. We use "backdrop" terminology for clarity.

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_modal_backdrop` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseModalBackdropInput {
    /// Whether the modal is open.
    pub is_open: Signal<bool>,

    /// Callback when the modal should close.
    pub on_close: Option<Callback<()>>,

    /// Whether clicking the backdrop should close the modal.
    pub should_close_on_interact_outside: bool,

    /// Whether the modal should prevent body scrolling.
    pub prevent_scroll: bool,
}

impl Default for UseModalBackdropInput {
    fn default() -> Self {
        Self {
            is_open: Signal::derive(|| false),
            on_close: None,
            should_close_on_interact_outside: true,
            prevent_scroll: true,
        }
    }
}

/// The return value of the `use_modal_backdrop` hook.
pub struct UseModalBackdropReturn {
    /// Props for the backdrop element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub backdrop_props: UseModalBackdropProps,

    /// Props for the modal content element. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub content_props: UseModalBackdropContentProps,

    /// The ID of the backdrop element.
    pub backdrop_id: String,
}

/// Props from `use_modal_backdrop` for the backdrop element.
#[derive(Debug, Clone)]
pub struct UseModalBackdropProps {
    pub id: String,
    pub on_click: EventHandler<web_sys::MouseEvent>,
}

impl UseModalBackdropProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseModalBackdropAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseModalBackdropAttrs {
        (Attr(attr::Id, self.id), self.on_click.into_on(ev::click))
    }
}

/// Attributes for the backdrop element.
pub type UseModalBackdropAttrs = (
    Attr<attr::Id, String>,
    On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,
);

/// Props from `use_modal_backdrop` for the modal content container.
#[derive(Debug, Clone)]
pub struct UseModalBackdropContentProps {
    pub on_click: EventHandler<web_sys::MouseEvent>,
}

impl UseModalBackdropContentProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseModalBackdropContentAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseModalBackdropContentAttrs {
        (self.on_click.into_on(ev::click),)
    }
}

/// Attributes for the modal content container (to stop click propagation).
pub type UseModalBackdropContentAttrs = (On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,);

/// Provides behavior for modal backdrops.
///
/// This hook handles:
/// - Backdrop click to close behavior
/// - Scroll prevention when the modal is open
///
/// It should be combined with `use_modal` for complete modal functionality.
///
/// # Example
///
/// ```ignore
/// let (is_open, set_is_open) = signal(false);
///
/// let backdrop = use_modal_backdrop(UseModalBackdropInput {
///     is_open: is_open.into(),
///     on_close: Some(Callback::new(move |_| set_is_open.set(false))),
///     should_close_on_interact_outside: true,
///     prevent_scroll: true,
/// });
///
/// view! {
///     <Show when=move || is_open.get()>
///         <div class="backdrop" {..backdrop.backdrop_props.into_attrs()}>
///             <div class="modal" {..backdrop.content_props.into_attrs()}>
///                 "Modal content"
///             </div>
///         </div>
///     </Show>
/// }
/// ```
pub fn use_modal_backdrop(input: UseModalBackdropInput) -> UseModalBackdropReturn {
    let UseModalBackdropInput {
        is_open,
        on_close,
        should_close_on_interact_outside,
        prevent_scroll,
    } = input;

    let backdrop_id = format!("modal-backdrop-{}", Uuid::new_v4());

    // Prevent body scrolling when modal is open
    let _prevent_scroll = use_prevent_scroll(UsePreventScrollInput {
        disabled: Signal::derive(move || !is_open.get() || !prevent_scroll),
    });

    // Handle click on backdrop
    let handle_backdrop_click = move |e: web_sys::MouseEvent| {
        // Only close if clicking directly on the backdrop, not its children
        if should_close_on_interact_outside {
            if let (Some(target), Some(current_target)) = (e.target(), e.current_target()) {
                if target == current_target {
                    if let Some(on_close) = on_close {
                        on_close.run(());
                    }
                }
            }
        }
    };

    // Stop propagation on modal content click to prevent closing when clicking inside
    let handle_content_click = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
    };

    UseModalBackdropReturn {
        backdrop_props: UseModalBackdropProps {
            id: backdrop_id.clone(),
            on_click: EventHandler::new(handle_backdrop_click),
        },
        content_props: UseModalBackdropContentProps {
            on_click: EventHandler::new(handle_content_click),
        },
        backdrop_id,
    }
}
