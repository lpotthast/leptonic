use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_picker::ColorPickerAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomColorPicker() -> impl IntoView {
    view! {
        <DocPage title="Color Picker Atom">
            <p>
                "The "<Code inline=true>"ColorPicker"</Code>" atom shares one color among the color atoms inside it, so that "
                "they form a picker of your own design. It renders no element. See the "
                <Link href=routes::doc::ColorPicker.materialize()>"Color Picker overview"</Link>" for the concept."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::color_picker::Hook.materialize()><Code inline=true>"use_color_picker_state"</Code></Link>
                    ", for the shared color. The atoms inside bring their own hooks."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "Put color atoms inside the picker and leave out their "<Code inline=true>"value"</Code>": each one shows "
                    "and changes the picker\u{2019}s color. Name the color space of a "<Code inline=true>"ColorArea<HSV>"</Code>
                    ", whose channels are optional; sliders, wheels and channel fields take it from their channel, and "
                    "swatches show any color."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::prelude::*,
                            utils::color::{Color, HSV, HsvChannel, RGB8},
                        };
                        use leptos::prelude::*;

                        let color = RwSignal::new(Color::from(HSV { hue: 210.0, saturation: 0.6, value: 0.8 }));

                        view! {
                            <ColorPicker value=color set_value=color>
                                <ColorArea<HSV> x_channel=HsvChannel::Saturation y_channel=HsvChannel::Brightness aria_label="Color">
                                    <ColorThumb/>
                                </ColorArea<HSV>>
                                <ColorSlider channel=HsvChannel::Hue>
                                    <ColorSliderTrack><ColorThumb/></ColorSliderTrack>
                                </ColorSlider>
                                <ColorSwatch/>
                            </ColorPicker>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "An HSV area, a hue slider, a hex field, a swatch and preset swatches, all on one color: change it with "
                    "any of them, and the others follow."
                </p>
                <Demo
                    description="ColorArea, ColorSlider, ColorField, ColorSwatch and ColorSwatchPicker atoms sharing one color"
                    source=include_str!("demos/color_picker.rs")
                >
                    <ColorPickerAtomDemo/>
                </Demo>
            </Section>

            <Section title="ColorPicker">
                <p>
                    "Holds a "<Link href=format!("{}#color", routes::doc::Color.materialize())><Code inline=true>"Color"</Code></Link>
                    ", which keeps the color space it was set in: a gray set by an HSV area keeps its hue, which an RGB "
                    "field couldn\u{2019}t tell. Each atom reads it in its own space. Bind it with "<Code inline=true>"value"</Code>
                    " and "<Code inline=true>"set_value"</Code>", or let the picker own it from "<Code inline=true>"default_value"</Code>
                    " and listen with "<Code inline=true>"on_change"</Code>"."
                </p>
                <Section title="Props" id="color-picker-props">
                    <ApiTable kind=ApiKind::Props of="atoms::color_picker::ColorPicker">
                        <ApiRow name="default_value" ty="Option<Color>" default="None">
                            "The initial color, unless "<Code inline=true>"value"</Code>" is set; "<Code inline=true>"None"</Code>": black."
                        </ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Color>>" default="None">"The color (controlled): a value or any signal."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Color>>" default="None">
                            "Receives the new color: an "<Code inline=true>"RwSignal"</Code>", "<Code inline=true>"WriteSignal"</Code>
                            ", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Color>>" default="None">"Called with every new color."</ApiRow>
                        <ApiRow name="children" ty="Children">"Required. The color atoms, and any other content."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>"The atom renders no element, so it has none. Style the atoms inside through theirs."</p>
            </Section>

            <Section title="Composition">
                <p>"These atoms show and change the picker\u{2019}s color when they have no "<Code inline=true>"value"</Code>" of their own:"</p>
                <ul>
                    <li><Link href=routes::doc::color_area::Atom.materialize()><Code inline=true>"ColorArea"</Code></Link></li>
                    <li><Link href=routes::doc::color_slider::Atom.materialize()><Code inline=true>"ColorSlider"</Code></Link></li>
                    <li><Link href=routes::doc::color_wheel::Atom.materialize()><Code inline=true>"ColorWheel"</Code></Link></li>
                    <li>
                        <Link href=routes::doc::color_field::Atom.materialize()><Code inline=true>"ColorField"</Code></Link>" and "
                        <Code inline=true>"ColorChannelField"</Code>" (emptying them leaves the color as it is)"
                    </li>
                    <li>
                        <Link href=routes::doc::color_swatch::Atom.materialize()><Code inline=true>"ColorSwatch"</Code></Link>
                        " (without "<Code inline=true>"color"</Code>")"
                    </li>
                    <li><Link href=routes::doc::ColorSwatchPicker.materialize()><Code inline=true>"ColorSwatchPicker"</Code></Link></li>
                </ul>
                <p>
                    "The themed "<Link href=routes::doc::color_picker::Component.materialize()>"ColorPicker component"</Link>
                    " is this atom with themed parts. Atoms of your own read the picker with "
                    <Code inline=true>"use_context::<ColorPickerContext>()"</Code>": its "<Code inline=true>"ColorPickerState"</Code>
                    " has the color and "<Code inline=true>"set_color"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorPicker.materialize()>"Color Picker"</Link></li>
                <li><Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link></li>
                <li><Link href=routes::doc::color_picker::Component.materialize()>"Color Picker Components"</Link></li>
                <li><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></li>
                <li><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
