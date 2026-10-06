use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_wheel::ColorWheelAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomColorWheel() -> impl IntoView {
    view! {
        <DocPage title="Color Wheel Atoms">
            <p>
                "The "<Code inline=true>"ColorWheel"</Code>", "<Code inline=true>"ColorWheelTrack"</Code>" and "
                <Code inline=true>"ColorThumb"</Code>" atoms render an unstyled ring of hues with a thumb on it. See the "
                <Link href=routes::doc::ColorWheel.materialize()>"Color Wheel overview"</Link>" for the concept and its "
                "keyboard interaction."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ColorWheel"</Code>" calls "
                    <Link href=format!("{}#use-color-wheel-state", routes::doc::color_wheel::Hook.materialize())>"use_color_wheel_state"</Link>
                    " and "
                    <Link href=format!("{}#use-color-wheel", routes::doc::color_wheel::Hook.materialize())>"use_color_wheel"</Link>
                    "; "<Code inline=true>"ColorThumb"</Code>" adds "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" for its data attributes."
                </p>
            </Section>

            <Section title="Example">
                <p>"A wheel for the hue of a color, bound to a signal:"</p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::prelude::*,
                            utils::color::{HSV, HsvChannel},
                        };
                        use leptos::prelude::*;

                        let color = RwSignal::new(HSV { hue: 210.0, saturation: 1.0, value: 1.0 });

                        view! {
                            <ColorWheel
                                channel=HsvChannel::Hue
                                outer_radius=100.0
                                inner_radius=74.0
                                value=color
                                set_value=color
                            >
                                <ColorWheelTrack/>
                                <ColorThumb classes="my-color-thumb"/>
                            </ColorWheel>
                        }
                    "#)}
                </Code>
                <p>
                    "The radii size the wheel: the track is "<Code inline=true>"2 * outer_radius"</Code>" pixels wide. The "
                    "channel determines the color type ("<Code inline=true>"HsvChannel::Hue"</Code>": an HSV wheel)."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Drag the thumb, click anywhere on the ring, or tab to the thumb and use the arrow keys. The wheel owns "
                    "its color here ("<Code inline=true>"default_value"</Code>"): the swatch follows "<Code inline=true>"on_change"</Code>
                    ", the committed hue updates when a drag or a key press ends ("<Code inline=true>"on_change_end"</Code>")."
                </p>
                <Demo
                    description="ColorWheel changing the hue of a blue, with a ColorSwatch of the result and a disabled toggle"
                    source=include_str!("demos/color_wheel.rs")
                >
                    <ColorWheelAtomDemo/>
                </Demo>
            </Section>

            <Section title="ColorWheel">
                <p>
                    "Renders a "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"position: relative"</Code>" around "
                    "the track and the thumb. Bind the color with "<Code inline=true>"value"</Code>" and "
                    <Code inline=true>"set_value"</Code>", or let the wheel own it from "<Code inline=true>"default_value"</Code>
                    " and listen with "<Code inline=true>"on_change"</Code>". It is generic over the channel type "
                    <Code inline=true>"Ch"</Code>": pass the hue channel of "<Code inline=true>"HSV"</Code>" or "
                    <Code inline=true>"HSL"</Code>" (see "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())>
                        <Code inline=true>"ColorValue"</Code>
                    </Link>"), and the color type "<Code inline=true>"Ch::Color"</Code>" follows from it. The wheel keeps the "
                    "other channels as they are. Inside a "<Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>
                    " atom, leave out "<Code inline=true>"value"</Code>": the wheel changes the picker\u{2019}s color."
                </p>

                <Section title="Props" id="color-wheel-props">
                    <ApiTable kind=ApiKind::Props of="ColorWheel">
                        <ApiRow name="channel" ty="Ch">"The color type\u{2019}s hue channel. Required."</ApiRow>
                        <ApiRow name="outer_radius" ty="f64">"The ring\u{2019}s outer radius, in pixels. Required."</ApiRow>
                        <ApiRow name="inner_radius" ty="f64">"The ring\u{2019}s inner radius, in pixels. Required."</ApiRow>
                        <ApiRow name="value" ty="Option<Signal<Ch::Color>>" default="None">"The color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color of the "<Code inline=true>"ColorPicker"</Code>" around it, if any."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<Ch::Color>>" default="None">"Receives the new color: an "<Code inline=true>"RwSignal"</Code>", a closure, a "<Code inline=true>"Callback"</Code>", \u{2026}"</ApiRow>
                        <ApiRow name="default_value" ty="Option<Ch::Color>" default="Ch::Color::default()">"The initial color when "<Code inline=true>"value"</Code>" isn\u{2019}t set."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<Ch::Color>>" default="None">"Called with the color whenever it changes, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<Ch::Color>>" default="None">"Called with the color when a drag or a key press ends."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables pointer and keyboard interaction and the input."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Names the wheel. Without it or "<Code inline=true>"aria_labelledby"</Code>", the channel\u{2019}s name does."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the wheel."</ApiRow>
                        <ApiRow name="name" ty="Option<String>" default="None">"The form field name of the input; its value is the hue."</ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of a form the input belongs to."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the wrapper."</ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"ColorWheelTrack"</Code>" and the "<Code inline=true>"ColorThumb"</Code>". Required."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorWheelTrack">
                <p>
                    "The ring: a "<Code inline=true>"<div>"</Code>" sized from the radii, with a conic gradient of the hues and "
                    "a clip path that cuts out the hole. A press on it moves the thumb there. It panics outside a "
                    <Code inline=true>"ColorWheel"</Code>" and as a second track."
                </p>

                <Section title="Props" id="color-wheel-track-props">
                    <ApiTable kind=ApiKind::Props of="ColorWheelTrack">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the track."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorThumb">
                <p>
                    "The thumb, next to the track: a "<Code inline=true>"<div>"</Code>" positioned on the middle of the ring at "
                    "the hue, filled with the hue at full saturation and brightness, containing the visually hidden "
                    <Code inline=true>"<input type=\"range\">"</Code>" that takes the focus. It is the same atom as in a "
                    <Link href=format!("{}#colorthumb", routes::doc::color_area::Atom.materialize())>"color area"</Link>" and a "
                    <Link href=format!("{}#colorthumb", routes::doc::color_slider::Atom.materialize())>"color slider"</Link>
                    ", with the props "<Code inline=true>"classes"</Code>", "<Code inline=true>"styles"</Code>" and optional "
                    <Code inline=true>"children"</Code>"."
                </p>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-disabled" ty="true">"On the wheel, the track and the thumb: the wheel is disabled."</ApiRow>
                    <ApiRow name="data-dragging" ty="true">"On the thumb: it is being dragged."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"On the thumb: its input has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"On the thumb: it has keyboard focus."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"On the thumb: the pointer is over it."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms set the ring\u{2019}s size, gradient and shape and the thumb\u{2019}s position and fill, and keep "
                    "their colors in Windows high contrast mode. The thumb\u{2019}s size, border and states are yours:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-color-thumb { width: 20px; height: 20px; border: 2px solid var(--surface); border-radius: 50%; }
                        .my-color-thumb[data-dragging] { width: 24px; height: 24px; }
                        .my-color-thumb[data-focus-visible] { outline: 2px solid var(--focus); }
                    ")}
                </Code>
            </Section>

            <Section title="Composition">
                <p>
                    "A wheel only changes the hue: pair it with a "<Link href=routes::doc::color_area::Atom.materialize()>"ColorArea"</Link>
                    " for saturation and brightness, bound to the same color, and show the result with a "
                    <Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorWheel.materialize()>"Color Wheel"</Link></li>
                <li><Link href=routes::doc::color_wheel::Hook.materialize()>"Color Wheel Hooks"</Link></li>
                <li><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></li>
                <li><Link href=routes::doc::color_slider::Atom.materialize()>"Color Slider Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
