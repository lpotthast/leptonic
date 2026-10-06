use std::borrow::Cow;

use indoc::indoc;
use leptonic::{
    atoms::slider::{
        Slider as SliderAtom, SliderMark, SliderMarks as SliderMarksAtom, SliderOutput,
        SliderThumb, SliderTrack, SliderTrackFill,
    },
    components::prelude::*,
    hooks::{SliderMark as SliderMarkSpec, SliderMarkValue, SliderMarks, SliderValues},
};
use leptos::prelude::*;
use ordered_float::OrderedFloat;

use super::demos::{
    slider_basic::SliderBasicDemo, slider_callbacks::SliderCallbacksDemo,
    slider_disabled::SliderDisabledDemo, slider_range::SliderRangeDemo,
    slider_vertical::SliderVerticalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageAtomSlider() -> impl IntoView {
    view! {
        <DocPage title="Slider atom">
            <p>
                "The "<Code inline=true>"Slider"</Code>" atoms are unstyled, composable components for sliders with any "
                "number of thumbs. See the "<Link href=routes::doc::Slider.materialize()>"Slider overview"</Link>
                " for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::slider::Hook.materialize()>"use_slider_state, use_slider, use_slider_thumb and use_slider_marks"</Link>
                    ". "<Code inline=true>"Slider"</Code>" creates the state and the group, every "
                    <Code inline=true>"SliderThumb"</Code>" adds a thumb, "<Code inline=true>"SliderMarks"</Code>" computes marks."
                </p>
            </Section>

            <Section title="Props">
                <Section title="Slider">
                    <ApiTable kind=ApiKind::Props of="atoms::slider::Slider">
                        <ApiRow name="values" ty="SliderValues">
                            <Code inline=true>"Uncontrolled(initial)"</Code>" or "<Code inline=true>"Controlled(signal)"</Code>
                            ". The number of values must match the number of "<Code inline=true>"SliderThumb"</Code>"s."
                        </ApiRow>
                        <ApiRow name="min" ty="f64" default="0.0">"Minimum value."</ApiRow>
                        <ApiRow name="max" ty="f64" default="100.0">"Maximum value."</ApiRow>
                        <ApiRow name="step" ty="Option<f64>" default="Some(1.0)">
                            "Step values snap to. "<Code inline=true>"None"</Code>" makes the slider continuous."
                        </ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">
                            "Horizontal or vertical. A horizontal slider runs from right to left when an enclosing "
                            <Code inline=true>"I18nProvider"</Code>" has a right-to-left locale."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<f64>>>" default="None">"Called whenever a value changes."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<Vec<f64>>>" default="None">
                            "Called when the user releases the last dragged thumb."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>" default="None">"Accessible name of the slider group."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Id of the element that labels the group."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">"Track, output, marks and any other content."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="SliderThumb">
                    <ApiTable kind=ApiKind::Props of="SliderThumb">
                        <ApiRow name="index" ty="Option<usize>" default="None">
                            "The thumb\u{2019}s index. Assigned in render order when not set."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<&'static str>" default="None">"Name of the range input for form submission."</ApiRow>
                        <ApiRow name="aria_label" ty="Option<Cow<'static, str>>" default="None">"Accessible name of the thumb."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_details, aria_errormessage" ty="Option<&'static str>" default="None">
                            "Ids of elements that label, describe or add details or an error message to the thumb."
                        </ApiRow>
                        <ApiRow name="decimal_places" ty="Option<usize>" default="None">"Decimal places of the formatted value."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the thumb element."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">
                            "Content of the thumb, e.g. a "<Code inline=true>"SliderThumbTooltip"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Other Atoms">
                    <DocTable headers=&["Atom", "Purpose"]>
                        <TableRow>
                            <TableCell><Code inline=true>"SliderTrack"</Code></TableCell>
                            <TableCell>"The track. Takes "<Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>
                                " and children: the fill and the thumbs."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"SliderTrackFill"</Code></TableCell>
                            <TableCell>"The filled part of the track: from the start to the thumb, or between two thumbs. Sliders with "
                                "more than two thumbs render no fill."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"SliderOutput"</Code></TableCell>
                            <TableCell>"Render prop ("<Code inline=true>"let:attrs let:values"</Code>") for an "
                                <Code inline=true>"<output>"</Code>": spread "<Code inline=true>"attrs"</Code>" onto it and show "
                                <Code inline=true>"values"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"SliderThumbTooltip"</Code></TableCell>
                            <TableCell>"A tooltip inside a "<Code inline=true>"SliderThumb"</Code>". "<Code inline=true>"popover"</Code>
                                " ("<Code inline=true>"Never"</Code>", "<Code inline=true>"When { hovered, dragged }"</Code>", "
                                <Code inline=true>"Always"</Code>") decides when it gets "<Code inline=true>"data-visible"</Code>
                                ", "<Code inline=true>"value_display"</Code>" formats the value."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"SliderMarks"</Code>", "<Code inline=true>"SliderMark"</Code></TableCell>
                            <TableCell>"Marks along the track, see "<a href="#marks">"Marks"</a>"."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::slider::*, hooks::SliderValues};

                        view! {
                            <Slider values=SliderValues::Uncontrolled(vec![50.0]) classes="my-slider">
                                <SliderTrack classes="my-track">
                                    <SliderTrackFill classes="my-fill"/>
                                    <SliderThumb aria_label="Volume" classes="my-thumb"/>
                                </SliderTrack>
                                <SliderOutput let:attrs let:values>
                                    <output {..attrs}>
                                        {move || format!("{:.0}%", values.get().first().copied().unwrap_or_default())}
                                    </output>
                                </SliderOutput>
                            </Slider>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <Demo description="Single-thumb slider atom with its value" source=include_str!("demos/slider_basic.rs")>
                    <SliderBasicDemo/>
                </Demo>
            </Section>

            <Section title="Range Slider">
                <p>
                    "Pass two values and render two "<Code inline=true>"SliderThumb"</Code>"s. The thumbs cannot cross each other."
                </p>

                <Demo description="Two-thumb range slider atom" source=include_str!("demos/slider_range.rs")>
                    <SliderRangeDemo/>
                </Demo>
            </Section>

            <Section title="Callbacks">
                <p>
                    <Code inline=true>"on_change"</Code>" fires on every value change, "<Code inline=true>"on_change_end"</Code>
                    " once when the user releases the thumb."
                </p>

                <Demo description="Slider atom logging on_change and on_change_end" source=include_str!("demos/slider_callbacks.rs")>
                    <SliderCallbacksDemo/>
                </Demo>
            </Section>

            <Section title="Vertical Slider">
                <p>"Set "<Code inline=true>"orientation=Orientation::Vertical"</Code>" and give the track a height."</p>

                <Demo description="Vertical slider atom" source=include_str!("demos/slider_vertical.rs")>
                    <SliderVerticalDemo/>
                </Demo>
            </Section>

            <Section title="Disabled Slider">
                <Demo description="Slider atom with a disabled toggle" source=include_str!("demos/slider_disabled.rs")>
                    <SliderDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Marks">
                <p>
                    <Code inline=true>"SliderMarks"</Code>" computes marks with "
                    <Link href=routes::doc::slider::Hook.materialize()>"use_slider_marks"</Link>" and passes them to its "
                    "children. Render each with "<Code inline=true>"SliderMark"</Code>", which positions it and adds the "
                    <Code inline=true>"in-range"</Code>" class while the mark lies within the selected range. "
                    <Code inline=true>"SliderMark"</Code>" does not render the mark\u{2019}s name; render it as a child."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        <SliderMarks marks=SliderMarks::Automatic { create_names: true } classes="my-marks" let:marks>
                            <For
                                each=move || marks.get()
                                key=|mark| OrderedFloat::from(mark.percentage)
                                children=|mark| view! {
                                    <SliderMark mark=mark.clone()>{mark.name.unwrap_or_default()}</SliderMark>
                                }
                            />
                        </SliderMarks>
                    "#)}
                </Code>

                <p>"Automatic marks, one per step:"</p>

                <Demo description="Slider atom with automatic marks at every step">
                    <SliderAtom
                        values=SliderValues::Uncontrolled(vec![5.0])
                        min=0.0
                        max=10.0
                        classes=["demo-slider", "demo-slider-stacked"]
                    >
                        <div class="demo-slider-row">
                            <span class="demo-slider-label demo-slider-label-wide">"Auto marks"</span>
                            <SliderTrack classes="demo-slider-track">
                                <SliderTrackFill classes="demo-slider-fill"/>
                                <SliderThumb aria_label="Auto marks" classes="demo-slider-thumb"/>
                            </SliderTrack>
                            <SliderOutput let:attrs let:values>
                                <output {..attrs} class="demo-slider-output">
                                    {move || format!("{:.0}", values.get().first().copied().unwrap_or_default())}
                                </output>
                            </SliderOutput>
                        </div>
                        <SliderMarksAtom marks=SliderMarks::Automatic { create_names: true } classes="demo-slider-marks" let:marks>
                            <For
                                each=move || marks.get()
                                key=|mark| OrderedFloat::from(mark.percentage)
                                children=|mark| view! { <SliderMark mark=mark.clone()>{mark.name.unwrap_or_default()}</SliderMark> }
                            />
                        </SliderMarksAtom>
                    </SliderAtom>
                </Demo>

                <p>"Custom marks at a value or a fraction of the track:"</p>

                <Demo description="Slider atom with custom Min, Mid and Max marks">
                    <SliderAtom values=SliderValues::Uncontrolled(vec![50.0]) classes=["demo-slider", "demo-slider-stacked", "demo-slider-blue"]>
                        <div class="demo-slider-row">
                            <span class="demo-slider-label demo-slider-label-wide">"Custom marks"</span>
                            <SliderTrack classes="demo-slider-track">
                                <SliderTrackFill classes="demo-slider-fill"/>
                                <SliderThumb aria_label="Custom marks" classes="demo-slider-thumb"/>
                            </SliderTrack>
                            <SliderOutput let:attrs let:values>
                                <output {..attrs} class="demo-slider-output">
                                    {move || format!("{:.0}", values.get().first().copied().unwrap_or_default())}
                                </output>
                            </SliderOutput>
                        </div>
                        <SliderMarksAtom
                            marks=SliderMarks::Custom {
                                marks: vec![
                                    SliderMarkSpec { value: SliderMarkValue::Value(0.0), name: Some(Cow::Borrowed("Min")) },
                                    SliderMarkSpec { value: SliderMarkValue::Percentage(0.5), name: Some(Cow::Borrowed("Mid")) },
                                    SliderMarkSpec { value: SliderMarkValue::Value(100.0), name: Some(Cow::Borrowed("Max")) },
                                ],
                            }
                            classes="demo-slider-marks"
                            let:marks
                        >
                            <For
                                each=move || marks.get()
                                key=|mark| OrderedFloat::from(mark.percentage)
                                children=|mark| view! { <SliderMark mark=mark.clone()>{mark.name.unwrap_or_default()}</SliderMark> }
                            />
                        </SliderMarksAtom>
                    </SliderAtom>
                </Demo>

                <p>"With two thumbs, marks between the thumbs are in range:"</p>

                <Demo description="Range slider atom with automatic marks highlighting the selected range">
                    <SliderAtom
                        values=SliderValues::Uncontrolled(vec![3.0, 7.0])
                        min=0.0
                        max=10.0
                        classes=["demo-slider", "demo-slider-stacked", "demo-slider-green"]
                    >
                        <div class="demo-slider-row">
                            <span class="demo-slider-label demo-slider-label-wide">"Range marks"</span>
                            <SliderTrack classes="demo-slider-track">
                                <SliderTrackFill classes="demo-slider-fill"/>
                                <SliderThumb aria_label="Range start" classes="demo-slider-thumb"/>
                                <SliderThumb aria_label="Range end" classes="demo-slider-thumb"/>
                            </SliderTrack>
                        </div>
                        <SliderMarksAtom
                            marks=SliderMarks::Automatic { create_names: true }
                            classes=["demo-slider-marks", "demo-slider-marks-no-output"]
                            let:marks
                        >
                            <For
                                each=move || marks.get()
                                key=|mark| OrderedFloat::from(mark.percentage)
                                children=|mark| view! { <SliderMark mark=mark.clone()>{mark.name.unwrap_or_default()}</SliderMark> }
                            />
                        </SliderMarksAtom>
                    </SliderAtom>
                </Demo>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-focus-visible" ty="true">"On "<Code inline=true>"SliderThumb"</Code>" while it has keyboard focus."</ApiRow>
                    <ApiRow name="data-dragging" ty="true">"On "<Code inline=true>"SliderThumb"</Code>" while it is dragged."</ApiRow>
                    <ApiRow name="data-visible" ty="true">
                        "On "<Code inline=true>"SliderThumbTooltip"</Code>" while it should be shown."
                    </ApiRow>
                </ApiTable>

                <p>
                    "The group element has "<Code inline=true>"aria-disabled=\"true\""</Code>" while disabled, and "
                    <Code inline=true>"SliderMark"</Code>" has the "<Code inline=true>"in-range"</Code>" class while in range."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms only set the positioning styles they compute (fill size, thumb and mark positions). "
                    "Give the track a size and "<Code inline=true>"position: relative"</Code>", and style the rest with classes:"
                </p>

                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-track { position: relative; height: 8px; background: lightgray; }
                        .my-fill { background: royalblue; }
                        .my-thumb { width: 20px; height: 20px; border-radius: 50%; background: royalblue; }
                        .my-thumb[data-focus-visible] { outline: 2px solid royalblue; outline-offset: 2px; }
                        .my-thumb[data-dragging] { cursor: grabbing; }
                        .my-slider[aria-disabled="true"] { opacity: 0.5; }
                    "#)}
                </Code>
            </Section>

            <Section title="Form Submission">
                <p>
                    "Give a "<Code inline=true>"SliderThumb"</Code>" a "<Code inline=true>"name"</Code>
                    " to submit its value with a form. The name is set on the thumb\u{2019}s range input."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        <Slider values=SliderValues::Uncontrolled(vec![50.0])>
                            <SliderTrack>
                                <SliderTrackFill/>
                                <SliderThumb name="volume" aria_label="Volume"/>
                            </SliderTrack>
                        </Slider>
                    "#)}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Slider.materialize()>"Slider overview"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"Slider hooks"</Link></li>
                <li><Link href=routes::doc::slider::Component.materialize()>"Slider component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
