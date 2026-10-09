use leptonic::{
    IntoAttrs,
    hooks::focus::{FocusWithinEvent, UseFocusWithinInput, use_focus_within},
};
use leptos::{prelude::*, web_sys};
use wasm_bindgen::JsCast;

#[component]
pub fn PageHookFocusWithin() -> impl IntoView {
    let (focus_within_count, set_focus_within_count) = signal(0u32);
    let (blur_within_count, set_blur_within_count) = signal(0u32);

    let focus_within = use_focus_within(UseFocusWithinInput {
        is_disabled: Signal::derive(|| false),
        on_focus_within: Some(Callback::new(move |_| {
            set_focus_within_count.update(|c| *c += 1);
        })),
        on_blur_within: Some(Callback::new(move |_| {
            set_blur_within_count.update(|c| *c += 1);
        })),
        on_focus_within_change: None,
    });

    let is_focus_within = focus_within.is_focus_within;

    // ---- Disabled section ----
    let (disabled_focus_count, set_disabled_focus_count) = signal(0u32);

    let disabled_fw = use_focus_within(UseFocusWithinInput {
        is_disabled: Signal::derive(|| true),
        on_focus_within: Some(Callback::new(move |_| {
            set_disabled_focus_count.update(|c| *c += 1);
        })),
        on_blur_within: None,
        on_focus_within_change: None,
    });

    let disabled_is_focus_within = disabled_fw.is_focus_within;

    // ---- Change callback section ----
    let (change_value, set_change_value) = signal(false);
    let (change_count, set_change_count) = signal(0u32);

    let change_fw = use_focus_within(UseFocusWithinInput {
        is_disabled: Signal::derive(|| false),
        on_focus_within: None,
        on_blur_within: None,
        on_focus_within_change: Some(Callback::new(move |is_within: bool| {
            set_change_value.set(is_within);
            set_change_count.update(|c| *c += 1);
        })),
    });

    // ---- Removal and disabling (useFocusWithin.test.js) ----
    let removal_shown = RwSignal::new(true);
    let removal_disabled = RwSignal::new(false);
    let removal_events = RwSignal::new(Vec::<String>::new());
    let log = move |entry: String| removal_events.update(|events| events.push(entry));
    let removal_fw = use_focus_within(UseFocusWithinInput {
        is_disabled: Signal::derive(|| false),
        on_focus_within: Some(Callback::new(move |_| log("focus".to_owned()))),
        on_blur_within: Some(Callback::new(move |e: FocusWithinEvent| {
            let target = e
                .event
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                .map(|t| t.id())
                .unwrap_or_default();
            log(format!("blur:{target}"));
        })),
        on_focus_within_change: Some(Callback::new(move |is_within: bool| {
            log(format!("change:{is_within}"));
        })),
    });

    view! {
        <div id="test-page-hook-focus-within">
            <h1>"Focus Within Hook Test Page"</h1>

            <section>
                <h2>"Basic Focus Within"</h2>
                <button id="test-fw-before">"Before"</button>

                <div id="test-fw-container" {..focus_within.props.into_attrs()}>
                    <input id="test-fw-input-a" type="text" placeholder="Input A" />
                    <input id="test-fw-input-b" type="text" placeholder="Input B" />
                </div>

                <div>
                    "Is focus-within: "
                    <span id="test-fw-is-focus-within">
                        {move || if is_focus_within.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Focus-within count: "
                    <span id="test-fw-focus-within-count">{focus_within_count}</span>
                </div>
                <div>
                    "Blur-within count: "
                    <span id="test-fw-blur-within-count">{blur_within_count}</span>
                </div>

                <button id="test-fw-outside">"Outside"</button>
            </section>

            <section>
                <h2>"Disabled Focus Within"</h2>
                <div id="test-fw-disabled-container" {..disabled_fw.props.into_attrs()}>
                    <input id="test-fw-disabled-input" type="text" placeholder="Disabled Input" />
                </div>

                <div>
                    "Is focus-within: "
                    <span id="test-fw-disabled-is-focus-within">
                        {move || if disabled_is_focus_within.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>
                    "Focus count: "
                    <span id="test-fw-disabled-focus-count">{disabled_focus_count}</span>
                </div>
            </section>

            <section>
                <h2>"Change Callback"</h2>
                <div id="test-fw-change-container" {..change_fw.props.into_attrs()}>
                    <input id="test-fw-change-input" type="text" placeholder="Change Input" />
                </div>

                <div>
                    "Change value: "
                    <span id="test-fw-change-value">
                        {move || if change_value.get() { "true" } else { "false" }}
                    </span>
                </div>
                <div>"Change count: " <span id="test-fw-change-count">{change_count}</span></div>
            </section>

            <section>
                <h2>"Removal and Disabling"</h2>
                <div id="test-fw-removal-container" {..removal_fw.props.into_attrs()}>
                    <Show when=move || removal_shown.get()>
                        <button id="test-fw-removal-hide" on:click=move |_| removal_shown.set(false)>
                            "Hide"
                        </button>
                    </Show>
                    // Stops its `focusout`, as leptonic's hooks may: no blur reaches the container.
                    <input
                        id="test-fw-removal-quiet"
                        aria-label="Quiet"
                        on:focusout=|e: web_sys::FocusEvent| e.stop_propagation()
                    />
                    <button
                        id="test-fw-removal-disable"
                        disabled=move || removal_disabled.get()
                        on:click=move |_| removal_disabled.set(true)
                    >
                        "Disable"
                    </button>
                </div>
                // Stops `focusin`, as leptonic's hooks do by default.
                <input
                    id="test-fw-removal-outer"
                    aria-label="Outer"
                    on:focusin=|e: web_sys::FocusEvent| e.stop_propagation()
                />
                <div>
                    "Events: "
                    <span id="test-fw-removal-events">{move || removal_events.get().join(",")}</span>
                </div>
            </section>

            <section>
                <h2>"Nested Containers"</h2>
                {
                    let nested_outer_fw = use_focus_within(UseFocusWithinInput {
                        is_disabled: Signal::derive(|| false),
                        on_focus_within: None,
                        on_blur_within: None,
                        on_focus_within_change: None,
                    });
                    let nested_outer_is_fw = nested_outer_fw.is_focus_within;
                    let nested_inner_fw = use_focus_within(UseFocusWithinInput {
                        is_disabled: Signal::derive(|| false),
                        on_focus_within: None,
                        on_blur_within: None,
                        on_focus_within_change: None,
                    });
                    let nested_inner_is_fw = nested_inner_fw.is_focus_within;

                    view! {
                        <div id="test-fw-nested-outer" {..nested_outer_fw.props.into_attrs()}>
                            <div id="test-fw-nested-inner" {..nested_inner_fw.props.into_attrs()}>
                                <input
                                    id="test-fw-nested-input"
                                    type="text"
                                    placeholder="Nested Input"
                                />
                            </div>
                        </div>
                        <div>
                            "Outer is focus-within: "
                            <span id="test-fw-nested-outer-is-focus-within">
                                {move || if nested_outer_is_fw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                        <div>
                            "Inner is focus-within: "
                            <span id="test-fw-nested-inner-is-focus-within">
                                {move || if nested_inner_is_fw.get() { "true" } else { "false" }}
                            </span>
                        </div>
                    }
                }
            </section>
        </div>
    }
}
