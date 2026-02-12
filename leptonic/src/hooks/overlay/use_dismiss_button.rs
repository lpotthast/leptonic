use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;

use crate::utils::EventHandler;
use crate::hooks::IntoAttrs;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useDismissButton.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// No intentional deviations from the react-aria implementation.
//
// =============================================================================

/// Input parameters for the dismiss button.
#[derive(Debug, Clone, Copy)]
pub struct UseDismissButtonInput {
    /// Callback when the button is pressed.
    pub on_dismiss: Option<leptos::prelude::Callback<()>>,
}

/// A visually hidden dismiss button for screen reader accessibility.
///
/// This provides an accessible way for screen reader users to dismiss overlays.
pub struct UseDismissButtonReturn {
    /// Props for the dismiss button.
    pub button_props: UseDismissButtonProps,
}

/// Props from `use_dismiss_button` that can be extracted and merged programmatically.
#[derive(Debug)]
pub struct UseDismissButtonProps {
    pub on_click: EventHandler<web_sys::MouseEvent>,
}

impl IntoAttrs for UseDismissButtonProps {
    type Attrs = UseDismissButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (self.on_click.into_on(ev::click),)
    }
}

/// Attributes for the dismiss button.
pub type UseDismissButtonAttrs = (On<ev::click, SharedEventCallback<web_sys::MouseEvent>>,);

/// Creates a visually hidden dismiss button for accessibility.
///
/// Screen reader users need a way to dismiss overlays. This provides
/// a button that can be placed at the end of an overlay.
///
/// # Example
///
/// ```ignore
/// let dismiss_button = use_dismiss_button(UseDismissButtonInput {
///     on_dismiss: Some(Callback::new(move |_| set_is_open.set(false))),
/// });
///
/// view! {
///     <button
///         class="visually-hidden"
///         aria-label="Dismiss"
///         {..dismiss_button.button_props.into_attrs()}
///     >
///         "Dismiss"
///     </button>
/// }
/// ```
pub fn use_dismiss_button(input: UseDismissButtonInput) -> UseDismissButtonReturn {
    let UseDismissButtonInput { on_dismiss } = input;

    let handle_click = move |_e: web_sys::MouseEvent| {
        if let Some(on_dismiss) = on_dismiss {
            on_dismiss.run(());
        }
    };

    UseDismissButtonReturn {
        button_props: UseDismissButtonProps {
            on_click: EventHandler::new(handle_click),
        },
    }
}
