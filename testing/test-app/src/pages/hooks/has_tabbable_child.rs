use leptonic::hooks::{IntoAttrs, UseHasTabbableChildInput, use_has_tabbable_child};
use leptos::prelude::*;

#[component]
pub fn PageHookHasTabbableChild() -> impl IntoView {
    let (show_child, set_show_child) = signal(true);

    // Section 1: Dynamic toggle
    let htc = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_child = htc.has_tabbable_child;

    // Section 2: No tabbable children
    let htc_none = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_none = htc_none.has_tabbable_child;

    // Section 3: Disabled hook
    let htc_disabled = use_has_tabbable_child(UseHasTabbableChildInput {
        is_disabled: Signal::derive(|| true),
    });
    let has_tabbable_disabled = htc_disabled.has_tabbable_child;

    // Section 4: Deeply nested tabbable child
    let htc_nested = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_nested = htc_nested.has_tabbable_child;

    // Section 5: Attribute mutation (disabled toggle on child button)
    let (child_disabled, set_child_disabled) = signal(false);
    let htc_attr = use_has_tabbable_child(UseHasTabbableChildInput::default());
    let has_tabbable_attr = htc_attr.has_tabbable_child;

    view! {
        <div id="test-page-hook-has-tabbable-child">
            <h1>"Has Tabbable Child Hook Test Page"</h1>

            <section>
                <h2>"Dynamic Toggle"</h2>
                <div id="test-htc-container" {..htc.props.into_attrs()}>
                    <Show when=move || show_child.get()>
                        <button id="test-htc-child-btn">"Child Button"</button>
                    </Show>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-result">
                        {move || if has_tabbable_child.get() { "true" } else { "false" }}
                    </span>
                </div>
                <button id="test-htc-toggle" on:click=move |_| set_show_child.update(|v| *v = !*v)>
                    "Toggle"
                </button>
            </section>

            <section>
                <h2>"No Tabbable Children"</h2>
                <div id="test-htc-none-container" {..htc_none.props.into_attrs()}>
                    <span>"Just a span"</span>
                    <div tabindex="-1">"Focusable but not tabbable"</div>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-none-result">
                        {move || if has_tabbable_none.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>

            <section>
                <h2>"Disabled Hook"</h2>
                <div id="test-htc-disabled-container" {..htc_disabled.props.into_attrs()}>
                    <button>"A button child"</button>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-disabled-result">
                        {move || if has_tabbable_disabled.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>

            <section>
                <h2>"Deeply Nested"</h2>
                <div id="test-htc-nested-container" {..htc_nested.props.into_attrs()}>
                    <div>
                        <div>
                            <button>"Deeply Nested Button"</button>
                        </div>
                    </div>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-nested-result">
                        {move || if has_tabbable_nested.get() { "true" } else { "false" }}
                    </span>
                </div>
            </section>

            <section>
                <h2>"Attribute Mutation"</h2>
                <div id="test-htc-attr-container" {..htc_attr.props.into_attrs()}>
                    <button disabled=move || child_disabled.get()>"Attr Button"</button>
                </div>
                <div>
                    "Has tabbable child: "
                    <span id="test-htc-attr-result">
                        {move || if has_tabbable_attr.get() { "true" } else { "false" }}
                    </span>
                </div>
                <button
                    id="test-htc-attr-toggle"
                    on:click=move |_| set_child_disabled.update(|v| *v = !*v)
                >
                    "Toggle Disabled"
                </button>
            </section>
        </div>
    }
}
