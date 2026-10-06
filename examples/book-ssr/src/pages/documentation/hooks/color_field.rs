use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_field::ColorFieldDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseColorField() -> impl IntoView {
    view! {
        <DocPage title="use_color_field">
            <p>
                "The "<Code inline=true>"use_color_field_state"</Code>" and "<Code inline=true>"use_color_field"</Code>
                " hooks build a text field for hex colors ("<Code inline=true>"#RRGGBB"</Code>"), part of the "
                <Link href=routes::doc::color::Hooks.materialize()>"color hooks"</Link>". See the "
                <Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useColorField"/>

            <Section title="Demo">
                <p>
                    "Type a hex color and leave the field to commit it. With the field focused, the arrow keys and the "
                    "scroll wheel step through the colors."
                </p>

                <Demo description="Hex color field with a color preview and a disabled toggle" source=include_str!("demos/color_field.rs")>
                    <ColorFieldDemo/>
                </Demo>
            </Section>

            <Section title="use_color_field_state">
                <p>
                    "Tracks two values: the text in the field and the parsed "<Code inline=true>"RGB8"</Code>" color. Typing only "
                    "accepts hex characters (with an optional leading "<Code inline=true>"#"</Code>", at most six digits); the "
                    "text is parsed when it is committed. If it doesn\u{2019}t parse, the field reverts to the last valid color. "
                    "An empty field commits as no color."
                </p>

                <Section title="Input" id="use-color-field-state-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorFieldStateInput">
                        <ApiRow name="default_value" ty="Option<RGB8>">"The initial color. "<Code inline=true>"None"</Code>" starts with an empty field."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<RGB8>>>">"Called when the color changes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-field-state-return">
                    <ApiTable kind=ApiKind::Return of="UseColorFieldStateReturn">
                        <ApiRow name="input_value" ty="Signal<String>">
                            "The text in the field. Bind it to the input\u{2019}s value ("<Code inline=true>"prop:value"</Code>")."
                        </ApiRow>
                        <ApiRow name="set_input_value" ty="Callback<String>">"Sets the text if it is valid hex input. Doesn\u{2019}t parse it."</ApiRow>
                        <ApiRow name="color_value" ty="Signal<Option<RGB8>>">"The committed color; "<Code inline=true>"None"</Code>" for an empty field."</ApiRow>
                        <ApiRow name="set_color_value" ty="Callback<Option<RGB8>>">"Sets the color programmatically and updates the text."</ApiRow>
                        <ApiRow name="commit" ty="Callback<()>">
                            "Parses the text and updates the color, or reverts the text to the last valid color. "
                            <Code inline=true>"use_color_field"</Code>" calls it on blur."
                        </ApiRow>
                        <ApiRow name="validate" ty="Callback<String, bool>">"Whether a text is valid partial or complete hex input."</ApiRow>
                        <ApiRow name="increment, decrement" ty="Callback<()>">"Change the color\u{2019}s hex value by one."</ApiRow>
                        <ApiRow name="increment_to_max, decrement_to_min" ty="Callback<()>">
                            "Set the color to "<Code inline=true>"#FFFFFF"</Code>" or "<Code inline=true>"#000000"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_color_field">
                <p>
                    "Turns a text input into a "<Code inline=true>"spinbutton"</Code>" over the hex values "
                    <Code inline=true>"#000000"</Code>" to "<Code inline=true>"#FFFFFF"</Code>": it filters typed text, commits "
                    "on blur, and steps the value with the keyboard or the scroll wheel while focused."
                </p>

                <Section title="Input" id="use-color-field-input">
                    <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                    <ApiTable kind=ApiKind::Input of="UseColorFieldInput">
                        <ApiRow name="state" ty="UseColorFieldStateReturn">"The state from "<Code inline=true>"use_color_field_state"</Code>"."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">
                            "Whether the field is disabled. Sets "<Code inline=true>"aria-disabled"</Code>" and turns off stepping; "
                            "set the native "<Code inline=true>"disabled"</Code>" attribute on the input yourself."
                        </ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>">"Whether the field is read-only. Turns off stepping."</ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>">"An accessible name for the input."</ApiRow>
                        <ApiRow name="is_wheel_disabled" ty="bool">"Turn off changing the value with the scroll wheel."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-field-return">
                    <ApiTable kind=ApiKind::Return of="UseColorFieldReturn">
                        <ApiRow name="input_props" ty="UseColorFieldInputProps">
                            <Code inline=true>"role=\"spinbutton\""</Code>", "<Code inline=true>"type=\"text\""</Code>", disabled "
                            "autocomplete, autocorrect and spellcheck, "<Code inline=true>"aria-label"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"aria-valuenow"</Code>"/"
                            <Code inline=true>"-valuemin"</Code>"/"<Code inline=true>"-valuemax"</Code>" (the hex value as an integer), "
                            <Code inline=true>"aria-valuetext"</Code>" (e.g. "<Code inline=true>"#4287F5"</Code>"), and the input, "
                            "focus, keyboard and wheel handlers. The value isn\u{2019}t included: bind "
                            <Code inline=true>"state.input_value"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-color-field-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let state = use_color_field_state(UseColorFieldStateInput {
                                default_value: Some(RGB8 { r: 66, g: 135, b: 245 }),
                                on_change: None,
                            });
                            let input_value = state.input_value;
                            let field = use_color_field(UseColorFieldInput {
                                state,
                                is_disabled: Signal::stored(false),
                                is_read_only: Signal::stored(false),
                                aria_label: Some("Hex color"),
                                is_wheel_disabled: false,
                            });

                            view! { <input prop:value=input_value {..field.input_props.into_attrs()} /> }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Keyboard">
                <KeyboardTable>
                    <KeyRow keys="ArrowUp / ArrowDown">"Increase or decrease the hex value by one."</KeyRow>
                    <KeyRow keys="PageUp / PageDown">"Increase or decrease the hex value by 16 ("<Code inline=true>"0x10"</Code>")."</KeyRow>
                    <KeyRow keys="Home">"Set the color to "<Code inline=true>"#000000"</Code>"."</KeyRow>
                    <KeyRow keys="End">"Set the color to "<Code inline=true>"#FFFFFF"</Code>"."</KeyRow>
                </KeyboardTable>

                <p>"The keys do nothing while the field is disabled or read-only."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorChannelField.materialize()>"use_color_channel_field"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorSwatch.materialize()>"use_color_swatch"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
