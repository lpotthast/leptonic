use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_wheel::ColorWheelDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseColorWheel() -> impl IntoView {
    view! {
        <DocPage title="Color Wheel Hooks">
            <p>
                "The "<Code inline=true>"use_color_wheel_state"</Code>" and "<Code inline=true>"use_color_wheel"</Code>
                " hooks build a ring of hues with a thumb on it. See the "
                <Link href=routes::doc::ColorWheel.materialize()>"Color Wheel overview"</Link>" for the concept and its "
                "keyboard interaction."
            </p>

            <ReactAria hook="useColorWheel"/>

            <Section title="Example">
                <p>
                    "Create the state, pass it to "<Code inline=true>"use_color_wheel"</Code>" with the ring\u{2019}s radii, "
                    "and spread the returned props onto the track, the thumb and an input inside the thumb. Each props value "
                    "carries the styles its element needs (the ring\u{2019}s size, clip path and gradient, the thumb\u{2019}s "
                    "position, the visually hidden input); split them with "<Code inline=true>"into_parts"</Code>". Put the "
                    "thumb next to the track, not inside it: the track\u{2019}s clip path would cut it off."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            hooks::*,
                            utils::color::HSV,
                        };
                        use leptos::prelude::*;

                        let state = use_color_wheel_state(UseColorWheelStateInput {
                            default_value: HSV::new(),
                            value: None,
                            is_disabled: false.into(),
                            on_change: None,
                            on_change_end: None,
                        });
                        let wheel = use_color_wheel(UseColorWheelInput {
                            state,
                            outer_radius: 100.0,
                            inner_radius: 74.0,
                            aria_label: MaybeProp::default(),
                            aria_labelledby: None,
                            aria_describedby: None,
                            aria_details: None,
                            name: None,
                            form: None,
                        });
                        let (track_attrs, track_styles) = wheel.track_props.into_parts();
                        let (thumb_attrs, thumb_styles) = wheel.thumb_props.into_parts();
                        let (input_attrs, input_styles) = wheel.input_props.into_parts();

                        view! {
                            // Give it `position: relative`: the thumb is positioned in it.
                            <div class="my-color-wheel">
                                <div {..track_attrs} style=track_styles></div>
                                <div {..thumb_attrs} style=thumb_styles>
                                    <input {..input_attrs} style=input_styles/>
                                </div>
                            </div>
                        }
                    "#)}
                </Code>
                <p>
                    "Give the thumb a size, a border and a fill, e.g. "<Code inline=true>"state.display_color()"</Code>
                    "; the demo source shows one way. Without a label, the wheel is named after the hue channel (\u{201c}Hue\u{201d})."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Click or drag anywhere on the ring, drag the thumb, or tab to it and use the arrow keys. The hooks "
                    "report no focus state; the demo shows the focus ring with "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                    " on the thumb, which contains the focused input."
                </p>

                <Demo description="Hue wheel for an HSV color, with a disabled toggle" source=include_str!("demos/color_wheel.rs")>
                    <ColorWheelDemo/>
                </Demo>
            </Section>

            <Section title="use_color_wheel_state">
                <p>
                    "Holds the color and its hue, which the wheel maps to an angle: 0\u{00b0} at three o\u{2019}clock, "
                    "increasing clockwise. The other channels keep their values. "<Code inline=true>"C"</Code>" is any "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())>
                        <Code inline=true>"ColorValue"</Code>
                    </Link>": the wheel changes the hue channel of "<Code inline=true>"HSV"</Code>" and "<Code inline=true>"HSL"</Code>
                    " colors, and the hue of the HSL form of others (RGB), keeping the color type. The "
                    "state owns the color, starting at "<Code inline=true>"default_value"</Code>", unless you bind it to app "
                    "state with "<Code inline=true>"value"</Code>"."
                </p>

                <Section title="Input" id="use-color-wheel-state-input">
                    <p>
                        "Pass a "<Code inline=true>"UseColorWheelStateInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseColorWheelStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color. Required."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<C>>" default="None">
                            "The color as app state, replacing "<Code inline=true>"default_value"</Code>": "
                            <Code inline=true>"ValueBinding::from(rw_signal)"</Code>" or "
                            <Code inline=true>"ValueBinding::new(signal, callback)"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the wheel is disabled."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>" default="None">"Called with the color whenever it changes, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>" default="None">"Called with the color when a drag or a key press ends."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-wheel-state-return">
                    <p>"A "<Code inline=true>"Copy"</Code>" "<Code inline=true>"ColorWheelState<C>"</Code>":"</p>
                    <ApiTable kind=ApiKind::Return of="ColorWheelState">
                        <ApiRow name="value" ty="Signal<C>">"The color."</ApiRow>
                        <ApiRow name="hue" ty="Signal<f64>">"Its hue, in degrees (0 to 360, exclusive)."</ApiRow>
                        <ApiRow name="step, page_step" ty="f64">"The hue\u{2019}s step and page step: 1\u{00b0} and 15\u{00b0}."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the wheel is disabled."</ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the thumb is being dragged."</ApiRow>
                    </ApiTable>
                    <p>"Its methods change the color; each calls "<Code inline=true>"on_change"</Code>" when the color changes:"</p>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(color)"</Code></TableCell>
                            <TableCell>"Sets the color."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_hue(hue)"</Code></TableCell>
                            <TableCell>"Sets the hue, snapped to the step; 360\u{00b0} wraps around to 0\u{00b0}."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment(step)"</Code>", "<Code inline=true>"decrement(step)"</Code></TableCell>
                            <TableCell>"Change the hue by "<Code inline=true>"step"</Code>" (at least the hue\u{2019}s step), wrapping around."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_hue_from_point(x, y, radius)"</Code></TableCell>
                            <TableCell>"Sets the hue of a point relative to the wheel\u{2019}s center ("<Code inline=true>"y"</Code>" pointing down)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"thumb_position(radius)"</Code></TableCell>
                            <TableCell>"The thumb\u{2019}s position relative to the center, on a circle of "<Code inline=true>"radius"</Code>" (tracked)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"display_color()"</Code></TableCell>
                            <TableCell>"The hue at full saturation and brightness, to fill the thumb with."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_dragging(dragging)"</Code></TableCell>
                            <TableCell>"Starts or ends dragging; ending it calls "<Code inline=true>"on_change_end"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"default_value()"</Code></TableCell>
                            <TableCell>"The color the wheel started with; a form reset restores it."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_color_wheel">
                <p>
                    "Adds pointer, touch and keyboard interaction and the ARIA semantics. A press on the ring (between "
                    "the radii) jumps to that hue and starts a drag; presses in the hole are ignored. A visually hidden range "
                    "input inside the thumb takes the focus, describes the wheel to screen readers with the hue and its "
                    "name (e.g. \u{201c}120\u{00b0}, green\u{201d}) and takes part in forms."
                </p>

                <Section title="Input" id="use-color-wheel-input">
                    <p>
                        "Pass a "<Code inline=true>"UseColorWheelInput"</Code>" with every field named; the Default column "
                        "gives the value for fields you don\u{2019}t need."

                    </p>
                    <ApiTable kind=ApiKind::Input of="UseColorWheelInput">
                        <ApiRow name="state" ty="ColorWheelState<C>">
                            "The state from "<Code inline=true>"use_color_wheel_state"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="outer_radius" ty="f64">"The ring\u{2019}s outer radius, in pixels. Required."</ApiRow>
                        <ApiRow name="inner_radius" ty="f64">"The ring\u{2019}s inner radius, in pixels. Required."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the wheel. Without it or "<Code inline=true>"aria_labelledby"</Code>", the channel\u{2019}s name does."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the wheel."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"The ids of the elements describing the wheel."</ApiRow>
                        <ApiRow name="aria_details" ty="Option<String>" default="None">"The ids of the elements with details about the wheel."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The form field name of the input."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the input belongs to."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-wheel-return">
                    <ApiTable kind=ApiKind::Return of="UseColorWheelReturn">
                        <ApiRow name="track_props" ty="PropsWithStyles<UseColorWheelTrackProps>">
                            "The pointer handling of the ring, and as styles its size ("<Code inline=true>"outer_radius * 2"</Code>
                            "), the conic gradient of hues and the clip path that cuts out the hole."
                        </ApiRow>
                        <ApiRow name="thumb_props" ty="PropsWithStyles<UseColorWheelThumbProps>">
                            "Dragging and keyboard handling, and the thumb\u{2019}s position as styles (centered on the "
                            "middle of the ring)."
                        </ApiRow>
                        <ApiRow name="input_props" ty="PropsWithStyles<UseColorWheelInputProps>">
                            "For the "<Code inline=true>"<input type=\"range\">"</Code>" inside the thumb: range, step and value of "
                            "the hue, its name and value text, and visually hiding styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></li>
                <li><Link href=routes::doc::color_wheel::Atom.materialize()>"Color Wheel Atoms"</Link></li>
                <li><Link href=routes::doc::color_slider::Hook.materialize()>"Color Slider Hooks"</Link></li>
                <li><Link href=routes::doc::color_area::Hook.materialize()>"Color Area Hooks"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
