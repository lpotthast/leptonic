use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{
    pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc},
    routes,
};

use super::demos::color_wheel::ColorWheelDemo;

#[component]
pub fn PageUseColorWheel() -> impl IntoView {
    view! {
        <Article>
            <p style="margin-bottom: 0.5em;">
                <Link href=routes::doc::color::Hooks.materialize()>"\u{2190} Color Hooks Overview"</Link>
            </p>

            <h1 id="use_color_wheel" class="anchor">
                "use_color_wheel"
                <AnchorLink href="#use_color_wheel" description="Direct link to article header"/>
            </h1>

            <p>
                "The "<Code inline=true>"use_color_wheel_state"</Code>" and "<Code inline=true>"use_color_wheel"</Code>
                " hooks provide state management and interaction for a circular hue wheel."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorWheel.html" target=LinkTarget::_Blank>
                    "useColorWheel"
                </LinkExt>
                "."
            </p>

            <h2 id="overview" class="anchor">
                "Overview"
                <AnchorLink href="#overview" description="Direct link to overview"/>
            </h2>

            <p>
                "The color wheel displays a conic gradient of channel values in an annular (donut) shape. "
                "Users interact by clicking or dragging on the annulus, dragging the thumb, or using keyboard navigation."
            </p>

            <p>
                "The hook is generic over any "<Code inline=true>"ColorValue"</Code>" type and operates on a single "
                "channel (typically hue). The other channels are preserved from the initial value but are not adjustable "
                "through this hook."
            </p>

            <h2 id="demo" class="anchor">
                "Interactive Demo"
                <AnchorLink href="#demo" description="Direct link to demo"/>
            </h2>

            <DemoShell
                source=include_str!("demos/color_wheel.rs")
                description="Circular hue wheel"
            >
                <ColorWheelDemo />
            </DemoShell>

            <h2 id="api" class="anchor">
                "API"
                <AnchorLink href="#api" description="Direct link to API section"/>
            </h2>

            <h3>"State Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorWheelStateInput<C: ColorValue> {
                        pub default_value: C,
                        pub channel: C::Channel,
                        pub disabled: Signal<bool>,
                        pub on_change: Option<Callback<C>>,
                        pub on_change_end: Option<Callback<C>>,
                    }
                ")}
            </Code>

            <h3>"State Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorWheelStateReturn<C: ColorValue> {
                        pub value: Signal<C>,
                        pub set_value: Callback<C>,
                        pub channel: C::Channel,
                        pub hue: Signal<f64>,
                        pub set_hue: Callback<f64>,
                        pub set_hue_from_point: Callback<(f64, f64, f64)>,
                        pub get_thumb_position: Callback<f64, (f64, f64)>,
                        pub increment: Callback<Option<f64>>,
                        pub decrement: Callback<Option<f64>>,
                        pub is_dragging: Signal<bool>,
                        pub set_dragging: Callback<bool>,
                        pub display_color: Signal<C>,
                        pub step: f64,
                        pub page_step: f64,
                        pub is_disabled: Signal<bool>,
                    }
                ")}
            </Code>

            <h3>"Hook Input"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorWheelInput<C: ColorValue> {
                        pub state: UseColorWheelStateReturn<C>,
                        pub outer_radius: f64,
                        pub inner_radius: f64,
                        pub disabled: Signal<bool>,
                        pub aria_label: Option<&'static str>,
                    }
                ")}
            </Code>

            <h3>"Hook Return"</h3>
            <Code language=Language::Rust>
                {indoc!(r"
                    pub struct UseColorWheelReturn {
                        pub track_props: UseColorWheelTrackProps,
                        pub thumb_props: UseColorWheelThumbProps,
                        pub background: Signal<String>,
                        pub clip_path: String,
                        pub thumb_x: Signal<f64>,
                        pub thumb_y: Signal<f64>,
                        pub track_size: f64,
                    }
                ")}
            </Code>

            <h2 id="keyboard-navigation" class="anchor">
                "Keyboard Navigation"
                <AnchorLink href="#keyboard-navigation" description="Direct link to keyboard navigation"/>
            </h2>

            <ul>
                <li><code>"Arrow Left / Arrow Down"</code>" - Decrement hue by step"</li>
                <li><code>"Arrow Right / Arrow Up"</code>" - Increment hue by step"</li>
                <li><code>"Shift + Arrow"</code>" - Increment/decrement by page step"</li>
                <li><code>"Page Up"</code>" - Increment by page step"</li>
                <li><code>"Page Down"</code>" - Decrement by page step"</li>
                <li><code>"Home"</code>" - Set to channel minimum"</li>
                <li><code>"End"</code>" - Set to channel maximum"</li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Circular hue selection (0\u{00b0}\u{2013}360\u{00b0})"</li>
                <li>"Conic gradient background with 13 stops"</li>
                <li>"SVG clip-path for annulus shape"</li>
                <li>"Pointer drag on track and thumb"</li>
                <li>"Full keyboard navigation"</li>
                <li>"ARIA slider role with value text including hue name"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link>" - 2D color area for two channels"</li>
                <li><Link href=routes::doc::hooks::UseColorSlider.materialize()>"use_color_slider"</Link>" - Single-channel color slider"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_color_wheel", link: "#use_color_wheel" },
                Toc::Leaf { title: "Overview", link: "#overview" },
                Toc::Leaf { title: "Demo", link: "#demo" },
                Toc::Leaf { title: "API", link: "#api" },
                Toc::Leaf { title: "Keyboard Navigation", link: "#keyboard-navigation" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
