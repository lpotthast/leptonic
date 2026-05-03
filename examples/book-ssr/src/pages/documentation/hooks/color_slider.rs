use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::color_slider::ColorSliderDemo;

#[component]
pub fn PageUseColorSlider() -> impl IntoView {
    view! {
        <Article>
            <p style="margin-bottom: 0.5em;">
                <Link href=routes::doc::color::Hooks.materialize()>"\u{2190} Color Hooks Overview"</Link>
            </p>

            <h1 id="use_color_slider" class="anchor">
                "use_color_slider"
                <AnchorLink href="#use_color_slider" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_color_slider_state"</Code>" and "<Code inline=true>"use_color_slider"</Code>
                " hooks provide state management and interaction for a slider that adjusts a single color channel."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorSlider.html" target=LinkTarget::_Blank>
                    "useColorSlider"
                </LinkExt>
                "."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to overview"/>
            </h2>

            <p>
                "The color slider wraps the general-purpose "<Code inline=true>"use_slider"</Code>
                " hook with color-specific gradient computation and ARIA value text enrichment. "
                "It auto-generates an accessibility label from the channel name (e.g., \"Hue\", \"Saturation\") and "
                "provides a CSS gradient background derived from the current color."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/color_slider.rs")
                description="Hue channel color slider"
            >
                <ColorSliderDemo />
            </DemoShell>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API section"/>
            </h2>

            <h3>"State Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorSliderStateInput<C: ColorValue> {
                        pub default_value: C,
                        pub channel: C::Channel,
                        pub disabled: Signal<bool>,
                        pub orientation: Signal<SliderOrientation>,
                        pub on_change: Option<Callback<C>>,
                        pub on_change_end: Option<Callback<C>>,
                    }
                ")}
            </Code>

            <h3>"State Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorSliderStateReturn<C: ColorValue> {
                        pub value: Signal<C>,
                        pub set_value: Callback<C>,
                        pub slider_state: UseSliderStateReturn,
                        pub display_color: Signal<C>,
                        pub thumb_value_label: Signal<String>,
                        pub channel: C::Channel,
                        pub is_dragging: Signal<bool>,
                    }
                ")}
            </Code>

            <h3>"Hook Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorSliderInput<C: ColorValue> {
                        pub state: UseColorSliderStateReturn<C>,
                        pub disabled: Signal<bool>,
                        pub orientation: Signal<SliderOrientation>,
                        pub aria_label: Option<&'static str>,
                        pub is_rtl: bool,
                        pub name: Option<&'static str>,
                    }
                ")}
            </Code>

            <h3>"Hook Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorSliderReturn {
                        pub slider: UseSliderReturn,
                        pub thumb: UseSliderThumbReturn,
                        pub background: Signal<String>,
                        pub track_style: Signal<String>,
                    }
                ")}
            </Code>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Single-channel color adjustment via slider interaction"</li>
                <li>"Auto-generated CSS gradient for the track background"</li>
                <li>"Enriched ARIA value text with hue/color name"</li>
                <li>"Horizontal and vertical orientation support"</li>
                <li>"Built on top of "<Code inline=true>"use_slider"</Code>" for full keyboard and pointer support"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link>" - 2D color area for two channels"</li>
                <li><Link href=crate::routes::doc::hooks::UseColorWheel.materialize()>"use_color_wheel"</Link>" - Circular hue wheel"</li>
                <li><Link href=crate::routes::doc::slider::Hook.materialize()>"use_slider"</Link>" - General-purpose slider hook"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_color_slider", link: "#use_color_slider" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
