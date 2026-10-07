use leptonic::{
    atoms::{
        button::Button,
        dialog::Dialog,
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::{InteractOutsideFilter, IntoAttrs, UseOverlayInput, UseOverlayReturn, use_overlay},
};
use leptos::prelude::*;

/// An overlay of `use_overlay` (react-aria's `useOverlay.test.js` `Example`), shown while open:
/// `#{id}` around the overlay element, with a button `#{id}-inside`. Closing counts in
/// `#{id}-closes`.
#[component]
fn HookOverlay(
    id: &'static str,
    is_open: RwSignal<bool>,
    #[prop(optional)] is_dismissable: bool,
    #[prop(optional)] is_keyboard_dismiss_disabled: bool,
    #[prop(optional)] filter: Option<InteractOutsideFilter>,
) -> impl IntoView {
    let closes = RwSignal::new(0_u32);
    let UseOverlayReturn { props, .. } = use_overlay(UseOverlayInput {
        is_open: is_open.into(),
        on_close: Callback::new(move |()| {
            closes.update(|closes| *closes += 1);
            is_open.set(false);
        }),
        is_dismissable: Signal::stored(is_dismissable),
        should_close_on_blur: Signal::stored(false),
        is_keyboard_dismiss_disabled: Signal::stored(is_keyboard_dismiss_disabled),
        should_close_on_interact_outside: filter,
        group: None,
    });
    view! {
        <div id=id style:display=move || if is_open.get() { "block" } else { "none" }>
            <div {..props.into_attrs()}>
                <button id=format!("{id}-inside")>"Inside " {id}</button>
            </div>
        </div>
        <p>{id} " closes: " <span id=format!("{id}-closes")>{closes}</span></p>
    }
}

/// `use_overlay` (react-aria's `useOverlay.test.js` setups) and nested modals:
/// - `#test-ov-a`: dismissable; interactions with `#test-ov-keep` don't close it
///   (`should_close_on_interact_outside`). Opened by `#test-ov-open-a`.
/// - `#test-ov-b`: not dismissable (Escape still closes it). Opened by `#test-ov-open-b`.
/// - `#test-ov-c`: keyboard dismissal disabled; Escape key presses reaching the page count in
///   `#test-ov-escapes`. Opened by `#test-ov-open-c`.
/// - `#test-ov-d`: dismissable; `#test-ov-open-a-d` opens it together with `#test-ov-a`.
/// - `#test-ov-outside`: a button outside every overlay.
/// - Nested modals: `#test-ov-modal-open` opens a dismissable modal ("Outer") whose
///   `#test-ov-modal-inner-open` opens a second one ("Inner", with `#test-ov-modal-inner-close`).
#[component]
pub fn PageHookOverlay() -> impl IntoView {
    let (a, b, c, d) = (
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
        RwSignal::new(false),
    );
    let escapes = RwSignal::new(0_u32);
    let outer = RwSignal::new(false);
    let inner = RwSignal::new(false);
    let keep = InteractOutsideFilter::new(|target: &leptos::web_sys::Element| {
        target.id() != "test-ov-keep"
    });
    view! {
        <h1>"Overlay"</h1>
        <div on:keydown=move |e: leptos::ev::KeyboardEvent| {
            if e.key() == "Escape" {
                escapes.update(|escapes| *escapes += 1);
            }
        }>
            <button id="test-ov-open-a" on:click=move |_| a.set(true)>"Open A"</button>
            <button id="test-ov-open-b" on:click=move |_| b.set(true)>"Open B"</button>
            <button id="test-ov-open-c" on:click=move |_| c.set(true)>"Open C"</button>
            <button id="test-ov-open-a-d" on:click=move |_| { a.set(true); d.set(true); }>
                "Open A and D"
            </button>
            <button id="test-ov-keep">"Keep"</button>
            <button id="test-ov-outside">"Outside"</button>
            <HookOverlay id="test-ov-a" is_open=a is_dismissable=true filter=keep />
            <HookOverlay id="test-ov-b" is_open=b />
            <HookOverlay id="test-ov-c" is_open=c is_dismissable=true is_keyboard_dismiss_disabled=true />
            <HookOverlay id="test-ov-d" is_open=d is_dismissable=true />
            <p>"Escapes: " <span id="test-ov-escapes">{escapes}</span></p>
        </div>

        <Button attr:id="test-ov-modal-open" on_press=move |_| outer.set(true)>"Open modal"</Button>
        <ModalBackdrop is_open=outer set_open=outer is_dismissable=true classes="test-ov-outer-backdrop">
            <ModalContent>
                <Dialog aria_label="Outer">
                    <Button attr:id="test-ov-modal-inner-open" on_press=move |_| inner.set(true)>
                        "Open inner"
                    </Button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
        <ModalBackdrop is_open=inner set_open=inner is_dismissable=true classes="test-ov-inner-backdrop">
            <ModalContent>
                <Dialog aria_label="Inner">
                    <Button attr:id="test-ov-modal-inner-close" on_press=move |_| inner.set(false)>
                        "Close inner"
                    </Button>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}
