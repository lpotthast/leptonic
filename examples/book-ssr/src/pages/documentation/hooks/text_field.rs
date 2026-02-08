use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use indoc::indoc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn PageUseTextField() -> impl IntoView {
    // Simple signals for demo
    let (text_value, set_text_value) = signal(String::new());
    let (search_value, set_search_value) = signal(String::new());
    let (number_value, set_number_value) = signal(50.0f64);

    // Use the label hook for proper label associations
    let UseLabelReturn {
        label_props: username_label_props,
        field_props: username_field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Label),
    });

    let UseLabelReturn {
        label_props: search_label_props,
        field_props: search_field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Label),
    });

    let UseLabelReturn {
        label_props: quantity_label_props,
        field_props: quantity_field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Label),
    });

    view! {
        <Article>
            <h1 id="text_field" class="anchor">
                "Text Field Hooks"
                <AnchorLink href="#text_field" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible text inputs, search fields, and number fields with validation support."</p>

            <h2 id="use_text_field" class="anchor">
                "use_text_field"
                <AnchorLink href="#use_text_field" description="Direct link to use_text_field"/>
            </h2>

            <Code>
                {indoc!(r#"
                    let UseTextFieldStateReturn { value, set_value } = use_text_field_state(String::new());

                    let UseTextFieldReturn { input_props, label_props, description_props, error_props } =
                        use_text_field(UseTextFieldInput {
                            value: value.into(),
                            placeholder: Some("Enter username"),
                            is_required: true,
                            max_length: Some(20),
                            on_change: Some(Callback::new(move |v| set_value.run(v))),
                            ..Default::default()
                        });

                    view! {
                        <label id=label_props.id for=label_props.html_for>"Username"</label>
                        <input {..input_props} />
                        <p id=description_props.id>"Description text"</p>
                    }
                "#)}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                <label
                    {..username_label_props.into_attrs()}
                    style="display: block; font-weight: 500; margin-bottom: 0.25em;"
                >
                    "Username"
                </label>
                <input
                    type="text"
                    {..username_field_props.into_attrs()}
                    placeholder="Enter username"
                    prop:value=move || text_value.get()
                    on:input=move |ev| set_text_value.set(event_target_value(&ev))
                    maxlength="20"
                    style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 250px;"
                />
                <p style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #666;">
                    "Choose a unique username."
                </p>
                <p style="margin: 0.5em 0 0 0; font-size: 0.85em;">
                    "Value: " { move || text_value.get() } " (" { move || text_value.get().len() } "/20)"
                </p>
            </div>

            <h2 id="use_search_field" class="anchor">
                "use_search_field"
                <AnchorLink href="#use_search_field" description="Direct link to use_search_field"/>
            </h2>

            <p>"Search field with clear button and Enter/Escape handling."</p>

            <Code>
                {indoc!(r#"
                    let UseSearchFieldStateReturn { value, set_value, clear } =
                        use_search_field_state(String::new());

                    let UseSearchFieldReturn { input_props, clear_button_props, label_props, .. } =
                        use_search_field(UseSearchFieldInput {
                            value: value.into(),
                            placeholder: Some("Search..."),
                            on_change: Some(Callback::new(move |v| set_value.run(v))),
                            on_submit: Some(Callback::new(move |v| { /* search */ })),
                            on_clear: Some(Callback::new(move |_| clear.run(()))),
                            ..Default::default()
                        });
                "#)}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                <label
                    {..search_label_props.into_attrs()}
                    style="display: block; font-weight: 500; margin-bottom: 0.25em;"
                >
                    "Search"
                </label>
                <div style="display: flex; gap: 0.5em;">
                    <input
                        type="search"
                        {..search_field_props.into_attrs()}
                        placeholder="Search..."
                        prop:value=move || search_value.get()
                        on:input=move |ev| set_search_value.set(event_target_value(&ev))
                        style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 250px;"
                    />
                    <button
                        on:click=move |_| set_search_value.set(String::new())
                        style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                    >
                        "Clear"
                    </button>
                </div>
                <p style="margin: 0.5em 0 0 0; font-size: 0.85em;">
                    "Press Enter to submit, Escape to clear"
                </p>
            </div>

            <h2 id="use_number_field" class="anchor">
                "use_number_field"
                <AnchorLink href="#use_number_field" description="Direct link to use_number_field"/>
            </h2>

            <p>"Numeric input with increment/decrement buttons and min/max/step validation."</p>

            <Code>
                {indoc!(r"
                    let UseNumberFieldStateReturn { value, set_value, increment, decrement, .. } =
                        use_number_field_state(50.0);

                    let UseNumberFieldReturn { input_props, increment_props, decrement_props, label_props, .. } =
                        use_number_field(UseNumberFieldInput {
                            value: value.into(),
                            min_value: 0.0,
                            max_value: 100.0,
                            step: 1.0,
                            on_change: Some(Callback::new(move |v| set_value.run(v))),
                            ..Default::default()
                        });
                ")}
            </Code>

            <div style="padding: 1em; border: 1px solid #ddd; border-radius: 8px; margin: 1em 0;">
                <label
                    {..quantity_label_props.into_attrs()}
                    style="display: block; font-weight: 500; margin-bottom: 0.25em;"
                >
                    "Quantity (0-100)"
                </label>
                <div style="display: flex; gap: 0.25em;">
                    <button
                        on:click=move |_| set_number_value.update(|v| *v = (*v - 1.0).max(0.0))
                        style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                    >
                        "-"
                    </button>
                    <input
                        type="number"
                        {..quantity_field_props.into_attrs()}
                        min="0"
                        max="100"
                        step="1"
                        prop:value=move || number_value.get()
                        on:input=move |ev| {
                            if let Ok(v) = event_target_value(&ev).parse::<f64>() {
                                set_number_value.set(v.clamp(0.0, 100.0));
                            }
                        }
                        style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 80px; text-align: center;"
                    />
                    <button
                        on:click=move |_| set_number_value.update(|v| *v = (*v + 1.0).min(100.0))
                        style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer;"
                    >
                        "+"
                    </button>
                </div>
                <p style="margin: 0.5em 0 0 0; font-size: 0.85em;">
                    "Value: " { move || format!("{:.0}", number_value.get()) }
                </p>
            </div>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Number field Arrow Up/Down"</strong> " - Increment/decrement by step"</li>
                <li><strong>"Number field Page Up/Down"</strong> " - Increment/decrement by larger amount"</li>
                <li><strong>"Search field Enter"</strong> " - Submit search"</li>
                <li><strong>"Search field Escape"</strong> " - Clear and blur"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Text field with validation (min/max length, pattern)"</li>
                <li>"Search field with clear button and submit handling"</li>
                <li>"Number field with increment/decrement and min/max/step"</li>
                <li>"Proper ARIA labels and descriptions"</li>
                <li>"Keyboard navigation support"</li>
                <li>"Disabled and read-only states"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Text Field Hooks", link: "#text_field" },
                Toc::Leaf { title: "use_text_field", link: "#use_text_field" },
                Toc::Leaf { title: "use_search_field", link: "#use_search_field" },
                Toc::Leaf { title: "use_number_field", link: "#use_number_field" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
