use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    slider_basic::SliderBasicDemo, slider_disabled::SliderDisabledDemo,
    slider_marks::SliderMarksDemo, slider_popover::SliderPopoverDemo,
    slider_range::SliderRangeDemo, slider_variants::SliderVariantsDemo, slider_vertical::SliderVerticalDemo,
    slider_volume::SliderVolumeDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageSlider() -> impl IntoView {
    view! {
        <DocPage title="Slider Components">
            <p>
                "The themed "<Code inline=true>"Slider"</Code>" picks a value and "<Code inline=true>"RangeSlider"</Code>
                " a range of values. See the "<Link href=routes::doc::Slider.materialize()>"Slider overview"</Link>
                " for when to use a slider."
            </p>

            <Demo description="Sliders with a coarse and a fine step" source=include_str!("demos/slider_basic.rs")>
                <SliderBasicDemo/>
            </Demo>

            <Section title="Slider">
                <p>
                    "Generic over the value type: integers ("<Code inline=true>"u8"</Code>", "<Code inline=true>"i32"</Code>
                    ", \u{2026}) or floats, taken from "<Code inline=true>"value"</Code>"."
                </p>
                <Section title="Props" id="slider-props">
                    <ApiTable kind=ApiKind::Props of="components::slider::Slider">
                        <ApiRow name="value" ty="NumberSignal<T>">"The value: a number or any signal of one. Required."</ApiRow>
                        <ApiRow name="set_value" ty="Out<T>">
                            "Receives the new value: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure or "<Code inline=true>"Callback"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<T>">"The range. Required."</ApiRow>
                        <ApiRow name="step" ty="Signal<T>" default="1">"The step values snap to."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">"The axis of the track, see "<AnchorLink href="#vertical">"Vertical"</AnchorLink>"."</ApiRow>
                        <ApiRow name="variant" ty="SliderVariant" default="Round">"Round or block (square) thumbs and track ends."</ApiRow>
                        <ApiRow name="popover" ty="SliderPopover" default="Never">"When the value shows above the thumb; see "<AnchorLink href="#value-popover">"Value Popover"</AnchorLink>"."</ApiRow>
                        <ApiRow name="marks" ty="SliderMarks" default="None">"Marks along the track; see "<AnchorLink href="#marks">"Marks"</AnchorLink>"."</ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="NumberFormatOptions::default()">"How the value is formatted (popover, marks, assistive technology)."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the slider, which has no visible label."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the slider."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="RangeSlider">
                <p>
                    "Two thumbs, which can\u{2019}t pass each other, select a range. It takes the same marks, steps and "
                    "popovers as "<Code inline=true>"Slider"</Code>"."
                </p>
                <Section title="Props" id="range-slider-props">
                    <ApiTable kind=ApiKind::Props of="RangeSlider">
                        <ApiRow name="value" ty="Signal<RangeInclusive<T>>">"The range: a plain value or any signal. Required."</ApiRow>
                        <ApiRow name="set_value" ty="Out<RangeInclusive<T>>">"Receives the new range. Required."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<T>">"The bounds of the slider. Required."</ApiRow>
                        <ApiRow name="step" ty="Signal<T>" default="1">"The step values snap to."</ApiRow>
                        <ApiRow name="thumb_labels" ty="Signal<(String, String)>" default="(\"Minimum\", \"Maximum\")">"Names of the two thumbs, next to the slider\u{2019}s name."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">"The axis of the track, see "<AnchorLink href="#vertical">"Vertical"</AnchorLink>"."</ApiRow>
                        <ApiRow name="variant" ty="SliderVariant" default="Round">"Round or block (square) thumbs and track ends."</ApiRow>
                        <ApiRow name="popover" ty="SliderPopover" default="Never">"When the values show above the thumbs."</ApiRow>
                        <ApiRow name="marks" ty="SliderMarks" default="None">"Marks along the track."</ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="NumberFormatOptions::default()">"How the values are formatted (popovers, marks, assistive technology)."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the slider, which has no visible label."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the slider."</ApiRow>
                    </ApiTable>
                </Section>

                <Demo description="A fine-stepped and a stepped range slider" source=include_str!("demos/slider_range.rs")>
                    <SliderRangeDemo/>
                </Demo>
            </Section>

            <Section title="Step and Precision">
                <p>
                    "The "<Code inline=true>"step"</Code>" decides which values you can select: "<Code inline=true>"min_value"</Code>
                    " plus multiples of the step. Integer sliders hold exact values; float sliders round to the step\u{2019}s "
                    "precision, so stepping by 0.1 gives 0.3, not 0.30000000000000004. A fine step suits values whose exact "
                    "number doesn\u{2019}t matter to the user, such as a volume."
                </p>

                <Demo description="Continuous volume slider between two icons" source=include_str!("demos/slider_volume.rs")>
                    <SliderVolumeDemo/>
                </Demo>
            </Section>

            <Section title="Marks">
                <p>
                    "Marks show the selectable values. "<Code inline=true>"SliderMarks::Automatic"</Code>
                    " places one mark per step (every n-th step on long ranges), optionally named by the formatted value. "
                    <Code inline=true>"SliderMarks::Custom"</Code>" places "<Code inline=true>"SliderMark"</Code>"s at values ("
                    <Code inline=true>"SliderMarkValue::Value"</Code>") or at fractions of the track ("
                    <Code inline=true>"SliderMarkValue::Percentage"</Code>"). Marks within the selected range are highlighted."
                </p>
                <Demo description="Automatic and custom marks, and a fractional step" source=include_str!("demos/slider_marks.rs")>
                    <SliderMarksDemo/>
                </Demo>
            </Section>

            <Section title="Value Popover">
                <p>
                    <Code inline=true>"popover"</Code>" shows the formatted value above a thumb: "
                    <Code inline=true>"SliderPopover::When { hovered, dragged }"</Code>" while the thumb is hovered or "
                    "dragged, "<Code inline=true>"SliderPopover::Always"</Code>" permanently. It only shows the value; the "
                    "thumb announces it to assistive technology."
                </p>
                <Demo description="Value popovers on interaction and always, on a slider and a range slider" source=include_str!("demos/slider_popover.rs")>
                    <SliderPopoverDemo/>
                </Demo>
            </Section>

            <Section title="Variants">
                <p>
                    <Code inline=true>"SliderVariant::Round"</Code>" draws round thumbs and track ends, "
                    <Code inline=true>"SliderVariant::Block"</Code>" square ones."
                </p>
                <Demo description="Round and block slider" source=include_str!("demos/slider_variants.rs")>
                    <SliderVariantsDemo/>
                </Demo>
            </Section>

            <Section title="Vertical">
                <p>
                    "With "<Code inline=true>"orientation=Orientation::Vertical"</Code>" the track stands upright, 10em tall "
                    "by default, and the value grows from the bottom up. "<Keys keys="ArrowUp"/>" and "<Keys keys="ArrowRight"/>
                    " increase it."
                </p>
                <Demo description="Vertical level slider with a status line" source=include_str!("demos/slider_vertical.rs")>
                    <SliderVerticalDemo/>
                </Demo>
            </Section>

            <Section title="Disabled">
                <p>
                    "A disabled slider ignores pointer and keyboard input, its thumbs leave the tab order, and the theme "
                    "dims it."
                </p>
                <Demo description="Slider with a disabled toggle" source=include_str!("demos/slider_disabled.rs")>
                    <SliderDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Accessibility">
                <p>
                    "The components behave as described on the "<Link href=routes::doc::Slider.materialize()>"Slider overview"</Link>
                    ". A thumb with keyboard focus shows a focus ring (the theme styles "<Code inline=true>"data-focus-visible"</Code>
                    "), and pointer hover is styled through "<Code inline=true>"data-hovered"</Code>", so touch doesn\u{2019}t "
                    "leave thumbs looking hovered."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt the sliders to your design:"</p>
                <CssVariables prefix="--slider-" scss=theme_scss!("slider")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Slider.materialize()>"Slider overview"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"Slider Hooks"</Link></li>
                <li><Link href=routes::doc::slider::Atom.materialize()>"Slider Atoms"</Link></li>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
