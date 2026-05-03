use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::label_basic::LabelBasicDemo;
use super::demos::label_invalid::LabelInvalidDemo;
use super::demos::label_span::LabelSpanDemo;
use super::demos::label_valid::LabelValidDemo;

#[component]
pub fn PageUseLabel() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_label" class="anchor">
                "use_label & use_field"
                <AnchorLink href="#use_label" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_label"</Code>" and "<Code inline=true>"use_field"</Code>" hooks are standalone hooks for creating accessible form labels and fields with proper ARIA associations."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useLabel.html" target=LinkTarget::_Blank>
                    "useLabel"
                </LinkExt>
                "."
            </p>

            <h2 id="basic-label" class="anchor">
                "use_label"
                <AnchorLink href="#basic-label" description="Direct link to use_label"/>
            </h2>

            <p>"Associates a label with a form field using matching IDs."</p>

            <Code language=Language::Rust>
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

            <DemoShell source=include_str!("demos/label_basic.rs")>
                <LabelBasicDemo />
            </DemoShell>

            <h3>"Span Labels (for non-native controls)"</h3>

            <p>"Use " <code>"LabelElementType::Span"</code> " for custom controls that don't support native labels:"</p>

            <DemoShell source=include_str!("demos/label_span.rs")>
                <LabelSpanDemo />
            </DemoShell>

            <h2 id="use_field" class="anchor">
                "use_field"
                <AnchorLink href="#use_field" description="Direct link to use_field"/>
            </h2>

            <p>"Complete field setup with label, description, and error message associations."</p>

            <Code language=Language::Rust>
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

            <DemoShell source=include_str!("demos/label_valid.rs")>
                <LabelValidDemo />
            </DemoShell>

            <h3>"Invalid Field"</h3>

            <DemoShell source=include_str!("demos/label_invalid.rs")>
                <LabelInvalidDemo />
            </DemoShell>

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

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::InputCategory.materialize()>"Input domain"</Link></li>
                <li><Link href=crate::routes::doc::text_field::Hook.materialize()>"use_text_field"</Link></li>
                <li><Link href=crate::routes::doc::checkbox::Hook.materialize()>"use_checkbox"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_label & use_field", link: "#use_label" },
                Toc::Leaf { title: "use_label", link: "#basic-label" },
                Toc::Leaf { title: "use_field", link: "#use_field" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
