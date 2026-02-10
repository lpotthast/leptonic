use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseSwitchHook() -> impl IntoView {
    // Switch state
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
        is_selected: is_selected.into(),
        is_disabled: false.into(),
        is_read_only: false.into(),
        name: Some("notifications"),
        value: Some("enabled"),
        on_change: Some(Callback::new(move |v: bool| set_selected.run(v))),
        validation_state: ValidationState::Valid,
        aria_label: Some("Enable notifications"),
    });

    // Toggle state
    let UseToggleStateReturn {
        is_selected: toggle_selected,
        toggle: do_toggle,
        ..
    } = use_toggle_state(false);

    let UseToggleReturn {
        props: toggle_props,
        ..
    } = use_toggle(UseToggleInput {
        is_selected: toggle_selected.into(),
        is_disabled: false.into(),
        is_read_only: false.into(),
        on_change: None,
        value: None,
    });

    view! {
        <Article>
            <h1 id="use_switch" class="anchor">
                "use_switch & use_toggle"
                <AnchorLink href="#use_switch" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating toggle switches with proper accessibility and hidden form inputs."</p>

            <h2 id="switch" class="anchor">
                "use_switch"
                <AnchorLink href="#switch" description="Direct link to switch"/>
            </h2>

            <p>"Creates an accessible switch with role=\"switch\" and a hidden input for form submission."</p>

            <Code>
                {indoc!(r#"
                    let UseSwitchStateReturn { is_selected, set_selected, toggle } = use_switch_state(false);

                    let UseSwitchReturn { switch_props, input_props, .. } = use_switch(UseSwitchInput {
                        is_selected: is_selected.into(),
                        is_disabled: false.into(),
                        is_read_only: false.into(),
                        name: Some("notifications"),
                        value: Some("enabled"),
                        on_change: Some(Callback::new(move |v| set_selected.run(v))),
                        ..Default::default()
                    });

                    view! {
                        <label {..switch_props.into_attrs()}>
                            <input type="checkbox" {..input_props.into_attrs()} /> // Hidden
                            <span class="switch-track">
                                <span class="switch-thumb"></span>
                            </span>
                            "Enable notifications"
                        </label>
                    }
                "#)}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                <label
                    {..switch_props.into_attrs()}
                    on:click=move |_| toggle.run(())
                    style="display: flex; align-items: center; gap: 1em; cursor: pointer; user-select: none;"
                >
                    // Hidden input for form submission
                    <input type="checkbox" {..input_props.into_attrs()} style="position: absolute; opacity: 0; width: 0; height: 0;" />

                    // Visual switch
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

                <p style="margin: 1em 0 0 0; font-size: 0.9em;">
                    "State: " <strong>{ move || if is_selected.get() { "ON" } else { "OFF" } }</strong>
                </p>
            </div>

            <h2 id="use_toggle" class="anchor">
                "use_toggle"
                <AnchorLink href="#use_toggle" description="Direct link to use_toggle"/>
            </h2>

            <p>"Lower-level hook for toggle behavior. Unlike " <code>"use_switch"</code> ", it doesn't include a hidden input and is more suitable for toggle buttons."</p>

            <Code>
                {indoc!(r#"
                    let UseToggleStateReturn { is_selected, toggle, .. } = use_toggle_state(false);

                    let UseToggleReturn { props: toggle_props, .. } = use_toggle(UseToggleInput {
                        is_selected: is_selected.into(),
                        is_disabled: false.into(),
                        is_read_only: false.into(),
                        ..Default::default()
                    });

                    view! {
                        <button {..toggle_props} aria-pressed=move || is_selected.get()>
                            "Toggle Me"
                        </button>
                    }
                "#)}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
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
            </div>

            <h2 id="comparison" class="anchor">
                "Switch vs Toggle"
                <AnchorLink href="#comparison" description="Direct link to comparison"/>
            </h2>

            <table style="width: 100%; border-collapse: collapse; margin: 1em 0;">
                <thead>
                    <tr>
                        <th style="border: 1px solid #ddd; padding: 0.5em; text-align: left;">"Feature"</th>
                        <th style="border: 1px solid #ddd; padding: 0.5em; text-align: left;">"use_switch"</th>
                        <th style="border: 1px solid #ddd; padding: 0.5em; text-align: left;">"use_toggle"</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td style="border: 1px solid #ddd; padding: 0.5em;">"ARIA role"</td>
                        <td style="border: 1px solid #ddd; padding: 0.5em;"><code>"switch"</code></td>
                        <td style="border: 1px solid #ddd; padding: 0.5em;"><code>"button"</code></td>
                    </tr>
                    <tr>
                        <td style="border: 1px solid #ddd; padding: 0.5em;">"Hidden input"</td>
                        <td style="border: 1px solid #ddd; padding: 0.5em;">"Yes (for forms)"</td>
                        <td style="border: 1px solid #ddd; padding: 0.5em;">"No"</td>
                    </tr>
                    <tr>
                        <td style="border: 1px solid #ddd; padding: 0.5em;">"Use case"</td>
                        <td style="border: 1px solid #ddd; padding: 0.5em;">"Settings toggles"</td>
                        <td style="border: 1px solid #ddd; padding: 0.5em;">"Toggle buttons"</td>
                    </tr>
                </tbody>
            </table>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Proper role=\"switch\" for accessibility"</li>
                <li>"Hidden input for form submission"</li>
                <li>"Toggle via click and keyboard (Space/Enter)"</li>
                <li>"Disabled and read-only states"</li>
                <li>"Change callbacks with new state value"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_switch & use_toggle", link: "#use_switch" },
                Toc::Leaf { title: "use_switch", link: "#switch" },
                Toc::Leaf { title: "use_toggle", link: "#use_toggle" },
                Toc::Leaf { title: "Comparison", link: "#comparison" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
