use indoc::indoc;
use leptonic::{components::prelude::*, hooks::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageUseLabel() -> impl IntoView {
    // Basic label hook
    let UseLabelReturn {
        label_props,
        field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Label),
    });

    // Span label for non-native controls
    let UseLabelReturn {
        label_props: span_label_props,
        field_props: span_field_props,
    } = use_label(UseLabelInput {
        id: None,
        label_element_type: Some(LabelElementType::Span),
    });

    // Complete field setup
    let UseFieldReturn {
        label_props: field_label_props,
        field_props: field_input_props,
        description_props,
        error_message_props: _,
    } = use_field(UseFieldInput {
        id: None,
        label: Some("Email Address".into()),
        description: Some("We'll never share your email.".into()),
        error_message: None,
        validation_state: ValidationState::Valid,
        is_required: false,
        is_disabled: false,
        is_read_only: false,
    });

    // Invalid field
    let UseFieldReturn {
        label_props: error_label_props,
        field_props: error_field_props,
        description_props: error_description_props,
        error_message_props,
    } = use_field(UseFieldInput {
        id: None,
        label: Some("Password".into()),
        description: Some("Minimum 8 characters.".into()),
        error_message: Some("Password is too short.".into()),
        validation_state: ValidationState::Invalid,
        is_required: false,
        is_disabled: false,
        is_read_only: false,
    });

    view! {
        <Article>
            <h1 id="use_label" class="anchor">
                "use_label & use_field"
                <AnchorLink href="#use_label" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible form labels and fields with proper ARIA associations."</p>

            <h2 id="basic-label" class="anchor">
                "use_label"
                <AnchorLink href="#basic-label" description="Direct link to use_label"/>
            </h2>

            <p>"Associates a label with a form field using matching IDs."</p>

            <Code>
                {indoc!(r#"
                    let UseLabelReturn { label_props, field_props } = use_label(UseLabelInput {
                        id: None, // Auto-generated
                        label_element_type: Some(LabelElementType::Label),
                    });

                    view! {
                        <label id=label_props.id for=label_props.html_for>"Username"</label>
                        <input type="text" id=field_props.id aria-labelledby=field_props.aria_labelledby />
                    }
                "#)}
            </Code>

            <div style="margin: 1.5em 0; padding: 1em; border: 1px solid #ddd; border-radius: 8px;">
                <div style="margin-bottom: 0.5em;">
                    <label
                        {..label_props.into_attrs()}
                        style="display: block; margin-bottom: 0.25em; font-weight: 500;"
                    >
                        "Username"
                    </label>
                    <input
                        type="text"
                        {..field_props.into_attrs()}
                        style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 200px;"
                    />
                </div>
            </div>

            <h3>"Span Labels (for non-native controls)"</h3>

            <p>"Use " <code>"LabelElementType::Span"</code> " for custom controls that don't support native labels:"</p>

            <div style="margin: 1.5em 0; padding: 1em; border: 1px solid #ddd; border-radius: 8px;">
                <span
                    {..span_label_props.into_attrs()}
                    style="display: block; margin-bottom: 0.25em; font-weight: 500;"
                >
                    "Non-native control"
                </span>
                <div
                    role="slider"
                    tabindex="0"
                    {..span_field_props.into_attrs()}
                    style="width: 200px; height: 20px; background: #ddd; border-radius: 10px; cursor: pointer;"
                >
                    <div style="width: 50%; height: 100%; background: var(--brand-color); border-radius: 10px;"></div>
                </div>
            </div>

            <h2 id="use_field" class="anchor">
                "use_field"
                <AnchorLink href="#use_field" description="Direct link to use_field"/>
            </h2>

            <p>"Complete field setup with label, description, and error message associations."</p>

            <Code>
                {indoc!(r#"
                    let UseFieldReturn {
                        label_props,
                        field_props,
                        description_props,
                        error_message_props,
                    } = use_field(UseFieldInput {
                        label: Some("Email".into()),
                        description: Some("We'll never share your email.".into()),
                        error_message: None,
                        validation_state: ValidationState::Valid,
                        ..Default::default()
                    });
                "#)}
            </Code>

            <h3>"Valid Field"</h3>

            <div style="margin: 1.5em 0; padding: 1em; border: 1px solid #ddd; border-radius: 8px;">
                <label
                    id=field_label_props.id.clone()
                    for=field_label_props.html_for.clone()
                    style="display: block; margin-bottom: 0.25em; font-weight: 500;"
                >
                    "Email Address"
                </label>
                <input
                    type="email"
                    id=field_input_props.id.clone()
                    aria-labelledby=field_input_props.aria_labelledby.clone()
                    aria-describedby=field_input_props.aria_describedby.clone()
                    aria-invalid=field_input_props.aria_invalid
                    aria-required=field_input_props.aria_required
                    style="padding: 0.5em; border: 1px solid #ccc; border-radius: 4px; width: 250px;"
                />
                <p
                    id=description_props.id.clone()
                    style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #666;"
                >
                    "We'll never share your email."
                </p>
            </div>

            <h3>"Invalid Field"</h3>

            <div style="margin: 1.5em 0; padding: 1em; border: 1px solid #ddd; border-radius: 8px;">
                <label
                    id=error_label_props.id.clone()
                    for=error_label_props.html_for.clone()
                    style="display: block; margin-bottom: 0.25em; font-weight: 500;"
                >
                    "Password"
                </label>
                <input
                    type="password"
                    {..error_field_props.into_attrs()}
                    value="short"
                    style="padding: 0.5em; border: 2px solid #dc3545; border-radius: 4px; width: 250px;"
                />
                <p
                    id=error_description_props.id.clone()
                    style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #666;"
                >
                    "Minimum 8 characters."
                </p>
                <p
                    id=error_message_props.id.clone()
                    role=error_message_props.role
                    aria-live=error_message_props.aria_live
                    style="margin: 0.25em 0 0 0; font-size: 0.85em; color: #dc3545; font-weight: 500;"
                >
                    "Password is too short."
                </p>
            </div>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hooks automatically set up proper ARIA associations:"</p>
            <ul>
                <li><code>"id"</code> " and " <code>"for"</code> " attributes link label to field"</li>
                <li><code>"aria-labelledby"</code> " for span-based labels"</li>
                <li><code>"aria-describedby"</code> " links field to description"</li>
                <li><code>"aria-errormessage"</code> " links field to error message"</li>
                <li><code>"aria-invalid"</code> " indicates validation state"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Auto-generated unique IDs"</li>
                <li>"Support for native label elements and span labels"</li>
                <li>"Complete ARIA associations for accessibility"</li>
                <li>"Validation state support (valid/invalid)"</li>
                <li>"Description and error message integration"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_label & use_field", link: "#use_label" },
                Toc::Leaf { title: "use_label", link: "#basic-label" },
                Toc::Leaf { title: "use_field", link: "#use_field" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
