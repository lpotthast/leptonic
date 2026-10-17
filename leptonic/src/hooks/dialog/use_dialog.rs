// Upstream: react-aria/src/dialog/useDialog.ts @ 99e6102368
// Upstream: react-aria/test/dialog/useDialog.test.js @ 99e6102368
use leptos::{ev, prelude::*};
use leptos_element_capture::{CapturedElement, ElementCaptureAttr};
use web_sys::FocusEvent;

use crate::{
    EventHandler, Id, IntoAttrs, OnEvent, Part,
    utils::{
        aria::AriaRole,
        id::use_id,
        part::Remount,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        slot_id::{SlotProps, use_slot},
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The title and content elements are slots (`Part<SlotProps>`: each mount takes their props,
//   one element at a time): the dialog references them only while they are rendered
//   (react-aria: `useSlotId`).
// - `is_entering` is a signal: focusing waits until it is `false`.
// - `fallback_aria_labelledby` names a dialog without title or label: react-aria-components labels
//   a `DialogTrigger`'s dialog by the trigger in the component, which our hook can't tell apart
//   from the `aria_labelledby` prop (which wins over the title).
//
// ## DIFFERENT BEHAVIOR
// - The missing-title warning runs once after mount (`dev_warn!`), when the slots are known.
// - The content element always gets an id, also when `aria_describedby` replaces it as the
//   description (react-aria: no `contentProps.id` then); it is referenced only by an alert
//   dialog without `aria_describedby`.
//
// =============================================================================

/// The role of a dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DialogRole {
    /// A standard dialog.
    #[default]
    Dialog,
    /// An alert dialog that requires user attention.
    AlertDialog,
}

/// Input of [`use_dialog`].
#[derive(Debug, Clone, Default)]
pub struct UseDialogInput {
    pub role: DialogRole,
    /// Names the dialog when it has no title element. Replaces the title as its name.
    pub aria_label: MaybeProp<String>,
    /// The ids of the elements naming the dialog, instead of its title element.
    pub aria_labelledby: Signal<Option<Id>>,
    /// The ids of the elements describing the dialog. Default for alert dialogs: their content
    /// element.
    pub aria_describedby: Signal<Option<Id>>,
    /// Whether the dialog is still animating in; it is focused once this is `false`.
    pub is_entering: Signal<bool>,
    /// The ids of the elements naming the dialog when it has neither a title element, nor
    /// `aria_label` or `aria_labelledby` (the trigger that opened it).
    pub fallback_aria_labelledby: Signal<Option<Id>>,
}

/// The return value of [`use_dialog`].
#[derive(Debug)]
pub struct UseDialogReturn {
    /// The dialog element.
    pub dialog: Part<UseDialogProps>,
    /// The title element (a heading), which names the dialog while it is rendered.
    pub title: Part<SlotProps>,
    /// The content element, which describes an alert dialog while it is rendered.
    pub content: Part<SlotProps>,
}

/// Props for the dialog element.
#[derive(Debug)]
pub struct UseDialogProps {
    pub id: Id,
    pub role: AriaRole,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub aria_describedby: Signal<Option<Id>>,
    pub tabindex: i32,
    /// Stops focus leaving during the iOS refocus workaround from reaching parent overlays.
    pub on_focusout: EventHandler<FocusEvent>,
    pub element_capture: ElementCaptureAttr,
}

impl Remount for UseDialogProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            role: self.role,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
            tabindex: self.tabindex,
            on_focusout: self.on_focusout.clone(),
            element_capture: self.element_capture.clone(),
        }
    }
}

impl IntoAttrs for UseDialogProps {
    type Attrs = UseDialogAttrs;

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("id", self.id),
            fixed_attr("role", self.role),
            maybe_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
            signal_attr("aria-describedby", self.aria_describedby),
            fixed_attr("tabindex", self.tabindex),
            self.on_focusout.into_on(ev::focusout),
            self.element_capture,
        )
    }
}

/// Attributes for the dialog element.
pub type UseDialogAttrs = (
    FixedAttr,
    FixedAttr,
    SignalAttr<Option<String>>,
    SignalAttr<Option<Id>>,
    SignalAttr<Option<Id>>,
    FixedAttr,
    OnEvent<ev::focusout>,
    ElementCaptureAttr,
);

