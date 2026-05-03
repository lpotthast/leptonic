use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::select::SelectDemo;

#[component]
pub fn PageUseSelectHook() -> impl IntoView {
    view! {
        <Article>
            <h1 id="select" class="anchor">
                "Select Hooks"
                <AnchorLink href="#select" description="Direct link to article header"/>
            </h1>

            <p>"Hooks for creating accessible dropdown select menus with keyboard navigation and hidden native select for form submission."</p>

            <p>
                "See the "<Link href=crate::routes::doc::Select.materialize()>"Select overview"</Link>" for concept guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useSelect.html" target=LinkTarget::_Blank>
                    "useSelect"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell source=include_str!("demos/select.rs")>
                <SelectDemo />
            </DemoShell>

            <h2 id="use_select" class="anchor">
                "use_select"
                <AnchorLink href="#use_select" description="Direct link to use_select"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseSelectReturn {
                        trigger_attrs,
                        value_props,
                        listbox_props,
                        hidden_props,
                        state,
                    } = use_select(UseSelectInput {
                        label: Some("Fruit".into()),
                        selected_key: selected.into(),
                        is_disabled: false.into(),
                        is_required: false.into(),
                        name: Some("fruit".into()),
                        on_selection_change: Some(Callback::new(move |key| set_selected.set(Some(key)))),
                        on_open_change: Some(Callback::new(move |open| set_is_open.set(open))),
                    });
                "#)}
            </Code>

            <p>"Returns props for:"</p>
            <ul>
                <li><code>"trigger_attrs"</code> " - The button that opens the dropdown"</li>
                <li><code>"value_props"</code> " - The displayed selected value"</li>
                <li><code>"listbox_props"</code> " - The dropdown listbox"</li>
                <li><code>"hidden_props"</code> " - Props for hidden native select"</li>
            </ul>

            <h2 id="use_hidden_select" class="anchor">
                "use_hidden_select"
                <AnchorLink href="#use_hidden_select" description="Direct link to use_hidden_select"/>
            </h2>

            <p>"Creates a hidden native select element for form submission:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseHiddenSelectReturn { container_attrs, select_attrs, input_attrs } =
                        use_hidden_select(UseHiddenSelectInput {
                            name: "fruit".to_string(),
                            trigger_ref: trigger_ref.into(),
                            label: Some("Fruit".into()),
                            is_disabled: false.into(),
                            selected_key: selected.into(),
                        });
                "#)}
            </Code>

            <p>"The hidden select ensures:"</p>
            <ul>
                <li>"Form submission works correctly"</li>
                <li>"Autofill works for returning users"</li>
                <li>"Compatibility with form libraries"</li>
            </ul>

            <h2 id="keyboard" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard" description="Direct link to keyboard"/>
            </h2>

            <ul>
                <li><strong>"Space/Enter"</strong> " - Open dropdown / select option"</li>
                <li><strong>"Arrow Down/Up"</strong> " - Open dropdown / navigate options"</li>
                <li><strong>"Escape"</strong> " - Close dropdown"</li>
                <li><strong>"Home/End"</strong> " - Jump to first/last option"</li>
                <li><strong>"Type characters"</strong> " - Type-ahead to jump to matching options"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Accessible dropdown with ARIA attributes"</li>
                <li>"Keyboard navigation"</li>
                <li>"Type-ahead selection"</li>
                <li>"Hidden native select for forms"</li>
                <li>"Disabled and required states"</li>
                <li>"Custom trigger and option rendering"</li>
            </ul>
            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Select.materialize()>"Select overview"</Link></li>
                <li><Link href=crate::routes::doc::select::Component.materialize()>"Select component"</Link></li>
                <li><Link href=crate::routes::doc::Listbox.materialize()>"Listbox overview"</Link></li>
                <li><Link href=crate::routes::doc::Combobox.materialize()>"Combobox overview"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Select Hooks", link: "#select" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "use_select", link: "#use_select" },
                Toc::Leaf { title: "use_hidden_select", link: "#use_hidden_select" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
