use leptonic::{
    components::prelude::*,
    hooks::*,
    utils::{classes::Classes, css::em},
};
use leptos::prelude::*;

#[component]
pub fn HasTabbableChildDemo() -> impl IntoView {
    let (show_button, set_show_button) = signal(true);
    let (show_input, set_show_input) = signal(true);
    let (disabled, set_disabled) = signal(false);

    let UseHasTabbableChildReturn {
        has_tabbable_child,
        props,
    } = use_has_tabbable_child(UseHasTabbableChildInput {
        is_disabled: disabled.into(),
    });

    // The container is a tab stop only while it has no tabbable children.
    let container_tabindex = move || if has_tabbable_child.get() { -1 } else { 0 };

    view! {
        <Stack orientation=StackOrientation::Vertical spacing=em(0.5) classes="demo-mb-1">
            <Checkbox state=(show_button, set_show_button) classes="demo-form-row">"Show button"</Checkbox>
            <Checkbox state=(show_input, set_show_input) classes="demo-form-row">"Show input"</Checkbox>
            <Checkbox state=(disabled, set_disabled) classes="demo-form-row">"Disabled"</Checkbox>
        </Stack>

        <div
            {..props.into_attrs()}
            tabindex=container_tabindex
            class=Classes::from("demo-focus-scope")
        >
            <p class=Classes::from("demo-container-title")>
                "Container (tabindex=" {container_tabindex} ")"
            </p>
            <div class=Classes::from("demo-focus-row")>
                <Show when=move || show_button.get()>
                    <button class=Classes::from("demo-focus-item")>"Tabbable button"</button>
                </Show>
                <Show when=move || show_input.get()>
                    <input type="text" placeholder="Tabbable input" class=Classes::from("demo-focus-item")/>
                </Show>
            </div>
        </div>

        <p>
            "Has tabbable child: "
            <strong class=Classes::builder().with_toggle(has_tabbable_child, "demo-state-active", "demo-state-inactive").build()>
                { move || if has_tabbable_child.get() { "true" } else { "false" } }
            </strong>
        </p>
    }
}