/// Provides the behavior and accessibility implementation for a dialog: its role, its name (the
/// title element, or `aria_label`), its description (an alert dialog's content), and focusing it
/// on mount.
///
/// Dismissing (Escape, outside clicks) is `use_modal_backdrop`'s job, containing focus the
/// `FocusScope`'s.
///
/// ```ignore
/// let dialog = use_dialog(UseDialogInput {
///     role: DialogRole::AlertDialog,
///     ..UseDialogInput::default()
/// });
///
/// view! {
///     <section {..dialog.dialog.attrs()}>
///         <h2 {..dialog.title.attrs()}>"Delete file?"</h2>
///         <p {..dialog.content.attrs()}>"It is deleted permanently."</p>
///         <button>"Delete"</button>
///     </section>
/// }
/// ```
pub fn use_dialog(input: UseDialogInput) -> UseDialogReturn {
    crate::metrics::subject!();
    let UseDialogInput {
        role: dialog_role,
        aria_label,
        aria_labelledby,
        aria_describedby,
        is_entering,
        fallback_aria_labelledby,
    } = input;

    // A dialog in a non-modal popover makes it contain focus.
    crate::hooks::overlay::use_overlay_focus_contain();

    let dialog_id = use_id("dialog");
    let title = use_slot("dialog-title");
    let content = use_slot("dialog-content");

    // The title names the dialog, unless `aria_label` does.
    let title_id = title.referenced_id;
    let aria_labelledby = Signal::derive(move || {
        aria_labelledby.get().or_else(|| {
            aria_label
                .read()
                .is_none()
                .then(|| title_id.get().or_else(|| fallback_aria_labelledby.get()))
                .flatten()
        })
    });
    // An alert dialog is described by its content.
    let content_id = content.referenced_id;
    let aria_describedby = Signal::derive(move || {
        aria_describedby.get().or_else(|| {
            (dialog_role == DialogRole::AlertDialog)
                .then(|| content_id.get())
                .flatten()
        })
    });

    #[cfg(feature = "ssr")]
    let _ = is_entering;

    let role = match dialog_role {
        DialogRole::Dialog => AriaRole::Dialog,
        DialogRole::AlertDialog => AriaRole::Alertdialog,
    };

    // Track whether we're in the middle of the iOS Safari VoiceOver refocus
    // workaround. While refocusing, blur events should not propagate to
    // parent overlays (which might close popovers).
    let is_refocusing = StoredValue::new(false);

    // Capture the dialog element for focus-on-mount behavior.
    let element = CapturedElement::new();

    // Focus the dialog on mount (or once it has entered), unless a child element is already
    // focused. This mirrors react-aria's useEffect in useDialog.
    Effect::new(move |_| {
        #[cfg(not(feature = "ssr"))]
        if !is_entering.get()
            && let Some(el) = element.get()
        {
            use crate::utils::focus::focus_safely;

            let el: &web_sys::Element = &el;

            // Whether the dialog already contains the active element (also inside shadow roots).
            let active_element = || {
                leptos_use::use_document()
                    .as_ref()
                    .and_then(crate::utils::shadow_dom::get_active_element)
            };
            let already_focused = active_element()
                .is_some_and(|active| crate::utils::shadow_dom::node_contains(el, &active));

            if !already_focused {
                focus_safely(el);

                // Safari on iOS does not move the VoiceOver cursor to the dialog
                // or announce that it has opened until it has rendered. A workaround
                // is to wait for half a second, then blur and re-focus the dialog.
                let el_clone = el.clone();
                let timeout = set_timeout_with_handle(
                    move || {
                        // Check that the dialog is still focused, or focus was lost to body.
                        let body = leptos_use::use_document()
                            .as_ref()
                            .and_then(web_sys::Document::body)
                            .map(web_sys::Element::from);
                        let is_still_focused = active_element()
                            .is_some_and(|a| a == el_clone || body.as_ref() == Some(&a));

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
                    std::time::Duration::from_millis(500),
                );
                // As react-aria's effect cleanup: a closed dialog must not be refocused.
                if let Ok(timeout) = timeout {
                    on_cleanup(move || timeout.clear());
                }
            }
        }
    });

    // Prevent blur events from reaching parent overlays (e.g., useOverlay)
    // during the iOS Safari VoiceOver refocus workaround.
    let handle_blur = move |e: FocusEvent| {
        // Removing the focused dialog fires a blur after its owner was disposed; a disposed
        // dialog isn't refocusing.
        if is_refocusing.try_get_value().unwrap_or(false) {
            e.stop_propagation();
        }
    };

    // A dialog needs a name. Checked once after mount, when the title slot is known.
    #[cfg(debug_assertions)]
    Effect::new(move |warned: Option<bool>| {
        if warned == Some(true) {
            return true;
        }
        let Some(el) = element.get() else {
            return false;
        };
        let named = el.has_attribute("aria-label") || el.has_attribute("aria-labelledby");
        if !named
            && aria_label.get_untracked().is_none()
            && aria_labelledby.get_untracked().is_none()
        {
            crate::utils::dev_warn!(
                "A dialog must have a title for accessibility: render a title element with \
                 `title` (the `DialogTitle` atom), or set `aria_label` or `aria_labelledby`."
            );
        }
        true
    });

    UseDialogReturn {
        dialog: Part::stored(UseDialogProps {
            id: dialog_id,
            role,
            aria_label,
            aria_labelledby,
            aria_describedby,
            tabindex: -1,
            on_focusout: EventHandler::new(handle_blur),
            element_capture: element.attr(),
        }),
        title: title.part,
        content: content.part,
    }
}
