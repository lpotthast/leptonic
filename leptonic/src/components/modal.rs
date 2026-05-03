use leptos::prelude::*;

use crate::{
    atoms::{
        dialog::{Dialog, DialogContext},
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::DialogRole,
    utils::{classes::Classes, styles::Styles},
};

/// A themed modal dialog component.
///
/// Composes [`ModalBackdrop`] + [`ModalContent`] + [`Dialog`] atoms for a fully
/// accessible modal with focus trapping, scroll prevention, and dismiss behavior.
///
/// Each modal is self-contained (rendered via `Portal`) — no global `ModalRoot` needed.
///
/// # Example
///
/// ```ignore
/// let (show, set_show) = signal(false);
///
/// view! {
///     <Button on_press=move |_| set_show.set(true)>"Open"</Button>
///
///     <Modal show_when=show on_close=move |_| set_show.set(false)>
///         <ModalHeader><ModalTitle>"Confirm"</ModalTitle></ModalHeader>
///         <ModalBody>"Are you sure?"</ModalBody>
///         <ModalFooter>
///             <ButtonWrapper>
///                 <Button on_press=move |_| set_show.set(false)>"Close"</Button>
///             </ButtonWrapper>
///         </ModalFooter>
///     </Modal>
/// }
/// ```
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Modal(
    /// Reactive signal controlling visibility.
    #[prop(into)]
    show_when: Signal<bool>,

    /// Called when the modal should close (Escape, backdrop click if dismissable).
    #[prop(into)]
    on_close: Callback<()>,

    /// Whether clicking outside / pressing Escape closes the modal.
    /// Defaults to `true` (most modals are dismissable).
    #[prop(default = true)]
    is_dismissable: bool,

    /// Whether Escape key dismiss is disabled (even when dismissable).
    #[prop(default = false)]
    is_keyboard_dismiss_disabled: bool,

    /// Dialog title for `aria-labelledby`. The [`ModalTitle`] component wires this automatically.
    #[prop(into, optional)]
    title: Option<String>,

    /// Dialog description for `aria-describedby`.
    #[prop(into, optional)]
    description: Option<String>,

    /// Accessible label (suppresses `aria-labelledby` when set).
    #[prop(into, optional)]
    aria_label: Option<String>,

    /// Dialog role. Defaults to `Dialog`.
    #[prop(default = DialogRole::Dialog)]
    role: DialogRole,

    /// Additional CSS classes on the modal panel.
    #[prop(into, optional)]
    classes: Classes,

    /// Additional CSS styles on the modal panel.
    #[prop(into, optional)]
    styles: Styles,

    children: ChildrenFn,
) -> impl IntoView {
    let title = StoredValue::new(title);
    let description = StoredValue::new(description);
    let aria_label = StoredValue::new(aria_label);
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);
    let children = StoredValue::new(children);

    view! {
        <ModalBackdrop
            is_open=show_when
            on_close=on_close
            is_dismissable=is_dismissable
            is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
        >
            <ModalContent>
                <Dialog
                    nostrip:title=title.get_value()
                    nostrip:description=description.get_value()
                    nostrip:aria_label=aria_label.get_value()
                    role=role
                >
                    <div class=classes.get_value().add("leptonic-modal") style=styles.get_value()>
                        {(children.get_value())()}
                    </div>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}

/// Modal header section.
#[component]
pub fn ModalHeader(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-modal-header") style=styles>{children()}</div> }
}

/// Modal title that also wires the dialog's `aria-labelledby` ID via [`DialogContext`].
#[component]
pub fn ModalTitle(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = use_context::<DialogContext>();
    let id = ctx.map(|c| c.title_id);
    view! { <div class=classes.add("leptonic-modal-title") style=styles id=id>{children()}</div> }
}

/// Modal body section.
#[component]
pub fn ModalBody(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-modal-body") style=styles>{children()}</div> }
}

/// Modal footer section.
#[component]
pub fn ModalFooter(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! { <div class=classes.add("leptonic-modal-footer") style=styles>{children()}</div> }
}
