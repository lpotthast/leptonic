use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageUseColorHooks() -> impl IntoView {
    view! {
        <DocPage title="Color Hooks">
            <p>
                "The color hooks are the building blocks of custom color pickers: 2D areas, channel sliders, hue wheels, "
                "text fields and swatches. Most of them pair a state hook, which owns the color, with a behavior hook, "
                "which adds interaction and ARIA. See the "<Link href=routes::doc::Color.materialize()>"Color overview"</Link>
                " for concept guidance."
            </p>

            <p>
                "They are based on "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useColorArea.html" target=LinkTarget::_Blank>
                    "react-aria\u{2019}s color hooks"
                </LinkExt>
                " from the @react-aria/color and @react-stately/color packages."
            </p>

            <Section title="Hook Families">
                <DocTable headers=&["Hooks", "Purpose"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link></TableCell>
                        <TableCell>
                            "2D gradient area for adjusting two channels at once (e.g. saturation and brightness). Composes "
                            <Code inline=true>"use_move"</Code>" with a center constraint."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorSlider.materialize()>"use_color_slider"</Link></TableCell>
                        <TableCell>"Linear slider for a single channel. Wraps "<Code inline=true>"use_slider"</Code>" with a gradient track and color-aware ARIA."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorWheel.materialize()>"use_color_wheel"</Link></TableCell>
                        <TableCell>"Circular hue wheel (0\u{00b0}\u{2013}360\u{00b0}): pointer input within the ring, conic gradient, polar thumb position."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorField.materialize()>"use_color_field"</Link></TableCell>
                        <TableCell>"Text input for hex color values ("<Code inline=true>"#RRGGBB"</Code>"). Validates while typing, commits on blur."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorChannelField.materialize()>"use_color_channel_field"</Link></TableCell>
                        <TableCell>"Numeric input for a single channel value, built on the number field hooks."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::hooks::UseColorSwatch.materialize()>"use_color_swatch"</Link></TableCell>
                        <TableCell>"Display-only color preview with "<Code inline=true>"role=\"img\""</Code>" and an accessible color name."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><a href="#use-color-picker-state">"use_color_picker_state"</a></TableCell>
                        <TableCell>"State only: one color and a setter, to share between the parts of a picker."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="The ColorValue Trait">
                <p>
                    "The color hooks are generic over the "<Code inline=true>"ColorValue"</Code>" trait, except "
                    <Code inline=true>"use_color_field"</Code>", which edits an "<Code inline=true>"RGB8"</Code>
                    " hex value. Each color type brings its own "<Code inline=true>"Channel"</Code>" type, so you can\u{2019}t "
                    "accidentally ask an RGB color for its hue."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r"
                        pub trait ColorValue: Clone + Copy + PartialEq + Debug + Send + Sync + 'static {
                            type Channel: Debug + Clone + Copy + PartialEq + Eq + Hash + Send + Sync + 'static;

                            fn get_channel_value(&self, channel: Self::Channel) -> f64;
                            fn with_channel_value(&self, channel: Self::Channel, value: f64) -> Self;
                            fn get_channel_range(channel: Self::Channel) -> ColorChannelRange;
                            fn channels() -> &'static [Self::Channel];
                            fn to_css_string(&self) -> String;
                            // ...
                        }
                    ")}
                </Code>

                <p>"Three implementations are provided, and they convert into each other with "<Code inline=true>"From"</Code>":"</p>

                <DocTable headers=&["Type", "Channels"]>
                    <TableRow><TableCell><Code inline=true>"HSV"</Code></TableCell><TableCell><Code inline=true>"HsvChannel"</Code>": Hue, Saturation, Brightness"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"HSL"</Code></TableCell><TableCell><Code inline=true>"HslChannel"</Code>": Hue, Saturation, Lightness"</TableCell></TableRow>
                    <TableRow><TableCell><Code inline=true>"RGB8"</Code></TableCell><TableCell><Code inline=true>"RgbChannel"</Code>": Red, Green, Blue"</TableCell></TableRow>
                </DocTable>
            </Section>

            <Section title="Usage Pattern">
                <p>
                    "Create the state, pass it to the behavior hook, and spread the returned props onto your elements. "
                    "The hook pages show complete demos, including styling."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
                        // 1. Create the state.
                        let state = use_color_area_state(UseColorAreaStateInput {
                            default_value: HSV::new(),
                            x_channel: HsvChannel::Saturation,
                            y_channel: HsvChannel::Brightness,
                            x_channel_step: None,
                            y_channel_step: None,
                            on_change: None,
                            on_change_end: None,
                        });

                        // 2. Add behavior and ARIA.
                        let area = use_color_area(UseColorAreaInput {
                            state,
                            is_disabled: Signal::stored(false),
                            aria_label: Some("Color"),
                            x_name: None,
                            y_name: None,
                            form: None,
                        });

                        // 3. Spread the props onto your elements.
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
            </Section>

            <Section title="use_color_picker_state">
                <p>
                    "The simplest color state: one color and a setter. Use it as the single source of truth of a picker and "
                    "keep the parts\u{2019} own states in sync with it. Takes its input by reference."
                </p>

                <Section title="Input" id="use-color-picker-state-input">
                    <ApiTable kind=ApiKind::Input of="UseColorPickerStateInput">
                        <ApiRow name="default_value" ty="C">"The initial color."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>">"Called when the color changes."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-color-picker-state-return">
                    <ApiTable kind=ApiKind::Return of="UseColorPickerStateReturn">
                        <ApiRow name="color" ty="Signal<C>">"The current color."</ApiRow>
                        <ApiRow name="set_color" ty="Callback<C>">"Sets the color and calls "<Code inline=true>"on_change"</Code>"."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Component.materialize()>"ColorPicker component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
