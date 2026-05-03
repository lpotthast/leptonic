use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::slider_basic::SliderBasicDemo;
use super::demos::slider_marks::SliderMarksDemo;
use super::demos::slider_volume::SliderVolumeDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageSlider() -> impl IntoView {
    view! {
        <Article>
            <h1 id="slider" class="anchor">
                "Slider"
                <AnchorLink href="#slider" description="Direct link to article header"/>
            </h1>

            <p>"Allow users to adjust a value within a specified range by sliding a handle."</p>

            <p>
                "All sliders require the "<Code inline=true>"min"</Code>", "<Code inline=true>"max"</Code>" and "<Code inline=true>"step"</Code>" properties, specifying the range of values the slider provides. "
                "Using smaller step values results in ever so slightly smoother sliders until they can be considered \"continuous\". "
                "You may exclude the "<Code inline=true>"step"</Code>" prop altogether to let the sliders use its full "<Code inline=true>"f64"</Code>" precision."
            </p>

            <p>
                "The slider always operates with "<Code inline=true>"f64"</Code>" values and may suffer from typical IEEE-math rounding problems. "
                "We use the "<Code inline=true>"value_display"</Code>" property to specify how a selected value should be rendered."
            </p>

            <DemoShell source=include_str!("demos/slider_basic.rs")>
                <SliderBasicDemo />
            </DemoShell>

            <h1 id="example" class="anchor">
                "Example - Volume slider"
                <AnchorLink href="#example" description="Direct link to section: Example"/>
            </h1>

            <p>"Continuous sliders are perfect when the exact value selected is of no particular interest to your user. For example, when operating a volume slider."</p>

            <DemoShell source=include_str!("demos/slider_volume.rs")>
                <SliderVolumeDemo />
            </DemoShell>

            <h2 id="marks" class="anchor">
                "Marks, Ranges, Popovers"
                <AnchorLink href="#marks" description="Direct link to section: Marks"/>
            </h2>

            <p>
                "Small step values result in lesser selectable values, as only values starting from min and increased by multiples of step are selectable. "
                "To help visualize the selectable values of the slider, marks can be automatically generated. "
                "Sliders also support range selection, popovers, and custom marks."
            </p>

            <DemoShell source=include_str!("demos/slider_marks.rs")>
                <SliderMarksDemo />
            </DemoShell>

            <h2 id="keyboard-input" class="anchor">
                "Keyboard input"
                <AnchorLink href="#keyboard-input" description="Direct link to section: Keyboard input"/>
            </h2>

            <p>
                "Slider knobs are keyboard-interactable and can be cycled through using the "<Code inline=true>"Tab"</Code>" key."
            </p>

            <ul>
                <li><Code inline=true>"Arrow Left"</Code>" / "<Code inline=true>"Arrow Down"</Code>" - Decrease by step"</li>
                <li><Code inline=true>"Arrow Right"</Code>" / "<Code inline=true>"Arrow Up"</Code>" - Increase by step"</li>
                <li><Code inline=true>"Shift + Arrow"</Code>" / "<Code inline=true>"Page Up"</Code>" / "<Code inline=true>"Page Down"</Code>" - Increase/decrease by page size (10% of range)"</li>
                <li><Code inline=true>"Home"</Code>" - Jump to minimum"</li>
                <li><Code inline=true>"End"</Code>" - Jump to maximum"</li>
            </ul>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --slider-margin
                    --slider-track-height
                    --slider-track-background-color
                    --slider-track-background-image
                    --slider-fill-height
                    --slider-fill-background-color
                    --slider-fill-background-image
                    --slider-thumb-size
                    --slider-thumb-border-width
                    --slider-thumb-border-color
                    --slider-thumb-border-style
                    --slider-thumb-background-color
                    --slider-thumb-halo-size
                    --slider-thumb-halo-size-while-dragged
                    --slider-thumb-halo-opacity
                    --slider-thumb-halo-background-color
                    --slider-thumb-transition-speed
                    --slider-thumb-box-shadow
                    --slider-thumb-tooltip-background-color
                    --slider-thumb-tooltip-color
                    --slider-thumb-tooltip-border-radius
                    --slider-thumb-tooltip-font-size
                    --slider-mark-size
                    --slider-mark-color
                    --slider-mark-color-in-range
                    --slider-mark-title-color
                    --slider-mark-title-color-in-range
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Slider", link: "#slider" },
                Toc::Leaf { title: "Example", link: "#example" },
                Toc::Leaf { title: "Marks, Ranges, Popovers", link: "#marks" },
                Toc::Leaf { title: "Keyboard input", link: "#keyboard-input" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
