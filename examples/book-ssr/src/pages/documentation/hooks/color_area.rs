use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_area::ColorAreaDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseColorArea() -> impl IntoView {
    view! {
        <DocPage title="use_color_area">
            <p>
                "The "<Code inline=true>"use_color_area_state"</Code>" and "<Code inline=true>"use_color_area"</Code>
                " hooks build a 2D color gradient area in which users adjust two channels of a color at once, e.g. saturation "
                "and brightness. They work with any "<Link href=routes::doc::color::Hooks.materialize()>"color type"</Link>
                ". See the "<Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useColorArea"/>

            <Section title="Demo">
                <p>"Drag the thumb, click anywhere in the area, or focus the thumb and use the keyboard."</p>

                <Demo description="Saturation and brightness area for an HSV color" source=include_str!("demos/color_area.rs")>
                    <ColorAreaDemo/>
                </Demo>
            </Section>

            <Section title="use_color_area_state">
                <p>
                    "Owns the color and maps the two axis channels to a normalized thumb position. The remaining channel ("
                    <Code inline=true>"z_channel"</Code>") stays as it is."
                </p>

                <Section title="Input" id="use-color-area-state-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorAreaStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color."</ApiRow>
                        <ApiRow name="x_channel" ty="C::Channel">"The channel on the horizontal axis."</ApiRow>
                        <ApiRow name="y_channel" ty="C::Channel">"The channel on the vertical axis."</ApiRow>
                        <ApiRow name="x_channel_step, y_channel_step" ty="Option<f64>">
                            "Override the step of the axis channels. "<Code inline=true>"None"</Code>" uses the channel\u{2019}s own step."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>">"Called when the color changes during interaction."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>">"Called when an interaction ends, e.g. a drag is released."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-area-state-return">
                    <ApiTable kind=ApiKind::Return of="UseColorAreaStateReturn">
                        <ApiRow name="value" ty="Signal<C>">"The current color."</ApiRow>
                        <ApiRow name="display_color" ty="Signal<C>">"The color to display (the same as "<Code inline=true>"value"</Code>" without alpha support)."</ApiRow>
                        <ApiRow name="x_value, y_value" ty="Signal<f64>">"The values of the axis channels."</ApiRow>
                        <ApiRow name="x_channel, y_channel, z_channel" ty="C::Channel">"The axis channels and the remaining one."</ApiRow>
                        <ApiRow name="x_channel_step, y_channel_step" ty="f64">"The steps of the axis channels."</ApiRow>
                        <ApiRow name="x_channel_page_step, y_channel_page_step" ty="f64">"The page steps of the axis channels."</ApiRow>
                        <ApiRow name="thumb_position" ty="Signal<(f64, f64)>">
                            "The thumb position, normalized to 0\u{2013}1 on both axes. Y is inverted: 0 is the top (maximum)."
                        </ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the user is dragging."</ApiRow>
                        <ApiRow name="set_dragging" ty="Callback<bool>">
                            "Sets the dragging state. Ending a drag calls "<Code inline=true>"on_change_end"</Code>"."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Callback<C>">"Sets the whole color, e.g. from another input."</ApiRow>
                        <ApiRow name="set_x_value, set_y_value" ty="Callback<f64>">
                            "Sets one axis channel. Calls "<Code inline=true>"on_change"</Code>"."
                        </ApiRow>
                        <ApiRow name="set_color_from_point" ty="Callback<(f64, f64)>">
                            "Sets the color from a normalized point (0\u{2013}1 on both axes, Y inverted)."
                        </ApiRow>
                        <ApiRow name="increment_x, decrement_x, increment_y, decrement_y" ty="Callback<Option<f64>>">
                            "Change an axis channel by the given step, or by the channel step for "<Code inline=true>"None"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_color_area">
                <p>
                    "Adds pointer, touch and keyboard interaction (through "
                    <Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>
                    " with a center constraint) and the ARIA semantics. The area is the movement boundary, the thumb the "
                    "movable element. Two visually hidden range inputs inside the thumb describe the area to screen readers "
                    "and take part in forms."
                </p>

                <Section title="Input" id="use-color-area-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorAreaInput">
                        <ApiRow name="state" ty="UseColorAreaStateReturn<C>">"The state from "<Code inline=true>"use_color_area_state"</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the area is disabled."</ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>">"An accessible name for the area and its inputs."</ApiRow>
                        <ApiRow name="x_name, y_name" ty="Option<&'static str>">"Form field names of the hidden range inputs."</ApiRow>
                        <ApiRow name="form" ty="Option<&'static str>">"The id of the form the inputs belong to."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-area-return">
                    <ApiTable kind=ApiKind::Return of="UseColorAreaReturn">
                        <ApiRow name="area_props" ty="UseColorAreaProps">
                            <Code inline=true>"role=\"group\""</Code>", "<Code inline=true>"aria-label"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>" and the pointer handling that moves the thumb to a clicked point."
                        </ApiRow>
                        <ApiRow name="thumb_props" ty="UseColorAreaThumbProps">
                            <Code inline=true>"role=\"presentation\""</Code>", dragging and keyboard handling."
                        </ApiRow>
                        <ApiRow name="x_input_props, y_input_props" ty="UseColorAreaInputProps">
                            "For two visually hidden "<Code inline=true>"<input type=\"range\">"</Code>" inside the thumb: range, step "
                            "and value of the axis channel, "<Code inline=true>"aria-roledescription=\"2D slider\""</Code>" and an "
                            <Code inline=true>"aria-valuetext"</Code>" naming all three channels (and the hue name, if any). The Y "
                            "input is hidden from assistive technology and the tab order."
                        </ApiRow>
                        <ApiRow name="background" ty="Signal<String>">"The CSS gradient background of the area."</ApiRow>
                        <ApiRow name="background_blend_mode" ty="Signal<Option<&'static str>>">
                            "A CSS "<Code inline=true>"background-blend-mode"</Code>" the gradient needs (e.g. "
                            <Code inline=true>"\"screen\""</Code>" for RGB). Apply it when "<Code inline=true>"Some"</Code>"."
                        </ApiRow>
                        <ApiRow name="thumb_color" ty="Signal<String>">"The CSS color of the thumb."</ApiRow>
                        <ApiRow name="thumb_x_percent" ty="Signal<f64>">"The thumb position for CSS "<Code inline=true>"left"</Code>" (0\u{2013}100)."</ApiRow>
                        <ApiRow name="thumb_y_percent" ty="Signal<f64>">"The thumb position for CSS "<Code inline=true>"bottom"</Code>" (0\u{2013}100)."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-color-area-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let state = use_color_area_state(UseColorAreaStateInput {
                                default_value: HSV::new(),
                                x_channel: HsvChannel::Saturation,
                                y_channel: HsvChannel::Brightness,
                                x_channel_step: None,
                                y_channel_step: None,
                                on_change: None,
                                on_change_end: None,
                            });
                            let area = use_color_area(UseColorAreaInput {
                                state,
                                is_disabled: Signal::stored(false),
                                aria_label: Some("Color"),
                                x_name: None,
                                y_name: None,
                                form: None,
                            });

                            view! {
                                <div {..area.area_props.into_attrs()}>
                                    <div {..area.thumb_props.into_attrs()}>
                                        <input {..area.x_input_props.into_attrs()} />
                                        <input {..area.y_input_props.into_attrs()} />
                                    </div>
                                </div>
                            }
                        "#)}
                    </Code>

                    <p>
                        "Position the thumb absolutely with "<Code inline=true>"thumb_x_percent"</Code>" and "
                        <Code inline=true>"thumb_y_percent"</Code>", set the area\u{2019}s background from "
                        <Code inline=true>"background"</Code>", and hide the inputs visually. The demo source shows a complete setup."
                    </p>
                </Section>
            </Section>

            <Section title="Keyboard">
                <p>"With the thumb\u{2019}s input focused:"</p>

                <KeyboardTable>
                    <KeyRow keys="ArrowLeft / ArrowRight / ArrowUp / ArrowDown">"Move the thumb by one pixel."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increase or decrease the Y channel by its page step."</KeyRow>
                    <KeyRow keys="Home / End">"Decrease or increase the X channel by its page step (reversed in RTL)."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorSlider.materialize()>"use_color_slider"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorWheel.materialize()>"use_color_wheel"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
