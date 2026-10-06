use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{color_area::ColorAreaAtomDemo, color_swatch::ColorSwatchPaletteDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomColor() -> impl IntoView {
    view! {
        <DocPage title="Color Atoms">
            <p>
                "The color atoms render the accessible markup of the "<Link href=routes::doc::color::Hooks.materialize()>"color hooks"</Link>
                " without any styling: "<Code inline=true>"ColorArea"</Code>" is a 2D gradient area for picking two channels of a "
                "color at once, "<Code inline=true>"ColorSwatch"</Code>" shows a color with an accessible name. See the "
                <Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <ul>
                    <li>
                        <Code inline=true>"ColorArea"</Code>" calls "
                        <Link href=format!("{}#use-color-area-state", routes::doc::hooks::UseColorArea.materialize())>"use_color_area_state"</Link>
                        " for the color and "
                        <Link href=format!("{}#use-color-area", routes::doc::hooks::UseColorArea.materialize())>"use_color_area"</Link>
                        " for the markup and interaction, which moves the thumb with "
                        <Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>"."
                    </li>
                    <li>
                        <Code inline=true>"ColorSwatch"</Code>" calls "
                        <Link href=routes::doc::hooks::UseColorSwatch.materialize()>"use_color_swatch"</Link>"."
                    </li>
                </ul>
                <p>
                    "Both atoms are generic over the color type "<Code inline=true>"C"</Code>": "<Code inline=true>"HSV"</Code>", "
                    <Code inline=true>"HSL"</Code>" or "<Code inline=true>"RGB8"</Code>" (see "
                    <Link href=format!("{}#the-colorvalue-trait", routes::doc::color::Hooks.materialize())>"the ColorValue trait"</Link>
                    "). The type can\u{2019}t be inferred from the props, so name it: "<Code inline=true>"<ColorArea<HSV> \u{2026}/>"</Code>"."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    "The atoms are not part of a prelude; import them from their modules. A saturation/brightness area for a blue, "
                    "with a swatch of the picked color:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{color_area::ColorArea, color_swatch::ColorSwatch},
                            utils::color::{HSV, HsvChannel},
                        };

                        let initial = HSV { hue: 210.0, saturation: 0.6, value: 0.8 };
                        let color = RwSignal::new(initial);

                        view! {
                            <ColorArea<HSV>
                                default_value=initial
                                x_channel=HsvChannel::Saturation
                                y_channel=HsvChannel::Brightness
                                aria_label="Saturation and brightness"
                                on_change=Callback::new(move |c| color.set(c))
                                classes="my-color-area"
                            />
                            <ColorSwatch<HSV> color=color classes="my-swatch"/>
                        }
                    "#)}
                </Code>
                <p>
                    "Without CSS the area has no size, so give it one (see Styling below)."
                </p>
            </Section>

            <Section title="Demo">
                <p>
                    "Drag in the area or click anywhere in it to pick a saturation (X) and a brightness (Y) for the hue "
                    "210\u{00b0}. The swatch and the values follow every change; \u{201c}Committed\u{201d} updates when a drag "
                    "or an arrow key press ends ("<Code inline=true>"on_change_end"</Code>")."
                </p>
                <Demo
                    description="ColorArea picking saturation and brightness, with a ColorSwatch of the result and a disabled toggle"
                    source=include_str!("demos/color_area.rs")
                >
                    <ColorAreaAtomDemo/>
                </Demo>

                <p>"A palette of swatches. Each one is labelled with its "<Code inline=true>"color_name"</Code>"."</p>
                <Demo description="A row of named ColorSwatch atoms" source=include_str!("demos/color_swatch.rs")>
                    <ColorSwatchPaletteDemo/>
                </Demo>
            </Section>

            <Section title="ColorArea">
                <p>
                    "Renders the area "<Code inline=true>"<div>"</Code>" with the gradient of the two channels as background, "
                    "and a thumb "<Code inline=true>"<div>"</Code>" inside it, positioned at the current value and filled with "
                    "the current color. The thumb contains two visually hidden "<Code inline=true>"<input type=\"range\">"</Code>
                    " elements, one per channel, which carry the accessibility semantics and the form values."
                </p>
                <p>
                    "The area owns its color: "<Code inline=true>"default_value"</Code>" sets it once, "
                    <Code inline=true>"on_change"</Code>" reports changes. The atom can\u{2019}t be changed from outside after it "
                    "mounted and uses the channels\u{2019} default steps; use the "
                    <Link href=routes::doc::hooks::UseColorArea.materialize()>"hooks"</Link>" for a custom step."
                </p>
                <Section title="Props" id="color-area-props">
                    <ApiTable kind=ApiKind::Props of="ColorArea">
                        <ApiRow name="default_value" ty="C">"The initial color. Its third channel (e.g. the hue) stays fixed."</ApiRow>
                        <ApiRow name="x_channel" ty="C::Channel">"The channel on the X axis, e.g. "<Code inline=true>"HsvChannel::Saturation"</Code>"."</ApiRow>
                        <ApiRow name="y_channel" ty="C::Channel">"The channel on the Y axis; its maximum is at the top."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Disables pointer and keyboard interaction and the inputs."</ApiRow>
                        <ApiRow name="on_change" ty="Option<Callback<C>>" default="None">"Called with the new color on every change."</ApiRow>
                        <ApiRow name="on_change_end" ty="Option<Callback<C>>" default="None">
                            "Called with the color when a drag or an arrow key press ends."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="Option<&'static str>" default="None">"Names the area and its inputs."</ApiRow>
                        <ApiRow name="x_name, y_name" ty="Option<&'static str>" default="None">
                            "Form field names of the X and Y inputs. Their values are the channel values (e.g. "
                            <Code inline=true>"0.6"</Code>" for 60% saturation)."
                        </ApiRow>
                        <ApiRow name="form" ty="Option<&'static str>" default="None">"The id of a form the inputs belong to."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the area."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"Content rendered inside the thumb."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ColorSwatch">
                <p>
                    "Renders a "<Code inline=true>"<div>"</Code>" with the color as background, "<Code inline=true>"role=\"img\""</Code>
                    " and an accessible name. The color is a signal, so the swatch follows a changing color."
                </p>
                <Section title="Props" id="color-swatch-props">
                    <ApiTable kind=ApiKind::Props of="ColorSwatch">
                        <ApiRow name="color" ty="Signal<C>">"The color to show."</ApiRow>
                        <ApiRow name="color_name" ty="Option<Signal<String>>" default="None">
                            "The accessible name. "<Code inline=true>"None"</Code>": the CSS color string, e.g. "
                            <Code inline=true>"rgb(30, 110, 200)"</Code>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Replaces the accessible name, including "<Code inline=true>"color_name"</Code>"."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the swatch."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"Content rendered inside the swatch."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "The color atoms render no "<Code inline=true>"data-*"</Code>" attributes. Select their parts and state "
                    "through the attributes they do render:"
                </p>
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="role" ty="group, presentation, img">
                        <Code inline=true>"group"</Code>" on the area, "<Code inline=true>"presentation"</Code>" on its thumb, "
                        <Code inline=true>"img"</Code>" on a swatch."
                    </ApiRow>
                    <ApiRow name="aria-disabled" ty="true">"On the area: it is disabled. The inputs are "<Code inline=true>"disabled"</Code>" then, too."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms set only the inline styles their function needs: the area gets its gradient ("
                    <Code inline=true>"background"</Code>", "<Code inline=true>"background-blend-mode"</Code>"), "
                    <Code inline=true>"position: relative"</Code>", "<Code inline=true>"touch-action: none"</Code>" and "
                    <Code inline=true>"user-select: none"</Code>"; the thumb gets "<Code inline=true>"position: absolute"</Code>", "
                    <Code inline=true>"left"</Code>", "<Code inline=true>"bottom"</Code>", "<Code inline=true>"transform"</Code>
                    " (centering it on the value) and the current color as "<Code inline=true>"background-color"</Code>"; a swatch "
                    "gets its "<Code inline=true>"background-color"</Code>". Both set "<Code inline=true>"forced-color-adjust: none"</Code>
                    ", so Windows high contrast mode keeps the colors."
                </p>
                <p>
                    "Everything else is yours: the size of the area and the swatch, and the thumb\u{2019}s size and border. The "
                    "thumb has no "<Code inline=true>"classes"</Code>" prop; select it as the area\u{2019}s child, or size it "
                    "through "<Code inline=true>"children"</Code>". It contains the focused input, so show the focus with "
                    <Code inline=true>":has(:focus-visible)"</Code>":"
                </p>
                <Code language=Language::Css>
                    {indoc!(r#"
                        .my-color-area {
                            width: 200px;
                            height: 200px;
                            border-radius: 6px;
                        }
                        .my-color-area[aria-disabled="true"] { opacity: 0.5; }

                        .my-color-area > [role="presentation"] {
                            box-sizing: border-box;
                            width: 20px;
                            height: 20px;
                            border: 2px solid white;
                            border-radius: 50%;
                        }
                        .my-color-area > [role="presentation"]:has(:focus-visible) {
                            outline: 2px solid royalblue;
                        }

                        .my-swatch {
                            width: 3em;
                            height: 3em;
                            border-radius: 8px;
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Accessibility">
                <ul>
                    <li>
                        "The area is a "<Code inline=true>"role=\"group\""</Code>" named by "<Code inline=true>"aria_label"</Code>
                        ". Its X input is the only focusable element and is announced as a \u{201c}2D slider\u{201d} whose "
                        <Code inline=true>"aria-valuetext"</Code>" names all three channels, e.g. "
                        "\u{201c}Saturation 60%, Brightness 80%, Hue 210\u{00b0}, azure\u{201d}. The Y input has "
                        <Code inline=true>"tabindex=\"-1\""</Code>" and "<Code inline=true>"aria-hidden=\"true\""</Code>
                        ", so screen readers announce a single control."
                    </li>
                    <li>
                        "Always pass an "<Code inline=true>"aria_label"</Code>" to the area: it has no visible label of its own."
                    </li>
                    <li>
                        "A swatch is a "<Code inline=true>"role=\"img\""</Code>" with "
                        <Code inline=true>"aria-roledescription=\"color swatch\""</Code>". Give it a "<Code inline=true>"color_name"</Code>
                        " where you can: the default name is the CSS color string, which is hard to understand when read aloud."
                    </li>
                </ul>
                <p>"With the area\u{2019}s input focused:"</p>
                <KeyboardTable>
                    <KeyRow keys="ArrowLeft / ArrowRight / ArrowUp / ArrowDown">
                        "Move the thumb by one pixel. The channels take the values at the thumb\u{2019}s position, snapped to "
                        "their steps."
                    </KeyRow>
                    <KeyRow keys="PageUp / PageDown">
                        "Increase or decrease the Y channel by its page step (10% for saturation, brightness and lightness)."
                    </KeyRow>
                    <KeyRow keys="Home / End">
                        "Decrease or increase the X channel by its page step (reversed in right-to-left layouts)."
                    </KeyRow>
                </KeyboardTable>

                <Section title="Known Issues">
                    <p>
                        "The keyboard support of "<Code inline=true>"ColorArea"</Code>" (from "
                        <Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link>
                        ") has these known issues:"
                    </p>
                    <ul>
                        <li>
                            "Arrow keys move the thumb by one pixel instead of one channel step ("<Keys keys="Shift"/>
                            " should move it by a page step). In the 200 pixel demo area, two presses change the saturation by 1%."
                        </li>
                        <li>
                            "Until the area was clicked or dragged, the first arrow key press starts from the top left corner "
                            "instead of the current value: the thumb and the color jump there."
                        </li>
                        <li>
                            <Keys keys="PageUp"/>", "<Keys keys="PageDown"/>", "<Keys keys="Home"/>" and "<Keys keys="End"/>" change "
                            "the color but don\u{2019}t move the thumb, and they don\u{2019}t call "<Code inline=true>"on_change_end"</Code>
                            ". The next arrow key press continues from the thumb\u{2019}s position and discards their change."
                        </li>
                    </ul>
                </Section>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorSwatch.materialize()>"use_color_swatch"</Link></li>
                <li><Link href=routes::doc::color::Component.materialize()>"ColorPicker component"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
