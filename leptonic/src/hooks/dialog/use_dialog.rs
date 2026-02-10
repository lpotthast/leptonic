use std::time::Duration;

use leptos::attr;
use leptos::attr::{Attr, Attribute};
use leptos::ev;
use leptos::ev::{on, On, SharedEventCallback};
use leptos::prelude::*;
use uuid::Uuid;
use web_sys::FocusEvent;

use crate::utils::element_capture::{CapturedElement, ElementCaptureAttr};
use crate::utils::focus::focus_safely;

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/@react-aria/dialog/src/useDialog.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
//
// - aria-modal
//   React-aria does NOT set `aria-modal` on the dialog element due to a Safari
//   iframe bug (https://bugs.webkit.org/show_bug.cgi?id=211934). Instead, it
//   relies on `useModal` setting `aria-hidden` on all elements outside the dialog.
//   In Leptonic, `aria-modal` is set by `use_modal` rather than `use_dialog`,
//   and we use `aria-modal` directly rather than the `aria-hidden` approach.
//
// - Escape key / dismiss handling
//   React-aria's `useDialog` does NOT handle Escape key or any dismiss behavior.
//   Dismissal is handled by `useOverlay` / `useModalOverlay`. In Leptonic,
//   this is handled by `use_modal`.
//
// - useOverlayFocusContain
//   React-aria calls `useOverlayFocusContain()` to signal to the parent `Overlay`
//   component that focus should be contained. In Leptonic, focus containment is
//   handled by the `FocusScope` atom which is applied by the consumer.
//
// - description_props
//   React-aria does not return separate description props. `aria-describedby` is
//   passed through via `filterDOMProps`. We return `description_props` for
//   convenience, making it easier to wire up the description element's ID.
//
// - Element capture
//   React-aria takes a `RefObject` for the dialog element. We use the
//   `CapturedElement` / `ElementCaptureAttr` pattern instead, which captures
//   the element automatically when `dialog_props` are spread onto the element.
//
// =============================================================================

/// Input parameters for the `use_dialog` hook.
///
/// This hook handles ARIA semantics (role, labeling) and focus management.
/// For dismiss behavior (Escape key, outside click) and `aria-modal`, use
/// `use_modal`. Compose both for a fully accessible modal dialog.
#[derive(Debug, Clone)]
pub struct UseDialogInput {
    /// The title of the dialog (for `aria-labelledby`).
    ///
    /// When set, a unique ID is generated and returned in `title_props.id`.
    /// Use this ID on the title element to create the ARIA association.
    pub title: Option<String>,

    /// A description of the dialog (for `aria-describedby`).
    ///
    /// When set, a unique ID is generated and returned in `description_props.id`.
    /// Use this ID on the description element to create the ARIA association.
    pub description: Option<String>,

    /// An accessible label for the dialog.
    ///
    /// When set, `aria-labelledby` is suppressed (the caller should set
    /// `aria-label` on the element directly). This is useful when the dialog
    /// has no visible title element.
    pub aria_label: Option<String>,

