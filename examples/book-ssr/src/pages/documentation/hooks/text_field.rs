use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::text_field_basic::TextFieldBasicDemo;
use super::demos::text_field_search::TextFieldSearchDemo;

#[component]
pub fn PageUseTextField() -> impl IntoView {
    view! {
        <Article>
            <h1 id="text_field" class="anchor">
                "Text Field Hooks"
                <AnchorLink href="#text_field" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible text inputs, search fields, and number fields with validation support. "
                "See the "<Link href=crate::routes::doc::TextField.materialize()>"Text Field overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useTextField.html" target=LinkTarget::_Blank>
                    "useTextField"
                </LinkExt>
                "."
            </p>

            <h2 id="use_text_field" class="anchor">
                "use_text_field"
                <AnchorLink href="#use_text_field" description="Direct link to use_text_field"/>
            </h2>

            <Code language=Language::Rust>
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

            <DemoShell source=include_str!("demos/text_field_basic.rs")>
                <TextFieldBasicDemo />
            </DemoShell>

            <h2 id="use_search_field" class="anchor">
                "use_search_field"
                <AnchorLink href="#use_search_field" description="Direct link to use_search_field"/>
            </h2>

            <p>"Search field with clear button and Enter/Escape handling."</p>

            <Code language=Language::Rust>
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

            <DemoShell source=include_str!("demos/text_field_search.rs")>
                <TextFieldSearchDemo />
            </DemoShell>

            <h2 id="use_number_field" class="anchor">
                "use_number_field"
                <AnchorLink href="#use_number_field" description="Direct link to use_number_field"/>
            </h2>

            <p>
                "Numeric input with increment/decrement buttons, locale-aware formatting, "
                "and floating-point precision handling. See the dedicated "
                <Link href="/doc/text-field/number-field-hook">"Number Field Hook"</Link>
                " page for full documentation and examples."
            </p>

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
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::TextField.materialize()>"Text Field overview"</Link></li>
                <li><Link href=crate::routes::doc::text_field::Component.materialize()>"Text Field component"</Link></li>
                <li><Link href=crate::routes::doc::Select.materialize()>"Select concept"</Link></li>
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
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
