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
        disabled: disabled.into(),
    });

    view! {
        <Stack orientation=StackOrientation::Vertical spacing=em(0.5) attr:style="margin-bottom: 1em;">
            <FormControl classes="demo-form-row">
                <Checkbox checked=show_button set_checked=set_show_button />
                <Label>"Show button"</Label>
            </FormControl>
            <FormControl classes="demo-form-row">
                <Checkbox checked=show_input set_checked=set_show_input />
                <Label>"Show input"</Label>
            </FormControl>
            <FormControl classes="demo-form-row">
                <Checkbox checked=disabled set_checked=set_disabled />
                <Label>"Disabled"</Label>
            </FormControl>
        </Stack>

        <div {..props.into_attrs()}>
            <p style="margin: 0 0 1em 0; font-weight: bold;">"Container"</p>
            <Stack orientation=StackOrientation::Horizontal spacing=em(0.5)>
                <Show when=move || show_button.get()>
                    <button class=Classes::from("demo-btn")>
                        "Tabbable Button"
                    </button>
                </Show>
                <Show when=move || show_input.get()>
                    <input
                        type="text"
                        placeholder="Tabbable input"
                        class=Classes::from("demo-input")
                    />
                </Show>
            </Stack>
        </div>

        <p>
            "Has tabbable child: "
            <strong class=Classes::builder().with_toggle(has_tabbable_child, "demo-state-active", "demo-state-inactive").build()>
                { move || if has_tabbable_child.get() { "true" } else { "false" } }
            </strong>
        </p>
    }
}
