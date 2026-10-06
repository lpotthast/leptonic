use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    slider_basic::SliderBasicDemo, slider_marks::SliderMarksDemo,
    slider_popover::SliderPopoverDemo, slider_range::SliderRangeDemo,
    slider_volume::SliderVolumeDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageSlider() -> impl IntoView {
    view! {
        <DocPage title="Slider component">
            <p>
                "The themed "<Code inline=true>"Slider"</Code>" and "<Code inline=true>"RangeSlider"</Code>
                " let you adjust a value, or a range of values, by dragging a thumb along a track. "
                "See the "<Link href=routes::doc::Slider.materialize()>"Slider overview"</Link>" for concept guidance."
            </p>

            <Demo description="Sliders with a coarse and a fine step" source=include_str!("demos/slider_basic.rs")>
                <SliderBasicDemo/>
            </Demo>

            <Section title="Props">
                <Section title="Slider">
                    <ApiTable kind=ApiKind::Props of="components::slider::Slider">
                        <ApiRow name="value" ty="Signal<f64>">"The current value. Required."</ApiRow>
                        <ApiRow name="set_value" ty="Out<f64>">"Receives the new value. Required."</ApiRow>
                        <ApiRow name="min, max" ty="f64">"The range of the slider. Required."</ApiRow>
                        <ApiRow name="step" ty="Option<f64>" default="None">
                            "The step between selectable values. Without it, the slider is continuous."
                        </ApiRow>
                        <ApiRow name="marks" ty="SliderMarks" default="SliderMarks::None">
                            "Marks along the track, see "<a href="#marks">"Marks"</a>"."
                        </ApiRow>
                        <ApiRow name="popover" ty="SliderPopover" default="SliderPopover::Never">
                            "When to show the value above the thumb, see "<a href="#value-popover">"Value Popover"</a>"."
                        </ApiRow>
                        <ApiRow name="value_display" ty="Option<Callback<f64, String>>" default="None">
                            "Formats values for the tooltip and the names of automatic marks."
                        </ApiRow>
                        <ApiRow name="variant" ty="SliderVariant" default="Round">
                            <Code inline=true>"Round"</Code>" or "<Code inline=true>"Block"</Code>
                            ", rendered as "<Code inline=true>"data-variant"</Code>"."
                        </ApiRow>
                        <ApiRow name="disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="RangeSlider">
                    <p>"Like "<Code inline=true>"Slider"</Code>", with two values instead of one:"</p>
                    <ApiTable kind=ApiKind::Props of="RangeSlider">
                        <ApiRow name="value_a, value_b" ty="Signal<f64>">"The start and end of the range. Required."</ApiRow>
                        <ApiRow name="set_value_a, set_value_b" ty="Out<f64>">
                            "Receive the new start and end. Required."
                        </ApiRow>
                        <ApiRow name="min, max" ty="f64">"The range of the slider. Required."</ApiRow>
                        <ApiRow name="step" ty="Option<f64>" default="None">
                            "The step between selectable values. Without it, the slider is continuous."
                        </ApiRow>
                        <ApiRow name="marks" ty="SliderMarks" default="SliderMarks::None">
                            "Marks along the track, see "<a href="#marks">"Marks"</a>"."
                        </ApiRow>
                        <ApiRow name="popover" ty="SliderPopover" default="SliderPopover::Never">
                            "When to show the values above the thumbs, see "<a href="#value-popover">"Value Popover"</a>"."
                        </ApiRow>
                        <ApiRow name="value_display" ty="Option<Callback<f64, String>>" default="None">
                            "Formats values for the tooltips and the names of automatic marks."
                        </ApiRow>
                        <ApiRow name="variant" ty="SliderVariant" default="Round">
                            <Code inline=true>"Round"</Code>" or "<Code inline=true>"Block"</Code>
                            ", rendered as "<Code inline=true>"data-variant"</Code>"."
                        </ApiRow>
                        <ApiRow name="disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Additional classes and styles."
                        </ApiRow>
                    </ApiTable>
                    <p>"The two thumbs cannot cross each other."</p>
                </Section>
            </Section>

            <Section title="Step and Precision">
                <p>
                    "The "<Code inline=true>"step"</Code>" prop decides which values you can select: "<Code inline=true>"min"</Code>
                    " plus multiples of "<Code inline=true>"step"</Code>". Smaller steps make the slider smoother until it "
                    "feels continuous. Leave out "<Code inline=true>"step"</Code>" to use the full "<Code inline=true>"f64"</Code>
                    " precision."
                </p>
                <p>
                    "Sliders always work with "<Code inline=true>"f64"</Code>" values, which are subject to the usual "
                    "floating-point rounding. Format values for display, as the demos do, instead of showing them raw."
                </p>
            </Section>

            <Section title="Volume Slider">
                <p>
                    "A continuous slider suits values whose exact number doesn\u{2019}t matter to the user, such as a volume."
                </p>

                <Demo description="Continuous volume slider between two icons" source=include_str!("demos/slider_volume.rs")>
                    <SliderVolumeDemo/>
                </Demo>
            </Section>

            <Section title="Marks">
                <p>
                    "Marks visualize the selectable values. "<Code inline=true>"SliderMarks::Automatic"</Code>
                    " generates one mark per step (it requires a "<Code inline=true>"step"</Code>"), optionally with a name "
                    "formatted by "<Code inline=true>"value_display"</Code>". "<Code inline=true>"SliderMarks::Custom"</Code>
                    " places "<Code inline=true>"SliderMark"</Code>"s at absolute values ("
                    <Code inline=true>"SliderMarkValue::Value"</Code>
                    ") or at a fraction of the track ("<Code inline=true>"SliderMarkValue::Percentage"</Code>"). A "
                    <Code inline=true>"min"</Code>" greater than "<Code inline=true>"max"</Code>" reverses the slider."
                </p>
                <Demo description="Automatic and custom marks, a fractional step and a reversed slider" source=include_str!("demos/slider_marks.rs")>
                    <SliderMarksDemo/>
                </Demo>
            </Section>

            <Section title="Range Slider">
                <p>
                    <Code inline=true>"RangeSlider"</Code>" selects a range with two thumbs. It takes the same marks, steps "
                    "and popovers as "<Code inline=true>"Slider"</Code>"."
                </p>
                <Demo description="A continuous and a stepped range slider" source=include_str!("demos/slider_range.rs")>
                    <SliderRangeDemo/>
                </Demo>
            </Section>

            <Section title="Value Popover">
                <p>
                    <Code inline=true>"popover"</Code>" shows the formatted value above the thumb: "
                    <Code inline=true>"SliderPopover::When { hovered, dragged }"</Code>" while the thumb is hovered or "
                    "dragged, "<Code inline=true>"SliderPopover::Always"</Code>" permanently."
                </p>
                <Demo description="Value popovers on interaction and always, on a slider and a range slider" source=include_str!("demos/slider_popover.rs")>
                    <SliderPopoverDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <p>"Each thumb is focusable; "<Keys keys="Tab"/>" moves between them."</p>
                <KeyboardTable>
                    <KeyRow keys="ArrowRight / ArrowUp">
                        "Increase by one step (by 1% of the range on continuous sliders)."
                    </KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">
                        "Decrease by one step (by 1% of the range on continuous sliders)."
                    </KeyRow>
                    <KeyRow keys="PageUp / Shift + ArrowRight / Shift + ArrowUp">
                        "Increase by a page: a tenth of the range, snapped to the step."
                    </KeyRow>
                    <KeyRow keys="PageDown / Shift + ArrowLeft / Shift + ArrowDown">
                        "Decrease by a page."
                    </KeyRow>
                    <KeyRow keys="Home">"Jump to the minimum."</KeyRow>
                    <KeyRow keys="End">"Jump to the maximum."</KeyRow>
                </KeyboardTable>
                <p>"In right-to-left layouts, the left and right arrow keys swap their meaning."</p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the sliders to your design:"</p>
                <CssVariables prefix="--slider-" scss=theme_scss!("slider")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Slider.materialize()>"Slider overview"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"use_slider"</Link></li>
                <li><Link href=routes::doc::slider::Atom.materialize()>"Slider atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
