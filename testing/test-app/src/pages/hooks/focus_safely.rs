use leptonic::{
    hooks::{UseFocusVisibleInput, get_modality, use_focus_visible},
    utils::focus::focus_safely,
};
use leptos::{html, prelude::*, svg};

/// `focus_safely` (react-aria's `focusSafely.test.js`): with virtual modality (a click without a
/// pointer, e.g. by a screen reader or a script), focusing waits for transitions; an element
/// removed in the meantime isn't focused. The modality at the time is shown in
/// `#test-focus-safely-modality`.
#[component]
pub fn PageHookFocusSafely() -> impl IntoView {
    // Tracks the interaction modality.
    let _ = use_focus_visible(UseFocusVisibleInput::default());
    let target = NodeRef::<html::Button>::new();
    // SVG elements take focus too.
    let svg_target = NodeRef::<svg::Svg>::new();
    let shown = RwSignal::new(true);
    let modality = RwSignal::new(String::new());
    let focus = move |remove: bool| {
        move |_| {
            modality.set(format!("{:?}", get_modality()));
            if let Some(target) = target.get_untracked() {
                focus_safely(&target);
            }
            if remove {
                shown.set(false);
            }
        }
    };

    view! {
        <div id="test-page-hook-focus-safely">
            <h1>"focus_safely"</h1>
            <Show when=move || shown.get()>
                <button id="test-focus-safely-target" node_ref=target>
                    "Target"
                </button>
            </Show>
            <button id="test-focus-safely-focus" on:click=focus(false)>
                "Focus the target"
            </button>
            <button id="test-focus-safely-remove" on:click=focus(true)>
                "Focus and remove the target"
            </button>
            <svg id="test-focus-safely-svg" tabindex="0" width="24" height="24" node_ref=svg_target>
                <circle cx="12" cy="12" r="10" />
            </svg>
            <button
                id="test-focus-safely-focus-svg"
                on:click=move |_| {
                    if let Some(svg_target) = svg_target.get_untracked() {
                        focus_safely(&svg_target);
                    }
                }
            >
                "Focus the SVG"
            </button>
            <div>
                "Modality: " <span id="test-focus-safely-modality">{move || modality.get()}</span>
            </div>
        </div>
    }
}