    /// The role of the dialog.
    pub role: DialogRole,
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

/// The return value of the `use_dialog` hook.
pub struct UseDialogReturn {
    /// Props for the dialog container element.
    ///
    /// Spread these onto the dialog element. The element is automatically captured
    /// for focus-on-mount behavior via an included [`ElementCaptureAttr`].
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
    Attr<attr::AriaLabelledby, Option<String>>,
    Attr<attr::AriaDescribedby, Option<String>>,
    Attr<attr::Tabindex, &'static str>,
    On<ev::blur, SharedEventCallback<FocusEvent>>,
    ElementCaptureAttr,
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
/// This hook handles:
/// - ARIA role (`dialog` or `alertdialog`)
/// - ARIA labeling (`aria-labelledby`, `aria-describedby`)
/// - Focus on mount (focuses the dialog unless a child already has focus)
/// - iOS Safari `VoiceOver` workaround (blur/refocus after 500ms)
///
/// The dialog element is captured automatically when `dialog_props` are spread
/// onto it — no manual `NodeRef` wiring required.
///
/// For dismiss behavior (Escape key, `aria-modal`), use `use_modal`.
/// Compose both for a fully accessible modal dialog.
///
/// # Example
///
/// ```ignore
/// let dialog = use_dialog(UseDialogInput {
///     title: Some("Confirm Action".to_string()),
///     description: Some("Are you sure you want to proceed?".to_string()),
///     aria_label: None,
///     role: DialogRole::Dialog,
/// });
///
/// view! {
///     <div {..dialog.dialog_props}>
///         <h2 id=dialog.title_props.id>"Confirm Action"</h2>
///         <p id=dialog.description_props.id>"Are you sure you want to proceed?"</p>
///         <button>"Cancel"</button>
///         <button>"Confirm"</button>
///     </div>
/// }
/// ```
#[allow(clippy::needless_pass_by_value)]
pub fn use_dialog(input: UseDialogInput) -> UseDialogReturn {
    let UseDialogInput {
        title,
        description,
        aria_label,
        role: dialog_role,
    } = input;

    let base_id = Uuid::new_v4();
    let dialog_id = format!("dialog-{base_id}");
    let title_id = format!("dialog-title-{base_id}");
    let description_id = format!("dialog-description-{base_id}");

    // If aria-label is provided, suppress aria-labelledby (the caller sets
    // aria-label on the element directly). Otherwise, link to the title element.
    let aria_labelledby = if aria_label.is_some() {
        None
    } else if title.is_some() {
        Some(title_id.clone())
    } else {
        None
    };

    let aria_describedby = if description.is_some() {
        Some(description_id.clone())
    } else {
        None
    };

    let role = match dialog_role {
        DialogRole::Dialog => "dialog",
        DialogRole::AlertDialog => "alertdialog",
    };

    // Track whether we're in the middle of the iOS Safari VoiceOver refocus
    // workaround. While refocusing, blur events should not propagate to
    // parent overlays (which might close popovers).
    let is_refocusing = StoredValue::new(false);

    // Capture the dialog element for focus-on-mount behavior.
    let element = CapturedElement::new();

    // Focus the dialog on mount, unless a child element is already focused.
    // This mirrors react-aria's useEffect in useDialog.
    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        if let Some(el) = element.get() {
            let el: &web_sys::Element = &el;

            // Check if the dialog already contains the active element.
            let already_focused = web_sys::window()
                .and_then(|w| w.document())
                .and_then(|d| d.active_element())
                .is_some_and(|active| el.contains(Some(&active)));

            if !already_focused {
                focus_safely(el);

                // Safari on iOS does not move the VoiceOver cursor to the dialog
                // or announce that it has opened until it has rendered. A workaround
                // is to wait for half a second, then blur and re-focus the dialog.
                let el_clone = el.clone();
                set_timeout(
                    move || {
                        // Check that the dialog is still focused, or focus was lost to body.
                        let current_active = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.active_element());
                        let body = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.body())
                            .map(web_sys::Element::from);

                        let is_still_focused = current_active
                            .as_ref()
                            .is_some_and(|a| *a == el_clone || body.as_ref() == Some(a));

                        if is_still_focused {
                            is_refocusing.set_value(true);
                            if let Some(html_el) =
                                wasm_bindgen::JsCast::dyn_ref::<web_sys::HtmlElement>(&el_clone)
                            {
                                html_el.blur().ok();
                            }
                            focus_safely(&el_clone);
                            is_refocusing.set_value(false);
                        }
                    },
                    Duration::from_millis(500),
                );
            }
        }
    });

    // Prevent blur events from reaching parent overlays (e.g., useOverlay)
    // during the iOS Safari VoiceOver refocus workaround.
    let handle_blur = move |e: FocusEvent| {
        if is_refocusing.get_value() {
            e.stop_propagation();
        }
    };

    UseDialogReturn {
        dialog_props: (
            Attr(attr::Id, dialog_id.clone()),
            Attr(attr::Role, role),
            Attr(attr::AriaLabelledby, aria_labelledby),
            Attr(attr::AriaDescribedby, aria_describedby),
            Attr(attr::Tabindex, "-1"),
            on(ev::blur, handle_blur).into_cloneable(),
            element.attr(),
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
