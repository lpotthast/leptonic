use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        dialog::{Dialog, DialogTitle},
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
/// let show = RwSignal::new(false);
///
/// view! {
///     <Button on_press=move |_| show.set(true)>"Open"</Button>
///
///     <Modal is_open=show set_open=show>
///         <ModalHeader><ModalTitle>"Confirm"</ModalTitle></ModalHeader>
///         <ModalBody>"Are you sure?"</ModalBody>
///         <ModalFooter>
///             <ButtonWrapper>
///                 <Button on_press=move |_| show.set(false)>"Close"</Button>
///             </ButtonWrapper>
///         </ModalFooter>
///     </Modal>
/// }
/// ```
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Modal(
    /// Whether the modal is open: a value or any signal.
    #[prop(into)]
    is_open: Signal<bool>,
    /// Receives the open state when Escape or (when dismissable) clicking the backdrop closes the
    /// modal: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into)]
    set_open: Out<bool>,

    /// Whether clicking outside closes the modal (Escape closes it unless
    /// `is_keyboard_dismiss_disabled`). Defaults to `true` (most modals are dismissable).
    #[prop(into, default = Signal::stored(true))]
    is_dismissable: Signal<bool>,

    /// Whether Escape key dismiss is disabled (even when dismissable).
    #[prop(into, optional)]
    is_keyboard_dismiss_disabled: Signal<bool>,

    /// Names the modal when it has no [`ModalTitle`] (which names it automatically).
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,

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
    let aria_label = StoredValue::new(aria_label);
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);
    let children = StoredValue::new(children);

    view! {
        <ModalBackdrop
            is_open=is_open
            set_open=set_open
            is_dismissable=is_dismissable
            is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
        >
            <ModalContent>
                <Dialog
                    aria_label=aria_label.get_value()
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

/// Modal title: a themed [`DialogTitle`], so the modal's dialog is labelled by it (`aria-labelledby`).
#[component]
pub fn ModalTitle(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    view! {
        <DialogTitle classes=classes.add("leptonic-modal-title") styles=styles>
            {children()}
        </DialogTitle>
    }
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
