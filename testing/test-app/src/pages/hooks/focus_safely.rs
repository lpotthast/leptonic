use leptonic::{
    focus_safely,
    hooks::focus::{get_modality, use_interaction_modality},
};
use leptos::{html, portal::Portal, prelude::*, svg, web_sys};
use wasm_bindgen::JsCast;

/// `focus_safely` (react-aria's `focusSafely.test.js`): with virtual modality (a click without a
/// pointer, e.g. by a screen reader or a script), focusing waits for transitions; an element
/// removed in the meantime isn't focused. The modality at the time is shown in
/// `#test-focus-safely-modality`.
#[component]
pub fn PageHookFocusSafely() -> impl IntoView {
    // Tracks the interaction modality.
    let _ = use_interaction_modality();
    let target = NodeRef::<html::Button>::new();
    // SVG elements take focus too.
    let svg_target = NodeRef::<svg::Svg>::new();
    let shown = RwSignal::new(true);
    let modality = RwSignal::new(String::new());
    let focus = move |remove: bool| {
        move |_| {
            modality.set(get_modality().map_or_else(|| "None".to_owned(), |m| format!("{m:?}")));
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
            <ShadowTarget />
        </div>
    }
}

/// focusSafely.test.js "focusSafely with Shadow DOM": a target inside a shadow root. The deepest
/// focused element's id is shown in the light DOM.
#[component]
fn ShadowTarget() -> impl IntoView {
    let host = NodeRef::<html::Div>::new();
    let target = NodeRef::<html::Button>::new();
    let shown = RwSignal::new(true);
    let focused = RwSignal::new(String::new());
    let listener = window_event_listener(leptos::ev::focusin, move |e: web_sys::FocusEvent| {
        let id = e
            .composed_path()
            .get(0)
            .dyn_into::<web_sys::Element>()
            .map(|element| element.id())
            .unwrap_or_default();
        focused.set(id);
    });
    on_cleanup(move || listener.remove());
    let focus = move |remove: bool| {
        move |_| {
            if let Some(target) = target.get_untracked() {
                focus_safely(&target);
            }
            if remove {
                shown.set(false);
            }
        }
    };
    view! {
        <div node_ref=host></div>
        <Show when=move || host.get().is_some()>
            <Portal mount=web_sys::Element::from(host.get_untracked().expect("mounted")) use_shadow=true>
                <Show when=move || shown.get()>
                    <button id="test-focus-safely-shadow-target" node_ref=target>
                        "Shadow target"
                    </button>
                </Show>
            </Portal>
        </Show>
        <button id="test-focus-safely-shadow-focus" on:click=focus(false)>
            "Focus the shadow target"
        </button>
        <button id="test-focus-safely-shadow-remove" on:click=focus(true)>
            "Focus and remove the shadow target"
        </button>
        <div>"Focused: " <span id="test-focus-safely-shadow-focused">{focused}</span></div>
    }
}
