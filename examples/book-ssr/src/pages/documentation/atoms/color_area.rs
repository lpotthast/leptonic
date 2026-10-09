use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_area::ColorAreaAtomDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomColorArea() -> impl IntoView {
    view! {
        <DocPage title="Color Area Atoms">
            <p>
                "The "<Code inline=true>"ColorArea"</Code>" and "<Code inline=true>"ColorThumb"</Code>" atoms render an "
                "unstyled 2D gradient area for picking two channels of a color at once. See the "
                <Link href=routes::doc::ColorArea.materialize()>"Color Area overview"</Link>" for the concept and its keyboard "
                "interaction."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ColorArea"</Code>" calls "
                    <Link href=format!("{}#use-color-area-state", routes::doc::color_area::Hook.materialize())>"use_color_area_state"</Link>
                    " for the color and "
                    <Link href=format!("{}#use-color-area", routes::doc::color_area::Hook.materialize())>"use_color_area"</Link>
                    " for the markup and interaction; "<Code inline=true>"ColorThumb"</Code>" adds "
                    <Link href=routes::doc::focus::UseFocusRing.materialize()>"use_focus_ring"</Link>" and "
                    <Link href=routes::doc::interactions::UseHover.materialize()>"use_hover"</Link>" for its data attributes."
                </p>
            </Section>

            <Section title="Example">
                <p>"A saturation and brightness area for a blue, bound to a signal:"</p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            HSV,
                            HsvChannel,
                            atoms::{color_area::ColorArea, color_thumb::ColorThumb},
                        };
                        use leptos::prelude::*;

                        let color = RwSignal::new(HSV { hue: 210.0, saturation: 0.6, brightness: 0.8 });

                        view! {
                            <ColorArea
                                value=color
                                set_value=color
                                x_channel=HsvChannel::Saturation
                                y_channel=HsvChannel::Brightness
                                aria_label="Saturation and brightness"
                                classes="my-color-area"
                            >
                                <ColorThumb classes="my-color-thumb"/>
                            </ColorArea>
                        }
                    "#)}
                </Code>
                <p>"Without CSS, the area has no size: give it one (see "<AnchorLink href="#styling">"Styling"</AnchorLink>")."</p>
            </Section>

            <Section title="Demo">
                <p>
                    "Drag in the area, click anywhere in it, or tab to the thumb and use the arrow keys to pick a saturation "
                    "(X) and a brightness (Y) for the hue 210\u{00b0}. The area owns its color here ("
                    <Code inline=true>"default_value"</Code>"): the swatch and the values follow "<Code inline=true>"on_change"</Code>
                    ", the committed color updates when a drag or a key press ends ("<Code inline=true>"on_change_end"</Code>")."
                </p>
                <Demo
                    description="ColorArea picking saturation and brightness, with a named ColorSwatch of the result and a disabled toggle"
                    source=include_str!("demos/color_area.rs")
                >
                    <ColorAreaAtomDemo/>
                </Demo>
            </Section>

            <Section title="ColorArea">
                <p>
                    "Renders the area "<Code inline=true>"<div>"</Code>" with the gradient of the two channels as background. "
                    "Bind the color with "<Code inline=true>"value"</Code>" and "<Code inline=true>"set_value"</Code>", or let "
                    "the area own it from "<Code inline=true>"default_value"</Code>" and listen with "
                    <Code inline=true>"on_change"</Code>". The color type "<Code inline=true>"C"</Code>" ("
                    <Code inline=true>"HSV"</Code>", "<Code inline=true>"HSL"</Code>" or "<Code inline=true>"RGB8"</Code>", see "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())>
                        <Code inline=true>"ColorValue"</Code>
                    </Link>") follows from the color you pass; name it where there is none ("
                    <Code inline=true>"<ColorArea<HSV>>"</Code>"). Inside a "
                    <Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>" atom, leave out "
                    <Code inline=true>"value"</Code>": the area changes the picker\u{2019}s color."
                </p>

                <Section title="Props" id="color-area-props">
                    <ApiTable kind=ApiKind::Props of="ColorArea">
                        <ApiRow name="value" ty="Option<Signal<C>>" default="None">"The color (controlled): a value or any signal. "<Code inline=true>"None"</Code>": the color of the "<Code inline=true>"ColorPicker"</Code>" around it, if any."</ApiRow>
                        <ApiRow name="set_value" ty="Option<Out<C>>" default="None">"Receives the new color: an "<Code inline=true>"RwSignal"</Code>", a closure, a "<Code inline=true>"Callback"</Code>", \u{2026}"</ApiRow>
                        <ApiRow name="default_value" ty="Option<C>" default="white">"The initial color when "<Code inline=true>"value"</Code>" isn\u{2019}t set."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>" default="None">"Called with the color whenever it changes, also while dragging."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>" default="None">"Called with the color when a drag or a key press ends."</ApiRow>
                        <ApiRow name="x_channel" ty="Option<Channel>" default="None">
                            "The channel on the X axis. "<Code inline=true>"None"</Code>": the color space\u{2019}s first axis (saturation for HSV)."
                        </ApiRow>
                        <ApiRow name="y_channel" ty="Option<Channel>" default="None">
                            "The channel on the Y axis, maximum at the top. "<Code inline=true>"None"</Code>": the color space\u{2019}s second axis."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables pointer and keyboard interaction and the inputs."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the area. It has no visible label, so set this or "<Code inline=true>"aria_labelledby"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The ids of the elements naming the area."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"The ids of the elements describing the area."</ApiRow>
                        <ApiRow name="x_name, y_name" ty="Option<String>" default="None">
                            "Form field names of the X and Y inputs. Their values are the channel values (e.g. "
                            <Code inline=true>"0.6"</Code>" for 60% saturation)."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<String>" default="None">"The id of a form the inputs belong to."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the area."</ApiRow>
                        <ApiRow name="children" ty="Children">"The "<Code inline=true>"ColorThumb"</Code>", and anything else to draw on the area. Required."</ApiRow>
                        <ApiRow name="aria_details" ty="Option<String>" default="None">"Ids of elements providing additional details."</ApiRow>
                        <ApiRow name="x_channel_step" ty="Option<f64>" default="None">"Step for the horizontal color channel; uses the channel default when omitted."</ApiRow>
                        <ApiRow name="y_channel_step" ty="Option<f64>" default="None">"Step for the vertical color channel; uses the channel default when omitted."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorThumb">
                <p>
                    "The one thumb of a "<Code inline=true>"ColorArea"</Code>": a "<Code inline=true>"<div>"</Code>" positioned at "
                    "the color and filled with it, containing the two visually hidden "
                    <Code inline=true>"<input type=\"range\">"</Code>" elements that carry the accessibility semantics and the "
                    "form values. The same atom is the thumb of a "
                    <Link href=format!("{}#colorthumb", routes::doc::color_slider::Atom.materialize())>"color slider"</Link>" and a "
                    <Link href=format!("{}#colorthumb", routes::doc::color_wheel::Atom.materialize())>"color wheel"</Link>
                    ", with one input there. It panics outside these atoms and as a second thumb."
                </p>

                <Section title="Props" id="color-thumb-props">
                    <ApiTable kind=ApiKind::Props of="ColorThumb">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the thumb."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"Content of the thumb."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-disabled" ty="true">"On the area and the thumb: the area is disabled."</ApiRow>
                    <ApiRow name="data-dragging" ty="true">"On the thumb: it is being dragged."</ApiRow>
                    <ApiRow name="data-focused" ty="true">"On the thumb: one of its inputs has focus."</ApiRow>
                    <ApiRow name="data-focus-visible" ty="true">"On the thumb: it has keyboard focus."</ApiRow>
                    <ApiRow name="data-hovered" ty="true">"On the thumb: the pointer is over it."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles beyond the inline ones their function needs. "<Code inline=true>"ColorArea"</Code>" renders a "<Code inline=true>"<div>"</Code>" "
                    "with the class "<Code inline=true>"leptonic-ColorArea"</Code>" and sets its gradient, "<Code inline=true>"position: relative"</Code>" and "
                    <Code inline=true>"touch-action: none"</Code>"; "<Code inline=true>"ColorThumb"</Code>" renders a "<Code inline=true>"<div>"</Code>" with "<Code inline=true>"leptonic-ColorThumb"</Code>" around "
                    "the visually hidden inputs and sets its position, a "<Code inline=true>"transform"</Code>" centering it on the color, and the color as "
                    <Code inline=true>"background-color"</Code>". Your "<Code inline=true>"classes"</Code>" follow the default class. Both set "<Code inline=true>"forced-color-adjust: none"</Code>", "
                    "so Windows high contrast mode keeps their colors. The sizes, borders and states are yours; the demo above uses this "
                    "CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-color-atoms-area { width: 200px; height: 200px; max-width: 100%; border-radius: 8px; cursor: crosshair; }
                        .demo-color-atoms-area[data-disabled] { opacity: 0.5; cursor: not-allowed; }
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
                    "Show the picked color next to the area with a "
                    <Link href=routes::doc::color_swatch::Atom.materialize()>"ColorSwatch"</Link>
                    ", as the demo does: a color shown only as a position in a gradient is lost on some users."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorArea.materialize()>"Color Area"</Link></li>
                <li><Link href=routes::doc::color_area::Hook.materialize()>"Color Area Hooks"</Link></li>
                <li><Link href=routes::doc::color_swatch::Atom.materialize()>"Color Swatch Atom"</Link></li>
                <li><Link href=routes::doc::color_slider::Atom.materialize()>"Color Slider Atoms"</Link></li>
                <li><Link href=routes::doc::color_wheel::Atom.materialize()>"Color Wheel Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
