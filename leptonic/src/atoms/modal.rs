use leptos::{context::Provider, portal::Portal, prelude::*};

use super::{dialog::DialogTriggerContext, focus_scope::FocusScope, press::ClearTriggerContexts};
use crate::{
    Out,
    hooks::{
        IntoAttrs, OverlayFocusContain, UseEnterAnimationInput, UseExitAnimationInput,
        UseModalBackdropInput, UseModalBackdropReturn, UseModalInput, UseModalReturn,
        UseOverlayAttrs, use_enter_animation, use_exit_animation, use_modal, use_modal_backdrop,
    },
    utils::{
        CapturedElement, classes::Classes, data_attributes::flag,
        default_class::with_default_class, styles::Styles, use_viewport_size::use_viewport_size,
    },
};

/// Context provided by [`ModalBackdrop`] for [`ModalContent`].
#[derive(Clone, Copy)]
struct ModalBackdropContext {
    modal_props_attrs: StoredValue<UseOverlayAttrs>,
    /// The modal element, whose exit animations the backdrop waits for.
    modal: CapturedElement,
    is_exiting: Signal<bool>,
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
    /// `is_keyboard_dismiss_disabled`.)
    #[prop(into, optional)]
    is_dismissable: Signal<bool>,

    /// Whether Escape key dismiss is disabled (even when dismissable).
    #[prop(into, optional)]
    is_keyboard_dismiss_disabled: Signal<bool>,

    /// Filter for which outside interactions should close the modal.
    #[prop(optional)]
    should_close_on_interact_outside: Option<crate::hooks::InteractOutsideFilter>,

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
    let exiting = |element| {
        use_exit_animation(UseExitAnimationInput {
            element,
            is_open,
            on_exit: None,
        })
        .is_exiting
    };
    let (is_backdrop_exiting, is_modal_exiting) = (exiting(backdrop), exiting(modal));
    let is_exiting = Signal::derive(move || is_backdrop_exiting.get() || is_modal_exiting.get());

    let modal_props_attrs = StoredValue::new(modal_props.into_attrs());

    // As react-aria-components' `ModalOverlay`: the visual viewport's and the page's size, for
    // styling (e.g. a backdrop covering the page, a modal fitting above the on-screen keyboard).
    let viewport = use_viewport_size();
    let size_styles = move || {
        let viewport = viewport.get();
        let page = page_size();
        Styles::new()
            .add_unchecked("--visual-viewport-width", format!("{}px", viewport.width))
            .add_unchecked("--visual-viewport-height", format!("{}px", viewport.height))
            .add_optional_unchecked("--page-width", page.map(|(width, _)| format!("{width}px")))
            .add_optional_unchecked(
                "--page-height",
                page.map(|(_, height)| format!("{height}px")),
            )
    };

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
            }>
                // No portal container while closed: a modal would make it inert.
                <Show when=move || is_open.get() || is_exiting.get()>
                    {
                        // Per opening: the entry of this opening's element.
                        let entering = CapturedElement::new();
                        let is_entering = use_enter_animation(UseEnterAnimationInput {
    element: entering,
    is_ready: Signal::stored(true),
    on_enter: None,
    })
                            .is_entering;
                        view! {
                    <Portal>
                        <div
                            {..backdrop.attr().chain(entering.attr())}
                            class=classes.get_value().add("leptonic-modal-backdrop")
                            style=move || size_styles().merge(styles.get_value())
                            data-entering=flag(is_entering)
                            data-exiting=flag(is_exiting)
                        >
                            // Pressing in the modal must not toggle it through the trigger's responder.
                            // A dialog inside switches on this modal's containment (which it has
                            // anyway), not that of an overlay around it.
                            <ClearTriggerContexts>
                                <Provider value=OverlayFocusContain::new()>{(children.get_value())()}</Provider>
                            </ClearTriggerContexts>
                        </div>
                    </Portal>
                        }
                    }
                </Show>
            </Provider>
        }
}

/// The modal panel. Wraps children with [`FocusScope`] for focus trapping
/// and applies `aria-modal` via `use_modal`.
///
/// Must be a child of [`ModalBackdrop`].
///
/// Default class: `leptonic-ModalContent`.
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
    let classes = with_default_class("leptonic-ModalContent", classes);
    let ctx = expect_context::<ModalBackdropContext>();

    let UseModalReturn { modal_props } = use_modal(UseModalInput { is_disabled: false });
    let entering = CapturedElement::new();
    let is_entering = use_enter_animation(UseEnterAnimationInput {
        element: entering,
        is_ready: Signal::stored(true),
        on_enter: None,
    })
    .is_entering;

    view! {
        <FocusScope
            // Not while exiting: the page is usable again.
            contain=Signal::derive(move || contain_focus && !ctx.is_exiting.get())
            restore_focus=restore_focus
            auto_focus=auto_focus
        >
            <div
                {..ctx.modal_props_attrs.get_value()}
                {..modal_props.into_attrs()}
                {..ctx.modal.attr().chain(entering.attr())}
                class=classes
                style=styles
                data-entering=flag(is_entering)
                data-exiting=flag(ctx.is_exiting)
            >
                {children()}
            </div>
        </FocusScope>
    }
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
