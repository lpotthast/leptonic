use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::number_field_basic::NumberFieldBasicDemo;
use super::demos::number_field_disabled::NumberFieldDisabledDemo;
use super::demos::number_field_fractional::NumberFieldFractionalDemo;

#[component]
pub fn PageUseNumberField() -> impl IntoView {
    view! {
        <Article>
            <h1 id="number-field" class="anchor">
                "Number Field Hook"
                <AnchorLink href="#number-field" description="Direct link to article header"/>
            </h1>

            <p>
                "Provides the behavior and accessibility for a number field with "
                "increment/decrement buttons, keyboard navigation, and locale-aware "
                "formatting. Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useNumberField.html" target=LinkTarget::_Blank>
                    "useNumberField"
                </LinkExt>
                "."
            </p>

            <h2 id="architecture" class="anchor">
                "Architecture"
                <AnchorLink href="#architecture" description="Direct link to architecture"/>
            </h2>

            <p>
                "The number field is split into two hooks:"
            </p>

            <ul>
                <li><code>"use_number_field_state"</code>" \u{2014} State management: dual-value model "
                    "(numeric value + display string), commit pipeline, increment/decrement with "
                    "floating-point precision, locale-aware parsing and formatting."</li>
                <li><code>"use_number_field"</code>" \u{2014} Behavior and accessibility: ARIA attributes, "
                    "keyboard handling, scroll wheel, auto-repeat buttons, input filtering, "
                    "commit on blur/Enter."</li>
            </ul>

            <h2 id="basic" class="anchor">
                "Basic Example"
                <AnchorLink href="#basic" description="Direct link to basic example"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let state = use_number_field_state(UseNumberFieldStateInput {
                        default_value: Some(50.0),
                        min_value: Some(0.0),
                        max_value: Some(100.0),
                        step: 1.0,
                        ..Default::default()
                    });

                    let field = use_number_field(UseNumberFieldInput {
                        state,
                        label: Some("Quantity".to_string()),
                        min_value: Some(0.0),
                        max_value: Some(100.0),
                        step: 1.0,
                        ..Default::default()
                    });

                    view! {
                        <div {..field.group_props.into_attrs()}>
                            <label {..field.label_props.into_attrs()}>"Quantity"</label>
                            <button {..field.decrement_button_props.into_attrs()}>"-"</button>
                            <input value=field.display_value {..field.input_props.into_attrs()} />
                            <button {..field.increment_button_props.into_attrs()}>"+"</button>
                        </div>
                    }
                "#)}
            </Code>

            <DemoShell source=include_str!("demos/number_field_basic.rs")>
                <NumberFieldBasicDemo />
            </DemoShell>

            <h2 id="fractional" class="anchor">
                "Fractional Step"
                <AnchorLink href="#fractional" description="Direct link to fractional step"/>
            </h2>

            <p>"Floating-point precision is handled automatically. Incrementing by 0.1 "
                "three times produces exactly 0.3, not 0.30000000000000004."</p>

            <DemoShell source=include_str!("demos/number_field_fractional.rs")>
                <NumberFieldFractionalDemo />
            </DemoShell>

            <h2 id="disabled" class="anchor">
                "Disabled State"
                <AnchorLink href="#disabled" description="Direct link to disabled"/>
            </h2>

            <DemoShell source=include_str!("demos/number_field_disabled.rs")>
                <NumberFieldDisabledDemo />
            </DemoShell>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Arrow Up/Down"</strong>" \u{2014} Increment/decrement by step"</li>
                <li><strong>"Page Up/Down"</strong>" \u{2014} Increment/decrement by step"</li>
                <li><strong>"Home"</strong>" \u{2014} Jump to minimum value"</li>
                <li><strong>"End"</strong>" \u{2014} Jump to maximum value"</li>
                <li><strong>"Enter"</strong>" \u{2014} Commit the current value"</li>
            </ul>

            <p>"Modifier keys (Ctrl, Shift, Alt, Meta) are ignored. Input is also committed "
                "automatically on blur."</p>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Dual-value state model (numeric + display string)"</li>
                <li>"Locale-aware number formatting and parsing via ICU4X"</li>
                <li>"Floating-point precision handling (no accumulation errors)"</li>
                <li>"Step snapping (values snap to valid step boundaries)"</li>
                <li>"Commit on blur and Enter key"</li>
                <li>"Input filtering (rejects invalid characters while typing)"</li>
                <li>"Auto-repeat on press-and-hold for increment/decrement buttons"</li>
                <li>"Scroll wheel increment/decrement when focused"</li>
                <li>"Screen reader announcements via live announcer"</li>
                <li>"ARIA group wrapper with " <code>"role=\"group\""</code></li>
                <li>"Dynamic " <code>"inputmode"</code>" based on platform (iPhone, Android)"</li>
                <li>"Composed button labels: " <code>"\"Increase {label}\""</code></li>
            </ul>

            <h2 id="accessibility" class="anchor">
                "Accessibility"
                <AnchorLink href="#accessibility" description="Direct link to accessibility"/>
            </h2>

            <ul>
                <li>"Input has " <code>"aria-roledescription=\"Number field\""</code>
                    " (except iOS, where it interferes with VoiceOver)"</li>
                <li>"Group wrapper has " <code>"role=\"group\""</code>
                    " with " <code>"aria-disabled"</code>" and " <code>"aria-invalid"</code></li>
                <li>"Buttons have " <code>"aria-controls"</code>" pointing to the input"</li>
                <li>"Autocorrect and spellcheck disabled on input"</li>
                <li>"Value changes announced to screen readers"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Number Field Hook", link: "#number-field" },
                Toc::Leaf { title: "Architecture", link: "#architecture" },
                Toc::Leaf { title: "Basic Example", link: "#basic" },
                Toc::Leaf { title: "Fractional Step", link: "#fractional" },
                Toc::Leaf { title: "Disabled State", link: "#disabled" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "Accessibility", link: "#accessibility" },
            ]
        }/>
    }
}
