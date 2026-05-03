use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::color_field::ColorFieldDemo;

#[component]
pub fn PageUseColorField() -> impl IntoView {
    view! {
        <Article>
            <p style="margin-bottom: 0.5em;">
                <Link href=routes::doc::color::Hooks.materialize()>"\u{2190} Color Hooks Overview"</Link>
            </p>

            <h1 id="use_color_field" class="anchor">
                "use_color_field"
                <AnchorLink href="#use_color_field" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_color_field_state"</Code>" and "<Code inline=true>"use_color_field"</Code>
                " hooks provide state management and interaction for a hex color text input."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorField.html" target=LinkTarget::_Blank>
                    "useColorField"
                </LinkExt>
                "."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to overview"/>
            </h2>

            <p>
                "The color field manages dual tracking of input text and a parsed RGB color value. "
                "It validates hex characters during typing and commits the value on blur. "
                "If an invalid value is entered, it reverts to the last known valid color on commit."
            </p>

            <p>
                "Arrow Up/Down keys increment and decrement the hex integer value by one step."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/color_field.rs")
                description="Hex color text input"
            >
                <ColorFieldDemo />
            </DemoShell>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API section"/>
            </h2>

            <h3>"State Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorFieldStateInput {
                        pub default_value: Option<RGB8>,
                        pub on_change: Option<Callback<Option<RGB8>>>,
                    }
                ")}
            </Code>

            <h3>"State Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorFieldStateReturn {
                        pub input_value: Signal<String>,
                        pub set_input_value: Callback<String>,
                        pub color_value: Signal<Option<RGB8>>,
                        pub set_color_value: Callback<Option<RGB8>>,
                        pub commit: Callback<()>,
                        pub validate: Callback<String, bool>,
                        pub increment: Callback<()>,
                        pub decrement: Callback<()>,
                        pub increment_to_max: Callback<()>,
                        pub decrement_to_min: Callback<()>,
                    }
                ")}
            </Code>

            <h3>"Hook Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorFieldInput {
                        pub state: UseColorFieldStateReturn,
                        pub disabled: Signal<bool>,
                        pub read_only: Signal<bool>,
                        pub aria_label: Option<&'static str>,
                    }
                ")}
            </Code>

            <h3>"Hook Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorFieldReturn {
                        pub input_props: UseColorFieldInputProps,
                    }
                ")}
            </Code>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Hex color text input with live validation"</li>
                <li>"Commit on blur with invalid-value revert"</li>
                <li>"Arrow Up/Down increment/decrement"</li>
                <li>"Disabled and read-only states"</li>
                <li>"ARIA attributes for accessibility"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::hooks::UseColorChannelField.materialize()>"use_color_channel_field"</Link>" - Numeric input for a single color channel"</li>
                <li><Link href=crate::routes::doc::hooks::UseColorSwatch.materialize()>"use_color_swatch"</Link>" - Display-only color swatch"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_color_field", link: "#use_color_field" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
