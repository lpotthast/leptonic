use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_slider::ColorSliderAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomColorSlider() -> impl IntoView {
    view! {
        <DocPage title="Color Slider Atoms">
            <p>
                "The "<Code inline=true>"ColorSlider"</Code>", "<Code inline=true>"ColorSliderTrack"</Code>", "
                <Code inline=true>"ColorSliderOutput"</Code>" and "<Code inline=true>"ColorThumb"</Code>" atoms render an "
                "unstyled slider that changes one channel of a color. See the "
                <Link href=routes::doc::ColorSlider.materialize()>"Color Slider overview"</Link>" for the concept and its "
                "keyboard interaction."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ColorSlider"</Code>" calls "
                    <Link href=format!("{}#use-color-slider-state", routes::doc::color_slider::Hook.materialize())>"use_color_slider_state"</Link>
                    " and "
                    <Link href=format!("{}#use-color-slider", routes::doc::color_slider::Hook.materialize())>"use_color_slider"</Link>
                    "; "<Code inline=true>"ColorSliderTrack"</Code>" adds "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>", and "
                    <Code inline=true>"ColorThumb"</Code>" adds "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" and "
                    <Code inline=true>"use_hover"</Code>" for its data attributes."
                </p>
            </Section>

            <Section title="Example">
                <p>"A hue slider for a blue, bound to a signal:"</p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::prelude::*,
                            utils::color::{HSV, HsvChannel},
                        };
                        use leptos::prelude::*;

                        let color = RwSignal::new(HSV { hue: 210.0, saturation: 0.6, brightness: 0.8 });

                        view! {
                            <ColorSlider channel=HsvChannel::Hue value=color set_value=color classes="my-color-slider">
                                <Label>"Hue"</Label>
                                <ColorSliderOutput/>
                                <ColorSliderTrack classes="my-color-slider-track">
                                    <ColorThumb classes="my-color-thumb"/>
                                </ColorSliderTrack>
                            </ColorSlider>
                        }
                    "#)}
                </Code>
                <p>
                    "The channel determines the color type: "<Code inline=true>"HsvChannel::Hue"</Code>" makes it an HSV "
                    "slider. Without CSS, the track has no height: give it one (see "<AnchorLink href="#styling">"Styling"</AnchorLink>")."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Three sliders change the hue, saturation and lightness of one color. Each track shows the colors its "
                    "channel goes through with the other two channels as they are; the committed color updates when a "
                    "drag or a key press ends ("<Code inline=true>"on_change_end"</Code>")."
                </p>
                <Demo
                    description="Hue, saturation and lightness sliders sharing one HSL color, with a swatch and a disabled toggle"
                    source=include_str!("demos/color_slider.rs")
                >
                    <ColorSliderAtomDemo/>
                </Demo>
            </Section>

            <Section title="ColorSlider">
                <p>
                    "Renders the slider\u{2019}s "<Code inline=true>"<div>"</Code>" and provides its parts: a "
                    <Link href=routes::doc::field::Atom.materialize()><Code inline=true>"Label"</Code></Link>", a "
                    <Code inline=true>"ColorSliderOutput"</Code>" and a "<Code inline=true>"ColorSliderTrack"</Code>". Bind the "
                    "color with "<Code inline=true>"value"</Code>" and "<Code inline=true>"set_value"</Code>", or let the slider "
                    "own it from "<Code inline=true>"default_value"</Code>" and listen with "<Code inline=true>"on_change"</Code>
                    ". It is generic over the channel type "<Code inline=true>"Ch"</Code>" ("<Code inline=true>"HsvChannel"</Code>", "
                    <Code inline=true>"HslChannel"</Code>" or "<Code inline=true>"RgbChannel"</Code>"); the color type "
                    <Code inline=true>"Ch::Color"</Code>" follows from it (see "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())>
                        <Code inline=true>"ColorValue"</Code>
                    </Link>"). Inside a "<Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>
                    " atom, leave out "<Code inline=true>"value"</Code>": the slider changes the picker\u{2019}s color."
                </p>

                <Section title="Props" id="color-slider-props">
                    <ApiTable kind=ApiKind::Props of="ColorSlider">
                        <ApiRow name="channel" ty="Ch">"The channel the slider changes. Required."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Ch::Color>>" default="None">"The color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color of the "<Code inline=true>"ColorPicker"</Code>" around it, if any."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Ch::Color>>" default="None">"Receives the new color: an "<Code inline=true>"RwSignal"</Code>", a closure, a "<Code inline=true>"Callback"</Code>", \u{2026}"</ApiRow>
                        <ApiRow name="default_value" ty="Option<Ch::Color>" default="Ch::Color::default()">"The initial color when "<Code inline=true>"value"</Code>" isn\u{2019}t set."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Ch::Color>>" default="None">"Called with the color whenever it changes, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<Ch::Color>>" default="None">"Called with the color when a drag or a key press ends."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>" default="Orientation::Horizontal">"The direction of the track."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables pointer and keyboard interaction and the input."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the slider when it has no "<Code inline=true>"Label"</Code>". Without either, the channel\u{2019}s name does."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the slider."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The form field name of the input; its value is the channel\u{2019}s value."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of a form the input belongs to."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the slider element."</ApiRow>
                        <ApiRow name="children" ty="Children">"The label, output and track. Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorSliderTrack">
                <p>
                    "The track: a "<Code inline=true>"<div>"</Code>" with the channel\u{2019}s gradient as background, which "
                    "moves the thumb to a pressed point. It is the slider\u{2019}s group ("<Code inline=true>"role=\"group\""</Code>
                    ", named by the slider\u{2019}s label). Put the "<Code inline=true>"ColorThumb"</Code>" in it. It panics "
                    "outside a "<Code inline=true>"ColorSlider"</Code>" and as a second track."
                </p>

                <Section title="Props" id="color-slider-track-props">
                    <ApiTable kind=ApiKind::Props of="ColorSliderTrack">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the track."</ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"ColorThumb"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorSliderOutput">
                <p>
                    "An "<Code inline=true>"<output>"</Code>" with the channel\u{2019}s formatted value (e.g. "
                    "\u{201c}210\u{00b0}\u{201d}), linked to the input. It panics outside a "<Code inline=true>"ColorSlider"</Code>
                    " and as a second output."
                </p>

                <Section title="Props" id="color-slider-output-props">
                    <ApiTable kind=ApiKind::Props of="ColorSliderOutput">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the output."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorThumb">
                <p>
                    "The thumb, inside the track: a "<Code inline=true>"<div>"</Code>" positioned at the channel\u{2019}s value, "
                    "filled with the color (for a hue, at full saturation and brightness), containing the visually hidden "
                    <Code inline=true>"<input type=\"range\">"</Code>" that takes the focus. It is the same atom as in a "
                    <Link href=format!("{}#colorthumb", routes::doc::color_area::Atom.materialize())>"color area"</Link>" and a "
                    <Link href=format!("{}#colorthumb", routes::doc::color_wheel::Atom.materialize())>"color wheel"</Link>
                    ", with the props "<Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>" and optional "
                    <Code inline=true>"children"</Code>"."
                </p>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-orientation" ty="horizontal | vertical">"On the slider, the track and the output: the orientation."</ApiRow>
                    <ApiRow name="data-disabled" ty="true">"On the slider, the track, the output and the thumb: the slider is disabled."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"On the track and the thumb: the pointer is over it."</ApiRow>
                    <ApiRow name="data-dragging" ty="true">"On the thumb: it is being dragged."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"On the thumb: its input has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"On the thumb: it has keyboard focus."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles beyond the inline ones their function needs. "<Code inline=true>"ColorSlider"</Code>" renders a "<Code inline=true>"<div>"</Code>" "
                    "with the class "<Code inline=true>"leptonic-ColorSlider"</Code>" around its label, output and track; "<Code inline=true>"ColorSliderTrack"</Code>" a "
                    <Code inline=true>"<div>"</Code>" with "<Code inline=true>"leptonic-ColorSliderTrack"</Code>", with its gradient, "<Code inline=true>"position: relative"</Code>" and "
                    <Code inline=true>"touch-action: none"</Code>"; "<Code inline=true>"ColorSliderOutput"</Code>" an "<Code inline=true>"<output>"</Code>" with "<Code inline=true>"leptonic-ColorSliderOutput"</Code>"; "
                    "and the "<Code inline=true>"ColorThumb"</Code>" ("<Code inline=true>"leptonic-ColorThumb"</Code>") gets its position, a "<Code inline=true>"transform"</Code>" centering it "
                    "on the value, and its fill. Your "<Code inline=true>"classes"</Code>" follow the default class. The track and the thumb keep their "
                    "colors in Windows high contrast mode. Lay out the slider, size the track and the thumb, and on a horizontal track "
                    "center the thumb vertically. The demo above uses this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-color-atoms-slider { display: grid; grid-template-columns: 1fr auto; gap: 0.25rem; }
                        .demo-color-atoms-slider-output { color: var(--muted); }
                        .demo-color-atoms-slider-track { grid-column: 1 / -1; height: 24px; border-radius: 4px; }
                        .demo-color-atoms-slider-track[data-disabled] { opacity: 0.5; cursor: not-allowed; }
                        .demo-color-atoms-slider-track > .demo-color-atoms-thumb { top: 50%; }
                        .demo-color-atoms-thumb { box-sizing: border-box; width: 20px; height: 20px; border: 2px solid var(--surface); border-radius: 50%; box-shadow: 0 0 0 1px var(--muted); cursor: grab; }
                        .demo-color-atoms-thumb[data-dragging] { cursor: grabbing; }
                        .demo-color-atoms-thumb[data-focus-visible] { outline: 2px solid var(--focus); outline-offset: 2px; }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "Bind several sliders to one color, as the demo does, to edit its channels one by one, or pair a hue "
                    "slider with a "<Link href=routes::doc::color_area::Atom.materialize()>"ColorArea"</Link>" for the other "
                    "two channels. Show the result with a "
                    <Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorSlider.materialize()>"Color Slider"</Link></li>
                <li><Link href=routes::doc::color_slider::Hook.materialize()>"Color Slider Hooks"</Link></li>
                <li><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></li>
                <li><Link href=routes::doc::color_wheel::Atom.materialize()>"Color Wheel Atoms"</Link></li>
                <li><Link href=routes::doc::slider::Atom.materialize()>"Slider Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
