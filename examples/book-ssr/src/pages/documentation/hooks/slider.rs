use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    slider_basic::SliderBasicDemo, slider_callbacks::SliderCallbacksDemo,
    slider_range::SliderRangeDemo, slider_vertical::SliderVerticalDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseSliderHook() -> impl IntoView {
    view! {
        <DocPage title="Slider Hooks">
            <p>
                "The slider hooks give markup you write the state, dragging, keyboard handling and ARIA of a slider with any "
                "number of thumbs. See the "<Link href=routes::doc::Slider.materialize()>"Slider overview"</Link>
                " for when to use a slider."
            </p>

            <ReactAria hook="useSlider"/>

            <Section title="Example">
                <p>"A slider combines up to four hooks:"</p>

                <DocTable headers=&["Hook", "What it does"]>
                    <TableRow>
                        <TableCell><AnchorLink href="#use-slider-state"><Code inline=true>"use_slider_state"</Code></AnchorLink></TableCell>
                        <TableCell>"Owns the values of all thumbs, applies the range and the step, and keeps thumbs from crossing."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#use-slider"><Code inline=true>"use_slider"</Code></AnchorLink></TableCell>
                        <TableCell>"The group and the track (a press moves the closest thumb), the label, description and output."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#use-slider-thumb"><Code inline=true>"use_slider_thumb"</Code></AnchorLink></TableCell>
                        <TableCell>"One thumb: dragging, keyboard handling and the visually hidden range input that carries the ARIA state."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#use-slider-marks"><Code inline=true>"use_slider_marks"</Code></AnchorLink></TableCell>
                        <TableCell>"Optional marks along the track, one per step or the ones you list."</TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "The hooks position the thumbs; you size the track and draw the fill. The "
                    <Link href=routes::doc::slider::Atom.materialize()>"Slider Atoms"</Link>
                    " render all of this for you."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            computed_pct,
                            computed_size,
                            hooks::slider::{UseSliderInput, UseSliderStateInput, UseSliderThumbInput, use_slider, use_slider_state, use_slider_thumb},
                            leptos_styles::{Styles, property::WidthProperty},
                        };
                        use leptos::prelude::*;

                        let state = use_slider_state(UseSliderStateInput {
                            default_values: Some(vec![50_u8]),
                            value: None,
                            min_value: Signal::stored(0),
                            max_value: Signal::stored(100),
                            step: Signal::stored(1),
                            is_disabled: Signal::default(),
                            orientation: Signal::stored(Orientation::Horizontal),
                            format_options: Signal::default(),
                            value_label: None,
                            page_size: None,
                            on_change: None,
                            on_change_end: None,
                        });
                        let slider = use_slider(UseSliderInput {
                            state,
                            id: None,
                            has_label: Signal::stored(false),
                            aria_label: "Volume".into(),
                            aria_labelledby: None,
                            aria_describedby: None,
                            aria_details: None,
                        });
                        let thumb = use_slider_thumb(UseSliderThumbInput {
                            state,
                            slider: slider.data.clone(),
                            track: slider.track_element,
                            index: 0,
                            is_disabled: Signal::default(),
                            is_required: Signal::default(),
                            is_invalid: Signal::default(),
                            name: None,
                            form: None,
                            has_label: Signal::stored(false),
                            aria_label: MaybeProp::default(),
                            aria_labelledby: None,
                            aria_describedby: None,
                            aria_errormessage: None,
                            aria_details: None,
                        });
                        let (track_attrs, track_styles) = slider.track_props.into_parts();
                        let (thumb_attrs, thumb_styles) = thumb.thumb_props.into_parts();
                        let fill_styles = Styles::new().add_reactive(move || {
                            WidthProperty.declare(computed_size(computed_pct(state.thumb_percent(0).as_percent())))
                        });

                        view! {
                            <div {..slider.group_props.into_attrs()}>
                                <div class="track" {..track_attrs} style=track_styles>
                                    <div class="fill" style=fill_styles></div>
                                    // The thumb positions itself on the track.
                                    <div class="thumb" {..thumb_attrs} style=thumb_styles>
                                        <input class="visually-hidden" {..thumb.input_props.into_attrs()}/>
                                    </div>
                                </div>
                                <output {..slider.output_props.into_attrs()}>{move || state.formatted_values()}</output>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "A single-thumb slider built from the hooks. Drag the thumb, press the track or focus the thumb with "
                    <Keys keys="Tab"/>" and use the arrow keys."
                </p>

                <Demo description="Volume slider built from use_slider and use_slider_thumb" source=include_str!("demos/slider_basic.rs")>
                    <SliderBasicDemo/>
                </Demo>
            </Section>

            <Section title="Range Slider">
                <p>
                    "Pass two values to "<Code inline=true>"use_slider_state"</Code>" and call "
                    <Code inline=true>"use_slider_thumb"</Code>" once per thumb with its "<Code inline=true>"index"</Code>
                    ". Thumbs can\u{2019}t cross each other, and a press on the track moves the closest thumb."
                </p>

                <Demo description="Two-thumb price range slider built from the hooks" source=include_str!("demos/slider_range.rs")>
                    <SliderRangeDemo/>
                </Demo>
            </Section>

            <Section title="Steps and Callbacks">
                <p>
                    "With "<Code inline=true>"step: Signal::stored(5.0)"</Code>", values snap to multiples of 5. "
                    <Code inline=true>"on_change"</Code>" runs on every change, also while dragging; "
                    <Code inline=true>"on_change_end"</Code>" runs when you release the thumb and after each keyboard change."
                </p>

                <Demo description="Stepped slider showing the last on_change and on_change_end" source=include_str!("demos/slider_callbacks.rs")>
                    <SliderCallbacksDemo/>
                </Demo>
            </Section>

            <Section title="Vertical Slider">
                <p>
                    "Set "<Code inline=true>"orientation"</Code>" to "<Code inline=true>"Orientation::Vertical"</Code>
                    ". The thumb positions itself from the top; draw the fill from the bottom."
                </p>

                <Demo description="Vertical slider built from the hooks" source=include_str!("demos/slider_vertical.rs")>
                    <SliderVerticalDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard Focus">
                <p>
                    "Keyboard focus is on a thumb\u{2019}s visually hidden input, so the thumb itself has to show it. The slider "
                    "hooks don\u{2019}t report whether focus is visible: compose "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                    " with "<Code inline=true>"within: true"</Code>" on each thumb. It sets "
                    <Code inline=true>"data-focus-visible"</Code>" on the thumb while its input has keyboard focus, as all demos "
                    "on this page do:"
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        let focus_ring = use_focus_ring(UseFocusRingInput {
                            target: FocusRingTarget::Within,
                            ..UseFocusRingInput::default()
                        });

                        view! {
                            <div class="thumb" {..thumb_attrs} {..focus_ring.props.into_attrs()} style=thumb_styles>
                                <input class="visually-hidden" {..thumb.input_props.into_attrs()}/>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="use_slider_state">
                <p>
                    "Manages the values of all thumbs. Generic over the value type: integers ("<Code inline=true>"u8"</Code>", "
                    <Code inline=true>"i32"</Code>", \u{2026}) or floats."
                </p>

                <Section title="Input" id="use-slider-state-input">
                    <p>
                        "Pass a "<Code inline=true>"UseSliderStateInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseSliderStateInput">
                        <ApiRow name="min_value, max_value" ty="Signal<T>">"The range. Required."</ApiRow>
                        <ApiRow name="step" ty="Signal<T>" default="1">"The step values snap to."</ApiRow>
                        <ApiRow name="default_values" ty="Option<Vec<T>>" default="None">
                            "The initial values, one per thumb, in ascending order. "<Code inline=true>"None"</Code>": one thumb at the minimum."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Vec<T>>>" default="None">"The values as app state, replacing "<Code inline=true>"default_values"</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Horizontal">"The axis of the track."</ApiRow>
                        <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="NumberFormatOptions::default()">
                            "How values are formatted for the output and assistive technology (decimal, in the locale of the i18n context)."
                        </ApiRow>
                        <ApiRow name="value_label" ty="Option<Callback<T, String>>" default="None">"Formats a thumb\u{2019}s value for assistive technology instead of "<Code inline=true>"format_options"</Code>"."</ApiRow>
                        <ApiRow name="page_size" ty="Option<Signal<T>>" default="None">
                            "The step of "<Keys keys="PageUp"/>" and "<Keys keys="PageDown"/>". "<Code inline=true>"None"</Code>
                            ": a tenth of the range, a multiple of the step."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Vec<T>>>" default="None">"Called with all values whenever they change, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<Vec<T>>>" default="None">"Called when the user stops dragging, and after a keyboard change."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-slider-state-return">
                    <p>
                        <Code inline=true>"SliderState<T>"</Code>" is "<Code inline=true>"Copy"</Code>
                        ". Its public signals:"
                    </p>
                    <ApiTable kind=ApiKind::Return of="SliderState">
                        <ApiRow name="values" ty="Signal<Vec<T>>">"The thumbs\u{2019} values, in ascending order."</ApiRow>
                        <ApiRow name="min_value, max_value" ty="Signal<T>">"The range."</ApiRow>
                        <ApiRow name="step" ty="Signal<T>">"The step between values."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>">"The axis of the track."</ApiRow>
                        <ApiRow name="focused_thumb" ty="Signal<Option<usize>>">"The thumb with focus."</ApiRow>
                    </ApiTable>
                    <p>"Its methods read and change it; thumbs are addressed by index:"</p>
                    <DocTable headers=&["Method", "Meaning"]>
                        <TableRow><TableCell><Code inline=true>"thumb_count()"</Code></TableCell><TableCell>"The number of thumbs."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"thumb_value(i), set_thumb_value(i, value)"</Code></TableCell><TableCell>"A thumb\u{2019}s value. Setting snaps to the step and stays between the neighbors."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"thumb_percent(i), set_thumb_percent(i, fraction)"</Code></TableCell><TableCell>"A thumb\u{2019}s position as a "<Code inline=true>"Fraction"</Code>" of the range."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"value_percent(value), percent_value(fraction)"</Code></TableCell><TableCell>"Converts between a value and its fraction of the range; "<Code inline=true>"percent_value"</Code>" rounds to the step, clamps to the bounds, and returns an "<Code inline=true>"Option<T>"</Code>"."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"thumb_min_value(i), thumb_max_value(i)"</Code></TableCell><TableCell>"A thumb\u{2019}s bounds: its neighbors\u{2019} values or the range."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"increment_thumb(i, size), decrement_thumb(i, size)"</Code></TableCell><TableCell>"Steps a thumb by "<Code inline=true>"size"</Code>" ("<Code inline=true>"Option<T>"</Code>", at least the step)."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"page_size()"</Code></TableCell><TableCell>"The step of "<Keys keys="PageUp"/>" and "<Keys keys="PageDown"/>"."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"is_thumb_dragging(i), set_thumb_dragging(i, dragging)"</Code></TableCell><TableCell>"Whether a thumb is dragged. When the last drag ends, "<Code inline=true>"on_change_end"</Code>" runs."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"is_thumb_editable(i), set_thumb_editable(i, editable)"</Code></TableCell><TableCell>"Whether a thumb can change (a disabled thumb can\u{2019}t)."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"set_focused_thumb(Some(i))"</Code></TableCell><TableCell>"Sets "<Code inline=true>"focused_thumb"</Code>"; the thumb\u{2019}s input then takes focus. "<Code inline=true>"None"</Code>" clears it."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"thumb_value_label(i), format_value(value)"</Code></TableCell><TableCell>"A thumb\u{2019}s value, or any value, formatted."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"formatted_values()"</Code></TableCell><TableCell>"All values formatted for an output: \u{201c}20\u{201d}, \u{201c}20 \u{2013} 80\u{201d} or a list."</TableCell></TableRow>
                        <TableRow><TableCell><Code inline=true>"default_values()"</Code></TableCell><TableCell>"The values the slider started with, restored by a form reset."</TableCell></TableRow>
                    </DocTable>
                </Section>

                <Section title="Example" id="use-slider-state-example">
                    <p>"Bind the values to app state with a "<Code inline=true>"ValueBinding"</Code>" instead of "<Code inline=true>"default_values"</Code>":"</p>
                    <Code language=Language::Rust>
                        {indoc!(r"
                            let price = RwSignal::new(vec![20.0, 80.0]);
                            let state = use_slider_state(UseSliderStateInput {
                                default_values: None,
                                value: Some(ValueBinding::from(price)),
                                min_value: Signal::stored(0.0),
                                max_value: Signal::stored(100.0),
                                step: Signal::stored(5.0),
                                is_disabled: Signal::default(),
                                orientation: Signal::stored(Orientation::Horizontal),
                                format_options: Signal::default(),
                                value_label: None,
                                page_size: None,
                                on_change: None,
                                on_change_end: None,
                            });
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="use_slider">
                <p>"The group around label, track and output, and the track: a press on it moves the closest thumb there and drags it."</p>

                <Section title="Input" id="use-slider-input">
                    <p>"Pass a "<Code inline=true>"UseSliderInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseSliderInput">
                        <ApiRow name="state" ty="SliderState<T>">"The slider state. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The group\u{2019}s id; the thumbs\u{2019} ids derive from it. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether a visible label is rendered (with "<Code inline=true>"label_props"</Code>")."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the slider when there is no visible label."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of further elements naming the slider."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Ids of further elements describing every thumb."</ApiRow>
                        <ApiRow name="aria_details" ty="Option<String>" default="None">
                            "Ids of elements with details about every thumb ("<Code inline=true>"aria-details"</Code>")."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-slider-return">
                    <ApiTable kind=ApiKind::Return of="UseSliderReturn">
                        <ApiRow name="group_props" ty="UseSliderGroupProps">"For the element around everything: "<Code inline=true>"role=\"group\""</Code>", named by the label."</ApiRow>
                        <ApiRow name="label_props" ty="UseSliderLabelProps">"For the label: a click focuses the first thumb."</ApiRow>
                        <ApiRow name="description_props" ty="SlotProps">"For a description; while rendered, it describes every thumb."</ApiRow>
                        <ApiRow name="error_message_props" ty="SlotProps">"For an error message. Render it only while the slider is invalid."</ApiRow>
                        <ApiRow name="track_props" ty="PropsWithStyles<UseSliderTrackProps>">
                            "For the track: pointer handling and "<Code inline=true>"position: relative"</Code>
                            ". Split it with "<Code inline=true>"into_parts()"</Code>" into attributes and styles."
                        </ApiRow>
                        <ApiRow name="output_props" ty="UseSliderOutputProps">"For an "<Code inline=true>"<output>"</Code>": "<Code inline=true>"for"</Code>" all thumbs, not announced while dragging."</ApiRow>
                        <ApiRow name="data" ty="SliderData">"What the thumbs need: the group\u{2019}s id, the label and the description."</ApiRow>
                        <ApiRow name="track_element" ty="CapturedElement">"The track, for the thumbs."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-slider-example">
                    <p>"With a visible label, set "<Code inline=true>"has_label"</Code>" and spread "<Code inline=true>"label_props"</Code>" on it:"</p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let slider = use_slider(UseSliderInput {
                                state,
                                id: None,
                                has_label: Signal::stored(true),
                                aria_label: MaybeProp::default(),
                                aria_labelledby: None,
                                aria_describedby: None,
                                aria_details: None,
                            });

                            view! {
                                <div {..slider.group_props.into_attrs()}>
                                    <span {..slider.label_props.into_attrs()}>"Volume"</span>
                                    // The track and its thumbs.
                                    <output {..slider.output_props.into_attrs()}>{move || state.formatted_values()}</output>
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_slider_thumb">
                <p>
                    "One thumb: dragged with a pointer, moved with the keyboard through its visually hidden "
                    <Code inline=true>"<input type=\"range\">"</Code>". Call it once per value."
                </p>

                <Section title="Input" id="use-slider-thumb-input">
                    <p>
                        "Pass a "<Code inline=true>"UseSliderThumbInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need. "<Code inline=true>"slider"</Code>" and "
                        <Code inline=true>"track"</Code>" come from the return of "<Code inline=true>"use_slider"</Code>"."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseSliderThumbInput">
                        <ApiRow name="state" ty="SliderState<T>">"The slider state. Required."</ApiRow>
                        <ApiRow name="slider" ty="SliderData">"What the slider passes its thumbs ("<Code inline=true>"UseSliderReturn::data"</Code>"). Required."</ApiRow>
                        <ApiRow name="track" ty="CapturedElement">"The track ("<Code inline=true>"UseSliderReturn::track_element"</Code>"). Required."</ApiRow>
                        <ApiRow name="index" ty="usize" default="0">"The thumb\u{2019}s value in the slider\u{2019}s values."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables this thumb only; the state\u{2019}s "<Code inline=true>"is_disabled"</Code>" disables all."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Whether a value is required (for form validation)."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Whether the value is invalid ("<Code inline=true>"aria-invalid"</Code>")."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The input\u{2019}s name, to submit its value with a form."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the input belongs to, if not its ancestor."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether the thumb has a visible label of its own (with "<Code inline=true>"label_props"</Code>")."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the thumb next to the slider\u{2019}s name, e.g. \u{201c}Minimum\u{201d}."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of further elements naming the thumb."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Ids of further elements describing the thumb."</ApiRow>
                        <ApiRow name="aria_errormessage" ty="Option<String>" default="None">"The id of the element with the thumb\u{2019}s error message."</ApiRow>
                        <ApiRow name="aria_details" ty="Option<String>" default="None">
                            "Ids of further elements with details about the thumb (next to the slider\u{2019}s)."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-slider-thumb-return">
                    <ApiTable kind=ApiKind::Return of="UseSliderThumbReturn">
                        <ApiRow name="thumb_props" ty="PropsWithStyles<UseSliderThumbProps>">
                            "For the thumb element: dragging, and its position ("<Code inline=true>"position: absolute"</Code>
                            ", centered on its value along the track). Split it with "<Code inline=true>"into_parts()"</Code>"."
                        </ApiRow>
                        <ApiRow name="input_props" ty="UseSliderThumbInputProps">
                            "For the visually hidden "<Code inline=true>"<input type=\"range\">"</Code>" inside the thumb: focus, keys "
                            "and the ARIA state ("<Code inline=true>"aria-valuetext"</Code>", min, max, step)."
                        </ApiRow>
                        <ApiRow name="label_props" ty="UseLabelProps">"For the thumb\u{2019}s own label, if it has one."</ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the thumb is dragged."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the thumb or the slider is disabled."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether the thumb\u{2019}s input has focus (by any means)."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-slider-thumb-example">
                    <p>"The second thumb of a range slider, submitted with a form:"</p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let maximum = use_slider_thumb(UseSliderThumbInput {
                                state,
                                slider: slider.data.clone(),
                                track: slider.track_element,
                                index: 1,
                                is_disabled: Signal::default(),
                                is_required: Signal::default(),
                                is_invalid: Signal::default(),
                                name: Some("max-price".to_owned()),
                                form: None,
                                has_label: Signal::stored(false),
                                aria_label: "Maximum".into(),
                                aria_labelledby: None,
                                aria_describedby: None,
                                aria_errormessage: None,
                                aria_details: None,
                            });

                            let (thumb_attrs, thumb_styles) = maximum.thumb_props.into_parts();

                            view! {
                                <div class="thumb" {..thumb_attrs} style=thumb_styles>
                                    <input class="visually-hidden" {..maximum.input_props.into_attrs()}/>
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_slider_marks">
                <p>
                    "Places marks along the track. It works with "<Code inline=true>"f64"</Code>" values, so convert the "
                    "slider\u{2019}s values. The "<Code inline=true>"SliderMarks"</Code>" atom of the "
                    <Link href=routes::doc::slider::Atom.materialize()>"Slider Atoms"</Link>" uses it."
                </p>

                <Section title="Input" id="use-slider-marks-input">
                    <ApiTable kind=ApiKind::Input of="UseSliderMarksInput">
                        <ApiRow name="min_value, max_value" ty="Signal<f64>">"The range. Required."</ApiRow>
                        <ApiRow name="step" ty="Signal<f64>">"The step; automatic marks sit on it. Required."</ApiRow>
                        <ApiRow name="values" ty="Signal<Vec<f64>>">"The thumbs\u{2019} values: marks up to the thumb (or between the first and the last thumb) are in range. Required."</ApiRow>
                        <ApiRow name="marks" ty="SliderMarkPlacement">
                            "Which marks to place. Required: "<Code inline=true>"SliderMarkPlacement::None"</Code>", "
                            <Code inline=true>"Automatic { create_names }"</Code>" (one per step, at most about 20) or "
                            <Code inline=true>"Custom { marks }"</Code>"."
                        </ApiRow>
                        <ApiRow name="format" ty="Callback<f64, String>">"Formats the names of automatic marks. Required."</ApiRow>
                    </ApiTable>
                    <p>
                        "A "<Code inline=true>"CustomSliderMark"</Code>" has a "<Code inline=true>"value"</Code>" ("
                        <Code inline=true>"SliderMarkValue::Value(v)"</Code>", a value of the range, or "
                        <Code inline=true>"SliderMarkValue::Fraction(f)"</Code>", a fraction of the track) and an optional "
                        <Code inline=true>"name"</Code>" (a "<Code inline=true>"MaybeProp<String>"</Code>": a text or a signal "
                        "of one). Marks outside the range are left out."
                    </p>
                </Section>

                <Section title="Return" id="use-slider-marks-return">
                    <p>"It returns a "<Code inline=true>"Signal<Vec<ComputedSliderMark>>"</Code>":"</p>
                    <ApiTable kind=ApiKind::Fields of="ComputedSliderMark">
                        <ApiRow name="percentage" ty="Fraction">"The position along the track."</ApiRow>
                        <ApiRow name="value" ty="f64">"The value of the range at the mark."</ApiRow>
                        <ApiRow name="name" ty="Option<String>">"The mark\u{2019}s name, to show next to it."</ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"is_in_range()"</Code>" tells whether the mark lies within the selected range: up to "
                        "the thumb (one thumb), between the first and the last thumb (several). It tracks the values."
                    </p>
                </Section>

                <Section title="Example" id="use-slider-marks-example">
                    <p>"A named mark every 10 along a slider ("<Code inline=true>"state"</Code>" is its "<Code inline=true>"SliderState<f64>"</Code>"), each placed at its "<Code inline=true>"percentage"</Code>" of the track:"</p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                computed_pct,
                                leptos_styles::{
                                    Styles,
                                    css::LengthPercentageAuto,
                                    property::LeftProperty,
                                },
                            };

                            let marks = use_slider_marks(UseSliderMarksInput {
                                min_value: state.min_value,
                                max_value: state.max_value,
                                step: Signal::stored(10.0),
                                values: state.values,
                                marks: SliderMarkPlacement::Automatic { create_names: true },
                                format: Callback::new(move |value| state.format_value(value)),
                            });

                            view! {
                                // Marks only show what the thumbs announce: hide them from assistive technology.
                                <div class="marks" aria-hidden="true">
                                    <For each=move || marks.get() key=|mark| mark.percentage.get().to_bits() let:mark>
                                        <span
                                            class="mark"
                                            style=Styles::new().add(LeftProperty.declare(LengthPercentageAuto::from(
                                                computed_pct(mark.percentage.as_percent()),
                                            )))
                                            data-in-range={
                                                let mark = mark.clone();
                                                move || mark.is_in_range().then_some("")
                                            }
                                        >
                                            {mark.name.clone().unwrap_or_default()}
                                        </span>
                                    </For>
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Slider.materialize()>"Slider overview"</Link></li>
                <li><Link href=routes::doc::slider::Atom.materialize()>"Slider Atoms"</Link></li>
                <li><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
                <li><Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
