use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::select_basic::SelectBasicDemo;
use super::demos::select_optional::SelectOptionalDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageSelect() -> impl IntoView {
    view! {
        <Article>
            <h1 id="select" class="anchor">
                "Select"
                <AnchorLink href="#select" description="Direct link to article header"/>
            </h1>

            <p>"Select inputs allow you to choose between different predefined values."</p>

            <p>"Lets assume this type definition, providing us with a set of values to choose from."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    enum Foo {
                        A,
                        B,
                        C,
                    }
                ")}
            </Code>

            <h2 id="variants" class="anchor">
                "Variants"
                <AnchorLink href="#variants" description="Direct link to section: Variants"/>
            </h2>

            <p>"There are three variants of the select component, accepting different inputs and changing only slightly in its behavior."</p>

            <DemoShell source=include_str!("demos/select_basic.rs")>
                <SelectBasicDemo />
            </DemoShell>

            <h3 id="variant-optional-select" class="anchor">
                "OptionalSelect"
                <AnchorLink href="#variant-optional-select" description="Direct link to section: Variant - OptionalSelect"/>
            </h3>

            <p>"As the name implies, this variant stores its chosen value in an " <Code inline=true>"Option"</Code> ", allowing the select to be initialized without a value and optionally allowing the user to deselect the current value."</p>

            <DemoShell source=include_str!("demos/select_optional.rs")>
                <SelectOptionalDemo />
            </DemoShell>

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to section: Keyboard navigation"/>
            </h2>

            <p>
                "The select component was designed with keyboard navigation in mind. "
                "Press "<Code inline=true>"Tab"</Code>" to jump to the next or "<Code inline=true>"Shift + Tab"</Code>" to jump to the previous select. "
                "Open the dropdown using "<Code inline=true>"Enter"</Code>". "
                "Preselect an available option using the "<Code inline=true>"ArrowDown"</Code>" and "<Code inline=true>"ArrowUp"</Code>" keys. "
                "When the dropdown is not open, starting to preselect an element using the arrow keys will open it. "
                "Choose an option by pressing "<Code inline=true>"Enter"</Code>". "
                "Close the dropdown by pressing "<Code inline=true>"Escape"</Code>"."
            </p>

            <p>
                "Select options can be searched. When opening the dropdown of available options, focus will automatically jump to the search input, allowing you to type instantly. "
                "When closing the dropdown, focus is automatically restored to the select, allowing you to "<Code inline=true>"Tab"</Code>" to the next element."
            </p>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --select-padding
                    --select-min-height
                    --select-selected-color
                    --select-selected-background-color
                    --select-selected-border
                    --select-selected-border-bottom
                    --select-selected-border-radius
                    --select-selected-badge-color
                    --select-selected-badge-background-color
                    --select-selected-placeholder-color
                    --select-focused-border-color
                    --select-dropdown-background-color
                    --select-dropdown-shadow
                    --select-search-color
                    --select-search-background-color
                    --select-no-items-color
                    --select-no-items-background-color
                    --select-item-color
                    --select-item-background-color
                    --select-item-padding
                    --select-item-disabled-background-color
                    --select-item-disabled-color
                    --select-item-preselected-background-color
                    --select-item-hover-background-color
                    --select-item-selected-background-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Select", link: "#select" },
                Toc::Group { title: "Variants", link: "#variants", inner: vec![
                    Toc::Leaf { title: "OptionalSelect", link: "#variant-optional-select" },
                ]},
                Toc::Leaf { title: "Keyboard navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
