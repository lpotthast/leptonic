use leptonic::{
    hooks::{
        IntoAttrs, LandmarkController, LandmarkRole, UseLandmarkInput, UseLandmarkReturn,
        use_landmark,
    },
    utils::CapturedElement,
};
use leptos::prelude::*;

/// A landmark of `role` named `label` (none when empty) around `children`.
#[component]
fn Landmark(
    role: LandmarkRole,
    #[prop(into)] label: String,
    #[prop(into)] id: String,
    children: Children,
) -> impl IntoView {
    let element = CapturedElement::new();
    let UseLandmarkReturn { props } = use_landmark(UseLandmarkInput {
        element,
        role,
        aria_label: (!label.is_empty()).then_some(label).into(),
        aria_labelledby: None,
        focus: None,
    });
    view! {
        <div {..props.into_attrs()} {..element.attr()} id=id>
            {children()}
        </div>
    }
}

/// Nested landmarks (react-aria's `useLandmark.test.tsx` "goes in dom order with two nested
/// landmarks" setup): a main landmark (`#test-lmn-main`) with a region (`#test-lmn-region-1`), a
/// text field (`#test-lmn-first`), another region (`#test-lmn-region-2`) and a text field
/// (`#test-lmn-last`).
///
/// - `#test-lmn-next`, `#test-lmn-previous`, `#test-lmn-main-button`, `#test-lmn-forward`: a
///   `LandmarkController`'s `focus_next`, `focus_previous`, `focus_main` and `navigate(Forward)`
///   from the focused element (call them with a script's `click()`, which doesn't move the focus).
/// - `#test-lmn-add-unlabelled`: adds two unlabelled navigation landmarks;
///   `#test-lmn-add-same-label`: two navigation landmarks labelled "Same" (duplicate-role
///   warnings).
#[component]
pub fn PageHookLandmarkNested() -> impl IntoView {
    let unlabelled = RwSignal::new(false);
    let same_label = RwSignal::new(false);
    // The controller lives as long as the page (dropping it would stop it).
    let controller = StoredValue::new(None::<std::sync::Arc<LandmarkController>>);
    Effect::new(move |_| {
        controller.set_value(Some(std::sync::Arc::new(LandmarkController::new())));
    });
    let with_controller = move |f: fn(&LandmarkController)| {
        controller.with_value(|controller| {
            if let Some(controller) = controller {
                f(controller);
            }
        });
    };
    view! {
        <h1>"Nested landmarks"</h1>
        <Landmark role=LandmarkRole::Main label="Content" id="test-lmn-main">
            <Landmark role=LandmarkRole::Region label="Region 1" id="test-lmn-region-1">
                <input type="checkbox" id="test-lmn-checkbox-1" aria-label="Checkbox 1" />
            </Landmark>
            <input id="test-lmn-first" aria-label="First name" />
            <Landmark role=LandmarkRole::Region label="Region 2" id="test-lmn-region-2">
                <input type="checkbox" id="test-lmn-checkbox-2" aria-label="Checkbox 2" />
            </Landmark>
            <input id="test-lmn-last" aria-label="Last name" />
        </Landmark>
        <Show when=move || unlabelled.get()>
            <Landmark role=LandmarkRole::Navigation label="" id="test-lmn-nav-1">"Nav 1"</Landmark>
            <Landmark role=LandmarkRole::Navigation label="" id="test-lmn-nav-2">"Nav 2"</Landmark>
        </Show>
        <Show when=move || same_label.get()>
            <Landmark role=LandmarkRole::Navigation label="Same" id="test-lmn-same-1">"Same 1"</Landmark>
            <Landmark role=LandmarkRole::Navigation label="Same" id="test-lmn-same-2">"Same 2"</Landmark>
        </Show>
        <div style="display: none">
            <button id="test-lmn-next" on:click=move |_| with_controller(|c| { c.focus_next(None); })>
                "Next"
            </button>
            <button id="test-lmn-previous" on:click=move |_| with_controller(|c| { c.focus_previous(None); })>
                "Previous"
            </button>
            <button id="test-lmn-main-button" on:click=move |_| with_controller(|c| { c.focus_main(); })>
                "Main"
            </button>
            <button
                id="test-lmn-forward"
                on:click=move |_| with_controller(|c| {
                    c.navigate(leptonic::hooks::LandmarkDirection::Forward, None);
                })
            >
                "Forward"
            </button>
        </div>
        <button id="test-lmn-add-unlabelled" on:click=move |_| unlabelled.set(true)>
            "Add unlabelled navigations"
        </button>
        <button id="test-lmn-add-same-label" on:click=move |_| same_label.set(true)>
            "Add navigations with the same label"
        </button>
    }
}
