use leptonic::{
    CapturedElement,
    hooks::overlay::{HideOutsideBehavior, UseHideOutsideInput, use_hide_outside},
};
use leptos::prelude::*;

/// `use_hide_outside`: a modal overlay (`#test-ho-modal`, `Inert`, rendered while open) and a
/// non-modal one (`#test-ho-nonmodal`, `KeepVisible`, rendered while open, outside the modal as a
/// portaled popover is), next to page content (`#test-ho-content`). Buttons open the modal
/// (`#test-ho-open-modal`) and the non-modal overlay from the page (`#test-ho-open-nonmodal`);
/// inside the modal, buttons close it (`#test-ho-close-modal`) and open the non-modal overlay
/// (`#test-ho-open-nonmodal-inside`), which has its own close button (`#test-ho-close-nonmodal`).
#[component]
pub fn PageHookHideOutside() -> impl IntoView {
    let modal_open = RwSignal::new(false);
    let nonmodal_open = RwSignal::new(false);
    view! {
        <div id="test-page-hook-hide-outside">
            <h1>"use_hide_outside"</h1>
            <p id="test-ho-content">"Page content"</p>
            <button id="test-ho-open-modal" on:click=move |_| modal_open.set(true)>
                "Open modal"
            </button>
            <button id="test-ho-open-nonmodal" on:click=move |_| nonmodal_open.set(true)>
                "Open non-modal"
            </button>
            <Overlay id="test-ho-modal" is_open=modal_open behavior=HideOutsideBehavior::Inert>
                <button id="test-ho-close-modal" on:click=move |_| modal_open.set(false)>
                    "Close modal"
                </button>
                <button id="test-ho-open-nonmodal-inside" on:click=move |_| nonmodal_open.set(true)>
                    "Open non-modal"
                </button>
            </Overlay>
            <Overlay
                id="test-ho-nonmodal"
                is_open=nonmodal_open
                behavior=HideOutsideBehavior::KeepVisible
            >
                <button id="test-ho-close-nonmodal" on:click=move |_| nonmodal_open.set(false)>
                    "Close non-modal"
                </button>
            </Overlay>
        </div>
    }
}

/// An overlay rendered while `is_open`, hiding the page as `behavior` says.
#[component]
fn Overlay(
    id: &'static str,
    is_open: RwSignal<bool>,
    behavior: HideOutsideBehavior,
    children: ChildrenFn,
) -> impl IntoView {
    let overlay = CapturedElement::new();
    use_hide_outside(UseHideOutsideInput {
        is_open: is_open.into(),
        overlay,
        behavior: Signal::stored(behavior),
    });
    view! {
        <Show when=move || is_open.get()>
            <div id={id} {..overlay.attr()}>
                {children()}
            </div>
        </Show>
    }
}
