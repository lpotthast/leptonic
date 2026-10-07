use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    slider_basic::SliderBasicDemo, slider_callbacks::SliderCallbacksDemo,
    slider_marks::SliderMarksDemo, slider_offset::SliderOffsetDemo, slider_range::SliderRangeDemo,
    slider_vertical::SliderVerticalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageAtomSlider() -> impl IntoView {
    let hook = |anchor: &str| format!("{}#{anchor}", routes::doc::slider::Hook.materialize());

    view! {
        <DocPage title="Slider Atoms">
            <p>
                "The slider atoms render an unstyled slider with any number of thumbs from parts that position themselves. "
                "See the "<Link href=routes::doc::Slider.materialize()>"Slider overview"</Link>" for when to use a slider."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Hook", "Used by"]>
                    <TableRow>
                        <TableCell><Link href=hook("use-slider-state")><Code inline=true>"use_slider_state"</Code></Link></TableCell>
                        <TableCell><Code inline=true>"Slider"</Code>": the values of all thumbs."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=hook("use-slider")><Code inline=true>"use_slider"</Code></Link></TableCell>
                        <TableCell><Code inline=true>"Slider"</Code>" (group, label, description), "<Code inline=true>"SliderTrack"</Code>", "<Code inline=true>"SliderOutput"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=hook("use-slider-thumb")><Code inline=true>"use_slider_thumb"</Code></Link></TableCell>
                        <TableCell><Code inline=true>"SliderThumb"</Code>", with "<Code inline=true>"use_focus_ring"</Code>" for "<Code inline=true>"data-focus-visible"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=hook("use-slider-marks")><Code inline=true>"use_slider_marks"</Code></Link></TableCell>
                        <TableCell><Code inline=true>"SliderMarks"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::interactions::UseHover.materialize()><Code inline=true>"use_hover"</Code></Link></TableCell>
                        <TableCell><Code inline=true>"SliderTrack"</Code>", "<Code inline=true>"SliderFill"</Code>", "<Code inline=true>"SliderThumb"</Code>": "<Code inline=true>"data-hovered"</Code>"."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::{field::Label, slider::*};
                        use leptos::prelude::*;

                        view! {
                            <Slider min_value=0 max_value=100 default_values=vec![50_u8] classes="my-slider">
                                <Label>"Volume"</Label>
                                <SliderTrack classes="my-track">
                                    <SliderFill classes="my-fill"/>
                                    <SliderThumb classes="my-thumb"/>
                                </SliderTrack>
                                <SliderOutput/>
                            </Slider>
                        }
                    "#)}
                </Code>
                <p>
                    "The value type comes from "<Code inline=true>"default_values"</Code>" or "<Code inline=true>"values"</Code>
                    ": write typed literals ("<Code inline=true>"vec![50_u8]"</Code>"), since the bounds\u{2019} conversions can\u{2019}t tell it."
                </p>
            </Section>

            <Section title="Demo">
                <Demo description="Slider atom with a label, a fill and its value" source=include_str!("demos/slider_basic.rs")>
                    <SliderBasicDemo/>
                </Demo>
            </Section>

            <Section title="Range Slider">
                <p>
                    "Pass two values and render two "<Code inline=true>"SliderThumb"</Code>"s with their "<Code inline=true>"index"</Code>
                    ". A thumb can\u{2019}t pass its neighbor. Here, "<Code inline=true>"values"</Code>" and "
                    <Code inline=true>"set_values"</Code>" bind the range to app state, and a "<Code inline=true>"SliderThumbTooltip"</Code>
                    " in each thumb shows its value while it is hovered or dragged."
                </p>

                <Demo description="Two-thumb range slider atom with value tooltips" source=include_str!("demos/slider_range.rs")>
                    <SliderRangeDemo/>
                </Demo>
            </Section>

            <Section title="Callbacks">
                <p>
                    <Code inline=true>"on_change"</Code>" runs on every value change, also while dragging; "
                    <Code inline=true>"on_change_end"</Code>" runs once when the user releases the thumb, and after each keyboard change."
                </p>

                <Demo description="Slider atom showing the last on_change and on_change_end" source=include_str!("demos/slider_callbacks.rs")>
                    <SliderCallbacksDemo/>
                </Demo>
            </Section>

            <Section title="Fill Offset">
                <p>
                    "A fill starts at the beginning of the track. For a value around a center, such as a balance, give "
                    <Code inline=true>"SliderFill"</Code>" an "<Code inline=true>"offset"</Code>": the fill then runs from that value to the thumb."
                </p>

                <Demo description="Balance slider whose fill starts at 0" source=include_str!("demos/slider_offset.rs")>
                    <SliderOffsetDemo/>
                </Demo>
            </Section>

            <Section title="Vertical Slider">
                <p>
                    "Set "<Code inline=true>"orientation=Orientation::Vertical"</Code>" and give the track a height and a width. "
                    "The thumbs position themselves from the top and the fill grows from the bottom; size the fill across the "
                    "track with "<Code inline=true>"[data-orientation=\"vertical\"]"</Code>"."
                </p>

                <Demo description="Vertical slider atom" source=include_str!("demos/slider_vertical.rs")>
                    <SliderVerticalDemo/>
                </Demo>
            </Section>

            <Section title="Marks">
                <p>
                    <Code inline=true>"SliderMarks"</Code>" computes the marks (one per step, or the ones you list) and passes them to "
                    "its children. Render each with "<Code inline=true>"SliderMark"</Code>", which positions it along the track and "
                    "sets "<Code inline=true>"data-in-range"</Code>" while the mark lies within the selected range; a mark\u{2019}s name "
                    "is its child. Marks are hidden from assistive technology, as the thumbs announce the values. Put "
                    <Code inline=true>"SliderMarks"</Code>" where it is as wide as the track: here, the slider stacks label, track and marks."
                </p>

                <Demo description="Range slider atom with named marks highlighting the selected range" source=include_str!("demos/slider_marks.rs") source_open=true>
                    <SliderMarksDemo/>
                </Demo>

                <p>"Marks are placed horizontally only: in a vertical slider, they don\u{2019}t follow the track."</p>
            </Section>

            <Section title="Slider">
                <p>
                    "The group around all parts. Generic over the value type: integers ("<Code inline=true>"u8"</Code>", "
                    <Code inline=true>"i32"</Code>", \u{2026}) or floats."
                </p>
                <Section title="Props" id="slider-props">
                    <ApiTable kind=ApiKind::Props of="atoms::slider::Slider">
                        <ApiRow name="min_value, max_value" ty="Signal<T>">"The range. Required."</ApiRow>
                        <ApiRow name="step" ty="Signal<T>" default="1">"The step values snap to."</ApiRow>
                        <ApiRow name="default_values" ty="Option<Vec<T>>" default="None">
                            "The initial values, one per "<Code inline=true>"SliderThumb"</Code>", in ascending order. "
                            <Code inline=true>"None"</Code>": one thumb at "<Code inline=true>"min_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="values" ty="Option<Signal<Vec<T>>>" default="None">
                            "The values from app state, replacing "<Code inline=true>"default_values"</Code>"."
                        </ApiRow>
                        <ApiRow name="set_values" ty="Option<Out<Vec<T>>>" default="None">
                            "Receives the new values: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure or "<Code inline=true>"Callback"</Code>"."
                        </ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">
                            "The axis of the track. A horizontal slider runs from right to left in a right-to-left locale."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Whether the value is invalid: sets "<Code inline=true>"aria-invalid"</Code>" on the thumbs\u{2019} inputs and "
                            "shows a "<Code inline=true>"FieldError"</Code>"\u{2019}s children (the slider has no validation errors of its own)."
                        </ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="NumberFormatOptions::default()">
                            "How values are formatted for the output, the tooltips and assistive technology."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group\u{2019}s id; the thumbs\u{2019} ids derive from it. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the slider when it has no "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the slider."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<T>>>" default="None">"Called whenever the values change, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<Vec<T>>>" default="None">
                            "Called when the user stops dragging, and after a keyboard change."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the group element."</ApiRow>
                        <ApiRow name="children" ty="Children">
                            "The "<Code inline=true>"SliderTrack"</Code>" and optionally a "<Code inline=true>"Label"</Code>", a "
                            <Code inline=true>"SliderOutput"</Code>", "<Code inline=true>"SliderMarks"</Code>", a "
                            <Code inline=true>"Description"</Code>" and a "<Code inline=true>"FieldError"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SliderTrack">
                <p>"The track ("<Code inline=true>"position: relative"</Code>"): a press on it moves the closest thumb there and drags it. Holds the fill and the thumbs."</p>
                <Section title="Props" id="slider-track-props">
                    <ApiTable kind=ApiKind::Props of="SliderTrack">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the track."</ApiRow>
                        <ApiRow name="children" ty="Children">"A "<Code inline=true>"SliderFill"</Code>" and the "<Code inline=true>"SliderThumb"</Code>"s. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SliderFill">
                <p>
                    "The filled part of the track ("<Code inline=true>"position: absolute"</Code>"): from the start or "
                    <Code inline=true>"offset"</Code>" to the thumb, or between the first and the last thumb. It sets its "
                    "position and length along the track; size it across the track with CSS."
                </p>
                <Section title="Props" id="slider-fill-props">
                    <ApiTable kind=ApiKind::Props of="SliderFill">
                        <ApiRow name="offset" ty="MaybeProp<f64>" default="None">
                            "Where the fill of a one-thumb slider starts, as a value of the range (e.g. 0 in a range from -10 to 10). "
                            <Code inline=true>"None"</Code>": the start of the track. Ignored with several thumbs."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the fill."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SliderThumb">
                <p>
                    "A thumb ("<Code inline=true>"position: absolute"</Code>", centered on its value along the track) with its "
                    "visually hidden "<Code inline=true>"<input type=\"range\">"</Code>". Render one per value."
                </p>
                <Section title="Props" id="slider-thumb-props">
                    <ApiTable kind=ApiKind::Props of="SliderThumb">
                        <ApiRow name="index" ty="usize" default="0">"The thumb\u{2019}s value in the slider\u{2019}s values."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables this thumb only."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s name, to submit the thumb\u{2019}s value with a form."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the input belongs to, if not its ancestor."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the thumb next to the slider\u{2019}s name, e.g. \u{201c}Minimum\u{201d}."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of further elements naming the thumb."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the thumb."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"Content of the thumb, e.g. a "<Code inline=true>"SliderThumbTooltip"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SliderThumbTooltip">
                <p>
                    "The formatted value of the "<Code inline=true>"SliderThumb"</Code>" around it, hidden from assistive "
                    "technology (the thumb\u{2019}s input announces the value). It sets "<Code inline=true>"data-visible"</Code>
                    " as "<Code inline=true>"popover"</Code>" says; hide it without the attribute."
                </p>
                <Section title="Props" id="slider-thumb-tooltip-props">
                    <ApiTable kind=ApiKind::Props of="SliderThumbTooltip">
                        <ApiRow name="popover" ty="SliderPopover" default="Never">
                            "When it is visible: "<Code inline=true>"SliderPopover::Never"</Code>", "
                            <Code inline=true>"When { hovered, dragged }"</Code>" (while the thumb is hovered, dragged or both) or "
                            <Code inline=true>"Always"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the tooltip."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SliderOutput">
                <p>"An "<Code inline=true>"<output>"</Code>" with the formatted values: \u{201c}20\u{201d} or \u{201c}20 \u{2013} 80\u{201d}. It isn\u{2019}t announced while dragging."</p>
                <Section title="Props" id="slider-output-props">
                    <ApiTable kind=ApiKind::Props of="SliderOutput">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the output."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SliderMarks">
                <p>"Computes the marks of the slider and renders its children with them, in a "<Code inline=true>"<div>"</Code>" hidden from assistive technology."</p>
                <Section title="Props" id="slider-marks-props">
                    <ApiTable kind=ApiKind::Props of="atoms::slider::SliderMarks">
                        <ApiRow name="marks" ty="SliderMarks">
                            "Which marks: "<Code inline=true>"SliderMarks::None"</Code>", "<Code inline=true>"Automatic { create_names }"</Code>
                            " (one per step, at most about 20, optionally named by the formatted value) or "
                            <Code inline=true>"Custom { marks }"</Code>" (see "<Link href=hook("use-slider-marks")>"use_slider_marks"</Link>"). Required."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the marks\u{2019} container."</ApiRow>
                        <ApiRow name="children" ty="Fn(Signal<Vec<ComputedSliderMark>>) -> impl IntoView">
                            "Renders the computed marks, usually a "<Code inline=true>"For"</Code>" of "<Code inline=true>"SliderMark"</Code>
                            "s; with "<Code inline=true>"let:marks"</Code>" in "<Code inline=true>"view!"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="SliderMark">
                <p>"One mark, positioned along the track ("<Code inline=true>"position: absolute"</Code>", "<Code inline=true>"left"</Code>" in percent)."</p>
                <Section title="Props" id="slider-mark-props">
                    <ApiTable kind=ApiKind::Props of="atoms::slider::SliderMark">
                        <ApiRow name="mark" ty="ComputedSliderMark">"The mark from "<Code inline=true>"SliderMarks"</Code>". Required."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the mark."</ApiRow>
                        <ApiRow name="children" ty="Children">"The mark\u{2019}s content, e.g. its name. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-orientation" ty="\"horizontal\" | \"vertical\"">"On the slider, track, fill and output."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"On the slider, track, fill, thumbs and output while disabled; on a thumb also while it alone is disabled."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"On the track, fill and thumbs while hovered."</ApiRow>
                    <ApiRow name="data-dragging" ty="true">"On a thumb while it is dragged."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"On a thumb while its input has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"On a thumb while its input has keyboard focus."</ApiRow>
                    <ApiRow name="data-visible" ty="true">"On a "<Code inline=true>"SliderThumbTooltip"</Code>" while shown."</ApiRow>
                    <ApiRow name="data-in-range" ty="true">"On a "<Code inline=true>"SliderMark"</Code>" within the selected range."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no look, and set only what they compute: the track is "<Code inline=true>"relative"</Code>
                    "; the fill, thumbs and marks are "<Code inline=true>"absolute"</Code>", positioned along the track (a thumb "
                    "is centered on its value with "<Code inline=true>"translate(-50%, -50%)"</Code>"). Their default classes "
                    "are "<Code inline=true>"leptonic-Slider"</Code>", "<Code inline=true>"leptonic-SliderTrack"</Code>", "
                    <Code inline=true>"leptonic-SliderFill"</Code>", "<Code inline=true>"leptonic-SliderThumb"</Code>", "
                    <Code inline=true>"leptonic-SliderThumbTooltip"</Code>", "<Code inline=true>"leptonic-SliderOutput"</Code>", "
                    <Code inline=true>"leptonic-SliderMarks"</Code>" and "<Code inline=true>"leptonic-SliderMark"</Code>
                    ". A thumb renders its visually hidden range input itself; a thumb\u{2019}s children (a "
                    <Code inline=true>"SliderThumbTooltip"</Code>") and a mark\u{2019}s children (its name) are yours."
                </p>
                <p>
                    "Give the track a size, let the fill span its height, center the thumbs across the track, and style "
                    "the state through the data attributes. The demos above use this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .demo-slider { display: flex; align-items: center; gap: 1rem; }
                        .demo-slider[data-disabled] { opacity: 0.5; }

                        .demo-slider-track { flex: 1; height: 8px; border-radius: 4px; background: var(--border); cursor: pointer; }
                        .demo-slider-fill { inset: 0 auto 0 0; border-radius: 4px; background: var(--accent); }

                        .demo-slider-thumb {
                            top: 50%;
                            left: 50%;
                            width: 20px;
                            height: 20px;
                            border: 2px solid var(--surface);
                            border-radius: 50%;
                            background: var(--accent);
                            cursor: grab;
                        }
                        .demo-slider-thumb[data-dragging] { cursor: grabbing; }
                        .demo-slider-thumb[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }

                        .demo-slider[data-orientation="vertical"] { height: 150px; }
                        .demo-slider-track[data-orientation="vertical"] { flex: none; width: 8px; height: 100%; }
                        .demo-slider-fill[data-orientation="vertical"] { inset: auto 0 0 0; }

                        .demo-slider-tooltip { display: none; position: absolute; bottom: 100%; left: 50%; transform: translateX(-50%); }
                        .demo-slider-tooltip[data-visible] { display: block; }

                        .demo-slider-mark { transform: translateX(-50%); color: var(--muted); }
                        .demo-slider-mark[data-in-range] { color: inherit; font-weight: 600; }
                    "#)}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "The slider is a field: put the "<Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" "
                    <Code inline=true>"Label"</Code>", "<Code inline=true>"Description"</Code>" and "<Code inline=true>"FieldError"</Code>
                    " inside it. The label names the slider (a click on it focuses the first thumb), a description describes every "
                    "thumb."
                </p>
                <p>
                    "Give a "<Code inline=true>"SliderThumb"</Code>" a "<Code inline=true>"name"</Code>" to submit its value with a "
                    "form: it is the name of the thumb\u{2019}s range input. A form reset restores the initial values."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Slider.materialize()>"Slider overview"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"Slider Hooks"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
                <li><Link href=routes::doc::NumberField.materialize()>"Number Field"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
