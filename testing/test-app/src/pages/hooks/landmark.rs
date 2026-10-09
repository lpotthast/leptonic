use leptonic::{
    CapturedElement, IntoAttrs,
    hooks::landmark::{LandmarkRole, UseLandmarkInput, UseLandmarkReturn, use_landmark},
};
use leptos::prelude::*;

/// A landmark of `role` named `label` around `children`.
#[component]
fn Landmark(
    role: LandmarkRole,
    #[prop(into)] label: Signal<String>,
    #[prop(into)] id: String,
    children: Children,
) -> impl IntoView {
    let element = CapturedElement::new();
    let UseLandmarkReturn { props } = use_landmark(UseLandmarkInput {
        element,
        role,
        aria_label: label.into(),
        aria_labelledby: None,
        focus: None,
    });
    view! {
        <div {..props.into_attrs()} {..element.attr()} id=id>
            {children()}
        </div>
    }
}

/// Landmarks (react-aria's `useLandmark.test.tsx` setups): a navigation with three links, a main
/// with a text field, a region inside an `aria-hidden` element (skipped) and a region shown by
/// `#test-lm-toggle`. `#test-lm-rename` renames the main landmark. `#test-lm-inert-toggle` shows
/// a region inside an `inert` element (as the page outside a modal), after the main landmark.
#[component]
pub fn PageHookLandmark() -> impl IntoView {
    let extra = RwSignal::new(false);
    let main_label = RwSignal::new("Content".to_owned());
    let inert_region = RwSignal::new(false);
    view! {
        <h1>"Landmarks"</h1>
        <Landmark role=LandmarkRole::Navigation label="Site" id="test-lm-nav">
            <ul>
                <li>
                    <a href="#home" id="test-lm-home">
                        "Home"
                    </a>
                </li>
                <li>
                    <a href="#about" id="test-lm-about">
                        "About"
                    </a>
                </li>
                <li>
                    <a href="#contact" id="test-lm-contact">
                        "Contact"
                    </a>
                </li>
            </ul>
        </Landmark>
        <Landmark role=LandmarkRole::Main label=main_label id="test-lm-main">
            <input id="test-lm-name" aria-label="First name" />
            <div aria-hidden="true">
                <Landmark role=LandmarkRole::Region label="Hidden" id="test-lm-hidden">
                    <button tabindex="-1">"Hidden"</button>
                </Landmark>
            </div>
            <Show when=move || extra.get()>
                <Landmark role=LandmarkRole::Region label="Extra" id="test-lm-extra">
                    <button id="test-lm-extra-button">"Extra"</button>
                </Landmark>
            </Show>
        </Landmark>
        <Show when=move || inert_region.get()>
            <div inert="">
                <Landmark role=LandmarkRole::Region label="Inert" id="test-lm-inert">
                    <button>"Inert"</button>
                </Landmark>
            </div>
        </Show>
        <button
            id="test-lm-inert-toggle"
            on:click=move |_| inert_region.update(|shown| *shown = !*shown)
        >
            "Toggle inert region"
        </button>
        <button id="test-lm-toggle" on:click=move |_| extra.update(|extra| *extra = !*extra)>
            "Toggle region"
        </button>
        <button id="test-lm-rename" on:click=move |_| main_label.set("Article".to_owned())>
            "Rename main"
        </button>
    }
}
