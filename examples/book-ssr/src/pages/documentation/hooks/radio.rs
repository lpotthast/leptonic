use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;

use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseRadioHook() -> impl IntoView {
    let UseRadioGroupStateReturn {
        selected_value,
        set_selected,
    } = use_radio_group_state(Some("option1".to_string()));

    let UseRadioGroupReturn {
        group_props,
        label_props,
        state,
    } = use_radio_group(UseRadioGroupInput {
        label: Some("Select an option".into()),
        description: None,
        error_message: None,
        is_disabled: Signal::derive(|| false),
        is_read_only: Signal::derive(|| false),
        is_required: false,
        value: selected_value.into(),
        validation_state: ValidationState::Valid,
        orientation: Orientation::Vertical,
        on_change: Some(Callback::new(move |value: String| {
            set_selected.run(value);
        })),
    });

    let options = vec![
        ("option1", "First Option"),
        ("option2", "Second Option"),
        ("option3", "Third Option"),
    ];

    view! {
        <Article>
            <h1 id="use_radio" class="anchor">
                "use_radio & use_radio_group"
                <AnchorLink href="#use_radio" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible radio buttons with group management and mutual exclusion."</p>

            <h2 id="radio-group" class="anchor">
                "use_radio_group"
                <AnchorLink href="#radio-group" description="Direct link to radio group"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseRadioGroupStateReturn { selected_value, set_selected } =
                        use_radio_group_state(Some("option1".to_string()));

                    let UseRadioGroupReturn { group_props, label_props, state } = use_radio_group(
                        UseRadioGroupInput {
                            label: Some("Select an option".into()),
                            value: selected_value.into(),
                            orientation: Orientation::Vertical,
                            on_change: Some(Callback::new(move |value| set_selected.run(value))),
                            ..Default::default()
                        }
                    );
                "#)}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                <fieldset
                    role=group_props.role
                    aria-labelledby=group_props.aria_labelledby.clone()
                    aria-orientation=group_props.aria_orientation
                    style="border: none; padding: 0; margin: 0;"
                >
                    <legend id=label_props.id.clone() style="font-weight: bold; margin-bottom: 0.5em;">
                        "Select an option"
                    </legend>

                    {options.into_iter().map(|(value, label)| {
                        let value_owned = value.to_string();
                        let value_for_check = value_owned.clone();
                        let value_for_change = value_owned.clone();
                        let state_clone = state.clone();
                        view! {
                            <label style="display: flex; align-items: center; gap: 0.5em; cursor: pointer; margin: 0.5em 0;">
                                <input
                                    type="radio"
                                    name=state.name
                                    value=value
                                    checked=move || state_clone.selected_value.get().as_ref() == Some(&value_for_check)
                                    on:change=move |_| state.set_selected_value.run(value_for_change.clone())
                                />
                                <span>{ label }</span>
                            </label>
                        }
                    }).collect::<Vec<_>>()}
                </fieldset>

                <p style="margin-top: 1em; font-size: 0.9em;">
                    "Selected: " <strong>{ move || selected_value.get().unwrap_or_else(|| "None".to_string()) }</strong>
                </p>
            </div>

            <h2 id="use_radio" class="anchor">
                "use_radio"
                <AnchorLink href="#use_radio" description="Direct link to use_radio"/>
            </h2>

            <p>"For individual radio buttons within a group:"</p>

            <Code>
                {indoc!(r#"
                    let UseRadioReturn { input_props, is_selected, .. } = use_radio(UseRadioInput {
                        value: "option1".to_string(),
                        state: group.state,
                        is_disabled: Signal::derive(|| false),
                        validation_state: ValidationState::Valid,
                        aria_label: Some("Option 1"),
                        name: Some("my-radio"),
                    });
                "#)}
            </Code>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <p>"Radio groups support standard keyboard navigation:"</p>
            <ul>
                <li><strong>"Arrow keys"</strong> " - Move between options"</li>
                <li><strong>"Space"</strong> " - Select the focused option"</li>
                <li><strong>"Tab"</strong> " - Move focus to/from the group"</li>
            </ul>

            <h2 id="orientation" class="anchor">
                "Orientation"
                <AnchorLink href="#orientation" description="Direct link to orientation"/>
            </h2>

            <p>"Radio groups support two orientations:"</p>
            <ul>
                <li><code>"Orientation::Vertical"</code> " - Up/Down arrows navigate"</li>
                <li><code>"Orientation::Horizontal"</code> " - Left/Right arrows navigate"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Mutual exclusion (only one selection at a time)"</li>
                <li>"Keyboard navigation with arrow keys"</li>
                <li>"Support for horizontal and vertical orientations"</li>
                <li>"Disabled and read-only modes"</li>
                <li>"Required field validation"</li>
                <li>"ARIA role and attributes for accessibility"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_radio & use_radio_group", link: "#use_radio" },
                Toc::Leaf { title: "Radio Group", link: "#radio-group" },
                Toc::Leaf { title: "use_radio", link: "#use_radio" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Orientation", link: "#orientation" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
