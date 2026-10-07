use indoc::indoc;
use leptos::prelude::*;

use super::demos::{color_channel_field::ColorChannelFieldDemo, color_field::ColorFieldDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseColorField() -> impl IntoView {
    view! {
        <DocPage title="Color Field Hooks">
            <p>
                "The "<Code inline=true>"use_color_field"</Code>" hooks build a field for a whole color as a hex code ("
                <Code inline=true>"#RRGGBB"</Code>"), the "<Code inline=true>"use_color_channel_field"</Code>" hooks a "
                "number field for one channel of a color, e.g. its hue or its red value. See the "
                <Link href=routes::doc::ColorField.materialize()>"Color Field overview"</Link>" for the concept and its "
                "keyboard interaction."
            </p>

            <ReactAria hook="useColorField"/>

            <Section title="Example">
                <p>
                    "A hex field: create the state, pass it to "<Code inline=true>"use_color_field"</Code>" and spread the "
                    "returned props onto a label and an input. The input props carry the text, "<Code inline=true>"disabled"</Code>
                    " and "<Code inline=true>"readonly"</Code>", as in a "<Link href=routes::doc::TextField.materialize()>"text field"</Link>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{hooks::*, utils::color::RGB8};
                        use leptos::prelude::*;

                        let state = use_color_field_state(UseColorFieldStateInput {
                            default_value: Some(RGB8 { r: 66, g: 135, b: 245 }),
                            ..UseColorFieldStateInput::default()
                        });
                        let field = use_color_field(UseColorFieldInput {
                            state,
                            has_label: true.into(),
                            id: None,
                            aria_label: MaybeProp::default(),
                            aria_labelledby: None,
                            aria_describedby: None,
                            is_disabled: false.into(),
                            is_read_only: false.into(),
                            is_required: false.into(),
                            placeholder: MaybeProp::default(),
                            auto_focus: false,
                            is_wheel_disabled: false,
                            on_focus: None,
                            on_blur: None,
                            on_focus_change: None,
                            on_key_down: None,
                            on_key_up: None,
                        });

                        view! {
                            <label {..field.label_props.into_attrs()}>"Accent color"</label>
                            <input {..field.input_props.into_attrs()}/>
                        }
                    "#)}
                </Code>

                <p>
                    "A field for the hue channel: a "<Link href=routes::doc::number_field::Hook.materialize()>"number field"</Link>
                    " with stepper buttons from "<Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>":"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            hooks::*,
                            utils::color::{HSV, HsvChannel},
                        };
                        use leptos::prelude::*;

                        let state = use_color_channel_field_state(UseColorChannelFieldStateInput {
                            default_value: Some(HSV::new()),
                            value: None,
                            channel: HsvChannel::Hue,
                            is_disabled: false.into(),
                            is_read_only: false.into(),
                            is_invalid: false.into(),
                            validate: None,
                            validation_behavior: ValidationBehavior::Aria,
                            name: None,
                            on_change: None,
                        });
                        let field = use_color_channel_field(UseColorChannelFieldInput {
                            state,
                            field: UseNumberFieldInput {
                                state: state.number,
                                id: None,
                                has_label: false.into(),
                                aria_label: MaybeProp::default(),
                                aria_labelledby: None,
                                aria_describedby: None,
                                is_required: false.into(),
                                placeholder: MaybeProp::default(),
                                auto_focus: false,
                                is_wheel_disabled: false,
                                increment_aria_label: MaybeProp::default(),
                                decrement_aria_label: MaybeProp::default(),
                                on_focus: None,
                                on_blur: None,
                                on_focus_change: None,
                                on_key_down: None,
                                on_key_up: None,
                            },
                        });
                        let (decrement, decrement_styles) = use_button(field.decrement_button).props.into_parts();
                        let (increment, increment_styles) = use_button(field.increment_button).props.into_parts();

                        view! {
                            <div {..field.group_props.into_attrs()}>
                                <button {..decrement} style=decrement_styles>"\u{2212}"</button>
                                <input {..field.input_props.into_attrs()}/>
                                <button {..increment} style=increment_styles>"+"</button>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "Type a hex color and press "<Keys keys="Enter"/>" or leave the field to commit it. With the field "
                    "focused, the arrow keys and the scroll wheel step through the colors."
                </p>

                <Demo description="Hex color field with a label, a color preview and a disabled toggle" source=include_str!("demos/color_field.rs")>
                    <ColorFieldDemo/>
                </Demo>

                <p>"A field for the hue channel. Type a hue, use the buttons, or focus the field and use the arrow keys."</p>

                <Demo description="Hue channel number field with stepper buttons and a disabled toggle" source=include_str!("demos/color_channel_field.rs")>
                    <ColorChannelFieldDemo/>
                </Demo>
            </Section>

            <Section title="use_color_field_state">
                <p>
                    "Tracks two values: the text in the field and the committed "<Code inline=true>"RGB8"</Code>" color. Typing "
                    "only accepts hex digits (at most six, after an optional "<Code inline=true>"#"</Code>"); the text is parsed "
                    "when it is committed, as "<Code inline=true>"#RRGGBB"</Code>" or the short form "<Code inline=true>"#RGB"</Code>
                    ". Text that doesn\u{2019}t parse reverts to the last color; an empty field commits as no color. The state "
                    "owns the color, starting at "<Code inline=true>"default_value"</Code>", unless you bind it to app state "
                    "with "<Code inline=true>"value"</Code>"."
                </p>

                <Section title="Input" id="use-color-field-state-input">
                    <p>"Start from "<Code inline=true>"UseColorFieldStateInput::default()"</Code>" and change single fields:"</p>
                    <ApiTable kind=ApiKind::Input of="UseColorFieldStateInput">
                        <ApiRow name="default_value" ty="Option<RGB8>" default="None">"The initial color; "<Code inline=true>"None"</Code>" starts with an empty field."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<RGB8>>>" default="None">
                            "The color as app state, replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the field invalid, whatever the validation says."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<RGB8>>>" default="None">
                            "Validates the committed color: "<Code inline=true>"Err"</Code>" with the error messages."
                        </ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="ValidationBehavior::Aria">
                            "Show errors as you edit ("<Code inline=true>"Aria"</Code>") or on form submission ("
                            <Code inline=true>"Native"</Code>"), see "<Link href=routes::doc::Forms.materialize()>"Forms & Validation"</Link>"."
                        </ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The field\u{2019}s name, to match server errors."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<RGB8>>>" default="None">"Called with the committed color."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-field-state-return">
                    <p>"A "<Code inline=true>"Copy"</Code>" "<Code inline=true>"ColorFieldState"</Code>":"</p>
                    <ApiTable kind=ApiKind::Return of="ColorFieldState">
                        <ApiRow name="input_value" ty="Signal<String>">"The text in the field."</ApiRow>
                        <ApiRow name="color_value" ty="Signal<Option<RGB8>>">"The committed color; "<Code inline=true>"None"</Code>" for an empty field."</ApiRow>
                        <ApiRow name="validation" ty="UseFormValidationStateReturn">"The validation state."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior">"When errors show."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell><Code inline=true>"set_color_value(color)"</Code></TableCell>
                            <TableCell>"Sets the committed color; the text follows."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"set_input_value(text)"</Code></TableCell>
                            <TableCell>"Sets the text without parsing it; check it with "<Code inline=true>"validate"</Code>" first."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"validate(text)"</Code></TableCell>
                            <TableCell>"Whether "<Code inline=true>"text"</Code>" may be typed: empty, or up to six hex digits after an optional "<Code inline=true>"#"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"commit()"</Code></TableCell>
                            <TableCell>
                                "Parses the text and commits the color, or reverts the text. "<Code inline=true>"use_color_field"</Code>
                                " calls it on blur and "<Keys keys="Enter"/>"."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment()"</Code>", "<Code inline=true>"decrement()"</Code></TableCell>
                            <TableCell>"Change the color\u{2019}s hex value by one."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"increment_to_max()"</Code>", "<Code inline=true>"decrement_to_min()"</Code></TableCell>
                            <TableCell>"Set the color to "<Code inline=true>"#FFFFFF"</Code>" or "<Code inline=true>"#000000"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"default_color_value()"</Code></TableCell>
                            <TableCell>"The color the field started with; a form reset restores it."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_color_field">
                <p>
                    "Makes an input a "<Link href=format!("{}#use-text-field", routes::doc::text_field::Hook.materialize())>"text field"</Link>
                    " for hex codes: it filters what you type, commits on blur and "<Keys keys="Enter"/>", and steps the "
                    "color with the keys of a spin button and the scroll wheel while focused. The input stays a "
                    <Code inline=true>"textbox"</Code>"."
                </p>

                <Section title="Input" id="use-color-field-input">
                    <p>"Pass a "<Code inline=true>"UseColorFieldInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseColorFieldInput">
                        <ApiRow name="state" ty="ColorFieldState">
                            "The state from "<Code inline=true>"use_color_field_state"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The input\u{2019}s id; generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether you render a visible label with "<Code inline=true>"label_props"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the input when it has no visible label."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the input."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"The ids of the elements describing the input."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the field is disabled."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Whether the field is read-only: focusable, but neither typing nor stepping changes it."</ApiRow>
                        <ApiRow name="is_required" ty="Signal<bool>" default="false">"Whether a color is required."</ApiRow>
                        <ApiRow name="placeholder" ty="MaybeProp<String>" default="None">"The input\u{2019}s placeholder."</ApiRow>
                        <ApiRow name="auto_focus" ty="bool" default="false">"Focus the input when it mounts."</ApiRow>
                        <ApiRow name="is_wheel_disabled" ty="bool" default="false">"Whether the scroll wheel leaves the color alone."</ApiRow>
                        <ApiRow name="on_focus, on_blur" ty="Option<Callback<FocusEvent>>" default="None">"Called when the input gains or loses focus."</ApiRow>
                        <ApiRow name="on_focus_change" ty="Option<Callback<bool>>" default="None">"Called with whether the input has focus."</ApiRow>
                        <ApiRow name="on_key_down, on_key_up" ty="Option<Callback<KeyboardEventWrapper>>" default="None">"Called on key presses in the input."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-field-return">
                    <ApiTable kind=ApiKind::Return of="UseColorFieldReturn">
                        <ApiRow name="label_props" ty="UseLabelProps">"For a visible label."</ApiRow>
                        <ApiRow name="input_props" ty="UseColorFieldInputProps">
                            "For the "<Code inline=true>"<input>"</Code>": the text, "<Code inline=true>"disabled"</Code>", "
                            <Code inline=true>"readonly"</Code>", the ARIA attributes of a text field, and the input, keyboard, "
                            "focus and wheel handlers."
                        </ApiRow>
                        <ApiRow name="description_props, error_message_props" ty="SlotProps">"For a description and an error message."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The input, once rendered."</ApiRow>
                        <ApiRow name="is_focused, is_focus_visible" ty="Signal<bool>">"Whether the input has focus, and whether to show a focus ring."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>">"Whether the displayed validation result is invalid."</ApiRow>
                        <ApiRow name="validation_errors" ty="Signal<Vec<String>>">"The displayed error messages."</ApiRow>
                        <ApiRow name="validation_details" ty="Signal<ValidityStateSnapshot>">"Detailed validity state."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_color_channel_field_state">
                <p>
                    "Holds the color and a "<Link href=format!("{}#use-number-field-state", routes::doc::number_field::Hook.materialize())>"number field state"</Link>
                    " of one channel, with the channel\u{2019}s range, step and formatting. "<Code inline=true>"C"</Code>" is any "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())>
                        <Code inline=true>"ColorValue"</Code>
                    </Link>". Typing a value into an empty field starts from the darkest color of the type."
                </p>

                <Section title="Input" id="use-color-channel-field-state-input">
                    <p>"Pass a "<Code inline=true>"UseColorChannelFieldStateInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                    <ApiTable kind=ApiKind::Input of="UseColorChannelFieldStateInput">
                        <ApiRow name="default_value" ty="Option<C>" default="None">"The initial color; "<Code inline=true>"None"</Code>" starts with an empty field."</ApiRow>
                        <ApiRow name="value" ty="Option<ValueBinding<Option<C>>>" default="None">
                            "The color as app state, replacing "<Code inline=true>"default_value"</Code>"."
                        </ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the field edits. Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Whether the field is disabled."</ApiRow>
                        <ApiRow name="is_read_only" ty="Signal<bool>" default="false">"Whether the field is read-only."</ApiRow>
                        <ApiRow name="is_invalid" ty="Signal<bool>" default="false">"Marks the field invalid, whatever the validation says."</ApiRow>
                        <ApiRow name="validate" ty="Option<ValidateFn<Option<f64>>>" default="None">"Validates the channel\u{2019}s value."</ApiRow>
                        <ApiRow name="validation_behavior" ty="ValidationBehavior" default="ValidationBehavior::Aria">"Show errors as you edit or on form submission."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The field\u{2019}s name, to match server errors."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Option<C>>>" default="None">"Called with the color when the channel\u{2019}s value is committed."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-channel-field-state-return">
                    <p>"A "<Code inline=true>"Copy"</Code>" "<Code inline=true>"ColorChannelFieldState<C>"</Code>":"</p>
                    <ApiTable kind=ApiKind::Return of="ColorChannelFieldState">
                        <ApiRow name="color_value" ty="Signal<Option<C>>">"The color."</ApiRow>
                        <ApiRow name="channel" ty="C::Channel">"The channel the field edits."</ApiRow>
                        <ApiRow name="number" ty="NumberFieldState<f64>">"The number field of the channel\u{2019}s value."</ApiRow>
                    </ApiTable>
                    <p>
                        "Its methods: "<Code inline=true>"set_color_value(color)"</Code>" sets the color, and "
                        <Code inline=true>"default_color_value()"</Code>" returns the color a form reset restores."
                    </p>
                </Section>
            </Section>

            <Section title="use_color_channel_field">
                <p>
                    "Calls "<Link href=format!("{}#use-number-field", routes::doc::number_field::Hook.materialize())>"use_number_field"</Link>
                    " with the channel\u{2019}s number field state. Without a label, the field is named after the channel "
                    "(e.g. \u{201c}Hue\u{201d}, \u{201c}Red\u{201d})."
                </p>

                <Section title="Input" id="use-color-channel-field-input">
                    <p>"Pass a "<Code inline=true>"UseColorChannelFieldInput"</Code>" with both fields named."</p>
                    <ApiTable kind=ApiKind::Input of="UseColorChannelFieldInput">
                        <ApiRow name="state" ty="ColorChannelFieldState<C>">
                            "The state from "<Code inline=true>"use_color_channel_field_state"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="field" ty="UseNumberFieldInput<f64>">
                            "Required. The number field\u{2019}s other settings (label, placeholder, focus callbacks, \u{2026}). Its "
                            <Code inline=true>"state"</Code>" is replaced by the channel\u{2019}s, so pass "
                            <Code inline=true>"state.number"</Code>"."
                        </ApiRow>

                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-channel-field-return">
                    <p>
                        "The "<Code inline=true>"UseNumberFieldReturn"</Code>" of "
                        <Link href=format!("{}#use-number-field", routes::doc::number_field::Hook.materialize())>"use_number_field"</Link>
                        ":"
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
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></li>
                <li><Link href=routes::doc::color_field::Atom.materialize()>"Color Field Atoms"</Link></li>
                <li><Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link></li>
                <li><Link href=routes::doc::color_slider::Hook.materialize()>"Color Slider Hooks"</Link></li>
                <li><Link href=routes::doc::color_swatch::Hook.materialize()>"use_color_swatch"</Link></li>
                <li><Link href=routes::doc::number_field::Hook.materialize()>"Number Field Hooks"</Link></li>
                <li><Link href=routes::doc::text_field::Hook.materialize()>"Text Field Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
