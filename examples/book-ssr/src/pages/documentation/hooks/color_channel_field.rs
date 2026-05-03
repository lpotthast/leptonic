use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::color_channel_field::ColorChannelFieldDemo;

#[component]
pub fn PageUseColorChannelField() -> impl IntoView {
    view! {
        <Article>
            <p style="margin-bottom: 0.5em;">
                <Link href=routes::doc::color::Hooks.materialize()>"\u{2190} Color Hooks Overview"</Link>
            </p>

            <h1 id="use_color_channel_field" class="anchor">
                "use_color_channel_field"
                <AnchorLink href="#use_color_channel_field" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_color_channel_field_state"</Code>" and "<Code inline=true>"use_color_channel_field"</Code>
                " hooks provide state and interaction for a numeric input that edits a single color channel."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorChannelField.html" target=LinkTarget::_Blank>
                    "useColorChannelField"
                </LinkExt>
                " (part of the color field family)."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to overview"/>
            </h2>

            <p>
                "This hook bridges a full color value to a single numeric channel input. "
                "It delegates to "<Code inline=true>"use_number_field"</Code>" with channel-derived range parameters "
                "(min, max, step) and auto-generates an ARIA label from the channel name (e.g., \"Hue\", \"Red\")."
            </p>

            <p>
                "Changes to the channel value are mapped back to the full color via "
                <Code inline=true>"ColorValue::with_channel_value"</Code>
                ". External color changes are synced back into the number field via a reactive effect."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/color_channel_field.rs")
                description="Hue channel numeric input"
            >
                <ColorChannelFieldDemo />
            </DemoShell>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API section"/>
            </h2>

            <h3>"State Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorChannelFieldStateInput<C: ColorValue> {
                        pub default_value: C,
                        pub channel: C::Channel,
                        pub on_change: Option<Callback<C>>,
                    }
                ")}
            </Code>

            <h3>"State Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorChannelFieldStateReturn<C: ColorValue> {
                        pub color_value: Signal<C>,
                        pub set_color_value: Callback<C>,
                        pub channel_value: Signal<Option<f64>>,
                        pub set_channel_value: Callback<Option<f64>>,
                        pub channel: C::Channel,
                        pub min_value: f64,
                        pub max_value: f64,
                        pub step: f64,
                    }
                ")}
            </Code>

            <h3>"Hook Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorChannelFieldInput<C: ColorValue> {
                        pub state: UseColorChannelFieldStateReturn<C>,
                        pub is_disabled: Signal<bool>,
                        pub is_read_only: Signal<bool>,
                        pub aria_label: Option<&'static str>,
                    }
                ")}
            </Code>

            <h3>"Hook Return"</h3>
            <p>
                "Returns "<Code inline=true>"UseNumberFieldReturn"</Code>
                " directly, providing group, input, increment button, decrement button, and label props."
            </p>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseNumberFieldReturn {
                        pub group_props: UseNumberFieldGroupProps,
                        pub input_props: UseNumberFieldInputProps,
                        pub increment_button_props: UseNumberFieldButtonProps,
                        pub decrement_button_props: UseNumberFieldButtonProps,
                        pub label_props: UseNumberFieldLabelProps,
                    }
                ")}
            </Code>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Single channel numeric editing with full number field interaction"</li>
                <li>"Auto-derived min, max, step from channel range"</li>
                <li>"Auto-generated ARIA label from channel name"</li>
                <li>"Bidirectional sync between color value and number field"</li>
                <li>"Increment/decrement buttons and keyboard support"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::hooks::UseColorField.materialize()>"use_color_field"</Link>" - Hex color text input"</li>
                <li><Link href=crate::routes::doc::hooks::UseColorSlider.materialize()>"use_color_slider"</Link>" - Single-channel color slider"</li>
                <li><Link href=crate::routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>" - General-purpose number field hook"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_color_channel_field", link: "#use_color_channel_field" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
