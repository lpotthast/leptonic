use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_area::ColorAreaDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseColorArea() -> impl IntoView {
    view! {
        <DocPage title="Color Area Hooks">
            <p>
                "The "<Code inline=true>"use_color_area_state"</Code>" and "<Code inline=true>"use_color_area"</Code>
                " hooks build a 2D gradient area in which users adjust two channels of a color at once. See the "
                <Link href=routes::doc::ColorArea.materialize()>"Color Area overview"</Link>" for the concept and its keyboard "
                "interaction."
            </p>

            <ReactAria hook="useColorArea"/>

            <Section title="Example">
                <p>
                    "Create the state, pass it to "<Code inline=true>"use_color_area"</Code>", and spread the returned props "
                    "onto the area, its thumb and two inputs inside the thumb. Each props value carries the styles its "
                    "element needs (the gradient, the thumb\u{2019}s position, the visually hidden inputs); split them with "
                    <Code inline=true>"into_parts"</Code>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            HSV,
                            HsvChannel,
                            hooks::color::{UseColorAreaInput, UseColorAreaStateInput, use_color_area, use_color_area_state},
                        };
                        use leptos::prelude::*;

                        let state = use_color_area_state(UseColorAreaStateInput {
                            default_value: HSV::new(),
                            value: None,
                            x_channel: Some(HsvChannel::Saturation),
                            y_channel: Some(HsvChannel::Brightness),
                            x_channel_step: None,
                            y_channel_step: None,
                            on_change: None,
                            on_change_end: None,
                        });
                        let area = use_color_area(UseColorAreaInput {
                            state,
                            is_disabled: false.into(),
                            aria_label: "Saturation and brightness".into(),
                            aria_labelledby: None,
                            aria_describedby: None,
                            aria_details: None,
                            x_name: None,
                            y_name: None,
                            form: None,
                        });
                        let (area_attrs, area_styles) = area.color_area_props.into_parts();
                        let (thumb_attrs, thumb_styles) = area.thumb_props.into_parts();
                        let (x_attrs, x_styles) = area.x_input_props.into_parts();
                        let (y_attrs, y_styles) = area.y_input_props.into_parts();

                        view! {
                            <div {..area_attrs} style=area_styles>
                                <div {..thumb_attrs} style=thumb_styles>
                                    <input {..x_attrs} style=x_styles/>
                                    <input {..y_attrs} style=y_styles/>
                                </div>
                            </div>
                        }
                    "#)}
                </Code>
                <p>
                    "Give the area a size and the thumb a size, a border and the current color as background ("
                    <Code inline=true>"state.value"</Code>"); the demo source shows one way."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Drag the thumb, click anywhere in the area, or tab to the thumb and use the keyboard. The hooks report "
                    "no focus state; the demo shows the focus ring with "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>
                    " on the thumb, which contains the focused input."
                </p>

                <Demo description="Saturation and brightness area for an HSV color, with a disabled toggle" source=include_str!("demos/color_area.rs")>
                    <ColorAreaDemo/>
                </Demo>
            </Section>

            <Section title="use_color_area_state">
                <p>
                    "Holds the color and its two axis channels; the remaining channel ("<Code inline=true>"z_channel"</Code>
                    ") stays as it is. "<Code inline=true>"C"</Code>" is any "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())>
                        <Code inline=true>"ColorValue"</Code>
                    </Link>". The state owns the color, starting at "<Code inline=true>"default_value"</Code>", unless you bind "
                    "it to app state with "<Code inline=true>"value"</Code>"."
                </p>

                <Section title="Input" id="use-color-area-state-input">
                    <p>"Pass a "<Code inline=true>"UseColorAreaStateInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseColorAreaStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color. Required."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<C>>" default="None">
                            "The color as app state, replacing "<Code inline=true>"default_value"</Code>": "
                            <Code inline=true>"ValueBinding::from(rw_signal)"</Code>" or "
                            <Code inline=true>"ValueBinding::new(signal, callback)"</Code>"."
                        </ApiRow>
                        <ApiRow name="x_channel" ty="Option<Channel>" default="None">
                            "The channel on the horizontal axis. "<Code inline=true>"None"</Code>": the color space\u{2019}s first axis."
                        </ApiRow>
                        <ApiRow name="y_channel" ty="Option<Channel>" default="None">
                            "The channel on the vertical axis. "<Code inline=true>"None"</Code>": the color space\u{2019}s second axis."
                        </ApiRow>
                        <ApiRow name="x_channel_step, y_channel_step" ty="Option<f64>" default="None">
                            "Override the steps of the axis channels. "<Code inline=true>"None"</Code>": the channel\u{2019}s own step."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>" default="None">"Called with the color whenever it changes, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>" default="None">"Called with the color when a drag or a key press ends."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-area-state-return">
                    <p>"A "<Code inline=true>"Copy"</Code>" "<Code inline=true>"ColorAreaState<C>"</Code>":"</p>
                    <ApiTable kind=ApiKind::Return of="ColorAreaState">
                        <ApiRow name="value" ty="Signal<C>">"The color."</ApiRow>
                        <ApiRow name="x_value, y_value" ty="Signal<f64>">"The values of the axis channels."</ApiRow>
                        <ApiRow name="x_channel, y_channel, z_channel" ty="Channel">"The axis channels and the remaining one."</ApiRow>
                        <ApiRow name="x_channel_step, y_channel_step" ty="f64">"The steps of the axis channels (arrow keys)."</ApiRow>
                        <ApiRow name="x_channel_page_step, y_channel_page_step" ty="f64">
                            "The page steps of the axis channels ("<Keys keys="Shift"/>" with the arrow keys, page keys, "
                            <Keys keys="Home"/>" and "<Keys keys="End"/>")."
                        </ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the thumb is being dragged."</ApiRow>
                    </ApiTable>
                    <p>"Its methods change the color; each calls "<Code inline=true>"on_change"</Code>" when the color changes:"</p>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_value(color)"</Code></TableCell>
                            <TableCell>"Sets the color."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_x_value(v)"</Code>", "<Code inline=true>"set_y_value(v)"</Code></TableCell>
                            <TableCell>"Set one axis channel."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment_x(step)"</Code>", "<Code inline=true>"decrement_x(step)"</Code>", "
                                <Code inline=true>"increment_y(step)"</Code>", "<Code inline=true>"decrement_y(step)"</Code></TableCell>
                            <TableCell>"Change an axis channel by "<Code inline=true>"step"</Code>", snapped to the channel\u{2019}s step and range."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_color_from_point(point)"</Code></TableCell>
                            <TableCell>"Sets both channels from a "<Code inline=true>"Point"</Code>" of the area, each coordinate from 0 to 1 ("<Code inline=true>"y"</Code>" from the top)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"thumb_position()"</Code></TableCell>
                            <TableCell>"The color\u{2019}s "<Code inline=true>"Point"</Code>" in the area, each coordinate from 0 to 1 (tracked)."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_dragging(dragging)"</Code></TableCell>
                            <TableCell>"Starts or ends dragging; ending it calls "<Code inline=true>"on_change_end"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"default_value()"</Code></TableCell>
                            <TableCell>"The color the area started with; a form reset restores it."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_color_area">
                <p>
                    "Adds pointer, touch and keyboard interaction and the ARIA semantics: the area is a group, and two "
                    "visually hidden range inputs inside the thumb describe it to screen readers and take part in forms."
                </p>

                <Section title="Input" id="use-color-area-input">
                    <p>"Pass a "<Code inline=true>"UseColorAreaInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorAreaInput">
                        <ApiRow name="state" ty="ColorAreaState<C>">"The state from "<Code inline=true>"use_color_area_state"</Code>". Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the area is disabled."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the area; its inputs are named \u{201c}<label>, Color picker\u{201d}. The area has no visible "
                            "label, so set this or "<Code inline=true>"aria_labelledby"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the area."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"The ids of the elements describing the inputs."</ApiRow>
                        <ApiRow name="aria_details" ty="Option<String>" default="None">"The ids of the elements with details about the inputs."</ApiRow>
                        <ApiRow name="x_name, y_name" ty="Option<String>" default="None">"Form field names of the hidden range inputs."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form the inputs belong to."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-area-return">
                    <ApiTable kind=ApiKind::Return of="UseColorAreaReturn">
                        <ApiRow name="color_area_props" ty="PropsWithStyles<UseColorAreaProps>">
                            <Code inline=true>"role=\"group\""</Code>", its name, the "
                            "pointer handling that moves the thumb to a pressed point, and the gradient as styles."
                        </ApiRow>
                        <ApiRow name="thumb_props" ty="PropsWithStyles<UseColorAreaThumbProps>">
                            <Code inline=true>"role=\"presentation\""</Code>", dragging and keyboard handling, and the thumb\u{2019}s "
                            "position as styles."
                        </ApiRow>
                        <ApiRow name="x_input_props, y_input_props" ty="PropsWithStyles<UseColorAreaInputProps>">
                            "For the two "<Code inline=true>"<input type=\"range\">"</Code>" elements inside the thumb: range, step "
                            "and value of the axis channel, "<Code inline=true>"aria-roledescription=\"2D slider\""</Code>", the value "
                            "text, and visually hiding styles."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></li>
                <li><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></li>
                <li><Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link></li>
                <li><Link href=routes::doc::color_slider::Hook.materialize()>"Color Slider Hooks"</Link></li>
                <li><Link href=routes::doc::color_wheel::Hook.materialize()>"Color Wheel Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
