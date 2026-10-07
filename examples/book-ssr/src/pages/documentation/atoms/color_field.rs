use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_field::ColorFieldAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomColorField() -> impl IntoView {
    view! {
        <DocPage title="Color Field Atoms">
            <p>
                "The "<Code inline=true>"ColorField"</Code>" and "<Code inline=true>"ColorChannelField"</Code>" atoms render "
                "unstyled fields for a color as hex text and for one channel of a color as a number. Put an "
                <Link href=format!("{}#input-textarea", routes::doc::text_field::Atom.materialize())>"Input"</Link>" and the "
                <Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link>" (a label, a description and an "
                "error message) inside. See the "<Link href=routes::doc::ColorField.materialize()>"Color Field overview"</Link>
                " for the concept and its keyboard interaction."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ColorField"</Code>" calls "
                    <Link href=format!("{}#use-color-field-state", routes::doc::color_field::Hook.materialize())>"use_color_field_state"</Link>
                    " and "
                    <Link href=format!("{}#use-color-field", routes::doc::color_field::Hook.materialize())>"use_color_field"</Link>
                    ", "<Code inline=true>"ColorChannelField"</Code>" calls "
                    <Link href=format!("{}#use-color-channel-field-state", routes::doc::color_field::Hook.materialize())>"use_color_channel_field_state"</Link>
                    " and "
                    <Link href=format!("{}#use-color-channel-field", routes::doc::color_field::Hook.materialize())>"use_color_channel_field"</Link>
                    ". The "<Code inline=true>"Input"</Code>" inside adds "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" for its data attributes."
                </p>
            </Section>

            <Section title="Example">
                <p>"A hex field and a field for the hue of an HSV color, each with a label:"</p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::prelude::*,
                            utils::color::{HSV, HsvChannel, RGB8},
                        };
                        use leptos::prelude::*;

                        let accent = RwSignal::new(Some(RGB8 { r: 30, g: 110, b: 200 }));
                        let tint = RwSignal::new(Some(HSV { hue: 210.0, saturation: 0.6, value: 0.8 }));

                        view! {
                            <ColorField value=accent set_value=accent name="accent">
                                <Label>"Accent color"</Label>
                                <Input/>
                                <FieldError/>
                            </ColorField>

                            <ColorChannelField channel=HsvChannel::Hue value=tint set_value=tint>
                                <Label>"Hue"</Label>
                                <Input/>
                            </ColorChannelField>
                        }
                    "#)}
                </Code>
                <p>
                    "The channel determines the color type of a "<Code inline=true>"ColorChannelField"</Code>": "
                    <Code inline=true>"HsvChannel::Hue"</Code>" edits an "<Code inline=true>"HSV"</Code>". Inside a "
                    <Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>
                    ", leave out "<Code inline=true>"value"</Code>": the fields show and change the picker\u{2019}s color."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "A hex field and fields for the red, green and blue channels, bound to one "
                    <Code inline=true>"RwSignal<Option<RGB8>>"</Code>". Type a value and press "<Keys keys="Enter"/>" or "
                    "leave the field to commit it, or step it with the arrow keys. Emptying the hex field empties all four."
                </p>
                <Demo
                    description="A hex ColorField and three RGB ColorChannelFields editing one color, with a swatch and a disabled toggle"
                    source=include_str!("demos/color_field.rs")
                >
                    <ColorFieldAtomDemo/>
                </Demo>
            </Section>

            <Section title="ColorField">
                <p>
                    "A field for a whole color as hex text ("<Code inline=true>"#RRGGBB"</Code>"), holding an "
                    <Code inline=true>"Option<RGB8>"</Code>" ("<Code inline=true>"None"</Code>": empty). Renders a "
                    <Code inline=true>"<div>"</Code>" around its children; with a "<Code inline=true>"name"</Code>", also a "
                    "hidden input submitting the hex code with a form."
                </p>
                <Section title="Props" id="color-field-props">
                    <ApiTable kind=ApiKind::Props of="atoms::color_field::ColorField">
                        <ApiRow name="default_value" ty="Option<RGB8>" default="None">
                            "The initial color, unless "<Code inline=true>"value"</Code>" is set; "<Code inline=true>"None"</Code>
                            ": empty. A form reset restores it."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<RGB8>>>" default="None">
                            "The color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color of the "
                            <Code inline=true>"ColorPicker"</Code>" around it, if any."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<RGB8>>>" default="None">
                            "Receives the committed color: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<RGB8>>>" default="None">"Called with the committed color."</ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Whether a color is required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<RGB8>>>" default="None">"Validates the committed color."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Link href=routes::doc::Form.materialize()>"Form"</Link>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The hidden input\u{2019}s name, submitting the color as "<Code inline=true>"#RRGGBB"</Code>
                            " (also matching server errors)."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form, if the field isn\u{2019}t inside it."</ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focuses the input when it mounts."</ApiRow>
                        <ApiRow name="is_wheel_disabled" ty="bool" default="false">"Leave the color alone on scroll (it steps while the field has focus)."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Labels the field when it has no "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling and describing elements."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the field\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The label, the input and any other content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorChannelField">
                <p>
                    "A "<Link href=routes::doc::NumberField.materialize()>"number field"</Link>" for one channel of a color, "
                    "formatted as the channel is (\u{201c}210\u{00b0}\u{201d}, \u{201c}60%\u{201d}, \u{201c}128\u{201d}). It is "
                    "generic over the channel type "<Code inline=true>"Ch"</Code>" ("<Code inline=true>"HsvChannel"</Code>", "
                    <Code inline=true>"HslChannel"</Code>" or "<Code inline=true>"RgbChannel"</Code>"); the color type "
                    <Code inline=true>"Ch::Color"</Code>" follows from it. Without a "<Code inline=true>"Label"</Code>
                    " or "<Code inline=true>"aria_label"</Code>", the channel\u{2019}s name labels the field. It has no stepper "
                    "buttons; the arrow keys and the scroll wheel step the value."
                </p>
                <Section title="Props" id="color-channel-field-props">
                    <ApiTable kind=ApiKind::Props of="ColorChannelField">
                        <ApiRow name="channel" ty="Ch">"Required. The channel the field edits."</ApiRow>
                        <ApiRow name="default_value" ty="Option<Ch::Color>" default="None">
                            "The initial color, unless "<Code inline=true>"value"</Code>" is set; "<Code inline=true>"None"</Code>": empty."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Option<Ch::Color>>>" default="None">
                            "The color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color of the "
                            <Code inline=true>"ColorPicker"</Code>" around it, if any."
                        </ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Option<Ch::Color>>>" default="None">
                            "Receives the color with the committed channel value: an "<Code inline=true>"RwSignal"</Code>", "
                            <Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<Ch::Color>>>" default="None">
                            "Called with the color when the channel\u{2019}s value is committed."
                        </ApiRow>
                        <ApiRow name="is_disabled, is_read_only" ty="Signal<bool>" default="false">"Disables the field or makes it read-only."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Whether a value is required."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">
                            "Marks the field invalid while "<Code inline=true>"true"</Code>", taking precedence over all other validation."
                        </ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<f64>>>" default="None">"Validates the channel\u{2019}s value."</ApiRow>
                        <ApiRow name="validation_behavior" ty="Option<ValidationBehavior>" default="None">
                            "When errors are shown. "<Code inline=true>"None"</Code>": the behavior of the surrounding "
                            <Code inline=true>"Form"</Code>", else "<Code inline=true>"Native"</Code>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">
                            "The hidden input\u{2019}s name, submitting the channel\u{2019}s value (e.g. "<Code inline=true>"0.6"</Code>
                            " for 60% saturation)."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of the form, if the field isn\u{2019}t inside it."</ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Labels the field when it has no "<Code inline=true>"Label"</Code>". Without either, the channel\u{2019}s name does."
                        </ApiRow>
                        <ApiRow name="aria_labelledby, aria_describedby" ty="Option<String>" default="None">"Further labelling and describing elements."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the field\u{2019}s "<Code inline=true>"<div>"</Code>"."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The label, the input and any other content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "On the field\u{2019}s "<Code inline=true>"<div>"</Code>" of both atoms, set to "<Code inline=true>"true"</Code>
                    " while the state applies. The input renders the data attributes of the "
                    <Link href=format!("{}#input-data-attributes", routes::doc::text_field::Atom.materialize())>"Input"</Link>"."
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-disabled" ty="true">"The field is disabled."</ApiRow>
                    <ApiRow name="data-readonly" ty="true">"The field is read-only."</ApiRow>
                    <ApiRow name="data-required" ty="true">"The field is required."</ApiRow>
                    <ApiRow name="data-invalid" ty="true">"The value is invalid."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>"Style the input through its data attributes, and the field through its own:"</p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-color-input { width: 8em; border: 1px solid var(--border); border-radius: 4px; font-family: monospace; }
                        .my-color-input[data-hovered] { border-color: var(--accent); }
                        .my-color-input[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 1px; }
                        .my-color-input[data-invalid] { border-color: var(--danger); }
                        .my-color-field[data-disabled] { opacity: 0.5; }
                    ")}
                </Code>
            </Section>

            <Section title="Composition">
                <p>
                    "Show the typed color next to the field with a "
                    <Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>", as the demo does. Inside a "
                    <Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>" atom, the fields share "
                    "the picker\u{2019}s color with areas, sliders and swatches, each in its own color space."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></li>
                <li><Link href=routes::doc::color_field::Hook.materialize()>"Color Field Hooks"</Link></li>
                <li><Link href=routes::doc::color_picker::Atom.materialize()>"Color Picker Atom"</Link></li>
                <li><Link href=routes::doc::number_field::Atom.materialize()>"Number Field Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
