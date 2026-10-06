use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_slider::ColorSliderDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseColorSlider() -> impl IntoView {
    view! {
        <DocPage title="use_color_slider">
            <p>
                "The "<Code inline=true>"use_color_slider_state"</Code>" and "<Code inline=true>"use_color_slider"</Code>
                " hooks build a slider that adjusts one channel of a color, e.g. its hue. They work with any "
                <Link href=routes::doc::color::Hooks.materialize()>"color type"</Link>". See the "
                <Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useColorSlider"/>

            <p>
                "The color slider wraps the general-purpose "<Link href=routes::doc::slider::Hook.materialize()>"slider hooks"</Link>
                " with a gradient background for the track and color-aware ARIA. Without an "<Code inline=true>"aria_label"</Code>
                ", it is labelled with the channel name (e.g. \u{201c}Hue\u{201d}), and its value text includes the hue name "
                "for hue channels."
            </p>

            <Section title="Demo">
                <Demo description="Hue slider for an HSV color" source=include_str!("demos/color_slider.rs")>
                    <ColorSliderDemo/>
                </Demo>
            </Section>

            <Section title="use_color_slider_state">
                <p>
                    "Owns the color and keeps it in sync with a single-thumb "<Code inline=true>"use_slider_state"</Code>
                    " whose range and step are those of the channel. Takes its input by reference."
                </p>

                <Section title="Input" id="use-color-slider-state-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorSliderStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color."</ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the slider controls."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the slider is disabled."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>">
                            "Horizontal or vertical. Affects keyboard navigation and how pointer positions are read."
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>">"Called when the color changes during interaction."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>">"Called when an interaction ends."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-slider-state-return">
                    <ApiTable kind=ApiKind::Return of="UseColorSliderStateReturn">
                        <ApiRow name="value" ty="Signal<C>">"The current color."</ApiRow>
                        <ApiRow name="set_value" ty="Callback<C>">"Sets the whole color."</ApiRow>
                        <ApiRow name="slider_state" ty="UseSliderStateReturn">"The underlying single-thumb slider state."</ApiRow>
                        <ApiRow name="display_color" ty="Signal<C>">
                            "The color for the gradient: for hue, full saturation and brightness at the current hue; otherwise "
                            "the color as it is."
                        </ApiRow>
                        <ApiRow name="thumb_value_label" ty="Signal<String>">"The formatted channel value."</ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the slider controls."</ApiRow>
                        <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the thumb is being dragged."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_color_slider">
                <p>
                    "Adds interaction and ARIA through "<Code inline=true>"use_slider"</Code>" and "
                    <Code inline=true>"use_slider_thumb"</Code>", plus the track gradient."
                </p>

                <Section title="Input" id="use-color-slider-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorSliderInput">
                        <ApiRow name="state" ty="UseColorSliderStateReturn<C>">"The state from "<Code inline=true>"use_color_slider_state"</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the thumb is disabled."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>">
                            "The direction of the track gradient. A horizontal gradient runs right to left in right-to-left locales."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>">
                            "An accessible name. "<Code inline=true>"None"</Code>" uses the channel name."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<&'static str>">"The form field name of the thumb\u{2019}s hidden input."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-slider-return">
                    <ApiTable kind=ApiKind::Return of="UseColorSliderReturn">
                        <ApiRow name="slider" ty="UseSliderReturn">
                            "Props for the group, label, output and track, as returned by "<Code inline=true>"use_slider"</Code>"."
                        </ApiRow>
                        <ApiRow name="thumb" ty="UseSliderThumbReturn">
                            "Props for the thumb and its hidden input, as returned by "<Code inline=true>"use_slider_thumb"</Code>
                            ", including the thumb\u{2019}s "<Code inline=true>"percentage"</Code>"."
                        </ApiRow>
                        <ApiRow name="background" ty="Signal<String>">"The CSS gradient for the track."</ApiRow>
                        <ApiRow name="track_style" ty="Signal<String>">
                            "A CSS style string with the gradient and "<Code inline=true>"forced-color-adjust: none"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-color-slider-example">
                    <Code language=Language::Rust>
                        {indoc!(r"
                            let state = use_color_slider_state(&UseColorSliderStateInput {
                                default_value: HSV::new(),
                                channel: HsvChannel::Hue,
                                is_disabled: Signal::stored(false),
                                orientation: Orientation::Horizontal.into(),
                                on_change: None,
                                on_change_end: None,
                            });
                            let slider = use_color_slider(UseColorSliderInput {
                                state,
                                is_disabled: Signal::stored(false),
                                orientation: Orientation::Horizontal.into(),
                                aria_label: None,
                                name: None,
                            });
                            let (track_attrs, track_styles) = slider.slider.track_props.into_parts();

                            view! {
                                <div {..track_attrs} style=track_styles>
                                    <div {..slider.thumb.thumb_props.into_attrs()}>
                                        <input {..slider.thumb.input_props.into_attrs()} />
                                    </div>
                                </div>
                            }
                        ")}
                    </Code>

                    <p>
                        "Add the gradient from "<Code inline=true>"background"</Code>" to the track\u{2019}s styles and position the "
                        "thumb with "<Code inline=true>"thumb.percentage"</Code>"; the demo source shows how."
                    </p>
                </Section>
            </Section>

            <Section title="Keyboard">
                <p>"With the thumb\u{2019}s input focused (horizontal, left-to-right):"</p>

                <KeyboardTable>
                    <KeyRow keys="ArrowRight / ArrowUp">"Increase the channel by its step."</KeyRow>
                    <KeyRow keys="ArrowLeft / ArrowDown">"Decrease the channel by its step."</KeyRow>
                    <KeyRow keys="Shift + ArrowRight / Shift + ArrowUp / PageUp">"Increase the channel by its page step."</KeyRow>
                    <KeyRow keys="Shift + ArrowLeft / Shift + ArrowDown / PageDown">"Decrease the channel by its page step."</KeyRow>
                    <KeyRow keys="Home / End">"Set the channel to its minimum or maximum."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorWheel.materialize()>"use_color_wheel"</Link></li>
                <li><Link href=routes::doc::slider::Hook.materialize()>"use_slider"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
