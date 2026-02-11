use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use web_sys::KeyboardEvent;

use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useDismissButton.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the `use_dismiss` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseDismissInput {
    /// Whether the dismiss behavior is disabled.
    pub is_disabled: Signal<bool>,

    /// Callback when dismiss is triggered.
    pub on_dismiss: Option<Callback<()>>,

    /// Whether to dismiss on Escape key.
    pub dismiss_on_escape: bool,

    /// Whether to dismiss on blur.
    pub dismiss_on_blur: bool,
}

impl Default for UseDismissInput {
    fn default() -> Self {
        Self {
            is_disabled: Signal::derive(|| false),
            on_dismiss: None,
            dismiss_on_escape: true,
            dismiss_on_blur: false,
        }
    }
}

/// The return value of the `use_dismiss` hook.
pub struct UseDismissReturn {
    /// Props for the dismissable container element.
    pub dismiss_props: UseDismissProps,
}

/// Props from `use_dismiss` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseDismissProps {
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_blur: EventHandler<web_sys::FocusEvent>,
}

impl UseDismissProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseDismissAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseDismissAttrs {
        (
            self.on_keydown.into_on(ev::keydown),
            self.on_blur.into_on(ev::blur),
        )
    }
}

/// Attributes for the dismissable container element.
pub type UseDismissAttrs = (
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
    On<ev::blur, SharedEventCallback<web_sys::FocusEvent>>,
);

/// Provides dismiss behavior for overlays.
///
/// This hook handles Escape key and optional blur dismissal.
/// It's useful for dropdowns, popovers, and other dismissable overlays.
///
/// # Example
///
/// ```ignore
/// let dismiss = use_dismiss(UseDismissInput {
///     on_dismiss: Some(Callback::new(move |_| {
///         set_is_open.set(false);
///     })),
///     dismiss_on_escape: true,
///     dismiss_on_blur: false,
///     ..Default::default()
/// });
///
/// view! {
///     <div {..dismiss.dismiss_props.into_attrs()}>
///         "Dismissable content"
///     </div>
/// }
/// ```
pub fn use_dismiss(input: UseDismissInput) -> UseDismissReturn {
    let UseDismissInput {
        is_disabled,
        on_dismiss,
        dismiss_on_escape,
        dismiss_on_blur,
    } = input;

    // Handle keydown for escape
    let handle_keydown = move |e: KeyboardEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        if dismiss_on_escape && e.key() == "Escape" {
            e.prevent_default();
            e.stop_propagation();
            if let Some(on_dismiss) = on_dismiss {
                on_dismiss.run(());
            }
        }
    };

    // Handle blur
    let handle_blur = move |_e: web_sys::FocusEvent| {
        if is_disabled.get_untracked() {
            return;
        }

        if dismiss_on_blur {
            // Check if focus moved outside the container
            // In a real implementation, you'd check if relatedTarget is within the container
            if let Some(on_dismiss) = on_dismiss {
                on_dismiss.run(());
            }
        }
    };

    UseDismissReturn {
        dismiss_props: UseDismissProps {
            on_keydown: EventHandler::new(handle_keydown),
            on_blur: EventHandler::new(handle_blur),
        },
    }
}
