use leptos::{context::Provider, portal::Portal, prelude::*};

use super::dialog::DialogTriggerContext;
use super::focus_scope::FocusScope;
use crate::{
    hooks::{
        IntoAttrs, OverlayTriggerState, UseModalBackdropInput, UseModalBackdropReturn,
        UseModalInput, UseModalReturn, UseOverlayAttrs, use_modal, use_modal_backdrop,
    },
    utils::{classes::Classes, styles::Styles},
};

/// Context provided by [`ModalBackdrop`] for [`ModalContent`].
#[derive(Clone, Copy)]
struct ModalBackdropContext {
    modal_props_attrs: StoredValue<UseOverlayAttrs>,
}

/// Backdrop overlay for a modal. Provides dismiss behavior (Escape key, outside click)
/// and scroll prevention via `use_modal_backdrop`.
///
/// Must contain a [`ModalContent`] child for focus trapping and `aria-modal`.
///
/// # Example
///
/// ```ignore
/// let is_open = RwSignal::new(false);
/// <ModalBackdrop state=is_open is_dismissable=true>
///     <ModalContent>
///         "Modal content here"
///     </ModalContent>
/// </ModalBackdrop>
/// ```
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn ModalBackdrop(
    /// Whether the modal is open: app state (`state=rw_signal`, `state=(read, write)`) or an
    /// [`OverlayTriggerState`]. Escape and (when dismissable) outside interaction close it.
    /// Default: the surrounding [`DialogTrigger`](super::dialog::DialogTrigger)'s state.
    #[prop(into, optional)]
    state: Option<OverlayTriggerState>,

    /// Whether clicking outside or pressing Escape closes the modal.
    #[prop(default = false)]
    is_dismissable: bool,

    /// Whether Escape key dismiss is disabled (even when dismissable).
    #[prop(default = false)]
    is_keyboard_dismiss_disabled: bool,

    /// Filter for which outside interactions should close the modal.
    #[prop(optional)]
    should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    children: ChildrenFn,
) -> impl IntoView {
    let state = state
        .or_else(|| use_context::<DialogTriggerContext>().map(|ctx| ctx.state))
        .expect("a <ModalBackdrop> needs a `state` or a surrounding <DialogTrigger>");
    let UseModalBackdropReturn { modal_props, id: _ } = use_modal_backdrop(UseModalBackdropInput {
        is_dismissable,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        ..UseModalBackdropInput::new(state)
    });
    let is_open = state.is_open;

    let modal_props_attrs = StoredValue::new(modal_props.into_attrs());

    // Store children, classes, styles in StoredValue (Copy) so Show's Fn closure can call it repeatedly.
    let children = StoredValue::new(children);
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);

    // The context reaches only this backdrop's content: with several backdrops side by side, each
    // `ModalContent` gets its own backdrop's props.
    view! {
        <Provider value=ModalBackdropContext { modal_props_attrs }>
            <Portal>
                <Show when=move || is_open.get()>
                    <div class=classes.get_value().add("leptonic-modal-backdrop") style=styles.get_value()>
                        {(children.get_value())()}
                    </div>
                </Show>
            </Portal>
        </Provider>
    }
}

/// The modal panel. Wraps children with [`FocusScope`] for focus trapping
/// and applies `aria-modal` via `use_modal`.
///
/// Must be a child of [`ModalBackdrop`].
#[component]
pub fn ModalContent(
    /// Whether to trap focus within the modal.
    #[prop(default = true)]
    contain_focus: bool,

    /// Whether to restore focus to the previously focused element on close.
    #[prop(default = true)]
    restore_focus: bool,

    /// Whether to auto-focus the first focusable element.
    #[prop(default = true)]
    auto_focus: bool,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<ModalBackdropContext>();

    let UseModalReturn { modal_props } = use_modal(UseModalInput { is_disabled: false });

    view! {
        <FocusScope contain=contain_focus restore_focus=restore_focus auto_focus=auto_focus>
            <div {..ctx.modal_props_attrs.get_value()} {..modal_props.into_attrs()} class=classes style=styles>
                {children()}
            </div>
        </FocusScope>
    }
}
