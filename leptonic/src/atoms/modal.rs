// Upstream: react-aria-components/src/Modal.tsx @ 99e6102368
// Upstream: react-aria-components/test/Modal.browser.test.tsx @ 99e6102368
// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
use leptos::{context::Provider, portal::Portal, prelude::*};
use leptos_classes::Classes;
use send_wrapper::SendWrapper;

use super::{
    dialog::DialogTriggerContext, dismiss_button::DismissButton, focus_scope::FocusScope,
    press::ClearTriggerContexts,
};
use crate::{
    CapturedElement, IntoAttrs, Out,
    hooks::{
        animation::{
            UseEnterAnimationInput, UseExitAnimationInput, use_enter_animation, use_exit_animation,
        },
        modal::{UseModalBackdropInput, UseModalBackdropReturn, use_modal_backdrop},
        overlay::{OverlayFocusContain, UseOverlayAttrs},
    },
    utils::{
        data_attributes::flag, default_class::with_default_class, styles::Styles,
        use_viewport_size::use_viewport_size,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `ModalBackdrop` is react-aria-components' `ModalOverlay`, `ModalContent` its `Modal`; a
//   `ModalContent` must be inside a `ModalBackdrop` (react-aria-components' `Modal` renders its
//   own overlay without one).
// - The open state comes from a surrounding `DialogTrigger`, or is passed directly: `is_open` +
//   `set_open` (C4), `default_open`, `on_open_change`.
// - Render props become `data-entering`/`data-exiting` plus plain children.
// - `ModalContent` wraps its content in a `FocusScope` (`contain_focus`, `restore_focus`,
//   `auto_focus`); react-aria-components' `Overlay` does that for the modal. `auto_focus` (off by
//   default, as there) focuses the first focusable element instead of the dialog.
//
// ## DIFFERENT BEHAVIOR
// - No `aria-modal` (as react-aria-components, WebKit bug 211934): the inert content outside
//   (`use_modal_backdrop`) makes the modal modal.
//
// - `on_enter`/`on_exit` are `Callback`s of the element (react-aria-components: functions that
//   may return a promise); Web Animations they start are awaited like CSS ones.
//
// ## OMITTED FEATURES
// - `UNSTABLE_portalContainer`, `UNSTABLE_deferUntilEntered`.
// - Deferring the reveal until an on-screen keyboard opened by an auto-focused input finished
//   its transition (`runAfterKeyboard`): leptonic doesn't track the on-screen keyboard yet.
//
// =============================================================================

/// Context provided by [`ModalBackdrop`] for [`ModalContent`].
#[derive(Clone, Copy)]
struct ModalBackdropContext {
    modal_props_attrs: StoredValue<UseOverlayAttrs>,
    /// The modal element, whose exit animations the backdrop waits for.
    modal: CapturedElement,
    is_exiting: Signal<bool>,
    /// Whether interacting outside closes the modal: then it gets a dismiss button for screen
    /// reader users.
    is_dismissable: Signal<bool>,
    close: Callback<()>,
    /// The `ModalContent`'s `on_exit`, called by the modal's exit animation.
    on_modal_exit: StoredValue<Option<Callback<SendWrapper<web_sys::Element>>>>,
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
/// <ModalBackdrop is_open set_open=is_open is_dismissable=true>
///     <ModalContent>
///         "Modal content here"
///     </ModalContent>
/// </ModalBackdrop>
/// ```
///
/// Data attributes: `data-entering`, `data-exiting`. CSS variables (as react-aria-components'
/// `ModalOverlay`): `--visual-viewport-width`, `--visual-viewport-height`, `--page-width`,
/// `--page-height`.
///
/// Default class: `leptonic-ModalBackdrop`.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn ModalBackdrop(
    /// Whether the modal is open (controlled): a value or any signal. Default: the surrounding
    /// `DialogTrigger`'s state.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the open state (closing): an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,
    /// Whether it starts open, with its own state. Ignored with `is_open`.
    #[prop(optional, into)]
    default_open: Option<bool>,
    /// Called when it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,

    /// Whether clicking outside closes the modal. (Escape closes it unless
    /// `is_keyboard_dismiss_disabled`.) A dismissable modal starts with a visually hidden dismiss
    /// button for screen reader users.
    #[prop(into, optional)]
    is_dismissable: Signal<bool>,

    /// Whether Escape key dismiss is disabled (even when dismissable).
    #[prop(into, optional)]
    is_keyboard_dismiss_disabled: Signal<bool>,

    /// Filter for which outside interactions should close the modal.
    #[prop(optional)]
    should_close_on_interact_outside: Option<crate::hooks::overlay::InteractOutsideFilter>,

    /// Called with the backdrop element when it starts entering (e.g. to start a Web Animation,
    /// which the entry waits for like for CSS animations).
    #[prop(into, optional)]
    on_enter: Option<Callback<SendWrapper<web_sys::Element>>>,

    /// Called with the backdrop element when it starts exiting; it stays rendered until the
    /// animations started then finished.
    #[prop(into, optional)]
    on_exit: Option<Callback<SendWrapper<web_sys::Element>>>,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    children: ChildrenFn,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ModalBackdrop", classes);
    let state = super::dialog::overlay_open_state(
        is_open,
        set_open,
        default_open,
        on_open_change,
        use_context::<DialogTriggerContext>(),
    );
    let UseModalBackdropReturn { modal_props, id: _ } = use_modal_backdrop(UseModalBackdropInput {
        is_dismissable,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        state,
        is_entering: Signal::stored(false),
    });
    let is_open = state.is_open;

    // Backdrop and modal stay rendered while either's exit animations run (`data-exiting`).
    let (backdrop, modal) = (CapturedElement::new(), CapturedElement::new());
    let on_modal_exit = StoredValue::new(None::<Callback<SendWrapper<web_sys::Element>>>);
    let exiting = |element, on_exit| {
        use_exit_animation(UseExitAnimationInput {
            element,
            is_open,
            on_exit,
        })
        .is_exiting
    };
    let (is_backdrop_exiting, is_modal_exiting) = (
        exiting(backdrop, on_exit),
        exiting(
            modal,
            Some(Callback::new(move |element| {
                if let Some(on_exit) = on_modal_exit.get_value() {
                    on_exit.run(element);
                }
            })),
        ),
    );
    let is_exiting = Signal::derive(move || is_backdrop_exiting.get() || is_modal_exiting.get());

    let modal_props_attrs = StoredValue::new(modal_props.into_attrs());

    // Store children, classes, styles in StoredValue (Copy) so Show's Fn closure can call it repeatedly.
    let children = StoredValue::new(children);
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);

    // The context reaches only this backdrop's content: with several backdrops side by side, each
    // `ModalContent` gets its own backdrop's props.
    view! {
        <Provider value=ModalBackdropContext {
            modal_props_attrs,
            modal,
            is_exiting,
            is_dismissable,
            close: Callback::new(move |()| state.close()),
            on_modal_exit,
        }>
            // No portal container while closed: a modal would make it inert.
            <Show when=move || is_open.get() || is_exiting.get()>
                {
                    // Per opening: the entry of this opening's element.
                    let entering = CapturedElement::new();
                    let is_entering = use_enter_animation(UseEnterAnimationInput {
                        element: entering,
                        is_ready: Signal::stored(true),
                        on_enter,
                    })
                    .is_entering;
                    // As react-aria-components' `ModalOverlay`: the visual viewport's and the
                    // page's size, for styling (e.g. a backdrop covering the page, a modal
                    // fitting above the on-screen keyboard). Followed while open only.
                    let viewport = use_viewport_size();
                    let size_styles = move || {
                        let viewport = viewport.get();
                        let page = page_size();
                        Styles::new()
                            .add_unchecked(
                                "--visual-viewport-width",
                                format!("{}px", viewport.width),
                            )
                            .add_unchecked(
                                "--visual-viewport-height",
                                format!("{}px", viewport.height),
                            )
                            .add_optional_unchecked(
                                "--page-width",
                                page.map(|(width, _)| format!("{width}px")),
                            )
                            .add_optional_unchecked(
                                "--page-height",
                                page.map(|(_, height)| format!("{height}px")),
                            )
                    };
                    view! {
                        <Portal>
                            <div
                                {..backdrop.attr().chain(entering.attr())}
                                class=classes.get_value()
                                style=move || size_styles().merge(styles.get_value())
                                data-entering=flag(is_entering)
                                data-exiting=flag(is_exiting)
                            >
                                // Pressing in the modal must not toggle it through the trigger's
                                // responder. A dialog inside switches on this modal's containment
                                // (which it has anyway), not that of an overlay around it.
                                <ClearTriggerContexts>
                                    <Provider value=OverlayFocusContain::new()>
                                        {(children.get_value())()}
                                    </Provider>
                                </ClearTriggerContexts>
                            </div>
                        </Portal>
                    }
                }
            </Show>
        </Provider>
    }
}

/// The modal panel. Wraps children with [`FocusScope`] for focus trapping and restoring. The
/// content outside is inert while it is open (its `ModalBackdrop`), which makes it modal; like
/// react-aria-components, it sets no `aria-modal`. Put a [`Dialog`](super::dialog::Dialog) in it:
/// the dialog takes the focus when the modal opens. In a dismissable [`ModalBackdrop`], it starts
/// with a visually hidden [`DismissButton`] for screen reader users who can't press Escape
/// (VoiceOver on iOS).
///
/// Must be a child of [`ModalBackdrop`].
///
/// Data attributes: `data-entering`, `data-exiting`.
///
/// Default class: `leptonic-ModalContent`.
#[component]
pub fn ModalContent(
    /// Whether to trap focus within the modal.
    #[prop(into, default = Signal::stored(true))]
    contain_focus: Signal<bool>,

    /// Whether to restore focus to the previously focused element on close. Read when the modal
    /// opens.
    #[prop(into, default = Signal::stored(true))]
    restore_focus: Signal<bool>,

    /// Whether to focus the first focusable element when the modal opens, instead of the dialog
    /// inside (react-aria-components: the dialog). Read when the modal opens. Default: `false`.
    #[prop(into, optional)]
    auto_focus: Signal<bool>,

    /// Called with the modal element when it starts entering (e.g. to start a Web Animation,
    /// which the entry waits for like for CSS animations).
    #[prop(into, optional)]
    on_enter: Option<Callback<SendWrapper<web_sys::Element>>>,

    /// Called with the modal element when it starts exiting; the backdrop stays rendered until
    /// the animations started then finished.
    #[prop(into, optional)]
    on_exit: Option<Callback<SendWrapper<web_sys::Element>>>,

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,

    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ModalContent", classes);
    let Some(ctx) = use_context::<ModalBackdropContext>() else {
        crate::utils::dev_warn!("A <ModalContent> must be inside a <ModalBackdrop>.");
        return ().into_any();
    };
    ctx.on_modal_exit.set_value(on_exit);

    let entering = CapturedElement::new();
    let is_entering = use_enter_animation(UseEnterAnimationInput {
        element: entering,
        is_ready: Signal::stored(true),
        on_enter,
    })
    .is_entering;
    let close = ctx.close;

    view! {
        <FocusScope
            // Not while exiting: the page is usable again.
            contain=Signal::derive(move || contain_focus.get() && !ctx.is_exiting.get())
            restore_focus=restore_focus.get_untracked()
            auto_focus=auto_focus.get_untracked()
        >
            <div
                {..ctx.modal_props_attrs.get_value()}
                {..ctx.modal.attr().chain(entering.attr())}
                class=classes
                style=styles
                data-entering=flag(is_entering)
                data-exiting=flag(ctx.is_exiting)
            >
                {move || {
                    ctx.is_dismissable.get().then(|| view! { <DismissButton on_dismiss=close /> })
                }}
                {children()}
            </div>
        </FocusScope>
    }
    .into_any()
}

/// The page's scrollable size (react-aria-components' `ModalOverlay`), without fractional parts
/// (which make Firefox add scrollbars). `None` during server-side rendering.
fn page_size() -> Option<(f64, f64)> {
    let document = leptos_use::use_document();
    let document = document.as_ref()?;
    let scrolling = document
        .body()
        .map(web_sys::Element::from)
        .filter(|body| crate::utils::scroll::is_scrollable(body, false))
        .or_else(|| document.scrolling_element())
        .or_else(|| document.document_element())?;
    let rect = scrolling.get_bounding_client_rect();
    Some((
        f64::from(scrolling.scroll_width()) - rect.width() % 1.0,
        f64::from(scrolling.scroll_height()) - rect.height() % 1.0,
    ))
}
