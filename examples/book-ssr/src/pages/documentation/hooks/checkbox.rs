use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;
use std::collections::HashSet;

#[component]
pub fn PageUseCheckboxHook() -> impl IntoView {
    // Simple checkbox
    let (is_checked, set_is_checked) = signal(false);
    let (is_indeterminate, set_is_indeterminate) = signal(false);
    let (is_disabled, set_is_disabled) = signal(false);

    let UseCheckboxReturn { input_props, .. } = use_checkbox(UseCheckboxInput {
        is_selected: is_checked.into(),
        is_indeterminate: is_indeterminate.into(),
        is_disabled: is_disabled.into(),
        is_read_only: false.into(),
        is_required: false,
        name: Some("example-checkbox".into()),
        value: Some("example".into()),
        validation_state: ValidationState::Valid,
        aria_label: None,
        on_change: Some(Callback::new(move |checked| {
            set_is_checked.set(checked);
            if checked {
                set_is_indeterminate.set(false);
            }
        })),
    });

    // Checkbox group
    let (group_value, _set_group_value) = signal(HashSet::<String>::new());
    let UseCheckboxGroupReturn {
        group_props,
        label_props: group_label_props,
        state: group_state,
    } = use_checkbox_group(UseCheckboxGroupInput {
        value: group_value.into(),
        label: Some("Fruits".into()),
        description: None,
        error_message: None,
        is_disabled: false.into(),
        is_read_only: false.into(),
        is_required: false,
        validation_state: ValidationState::Valid,
        orientation: Orientation::Vertical,
        on_change: None,
    });

    view! {
        <Article>
            <h1 id="use_checkbox" class="anchor">
                "use_checkbox & use_checkbox_group"
                <AnchorLink href="#use_checkbox" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible checkboxes with support for indeterminate state and checkbox groups."</p>

            <h2 id="single-checkbox" class="anchor">
                "use_checkbox"
                <AnchorLink href="#single-checkbox" description="Direct link to single checkbox"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let (is_checked, set_is_checked) = signal(false);

                    let UseCheckboxReturn { input_attrs, .. } = use_checkbox(UseCheckboxInput {
                        is_selected: is_checked.into(),
                        is_indeterminate: false.into(),
                        is_disabled: false.into(),
                        is_read_only: false.into(),
                        is_required: false.into(),
                        name: Some("my-checkbox".into()),
                        value: Some("value".into()),
                        validation_state: ValidationState::Valid.into(),
                        on_change: Some(Callback::new(move |checked| set_is_checked.set(checked))),
                    });

                    view! {
                        <input type="checkbox" {..input_attrs} />
                    }
                "#)}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                <label style="display: flex; align-items: center; gap: 0.5em; cursor: pointer;">
                    <input type="checkbox" {..input_props.into_attrs()} />
                    <span>"Accept terms and conditions"</span>
                </label>

                <p style="margin: 1em 0 0.5em 0; font-size: 0.9em;">
                    "Checked: " <strong>{ move || is_checked.get().to_string() }</strong>
                    " | Indeterminate: " <strong>{ move || is_indeterminate.get().to_string() }</strong>
                </p>

                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                    <button
                        on:click=move |_| set_is_indeterminate.update(|v| *v = !*v)
                        style="padding: 0.4em 0.8em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                    >
                        "Toggle Indeterminate"
                    </button>
                    <button
                        on:click=move |_| set_is_disabled.update(|v| *v = !*v)
                        style="padding: 0.4em 0.8em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                    >
                        { move || if is_disabled.get() { "Enable" } else { "Disable" } }
                    </button>
                </Stack>
            </div>

            <h2 id="checkbox-group" class="anchor">
                "use_checkbox_group"
                <AnchorLink href="#checkbox-group" description="Direct link to checkbox group"/>
            </h2>

            <p>"Manages a group of checkboxes with shared state."</p>

            <Code>
                {indoc!(r#"
                    let UseCheckboxGroupReturn { group_attrs, label_props, state } =
                        use_checkbox_group(UseCheckboxGroupInput {
                            label: Some("Fruits".into()),
                            description: None,
                            error_message: None,
                            is_disabled: false.into(),
                            is_read_only: false.into(),
                            validation_state: ValidationState::Valid.into(),
                            orientation: Orientation::Vertical,
                            on_change: None,
                        });

                    // Use state.add_value, state.remove_value, state.toggle_value
                    // state.is_selected to check individual items
                "#)}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                <fieldset
                    role=group_props.role
                    aria-labelledby=group_props.aria_labelledby.clone()
                    style="border: none; padding: 0; margin: 0;"
                >
                    <legend id=group_label_props.id.clone() style="font-weight: bold; margin-bottom: 0.5em;">
                        "Select your favorite fruits"
                    </legend>

                    {["Apple", "Banana", "Cherry", "Date"].into_iter().map(|fruit| {
                        let fruit_value = fruit.to_string();
                        let fruit_for_check = fruit_value.clone();
                        let fruit_for_toggle = fruit_value.clone();
                        let state = group_state.clone();
                        let state_for_check = group_state.clone();
                        view! {
                            <label style="display: flex; align-items: center; gap: 0.5em; cursor: pointer; margin: 0.25em 0;">
                                <input
                                    type="checkbox"
                                    checked=move || state_for_check.is_selected.run(fruit_for_check.clone())
                                    on:change=move |_| { state.toggle_value.run(fruit_for_toggle.clone()); }
                                />
                                <span>{ fruit }</span>
                            </label>
                        }
                    }).collect::<Vec<_>>()}
                </fieldset>

                <p style="margin-top: 1em; font-size: 0.9em;">
                    "Selected: " { move || format!("{:?}", group_state.value.get()) }
                </p>
            </div>

            <h2 id="indeterminate" class="anchor">
                "Indeterminate State"
                <AnchorLink href="#indeterminate" description="Direct link to indeterminate"/>
            </h2>

            <p>"The indeterminate state is useful for \"select all\" checkboxes:"</p>
            <ul>
                <li>"Unchecked when no children are selected"</li>
                <li>"Indeterminate when some children are selected"</li>
                <li>"Checked when all children are selected"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Native checkbox with controlled state"</li>
                <li>"Indeterminate state support"</li>
                <li>"Disabled and read-only modes"</li>
                <li>"Required field support"</li>
                <li>"Validation state (valid/invalid)"</li>
                <li>"Group management with add/remove/toggle operations"</li>
                <li>"Horizontal and vertical orientations for groups"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_checkbox", link: "#use_checkbox" },
                Toc::Leaf { title: "Single Checkbox", link: "#single-checkbox" },
                Toc::Leaf { title: "Checkbox Group", link: "#checkbox-group" },
                Toc::Leaf { title: "Indeterminate State", link: "#indeterminate" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
