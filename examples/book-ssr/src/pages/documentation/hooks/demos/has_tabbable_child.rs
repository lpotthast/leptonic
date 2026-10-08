use leptonic::{atoms::checkbox::{CheckboxButton, CheckboxField}, hooks::*};
use leptos::prelude::*;

#[component]
pub fn HasTabbableChildDemo() -> impl IntoView {
    let show_button = RwSignal::new(true);
    let show_input = RwSignal::new(true);
    let disabled = RwSignal::new(false);

    let UseHasTabbableChildReturn {
        has_tabbable_child,
        props,
    } = use_has_tabbable_child(UseHasTabbableChildInput {
        is_disabled: disabled.into(),
    });

    // The container is a tab stop only while it has no tabbable children.
    let container_tabindex = move || if has_tabbable_child.get() { -1 } else { 0 };

    view! {
        <div role="group" aria-label="Attachments" class="demo-focus-scope" tabindex={container_tabindex} {..props.into_attrs()}>
            <Show when=move || show_button.get() fallback=|| view! { <p class="demo-focus-hint">"No actions."</p> }>
                <button type="button" class="demo-focus-item">"Add file"</button>
            </Show>
            <Show when=move || show_input.get()>
                <label class="demo-focus-label">
                    "Note "
                    <input type="text" class="demo-focus-item"/>
                </label>
            </Show>
        </div>

        <p class="demo-status">
            {move || if has_tabbable_child.get() {
                "The group has a tabbable child, so Tab skips the group itself (tabindex -1)."
            } else {
                "The group has no tabbable child, so it is a tab stop itself (tabindex 0)."
            }}
        </p>

        <div class="demo-controls">
            <CheckboxField is_selected=show_button set_selected=show_button>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Show button"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=show_input set_selected=show_input>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Show field"
                </CheckboxButton>
            </CheckboxField>
            <CheckboxField is_selected=disabled set_selected=disabled>
                <CheckboxButton classes="demo-check">
                    <span class="demo-check-box" aria-hidden="true"></span>
                    "Disabled"
                </CheckboxButton>
            </CheckboxField>
        </div>
    }
}
