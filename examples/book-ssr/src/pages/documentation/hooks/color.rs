use leptos::prelude::*;

use super::demos::color_picker_state::ColorPickerStateDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseColorHooks() -> impl IntoView {
    view! {
        <DocPage title="use_color_picker_state">
            <p>
                "The "<Code inline=true>"use_color_picker_state"</Code>" hook holds the one color that the parts of a color "
                "picker share. The "<Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>" atom uses "
                "it for the atoms inside it; call it yourself for parts built from hooks. See the "
                <Link href=routes::doc::ColorPicker.materialize()>"Color Picker overview"</Link>" for the concept."
            </p>

            <ReactAriaSource path="color/useColorPickerState.ts" package=UpstreamPackage::ReactStately/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseColorPickerStateInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ". The shared color is a "<Link href=format!("{}#color", routes::doc::Color.materialize())>"Color"</Link>
                    ", which keeps the color space it was set in."
                </p>
                <ApiTable kind=ApiKind::Input of="UseColorPickerStateInput">
                    <ApiRow name="default_value" ty="Color" default="black">"The initial color. Ignored with "<Code inline=true>"value"</Code>"."</ApiRow>
                    <ApiRow name="value" ty="Option<ValueBinding<Color>>" default="None">"The color as app state, replacing "<Code inline=true>"default_value"</Code>"."</ApiRow>
                    <ApiRow name="on_change" ty="Option<Callback<Color>>" default="None">"Called with every new color."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <p>
                    "A "<Code inline=true>"Copy"</Code>" "<Code inline=true>"ColorPickerState"</Code>". "
                    <Code inline=true>"set_color(color)"</Code>" sets the color and calls "<Code inline=true>"on_change"</Code>"."
                </p>
                <ApiTable kind=ApiKind::Fields of="ColorPickerState">
                    <ApiRow name="color" ty="Signal<Color>">"The color, in the space it was last set in."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <p>
                    "Use the picker state as the single source of truth. Bind each interactive part to it with "
                    <Code inline=true>"value"</Code>", converting the shared "<Code inline=true>"Color"</Code>" into the "
                    "part\u{2019}s color space with "<Code inline=true>"Color::to"</Code>". Parts that only show the color, like a "
                    <Link href=routes::doc::color_swatch::Hook.materialize()>"swatch"</Link>", read it directly."
                </p>

                <Demo
                    description="A color area and a hue slider edit one picker color, a swatch shows it, and a button sets it from code"
                    source=include_str!("demos/color_picker_state.rs")
                >
                    <ColorPickerStateDemo/>
                </Demo>
            </Section>

            <Section title="Hooks of the Parts">
                <p>
                    "Each part of a picker is a concept of its own, with a state hook that owns the color and a hook that "
                    "adds interaction and ARIA. Their pages show how to render and style them."
                </p>
                <DocTable headers=&["Part", "Hooks"]>
                    <TableRow>
                        <TableCell><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-color-area-state", routes::doc::color_area::Hook.materialize())>
                                <Code inline=true>"use_color_area_state"</Code>
                            </Link>", "
                            <Link href=format!("{}#use-color-area", routes::doc::color_area::Hook.materialize())>
                                <Code inline=true>"use_color_area"</Code>
                            </Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::ColorField.materialize()>"Color Field"</Link></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-color-field-state", routes::doc::color_field::Hook.materialize())>
                                <Code inline=true>"use_color_field_state"</Code>
                            </Link>", "
                            <Link href=format!("{}#use-color-field", routes::doc::color_field::Hook.materialize())>
                                <Code inline=true>"use_color_field"</Code>
                            </Link>", "
                            <Link href=format!("{}#use-color-channel-field-state", routes::doc::color_field::Hook.materialize())>
                                <Code inline=true>"use_color_channel_field_state"</Code>
                            </Link>", "
                            <Link href=format!("{}#use-color-channel-field", routes::doc::color_field::Hook.materialize())>
                                <Code inline=true>"use_color_channel_field"</Code>
                            </Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-color-slider-state", routes::doc::color_slider::Hook.materialize())>
                                <Code inline=true>"use_color_slider_state"</Code>
                            </Link>", "
                            <Link href=format!("{}#use-color-slider", routes::doc::color_slider::Hook.materialize())>
                                <Code inline=true>"use_color_slider"</Code>
                            </Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch"</Link></TableCell>
                        <TableCell>
                            <Link href=routes::doc::color_swatch::Hook.materialize()><Code inline=true>"use_color_swatch"</Code></Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-color-wheel-state", routes::doc::color_wheel::Hook.materialize())>
                                <Code inline=true>"use_color_wheel_state"</Code>
                            </Link>", "
                            <Link href=format!("{}#use-color-wheel", routes::doc::color_wheel::Hook.materialize())>
                                <Code inline=true>"use_color_wheel"</Code>
                            </Link>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
                <li><Link href=routes::doc::color_picker::Atom.materialize()>"Color Picker Atom"</Link></li>
                <li><Link href=routes::doc::color_area::Hook.materialize()>"Color Area Hooks"</Link></li>
                <li><Link href=routes::doc::color_wheel::Hook.materialize()>"Color Wheel Hooks"</Link></li>
                <li><Link href=routes::doc::color_swatch::Hook.materialize()>"use_color_swatch"</Link></li>
                <li><Link href=routes::doc::Color.materialize()>"Color"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
