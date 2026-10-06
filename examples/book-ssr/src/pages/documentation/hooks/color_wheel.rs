use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_wheel::ColorWheelDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseColorWheel() -> impl IntoView {
    view! {
        <DocPage title="use_color_wheel">
            <p>
                "The "<Code inline=true>"use_color_wheel_state"</Code>" and "<Code inline=true>"use_color_wheel"</Code>
                " hooks build a circular wheel for one angular channel of a color, typically the hue. They work with any "
                <Link href=routes::doc::color::Hooks.materialize()>"color type"</Link>". See the "
                <Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useColorWheel"/>

            <Section title="Demo">
                <p>
                    "The wheel shows a conic gradient of the channel in an annulus (donut) shape. Click or drag anywhere on "
                    "the ring, drag the thumb, or focus it and use the keyboard. The same hooks drive an HSV and an HSL wheel."
                </p>

                <Demo description="Hue wheel for an HSV or HSL color" source=include_str!("demos/color_wheel.rs")>
                    <ColorWheelDemo/>
                </Demo>
            </Section>

            <Section title="use_color_wheel_state">
                <p>
                    "Owns the color and maps the wheel\u{2019}s channel to an angle: 0\u{00b0} at the top, increasing "
                    "clockwise. The other channels keep the values of the initial color; the wheel doesn\u{2019}t change them."
                </p>

                <Section title="Input" id="use-color-wheel-state-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorWheelStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color."</ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the wheel controls, typically the hue."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the wheel is disabled."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>">"Called when the color changes during interaction."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>">"Called when an interaction ends, e.g. a drag is released."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-wheel-state-return">
                    <ApiTable kind=ApiKind::Return of="UseColorWheelStateReturn">
                        <ApiRow name="value" ty="Signal<C>">"The current color."</ApiRow>
                        <ApiRow name="set_value" ty="Callback<C>">
                            "Sets the color programmatically. Does not call "<Code inline=true>"on_change"</Code>"."
                        </ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the wheel controls."</ApiRow>
                        <ApiRow name="hue" ty="Signal<f64>">"The channel\u{2019}s value, e.g. the hue in degrees (0\u{2013}360)."</ApiRow>
                        <ApiRow name="set_hue" ty="Callback<f64>">"Sets the channel\u{2019}s value."</ApiRow>
                        <ApiRow name="set_hue_from_point" ty="Callback<(f64, f64, f64)>">
                            "Sets the channel from a point "<Code inline=true>"(x, y, radius)"</Code>" relative to the wheel\u{2019}s center."
                        </ApiRow>
                        <ApiRow name="get_thumb_position" ty="Callback<f64, (f64, f64)>">
                            "The thumb position "<Code inline=true>"(x, y)"</Code>" relative to the center, on a circle of the given radius."
                        </ApiRow>
                        <ApiRow name="increment, decrement" ty="Callback<Option<f64>>">
                            "Change the channel by the given step (at least the channel\u{2019}s step), wrapping around."
                        </ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the user is dragging."</ApiRow>
                        <ApiRow name="set_dragging" ty="Callback<bool>">
                            "Sets the dragging state. Ending a drag calls "<Code inline=true>"on_change_end"</Code>"."
                        </ApiRow>
                        <ApiRow name="display_color" ty="Signal<C>">
                            "The color at the current channel value with maximum vividness, for gradients and the thumb."
                        </ApiRow>
                        <ApiRow name="step, page_step" ty="f64">"The channel\u{2019}s step and page step (1\u{00b0} and 15\u{00b0} for hue)."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the wheel is disabled."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_color_wheel">
                <p>
                    "Adds pointer, touch and keyboard interaction and the ARIA semantics. A pointer press on the ring (between "
                    "the inner and outer radius) jumps to that angle and starts a drag; presses in the hole are ignored. The "
                    "thumb is dragged with "<Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>
                    ". A visually hidden range input inside the thumb is the focus target and describes the wheel to screen readers."
                </p>

                <Section title="Input" id="use-color-wheel-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorWheelInput">
                        <ApiRow name="state" ty="UseColorWheelStateReturn<C>">"The state from "<Code inline=true>"use_color_wheel_state"</Code>"."</ApiRow>
                        <ApiRow name="outer_radius" ty="f64">"The outer radius of the wheel, in pixels."</ApiRow>
                        <ApiRow name="inner_radius" ty="f64">"The inner radius, in pixels. It cuts the hole of the annulus."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the wheel is disabled."</ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>">"An accessible name for the hidden input."</ApiRow>
                        <ApiRow name="name" ty="Option<&'static str>">"The form field name of the hidden input."</ApiRow>
                        <ApiRow name="form" ty="Option<&'static str>">"The id of the form the input belongs to."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-wheel-return">
                    <ApiTable kind=ApiKind::Return of="UseColorWheelReturn">
                        <ApiRow name="track_props" ty="UseColorWheelTrackProps">
                            "The pointer handling of the ring. Give the track "<Code inline=true>"touch-action: none"</Code>"."
                        </ApiRow>
                        <ApiRow name="thumb_props" ty="UseColorWheelThumbProps">
                            "Dragging, the "<Keys keys="PageUp"/>", "<Keys keys="PageDown"/>", "<Keys keys="Home"/>" and "<Keys keys="End"/>
                            " keys, focus handling and "<Code inline=true>"data-focus-visible"</Code>" while the input has keyboard focus."
                        </ApiRow>
                        <ApiRow name="input_props" ty="UseColorWheelInputProps">
                            "For a visually hidden "<Code inline=true>"<input type=\"range\">"</Code>" inside the thumb: range, step and "
                            "value of the channel, "<Code inline=true>"aria-label"</Code>" and an "<Code inline=true>"aria-valuetext"</Code>
                            " with the hue name (e.g. \u{201c}120\u{00b0}, green\u{201d})."
                        </ApiRow>
                        <ApiRow name="background" ty="Signal<String>">"The conic gradient background of the track."</ApiRow>
                        <ApiRow name="clip_path" ty="String">"The CSS "<Code inline=true>"clip-path"</Code>" cutting the track into an annulus."</ApiRow>
                        <ApiRow name="thumb_x, thumb_y" ty="Signal<f64>">
                            "The thumb\u{2019}s center relative to the track, in pixels, for CSS "<Code inline=true>"left"</Code>" and "
                            <Code inline=true>"top"</Code>"."
                        </ApiRow>
                        <ApiRow name="track_size" ty="f64">"The width and height of the track ("<Code inline=true>"outer_radius * 2"</Code>")."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-color-wheel-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let state = use_color_wheel_state(UseColorWheelStateInput {
                                default_value: HSV::new(),
                                channel: HsvChannel::Hue,
                                is_disabled: Signal::stored(false),
                                on_change: None,
                                on_change_end: None,
                            });
                            let wheel = use_color_wheel(UseColorWheelInput {
                                state,
                                outer_radius: 100.0,
                                inner_radius: 70.0,
                                is_disabled: Signal::stored(false),
                                aria_label: Some("Hue"),
                                name: None,
                                form: None,
                            });

                            view! {
                                <div {..wheel.track_props.into_attrs()}>
                                    <div {..wheel.thumb_props.into_attrs()}>
                                        <input {..wheel.input_props.into_attrs()} />
                                    </div>
                                </div>
                            }
                        "#)}
                    </Code>

                    <p>
                        "Size the track with "<Code inline=true>"track_size"</Code>", set its "<Code inline=true>"clip-path"</Code>
                        " and background, position the thumb absolutely at "<Code inline=true>"thumb_x"</Code>"/"
                        <Code inline=true>"thumb_y"</Code>" (centered with a translate), and hide the input visually. The demo "
                        "source shows a complete setup."
                    </p>
                </Section>
            </Section>

            <Section title="Keyboard">
                <p>"With the thumb\u{2019}s input focused:"</p>

                <KeyboardTable>
                    <KeyRow keys="ArrowRight / ArrowUp">"Increase the channel by one step, wrapping around."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">"Decrease the channel by one step, wrapping around."</KeyRow>
                    <KeyRow keys="Shift + Arrow keys / PageUp / PageDown">"Change the channel by its page step."</KeyRow>
                    <KeyRow keys="Home">"Set the channel to its minimum (0\u{00b0})."</KeyRow>
                    <KeyRow keys="End">"Set the channel to its maximum minus one step (359\u{00b0})."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorSlider.materialize()>"use_color_slider"</Link></li>
                <li><Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
