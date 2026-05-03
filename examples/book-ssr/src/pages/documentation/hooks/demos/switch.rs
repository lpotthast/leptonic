use leptonic::hooks::*;
use leptos::prelude::*;
use leptos_classes::Classes;

#[component]
pub fn SwitchDemo() -> impl IntoView {
    let UseSwitchStateReturn {
        is_selected,
        set_selected,
        toggle,
    } = use_switch_state(false);

    let UseSwitchReturn {
        switch_props,
        input_props,
        ..
    } = use_switch(UseSwitchInput {
        is_selected,
        is_disabled: false.into(),
        is_read_only: false.into(),
        name: Some("notifications"),
        value: Some("enabled"),
        on_change: Some(Callback::new(move |v: bool| set_selected.run(v))),
        aria_label: Some("Enable notifications"),
        ..Default::default()
    });

    // Toggle state for the use_toggle demo
    let UseToggleStateReturn {
        is_selected: toggle_selected,
        toggle: do_toggle,
        ..
    } = use_toggle_state(false);

    let UseToggleReturn {
        props: toggle_props,
        ..
    } = use_toggle(UseToggleInput {
        is_selected: toggle_selected,
        is_disabled: false.into(),
        is_read_only: false.into(),
        on_change: None,
        value: None,
    });

    view! {
        <h3 style="margin-top: 0;">"Switch"</h3>
        <label
            {..switch_props.into_attrs()}
            on:click=move |_| toggle.run(())
            style="display: flex; align-items: center; gap: 1em; cursor: pointer; user-select: none;"
        >
            <input type="checkbox" {..input_props.into_attrs()} style="position: absolute; opacity: 0; width: 0; height: 0;" />

            <div style=move || format!(
                "width: 50px; height: 26px; border-radius: 13px; padding: 2px; transition: all 0.2s; {}",
                if is_selected.get() {
                    "background: var(--brand-color);"
                } else {
                    "background: #ccc;"
                }
            )>
                <div style=move || format!(
                    "width: 22px; height: 22px; border-radius: 50%; background: white; transition: all 0.2s; {}",
                    if is_selected.get() {
                        "transform: translateX(24px);"
                    } else {
                        "transform: translateX(0);"
                    }
                )></div>
            </div>

            <span>"Enable notifications"</span>
        </label>

        <p class=Classes::from("demo-mt-1")>
            "State: " <strong>{ move || if is_selected.get() { "ON" } else { "OFF" } }</strong>
        </p>

        <h3>"Toggle Button"</h3>
        <button
            {..toggle_props.into_attrs()}
            on:click=move |_| do_toggle.run(())
            aria-pressed=move || toggle_selected.get()
            style=move || format!(
                "padding: 0.75em 1.5em; border-radius: 8px; cursor: pointer; font-weight: bold; transition: all 0.2s; {}",
                if toggle_selected.get() {
                    "background: var(--brand-color); color: white; border: 2px solid var(--brand-color);"
                } else {
                    "background: white; color: #333; border: 2px solid #ccc;"
                }
            )
        >
            { move || if toggle_selected.get() { "Toggled ON" } else { "Toggled OFF" } }
        </button>
    }
}
