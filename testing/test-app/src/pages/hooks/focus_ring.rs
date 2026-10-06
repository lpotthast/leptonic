use leptonic::hooks::{IntoAttrs, UseFocusRingInput, use_focus_ring};
use leptos::prelude::*;

#[component]
pub fn PageHookFocusRing() -> impl IntoView {
    let focus_ring = use_focus_ring(UseFocusRingInput {
        is_disabled: Signal::derive(|| false),
        within: false,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let is_focus_visible = focus_ring.is_focus_visible;
    let is_focused = focus_ring.is_focused;

    let focus_ring_within = use_focus_ring(UseFocusRingInput {
        is_disabled: Signal::derive(|| false),
        within: true,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let within_is_focus_visible = focus_ring_within.is_focus_visible;
    let within_is_focused = focus_ring_within.is_focused;

    let disabled_focus_ring = use_focus_ring(UseFocusRingInput {
        is_disabled: Signal::derive(|| true),
        within: false,
        auto_focus: false,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });

    let disabled_is_focus_visible = disabled_focus_ring.is_focus_visible;
    let disabled_is_focused = disabled_focus_ring.is_focused;

    view! {
        <div id="test-page-hook-focus-ring">
            <h1>"Focus Ring Hook Test Page"</h1>

            <section>
                <h2>"Basic Focus Ring"</h2>
                <button id="test-fr-before">"Before"</button>

                <div id="test-fr-target" tabindex="0" {..focus_ring.props.into_attrs()}>
                    "Focus Ring Target"
                </div>

                <div>
                    "Is focus-visible: "
                    <span id="test-fr-is-focus-visible">
                        {move || if is_focus_visible.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Is focused: "
                    <span id="test-fr-is-focused">
                        {move || if is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>

                <button id="test-fr-elsewhere">"Elsewhere"</button>
            </section>

            <section>
                <h2>"Within Mode"</h2>
                <button id="test-fr-within-before">"Within Before"</button>

                <div id="test-fr-within-container" {..focus_ring_within.props.into_attrs()}>
                    <button id="test-fr-within-child-1">"Within Child 1"</button>
                    <button id="test-fr-within-child-2">"Within Child 2"</button>
                </div>

                <div>
                    "Within is focus-visible: "
                    <span id="test-fr-within-is-focus-visible">
                        {move || if within_is_focus_visible.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Within is focused: "
                    <span id="test-fr-within-is-focused">
                        {move || if within_is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>

                <button id="test-fr-within-elsewhere">"Within Elsewhere"</button>
            </section>

            <section>
                <h2>"Disabled Focus Ring"</h2>
                <div
                    id="test-fr-disabled-target"
                    tabindex="0"
                    {..disabled_focus_ring.props.into_attrs()}
                >
                    "Disabled Focus Ring Target"
                </div>

                <div>
                    "Is focused: "
                    <span id="test-fr-disabled-is-focused">
                        {move || if disabled_is_focused.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Is focus-visible: "
                    <span id="test-fr-disabled-is-focus-visible">
                        {move || if disabled_is_focus_visible.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>
        </div>
    }
}
