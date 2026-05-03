use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    slider_basic::SliderBasicDemo, slider_callbacks::SliderCallbacksDemo,
    slider_disabled::SliderDisabledDemo, slider_range::SliderRangeDemo,
    slider_vertical::SliderVerticalDemo,
};
use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

#[component]
pub fn PageAtomSlider() -> impl IntoView {
    view! {
        <Article>
            <h1 id="slider" class="anchor">
                "Slider"
                <AnchorLink href="#slider" description="Direct link to article header"/>
            </h1>

            <p>
                "The Slider atom wraps the "
                <code>"use_slider_state"</code>", "<code>"use_slider"</code>", and "<code>"use_slider_thumb"</code>
                " hooks into composable components. It provides accessible range sliders with keyboard navigation, "
                "drag support, and multi-thumb range selection out of the box."
            </p>

            <h2 id="basic" class="anchor">
                "Basic Slider"
                <AnchorLink href="#basic" description="Direct link to basic slider"/>
            </h2>

            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::atoms::slider::*;

                    view! {
                        <Slider values=SliderValues::Uncontrolled(vec![50.0])>
                            <SliderTrack>
                                <SliderTrackFill/>
                                <SliderThumb/>
                            </SliderTrack>
                            <SliderOutput let:attrs let:values>
                                <output {..attrs}>
                                    { move || format!("{}%", values.get()[0] as i32) }
                                </output>
                            </SliderOutput>
                        </Slider>
                    }
                "#)}
            </Code>

            <DemoShell>
                <SliderBasicDemo />
            </DemoShell>

            <h2 id="range" class="anchor">
                "Range Slider"
                <AnchorLink href="#range" description="Direct link to range slider"/>
            </h2>

            <p>"Pass two values via "<code>"SliderValues::Uncontrolled(vec![...])"</code>" and render two "<code>"SliderThumb"</code>" components. Thumbs are automatically constrained and cannot cross each other."</p>

            <DemoShell source=include_str!("demos/slider_range.rs")>
                <SliderRangeDemo />
            </DemoShell>

            <h2 id="callbacks" class="anchor">
                "Callbacks"
                <AnchorLink href="#callbacks" description="Direct link to callbacks"/>
            </h2>

            <p>
                <code>"on_change"</code>" fires during interaction (every value update). "
                <code>"on_change_end"</code>" fires once when the user finishes dragging."
            </p>

            <Code language=Language::Rust>
                {indoc!(r"
                    <Slider
                        values=SliderValues::Uncontrolled(vec![50.0])
                        on_change=Callback::new(move |values: Vec<f64>| {
                            // Fires continuously during drag
                        })
                        on_change_end=Callback::new(move |values: Vec<f64>| {
                            // Fires once when drag ends
                        })
                    >
                ")}
            </Code>

            <DemoShell source=include_str!("demos/slider_callbacks.rs")>
                <SliderCallbacksDemo />
            </DemoShell>

            <h2 id="vertical" class="anchor">
                "Vertical Slider"
                <AnchorLink href="#vertical" description="Direct link to vertical slider"/>
            </h2>

            <p>"Set "<code>"orientation=SliderOrientation::Vertical"</code>" for a vertical layout."</p>

            <DemoShell source=include_str!("demos/slider_vertical.rs")>
                <SliderVerticalDemo />
            </DemoShell>

            <h2 id="disabled" class="anchor">
                "Disabled Slider"
                <AnchorLink href="#disabled" description="Direct link to disabled slider"/>
            </h2>

            <DemoShell source=include_str!("demos/slider_disabled.rs")>
                <SliderDisabledDemo />
            </DemoShell>

            <h2 id="form-name" class="anchor">
                "Form Submission"
                <AnchorLink href="#form-name" description="Direct link to form submission"/>
            </h2>

            <p>"Pass a "<code>"name"</code>" prop to "<code>"SliderThumb"</code>" to include the value in form submissions via a hidden input."</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    <Slider values=SliderValues::Uncontrolled(vec![50.0])>
                        <SliderTrack>
                            <SliderTrackFill/>
                            <SliderThumb name="volume"/>
                        </SliderTrack>
                    </Slider>
                "#)}
            </Code>

            <h2 id="api" class="anchor">
                "API Reference"
                <AnchorLink href="#api" description="Direct link to API"/>
            </h2>

            <h3>"Slider"</h3>
            <ul>
                <li><code>"values: SliderValues"</code>" - Uncontrolled (with initial values) or Controlled (with external signal)"</li>
                <li><code>"min: f64"</code>" - Minimum value (default: 0.0)"</li>
                <li><code>"max: f64"</code>" - Maximum value (default: 100.0)"</li>
                <li><code>"step: f64"</code>" - Step increment (default: 1.0)"</li>
                <li><code>"orientation: Signal<SliderOrientation>"</code>" - Horizontal or Vertical"</li>
                <li><code>"disabled: Signal<bool>"</code>" - Disabled state"</li>
                <li><code>"on_change: Option<Callback<Vec<f64>>>"</code>" - Called during interaction"</li>
                <li><code>"on_change_end: Option<Callback<Vec<f64>>>"</code>" - Called when dragging ends"</li>
                <li><code>"aria_label: Option<&'static str>"</code>" - Accessibility label"</li>
                <li><code>"aria_labelledby: Option<String>"</code>" - ID of labeling element"</li>
                <li><code>"is_rtl: bool"</code>" - Right-to-left layout"</li>
            </ul>

            <h3>"SliderThumb"</h3>
            <ul>
                <li><code>"index: Option<usize>"</code>" - Explicit thumb index (auto-assigned if omitted)"</li>
                <li><code>"name: Option<&'static str>"</code>" - Form submission name"</li>
                <li><code>"aria_label: Option<Cow<'static, str>>"</code>" - Accessible label"</li>
                <li><code>"validation_state: Option<ValidationState>"</code>" - Validation state"</li>
                <li><code>"decimal_places: Option<usize>"</code>" - Display formatting"</li>
                <li><code>"is_required: bool"</code>" - ARIA required"</li>
                <li><code>"is_rtl: Option<bool>"</code>" - Overrides Slider-level RTL"</li>
            </ul>

            <h3>"SliderTrack"</h3>
            <p>"Renders the track element. Place "<code>"SliderTrackFill"</code>" and "<code>"SliderThumb"</code>" as children."</p>

            <h3>"SliderTrackFill"</h3>
            <p>"Renders the fill region. Supports 1-thumb (left fill) and 2-thumb (range fill) sliders."</p>

            <h3>"SliderOutput"</h3>
            <p>"Render prop component. Provides "<code>"UseSliderOutputAttrs"</code>" and "<code>"Signal<Vec<f64>>"</code>" to its children."</p>

        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Slider", link: "#slider" },
                Toc::Leaf { title: "Basic Slider", link: "#basic" },
                Toc::Leaf { title: "Range Slider", link: "#range" },
                Toc::Leaf { title: "Callbacks", link: "#callbacks" },
                Toc::Leaf { title: "Vertical Slider", link: "#vertical" },
                Toc::Leaf { title: "Disabled Slider", link: "#disabled" },
                Toc::Leaf { title: "Form Submission", link: "#form-name" },
                Toc::Leaf { title: "API Reference", link: "#api" },
            ]
        }/>
    }
}
