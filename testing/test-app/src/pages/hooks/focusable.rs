use leptonic::{
    IntoAttrs,
    hooks::focus::{UseFocusableInput, use_focusable},
};
use leptos::prelude::*;

use crate::pages::prevent_focus_steal;

#[component]
pub fn PageHookFocusable() -> impl IntoView {
    let (keydown_count, set_keydown_count) = signal(0u32);
    let (keyup_count, set_keyup_count) = signal(0u32);

    let normal = use_focusable(UseFocusableInput {
        is_disabled: Signal::derive(|| false),
        auto_focus: false,
        exclude_from_tab_order: Signal::derive(|| false),
        on_key_down: Some(Callback::new(move |_| {
            set_keydown_count.update(|c| *c += 1);
        })),
        on_key_up: Some(Callback::new(move |_| {
            set_keyup_count.update(|c| *c += 1);
        })),
        ..Default::default()
    });

    let normal_focus_handle = normal.focus_handle;

    let disabled = use_focusable(UseFocusableInput {
        is_disabled: Signal::derive(|| true),
        ..Default::default()
    });

    let excluded = use_focusable(UseFocusableInput {
        is_disabled: Signal::derive(|| false),
        exclude_from_tab_order: Signal::derive(|| true),
        ..Default::default()
    });

    let autofocus = use_focusable(UseFocusableInput {
        is_disabled: Signal::derive(|| false),
        auto_focus: true,
        ..Default::default()
    });

    view! {
        <div id="test-page-hook-focusable">
            <h1>"Focusable Hook Test Page"</h1>

            <section>
                <h2>"Focusable Elements"</h2>
                <div id="test-fcbl-normal" {..normal.props.into_attrs()}>
                    "Normal Focusable"
                </div>

                <div id="test-fcbl-disabled" {..disabled.props.into_attrs()}>
                    "Disabled Focusable"
                </div>

                <div id="test-fcbl-excluded" {..excluded.props.into_attrs()}>
                    "Excluded Focusable"
                </div>

                <button id="test-fcbl-tab-target">"Tab Target"</button>

                <div id="test-fcbl-autofocus" {..autofocus.props.into_attrs()}>
                    "Auto-focus Focusable"
                </div>
            </section>

            <section>
                <h2>"Keyboard Events"</h2>
                <div>
                    "Keydown count: " <span id="test-fcbl-keydown-count">{keydown_count}</span>
                </div>
                <div>"Keyup count: " <span id="test-fcbl-keyup-count">{keyup_count}</span></div>

                <button
                    id="test-fcbl-focus-btn"
                    on:mousedown=prevent_focus_steal
                    on:click=move |_| normal_focus_handle.focus()
                >
                    "Programmatic Focus"
                </button>
            </section>

            <section>
                <h2>"Dynamic Disabled"</h2>
                {
                    let (dynamic_disabled, set_dynamic_disabled) = signal(false);
                    let dynamic = use_focusable(UseFocusableInput {
                        is_disabled: dynamic_disabled.into(),
                        ..Default::default()
                    });

                    view! {
                        <div id="test-fcbl-dynamic" {..dynamic.props.into_attrs()}>
                            "Dynamic Focusable"
                        </div>
                        <button
                            id="test-fcbl-dynamic-toggle"
                            on:click=move |_| set_dynamic_disabled.update(|v| *v = !*v)
                        >
                            "Toggle Disabled"
                        </button>
                    }
                }
            </section>
        </div>
    }
}
