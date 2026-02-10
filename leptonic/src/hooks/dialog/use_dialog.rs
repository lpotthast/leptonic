use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::KeyboardEvent;

use crate::utils::aria::AriaModal;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dialog/src/useDialog.ts

/// Input parameters for the `use_dialog` hook.
#[derive(Debug, Clone)]
pub struct UseDialogInput {
    /// The title of the dialog (for aria-labelledby).
    pub title: Option<String>,

    /// A description of the dialog (for aria-describedby).
    pub description: Option<String>,

    /// The role of the dialog.
    pub role: DialogRole,

    /// Whether the dialog is dismissable with Escape.
    pub is_dismissable: bool,

    /// Callback when the dialog should close.
    pub on_close: Option<Callback<()>>,
}

/// The role of a dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogRole {
    /// A standard dialog.
    #[default]
    Dialog,
    /// An alert dialog that requires user attention.
    AlertDialog,
}

impl Default for UseDialogInput {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            role: DialogRole::Dialog,
            is_dismissable: true,
            on_close: None,
        }
    }
}

/// The return value of the `use_dialog` hook.
pub struct UseDialogReturn {
    /// Props for the dialog container element.
    pub dialog_props: UseDialogAttrs,

    /// Props for the title element.
    pub title_props: UseDialogTitleProps,

    /// Props for the description element.
    pub description_props: UseDialogDescriptionProps,

    /// The ID of the dialog.
    pub dialog_id: String,
}

/// Attributes for the dialog container element.
pub type UseDialogAttrs = (
    Attr<attr::Id, String>,
    Attr<attr::Role, &'static str>,
    Attr<attr::AriaModal, AriaModal>,
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
);

/// Props for the dialog title element.
#[derive(Debug, Clone)]
pub struct UseDialogTitleProps {
    /// The id of the title element.
    pub id: String,
}

/// Props for the dialog description element.
#[derive(Debug, Clone)]
pub struct UseDialogDescriptionProps {
    /// The id of the description element.
    pub id: String,
}

/// Provides the behavior and accessibility implementation for a dialog.
///
/// A dialog is a window overlaid on either the primary window or another dialog.
/// Content outside the dialog is inert, meaning users cannot interact with it.
///
/// # Example
///
/// ```ignore
/// let (is_open, set_is_open) = signal(false);
///
/// let dialog = use_dialog(UseDialogInput {
///     title: Some("Confirm Action".to_string()),
///     description: Some("Are you sure you want to proceed?".to_string()),
///     on_close: Some(Callback::new(move |_| set_is_open.set(false))),
///     ..Default::default()
/// });
///
/// view! {
///     <Show when=move || is_open.get()>
///         <div class="dialog-overlay">
///             <div {..dialog.dialog_props}>
///                 <h2 id=dialog.title_props.id>"Confirm Action"</h2>
///                 <p id=dialog.description_props.id>"Are you sure you want to proceed?"</p>
///                 <button on:click=move |_| set_is_open.set(false)>"Cancel"</button>
///                 <button on:click=move |_| { /* confirm action */ }>"Confirm"</button>
///             </div>
///         </div>
///     </Show>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_dialog(input: UseDialogInput) -> UseDialogReturn {
    let base_id = Uuid::new_v4();
    let dialog_id = format!("dialog-{base_id}");
    let title_id = format!("dialog-title-{base_id}");
    let description_id = format!("dialog-description-{base_id}");

    let on_close = input.on_close;
    let is_dismissable = input.is_dismissable;

    // Build aria-labelledby
    let aria_labelledby = if input.title.is_some() {
        Some(title_id.clone())
    } else {
        None
    };

    // Build aria-describedby
    let aria_describedby = if input.description.is_some() {
        Some(description_id.clone())
    } else {
        None
    };

    // Determine role string
    let role = match input.role {
        DialogRole::Dialog => "dialog",
        DialogRole::AlertDialog => "alertdialog",
    };

    // Handle keydown for escape
    let handle_keydown = move |e: KeyboardEvent| {
        if is_dismissable && e.key() == "Escape" {
            e.prevent_default();
            e.stop_propagation();
            if let Some(on_close) = on_close {
                on_close.run(());
            }
        }
    };

    UseDialogReturn {
        dialog_props: (
            Attr(attr::Id, dialog_id.clone()),
            Attr(attr::Role, role),
            Attr(attr::AriaModal, AriaModal::True),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDescribedby, aria_describedby),
            Attr(attr::Tabindex, "-1"),
            on(ev::keydown, handle_keydown).into_cloneable(),
        ),
        title_props: UseDialogTitleProps { id: title_id },
        description_props: UseDialogDescriptionProps { id: description_id },
        dialog_id,
    }
}

/// State for managing dialog visibility and confirmed state.
#[derive(Clone, Copy)]
pub struct UseDialogStateReturn {
    /// Whether the dialog is open.
    pub is_open: Signal<bool>,

    /// Open the dialog.
    pub open: Callback<()>,

    /// Close the dialog.
    pub close: Callback<()>,

    /// Close the dialog with a confirmed result.
    pub confirm: Callback<()>,

    /// Whether the dialog was confirmed (vs cancelled).
    pub is_confirmed: Signal<bool>,
}

/// Creates internal state for a dialog component with confirmation tracking.
pub fn use_dialog_state(default_open: bool) -> UseDialogStateReturn {
    let (is_open, set_is_open) = signal(default_open);
    let (is_confirmed, set_is_confirmed) = signal(false);

    UseDialogStateReturn {
        is_open: is_open.into(),
        open: Callback::new(move |_| {
            set_is_confirmed.set(false);
            set_is_open.set(true);
        }),
        close: Callback::new(move |_| {
            set_is_open.set(false);
        }),
        confirm: Callback::new(move |_| {
            set_is_confirmed.set(true);
            set_is_open.set(false);
        }),
        is_confirmed: is_confirmed.into(),
    }
}
