use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::color_area::ColorAreaDemo;

#[component]
pub fn PageUseColorArea() -> impl IntoView {
    view! {
        <Article>
            <p style="margin-bottom: 0.5em;">
                <Link href=routes::doc::color::Hooks.materialize()>"\u{2190} Color Hooks Overview"</Link>
            </p>

            <h1 id="use_color_area" class="anchor">
                "use_color_area"
                <AnchorLink href="#use_color_area" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_color_area_state"</Code>" and "<Code inline=true>"use_color_area"</Code>
                " hooks provide state management and interaction for a 2D color gradient area where users can adjust two color channels simultaneously."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorArea.html" target=LinkTarget::_Blank>
                    "useColorArea"
                </LinkExt>
                "."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/color_area.rs")
                description="2D color gradient area"
            >
                <ColorAreaDemo />
            </DemoShell>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API section"/>
            </h2>

            <h3>"State Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorAreaStateInput<C: ColorValue> {
                        pub default_value: C,
                        pub x_channel: C::Channel,
                        pub y_channel: C::Channel,
                        pub x_channel_step: Option<f64>,
                        pub y_channel_step: Option<f64>,
                        pub on_change: Option<Callback<C>>,
                        pub on_change_end: Option<Callback<C>>,
                    }
                ")}
            </Code>

            <h3>"State Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorAreaStateReturn<C: ColorValue> {
                        pub value: Signal<C>,
                        pub x_value: Signal<f64>,
                        pub y_value: Signal<f64>,
                        pub x_channel: C::Channel,
                        pub y_channel: C::Channel,
                        pub z_channel: C::Channel,
                        pub is_dragging: Signal<bool>,
                        pub set_dragging: Callback<bool>,
                        pub set_color_from_point: Callback<(f64, f64)>,
                        pub thumb_position: Signal<(f64, f64)>,
                        pub set_value: Callback<C>,
                        pub increment_x: Callback<Option<f64>>,
                        pub decrement_x: Callback<Option<f64>>,
                        pub increment_y: Callback<Option<f64>>,
                        pub decrement_y: Callback<Option<f64>>,
                        pub display_color: Signal<C>,
                    }
                ")}
            </Code>

            <h3>"Hook Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorAreaInput<C: ColorValue> {
                        pub state: UseColorAreaStateReturn<C>,
                        pub disabled: Signal<bool>,
                        pub aria_label: Option<&'static str>,
                        pub is_rtl: bool,
                    }
                ")}
            </Code>

            <h3>"Hook Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorAreaReturn {
                        pub area_props: UseColorAreaProps,
                        pub thumb_props: UseColorAreaThumbProps,
                        pub background: Signal<String>,
                        pub thumb_color: Signal<String>,
                        pub thumb_x_percent: Signal<f64>,
                        pub thumb_y_percent: Signal<f64>,
                    }
                ")}
            </Code>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"2D gradient interaction for any two color channels"</li>
                <li>"Pointer, touch, and keyboard navigation"</li>
                <li>"Auto-generated CSS gradient background"</li>
                <li>"Thumb position tracking as percentages"</li>
                <li>"Accessible ARIA attributes"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_color_area", link: "#use_color_area" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
