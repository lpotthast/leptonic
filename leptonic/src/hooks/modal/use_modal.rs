use leptos::attr;
use leptos::attr::Attr;
use leptos::ev;
use leptos::ev::{On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::aria::AriaModal;
use crate::utils::EventHandler;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/overlays/src/useModal.ts

/// Input parameters for the `use_modal` hook.
#[derive(Debug, Clone, Copy)]
pub struct UseModalInput {
    /// Whether the modal is open.
    pub is_open: Signal<bool>,

    /// Callback when the modal should close.
    pub on_close: Option<Callback<()>>,

    /// Whether to close when pressing Escape.
    pub is_dismissable: bool,

    /// Whether to close when clicking outside.
    pub should_close_on_interact_outside: bool,

    /// Whether the modal is a keyboard-dismissable modal.
    pub is_keyboard_dismiss_disabled: bool,
}

impl Default for UseModalInput {
    fn default() -> Self {
        Self {
            is_open: Signal::derive(|| false),
            on_close: None,
            is_dismissable: true,
            should_close_on_interact_outside: true,
            is_keyboard_dismiss_disabled: false,
        }
    }
}

/// The return value of the `use_modal` hook.
pub struct UseModalReturn {
    /// Props for programmatic merging. Call `.to_attrs()` or `.into_attrs()` for view spreading.
    pub modal_props: UseModalProps,

    /// The ID of the modal.
    pub id: String,
}

/// Props from `use_modal` that can be extracted and merged programmatically.
#[derive(Debug, Clone)]
pub struct UseModalProps {
    pub id: String,
    pub role: &'static str,
    pub aria_modal: AriaModal,
    pub tabindex: &'static str,
    pub on_keydown: EventHandler<KeyboardEvent>,
}

impl UseModalProps {
    /// Convert to spreadable attributes for Leptos views, cloning internally.
    #[must_use]
    pub fn to_attrs(&self) -> UseModalAttrs {
        self.clone().into_attrs()
    }

    /// Convert to spreadable attributes for Leptos views, consuming self.
    #[must_use]
    pub fn into_attrs(self) -> UseModalAttrs {
        (
            Attr(attr::Id, self.id),
            Attr(attr::Role, self.role),
            Attr(attr::AriaModal, self.aria_modal),
            Attr(attr::Tabindex, self.tabindex),
            self.on_keydown.into_on(ev::keydown),
        )
    }
}

/// These attributes must be spread onto the target element: `<div {..attrs} />`
pub type UseModalAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaModal, AriaModal>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Provides accessibility and behavior for modal dialogs.
///
/// This hook manages focus trapping and escape key handling for modals.
/// It should be used together with `use_modal_backdrop` for a complete modal experience.
///
/// # Example
///
/// ```ignore
/// let (is_open, set_is_open) = signal(false);
///
/// let modal = use_modal(UseModalInput {
///     is_open: is_open.into(),
///     on_close: Some(Callback::new(move |_| set_is_open.set(false))),
///     is_dismissable: true,
///     ..Default::default()
/// });
///
/// view! {
///     <Show when=move || is_open.get()>
///         <div {..modal.modal_props.into_attrs()}>
///             "Modal content"
///             <button on:click=move |_| set_is_open.set(false)>"Close"</button>
///         </div>
///     </Show>
/// }
/// ```
pub fn use_modal(input: UseModalInput) -> UseModalReturn {
    let UseModalInput {
        is_open,
        on_close,
        is_dismissable,
        should_close_on_interact_outside,
        is_keyboard_dismiss_disabled,
    } = input;

    let modal_id = format!("modal-{}", Uuid::new_v4());

    // Handle keydown for escape
    let handle_keydown = move |e: KeyboardEvent| {
        if is_dismissable && !is_keyboard_dismiss_disabled && e.key() == "Escape" {
            e.prevent_default();
            e.stop_propagation();
            if let Some(on_close) = on_close {
                on_close.run(());
            }
        }
    };

    UseModalReturn {
        modal_props: UseModalProps {
            id: modal_id.clone(),
            role: "dialog",
            aria_modal: AriaModal::True,
            tabindex: "-1",
            on_keydown: EventHandler::new(handle_keydown),
        },
        id: modal_id,
    }
}

/// State for managing modal visibility.
#[derive(Clone, Copy)]
pub struct UseModalStateReturn {
    /// Whether the modal is open.
    pub is_open: Signal<bool>,

    /// Open the modal.
    pub open: Callback<()>,

    /// Close the modal.
    pub close: Callback<()>,

    /// Toggle the modal.
    pub toggle: Callback<()>,
}

/// Creates internal state for a modal component.
pub fn use_modal_state(default_open: bool) -> UseModalStateReturn {
    let (is_open, set_is_open) = signal(default_open);

    UseModalStateReturn {
        is_open: is_open.into(),
        open: Callback::new(move |_| set_is_open.set(true)),
        close: Callback::new(move |_| set_is_open.set(false)),
        toggle: Callback::new(move |_| set_is_open.update(|v| *v = !*v)),
    }
}
