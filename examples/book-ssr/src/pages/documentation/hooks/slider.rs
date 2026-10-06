use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    slider_basic::SliderBasicDemo, slider_callbacks::SliderCallbacksDemo,
    slider_disabled::SliderDisabledDemo, slider_range::SliderRangeDemo,
    slider_vertical::SliderVerticalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseSliderHook() -> impl IntoView {
    view! {
        <DocPage title="Slider Hooks">
            <p>
                "The slider hooks build accessible sliders with keyboard and pointer support and any number of thumbs. "
                "See the "<Link href=routes::doc::Slider.materialize()>"Slider overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useSlider"/>

            <Section title="Architecture">
                <p>"A slider is built from up to four hooks, following react-aria\u{2019}s split:"</p>

                <DocTable headers=&["Hook", "Responsibility"]>
                    <TableRow>
                        <TableCell><Code inline=true>"use_slider_state"</Code></TableCell>
                        <TableCell>"Owns the values of all thumbs, applies min, max and step, and keeps thumbs from crossing."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_slider"</Code></TableCell>
                        <TableCell>"The group and the track: moves the closest thumb on track clicks, provides label and output ids."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_slider_thumb"</Code></TableCell>
                        <TableCell>"One thumb: dragging, keyboard handling and the visually hidden range input that carries the ARIA state."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_slider_marks"</Code></TableCell>
                        <TableCell>"Optional marks along the track, automatic from the step or custom."</TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "The "<Link href=routes::doc::slider::Atom.materialize()>"Slider atom"</Link>
                    " wraps these hooks in components. Use the hooks when you need full control over the markup."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let state = use_slider_state(UseSliderStateInput {
                            values: SliderValues::Uncontrolled(vec![50.0]),
                            min_value: 0.0,
                            max_value: 100.0,
                            step: Some(1.0),
                            is_disabled: false.into(),
                            orientation: Orientation::Horizontal.into(),
                            on_change: None,
                            on_change_end: None,
                        });

                        let slider = use_slider(UseSliderInput {
                            state,
                            aria_label: Some("Volume"),
                            aria_labelledby: None,
                        });
                        let (track_attrs, track_styles) = slider.track_props.into_parts();

                        let thumb = use_slider_thumb(UseSliderThumbInput {
                            state,
                            track: slider.track_ref,
                            index: 0,
                            aria_label: Some("Volume".into()),
                            is_disabled: state.is_disabled,
                            // ... remaining fields `None` / `false`
                        });

                        view! {
                            <div {..slider.group_props.into_attrs()}>
                                <div class="track" {..track_attrs} style=track_styles>
                                    // Position the thumb from `thumb.percentage` (0-100).
                                    <div class="thumb" {..thumb.thumb_props.into_attrs()}>
                                        <input class="visually-hidden" {..thumb.input_props.into_attrs()}/>
                                    </div>
                                </div>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"A single-thumb slider built from the hooks. Drag the thumb, click the track or use the arrow keys."</p>

                <Demo description="Volume slider built from use_slider and use_slider_thumb" source=include_str!("demos/slider_basic.rs")>
                    <SliderBasicDemo/>
                </Demo>
            </Section>

            <Section title="use_slider_state">
                <Section title="Input" id="use-slider-state-input">
                    <p>
                        <Code inline=true>"UseSliderStateInput"</Code>" has no "<Code inline=true>"Default"</Code>
                        " implementation; set every field."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseSliderStateInput">
                        <ApiRow name="values" ty="SliderValues">
                            <Code inline=true>"Uncontrolled(initial)"</Code>" lets the hook own the values, "
                            <Code inline=true>"Controlled(signal)"</Code>" reads them from your signal (update it from "
                            <Code inline=true>"on_change"</Code>"). The number of values is the number of thumbs."
                        </ApiRow>
                        <ApiRow name="min_value, max_value" ty="f64">"The range of the slider."</ApiRow>
                        <ApiRow name="step" ty="Option<f64>">
                            "The step values snap to. "<Code inline=true>"None"</Code>" makes the slider continuous."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>">
                            <Code inline=true>"Horizontal"</Code>" or "<Code inline=true>"Vertical"</Code>"."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<f64>>>">"Called with all values whenever a value changes."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<Vec<f64>>>">
                            "Called with all values when the user releases the last dragged thumb."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-slider-state-return">
                    <p>
                        <Code inline=true>"UseSliderStateReturn"</Code>" is "<Code inline=true>"Copy"</Code>
                        "; pass it to the other slider hooks. "<Code inline=true>"ThumbIdx"</Code>" is the index of a thumb ("
                        <Code inline=true>"usize"</Code>")."
                    </p>

                    <ApiTable kind=ApiKind::Return of="UseSliderStateReturn">
                        <ApiRow name="values" ty="Signal<Vec<f64>>">"The values of all thumbs."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>">"The orientation."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="f64">"The range."</ApiRow>
                        <ApiRow name="step" ty="Option<f64>">"The step, "<Code inline=true>"None"</Code>" for a continuous slider."</ApiRow>
                        <ApiRow name="page_size" ty="f64">
                            "The step of "<Keys keys="PageUp"/>" / "<Keys keys="PageDown"/>" and "<Keys keys="Shift"/>
                            " + arrow keys: a tenth of the range, snapped to the step."
                        </ApiRow>
                        <ApiRow name="num_thumbs" ty="usize">"The number of thumbs."</ApiRow>
                        <ApiRow name="get_value_percent, get_percent_value" ty="Callback<f64, f64>">
                            "Convert between values and fractions of the range (0.0\u{2013}1.0)."
                        </ApiRow>
                        <ApiRow name="get_thumb_value" ty="Callback<ThumbIdx, f64>">"The value of a thumb."</ApiRow>
                        <ApiRow name="get_thumb_percent" ty="Callback<ThumbIdx, f64>">"The value of a thumb as a fraction of the range."</ApiRow>
                        <ApiRow name="get_thumb_min_value, get_thumb_max_value" ty="Callback<ThumbIdx, f64>">
                            "The range a thumb can move in: bounded by the neighboring thumbs."
                        </ApiRow>
                        <ApiRow name="set_thumb_value" ty="Callback<(ThumbIdx, f64)>">
                            "Change a thumb\u{2019}s value programmatically. Values are clamped between the neighboring thumbs."
                        </ApiRow>
                        <ApiRow name="set_thumb_percent" ty="Callback<(ThumbIdx, f64)>">"Set a thumb\u{2019}s value as a fraction of the range."</ApiRow>
                        <ApiRow name="increment_thumb, decrement_thumb" ty="Callback<(ThumbIdx, Option<f64>)>">
                            "Step a thumb up or down, by the given step size or the step."
                        </ApiRow>
                        <ApiRow name="is_thumb_dragging" ty="Callback<ThumbIdx, bool>">"Whether a thumb is being dragged."</ApiRow>
                        <ApiRow name="set_thumb_dragging" ty="Callback<(ThumbIdx, bool)>">
                            "Mark a thumb as dragged. "<Code inline=true>"on_change_end"</Code>" fires when the last dragged thumb is released."
                        </ApiRow>
                        <ApiRow name="focused_thumb" ty="Signal<Option<ThumbIdx>>">"The index of the focused thumb."</ApiRow>
                        <ApiRow name="set_focused_thumb" ty="Callback<Option<ThumbIdx>>">"Set the focused thumb."</ApiRow>
                        <ApiRow name="is_thumb_editable" ty="Callback<ThumbIdx, bool>">"Whether a thumb can be changed by the user."</ApiRow>
                        <ApiRow name="set_thumb_editable" ty="Callback<(ThumbIdx, bool)>">"Allow or forbid changing a thumb."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_slider">
                <Section title="Input" id="use-slider-input">
                    <ApiTable kind=ApiKind::Input of="UseSliderInput">
                        <ApiRow name="state" ty="UseSliderStateReturn">"The slider state."</ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>">"Accessible name of the slider group."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>">"Id of the element that labels the group."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-slider-return">
                    <ApiTable kind=ApiKind::Return of="UseSliderReturn">
                        <ApiRow name="group_props" ty="UseSliderGroupProps">
                            <Code inline=true>"role=\"group\""</Code>", id, label and "<Code inline=true>"aria-disabled"</Code>
                            " for the container. Spread with "<Code inline=true>"{..group_props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="track_props" ty="PropsWithStyles<UseSliderTrackProps>">
                            "Pointer handling for the track and "<Code inline=true>"touch-action: none"</Code>". Call "
                            <Code inline=true>"into_parts()"</Code>", spread the attributes and set "
                            <Code inline=true>"style"</Code>" to the styles."
                        </ApiRow>
                        <ApiRow name="track_ref" ty="CapturedElement">
                            "The captured track element. Pass it to "<Code inline=true>"use_slider_thumb"</Code>"."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseSliderLabelProps">
                            "A generated "<Code inline=true>"id"</Code>" for a label element."
                        </ApiRow>
                        <ApiRow name="output_props" ty="UseSliderOutputProps">
                            "Id, "<Code inline=true>"for"</Code>" and "<Code inline=true>"aria-live=\"off\""</Code>
                            " for an "<Code inline=true>"<output>"</Code>" showing the value."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_slider_thumb">
                <Section title="Input" id="use-slider-thumb-input">
                    <ApiTable kind=ApiKind::Input of="UseSliderThumbInput">
                        <ApiRow name="state" ty="UseSliderStateReturn">"The slider state."</ApiRow>
                        <ApiRow name="track" ty="CapturedElement">"The "<Code inline=true>"track_ref"</Code>" of "<Code inline=true>"use_slider"</Code>"."</ApiRow>
                        <ApiRow name="index" ty="usize">"The index of this thumb."</ApiRow>
                        <ApiRow name="name" ty="Option<&'static str>">"Name of the range input for form submission."</ApiRow>
                        <ApiRow name="aria_label" ty="Option<Cow<'static, str>>">"Accessible name of the thumb."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby, aria_details, aria_errormessage" ty="Option<&'static str>">
                            "Ids of elements that label, describe or add details or an error message to the thumb."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">
                            "Whether this thumb is disabled. Pass "<Code inline=true>"state.is_disabled"</Code>" to follow the slider."
                        </ApiRow>
                        <ApiRow name="decimal_places" ty="Option<usize>">
                            "Decimal places of "<Code inline=true>"display_value"</Code>" and "<Code inline=true>"aria-valuetext"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_valuetext" ty="Option<Signal<String>>">
                            "Replaces the formatted value as "<Code inline=true>"aria-valuetext"</Code>", e.g. \u{201c}20 \u{b0}C\u{201d}."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-slider-thumb-return">
                    <ApiTable kind=ApiKind::Return of="UseSliderThumbReturn">
                        <ApiRow name="thumb_props" ty="UseSliderThumbProps">
                            "Pointer, key, focus and hover handlers and "<Code inline=true>"data-focus-visible"</Code>
                            " for the visible thumb element. You position it yourself."
                        </ApiRow>
                        <ApiRow name="input_props" ty="UseSliderThumbInputProps">
                            "Attributes of the "<Code inline=true>"<input type=\"range\">"</Code>
                            " inside the thumb. It is the focus target and carries the slider role and "
                            <Code inline=true>"aria-value*"</Code>" attributes. Hide it visually, not with "
                            <Code inline=true>"display: none"</Code>"."
                        </ApiRow>
                        <ApiRow name="value" ty="Signal<f64>">"The value of this thumb."</ApiRow>
                        <ApiRow name="percentage" ty="Signal<f64>">"The position of this thumb in percent (0\u{2013}100)."</ApiRow>
                        <ApiRow name="display_value" ty="Signal<String>">"The formatted value."</ApiRow>
                        <ApiRow name="is_dragging, is_hovered, is_focused, is_focus_visible" ty="Signal<bool>">
                            "Interaction state of the thumb."
                        </ApiRow>
                        <ApiRow name="thumb_id" ty="String">"The id of the range input."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_slider_marks">
                <p>
                    "Computes marks along the track, each with a reactive "<Code inline=true>"in_range"</Code>
                    " flag. The "<Link href=routes::doc::slider::Atom.materialize()>"Slider atom"</Link>" page shows them in action."
                </p>

                <Section title="Input" id="use-slider-marks-input">
                    <ApiTable kind=ApiKind::Input of="UseSliderMarksInput">
                        <ApiRow name="state" ty="UseSliderStateReturn">"The slider state."</ApiRow>
                        <ApiRow name="marks" ty="SliderMarks">
                            <Code inline=true>"None"</Code>", "<Code inline=true>"Automatic { create_names }"</Code>
                            " (one mark per step, at most about 20; requires a step) or "
                            <Code inline=true>"Custom { marks }"</Code>"."
                        </ApiRow>
                        <ApiRow name="value_display" ty="Option<Callback<f64, String>>">
                            "Formats the names of automatic marks."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-slider-marks-return">
                    <ApiTable kind=ApiKind::Return of="UseSliderMarksReturn">
                        <ApiRow name="marks" ty="Signal<Vec<ComputedSliderMark>>">
                            "The marks, each with "<Code inline=true>"percentage"</Code>" (0.0\u{2013}1.0), "
                            <Code inline=true>"in_range: Signal<bool>"</Code>" and an optional "<Code inline=true>"name"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Custom Marks">
                    <p>
                        "Custom marks sit at a value or at a fraction of the track. Marks outside the slider range are "
                        "dropped with a warning. With two thumbs, "<Code inline=true>"in_range"</Code>
                        " is true for marks between them."
                    </p>

                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let marks = use_slider_marks(UseSliderMarksInput {
                                state,
                                marks: SliderMarks::Custom {
                                    marks: vec![
                                        SliderMark { value: SliderMarkValue::Value(0.0), name: Some("Min".into()) },
                                        SliderMark { value: SliderMarkValue::Percentage(0.5), name: Some("Mid".into()) },
                                        SliderMark { value: SliderMarkValue::Value(100.0), name: Some("Max".into()) },
                                    ],
                                },
                                value_display: None,
                            });
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Range Slider">
                <p>
                    "Pass two values to "<Code inline=true>"use_slider_state"</Code>" and call "
                    <Code inline=true>"use_slider_thumb"</Code>" once per thumb. Thumbs cannot cross each other, and a click "
                    "on the track moves the closest thumb."
                </p>

                <Demo description="Two-thumb price range slider built from the hooks" source=include_str!("demos/slider_range.rs")>
                    <SliderRangeDemo/>
                </Demo>
            </Section>

            <Section title="Steps and Callbacks">
                <p>
                    "With "<Code inline=true>"step: Some(5.0)"</Code>", values snap to multiples of 5. "
                    <Keys keys="Shift"/>" + arrow keys and "<Keys keys="PageUp"/>" / "<Keys keys="PageDown"/>" move by the page size. "
                    <Code inline=true>"on_change"</Code>" fires on every change, "<Code inline=true>"on_change_end"</Code>
                    " when you release the thumb."
                </p>

                <Demo description="Stepped slider logging on_change and on_change_end" source=include_str!("demos/slider_callbacks.rs")>
                    <SliderCallbacksDemo/>
                </Demo>
            </Section>

            <Section title="Vertical Slider">
                <p>
                    "Set "<Code inline=true>"orientation"</Code>" to "<Code inline=true>"Orientation::Vertical"</Code>
                    " and position the fill and thumb from the bottom. The up and down arrow keys work in both orientations."
                </p>

                <Demo description="Vertical slider built from the hooks" source=include_str!("demos/slider_vertical.rs")>
                    <SliderVerticalDemo/>
                </Demo>
            </Section>

            <Section title="Disabled Slider">
                <p>
                    "Disable the state with "<Code inline=true>"is_disabled"</Code>" and pass "<Code inline=true>"state.is_disabled"</Code>
                    " to each thumb. The group gets "<Code inline=true>"aria-disabled"</Code>", the range inputs are disabled."
                </p>

                <Demo description="Slider with a disabled toggle" source=include_str!("demos/slider_disabled.rs")>
                    <SliderDisabledDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowRight / ArrowUp">"Increase by one step (1% of the range for continuous sliders)."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">"Decrease by one step."</KeyRow>
                    <KeyRow keys="Shift + Arrow keys / PageUp / PageDown">"Increase or decrease by the page size."</KeyRow>
                    <KeyRow keys="Home">"Set to the minimum."</KeyRow>
                    <KeyRow keys="End">"Set to the maximum."</KeyRow>
                </KeyboardTable>

                <p>
                    "In a right-to-left layout (the locale of an enclosing "<Code inline=true>"I18nProvider"</Code>
                    "), the left and right arrow keys are swapped and track clicks are mirrored horizontally."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Slider.materialize()>"Slider overview"</Link></li>
                <li><Link href=routes::doc::slider::Atom.materialize()>"Slider atom"</Link></li>
                <li><Link href=routes::doc::slider::Component.materialize()>"Slider component"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
