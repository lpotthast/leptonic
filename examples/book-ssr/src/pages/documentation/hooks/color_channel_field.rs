use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_channel_field::ColorChannelFieldDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseColorChannelField() -> impl IntoView {
    view! {
        <DocPage title="use_color_channel_field">
            <p>
                "The "<Code inline=true>"use_color_channel_field_state"</Code>" and "<Code inline=true>"use_color_channel_field"</Code>
                " hooks build a number field that edits a single channel of a color, e.g. its hue or red value. They work "
                "with any "<Link href=routes::doc::color::Hooks.materialize()>"color type"</Link>". See the "
                <Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useColorChannelField"/>

            <Section title="Demo">
                <p>"Type a hue, use the buttons, or focus the field and use the arrow keys."</p>

                <Demo description="Hue channel number field with stepper buttons and a disabled toggle" source=include_str!("demos/color_channel_field.rs")>
                    <ColorChannelFieldDemo/>
                </Demo>
            </Section>

            <Section title="use_color_channel_field_state">
                <p>
                    "Owns the full color and exposes the value of one channel. Setting the channel value clamps it to the "
                    "channel\u{2019}s range and maps it back to the full color with "
                    <Code inline=true>"ColorValue::with_channel_value"</Code>"."
                </p>

                <Section title="Input" id="use-color-channel-field-state-input">
                    <p>
                        "The input has no "<Code inline=true>"Default"</Code>"; set every field. The hook takes it by reference: "
                        <Code inline=true>"use_color_channel_field_state(&input)"</Code>"."
                    </p>

                    <ApiTable kind=ApiKind::Input of="UseColorChannelFieldStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color."</ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the field edits."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>">"Called with the full color when it changes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-channel-field-state-return">
                    <ApiTable kind=ApiKind::Return of="UseColorChannelFieldStateReturn">
                        <ApiRow name="color_value" ty="Signal<C>">"The current color."</ApiRow>
                        <ApiRow name="set_color_value" ty="Callback<C>">"Sets the full color."</ApiRow>
                        <ApiRow name="channel_value" ty="Signal<Option<f64>>">"The value of the channel."</ApiRow>
                        <ApiRow name="set_channel_value" ty="Callback<Option<f64>>">
                            "Sets the channel, clamped to its range. "<Code inline=true>"None"</Code>" is ignored."
                        </ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the field edits."</ApiRow>
                        <ApiRow name="min_value, max_value, step" ty="f64">"The channel\u{2019}s range and step."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_color_channel_field">
                <p>
                    "A thin wrapper around "<Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>
                    ": it creates the number field state with the channel\u{2019}s range and step, labels the field with the "
                    "channel name (e.g. \u{201c}Hue\u{201d}, \u{201c}Red\u{201d}) unless you pass an "<Code inline=true>"aria_label"</Code>", and keeps field and "
                    "color in sync in both directions."
                </p>

                <Section title="Input" id="use-color-channel-field-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorChannelFieldInput">
                        <ApiRow name="state" ty="UseColorChannelFieldStateReturn<C>">
                            "The state from "<Code inline=true>"use_color_channel_field_state"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the field is disabled."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>">"Whether the field is read-only."</ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>">
                            "An accessible name. "<Code inline=true>"None"</Code>" uses the channel name."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-channel-field-return">
                    <p>
                        "Returns the "<Code inline=true>"UseNumberFieldReturn"</Code>" of "
                        <Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>
                        "."
                    </p>

                    <ApiTable kind=ApiKind::Return of="UseNumberFieldReturn">
                        <ApiRow name="group_props" ty="UseNumberFieldGroupProps">"For the element wrapping the input and the buttons."</ApiRow>
                        <ApiRow name="label_props" ty="UseLabelProps">"For a visible label."</ApiRow>
                        <ApiRow name="input_props" ty="UseNumberFieldInputProps">"For the text input, including its value."</ApiRow>
                        <ApiRow name="increment_button, decrement_button" ty="UseButtonInput">
                            "The stepper buttons\u{2019} configuration. Pass them to "
                            <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>"."
                        </ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">"For a description and an error message."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The input, once rendered."</ApiRow>
                        <ApiRow name="is_focused, is_focus_visible" ty="Signal<bool>">"Whether the input has focus, and whether to show a focus ring."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation result is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">"Detailed validity state."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-color-channel-field-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let state = use_color_channel_field_state(&UseColorChannelFieldStateInput {
                                default_value: HSV::new(),
                                channel: HsvChannel::Hue,
                                on_change: None,
                            });
                            let field = use_color_channel_field(UseColorChannelFieldInput {
                                state,
                                is_disabled: Signal::stored(false),
                                is_read_only: Signal::stored(false),
                                aria_label: None,
                            });
                            let (decrement, decrement_styles) = use_button(field.decrement_button).props.into_parts();
                            let (increment, increment_styles) = use_button(field.increment_button).props.into_parts();

                            view! {
                                <div {..field.group_props.into_attrs()}>
                                    <button {..decrement} style=decrement_styles>"\u{2212}"</button>
                                    <input {..field.input_props.into_attrs()} />
                                    <button {..increment} style=increment_styles>"+"</button>
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Keyboard">
                <p>
                    "As in "<Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link>
                    ", with the channel\u{2019}s step:"
                </p>

                <KeyboardTable>
                    <KeyRow keys="ArrowUp / PageUp">"Increase the channel by one step."</KeyRow>
                    <KeyRow keys="ArrowDown / PageDown">"Decrease the channel by one step."</KeyRow>
                    <KeyRow keys="Home / End">"Set the channel to its minimum or maximum."</KeyRow>
                </KeyboardTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorField.materialize()>"use_color_field"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorSlider.materialize()>"use_color_slider"</Link></li>
                <li><Link href=routes::doc::text_field::NumberFieldHook.materialize()>"use_number_field"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
