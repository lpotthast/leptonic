use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_swatch::ColorSwatchPaletteDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomColorSwatch() -> impl IntoView {
    view! {
        <DocPage title="Color Swatch Atom">
            <p>
                "The "<Code inline=true>"ColorSwatch"</Code>" atom renders an unstyled color preview with an accessible name. "
                "See the "<Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch overview"</Link>" for the concept."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Link href=routes::doc::color_swatch::Hook.materialize()><Code inline=true>"use_color_swatch"</Code></Link>
                    ", for the role, the accessible name and the CSS color."
                </p>
            </Section>

            <Section title="Example">
                <p>
                    <Code inline=true>"color"</Code>" takes any color: an "<Code inline=true>"RGB8"</Code>", "
                    <Code inline=true>"HSV"</Code>", "<Code inline=true>"HSL"</Code>" or "<Code inline=true>"Color"</Code>
                    " value, or a signal of one (see "
                    <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())><Code inline=true>"Color Values"</Code></Link>"):"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{atoms::prelude::*, utils::color::RGB8};
                        use leptos::prelude::*;

                        view! {
                            <ColorSwatch
                                color={RGB8 { r: 30, g: 110, b: 200 }}
                                color_name="Ocean blue".to_owned()
                                classes="my-swatch"
                            />
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>"A palette of swatches. Each one is named with its "<Code inline=true>"color_name"</Code>"."</p>
                <Demo description="A row of named ColorSwatch atoms" source=include_str!("demos/color_swatch.rs")>
                    <ColorSwatchPaletteDemo/>
                </Demo>
            </Section>

            <Section title="ColorSwatch">
                <p>
                    "Renders a "<Code inline=true>"<div>"</Code>" with the color as background, "<Code inline=true>"role=\"img\""</Code>
                    ", "<Code inline=true>"aria-roledescription=\"color swatch\""</Code>" and an accessible name. The color is a "
                    "signal, so the swatch follows a changing color. Inside a "
                    <Link href=routes::doc::color_picker::Atom.materialize()>"ColorPicker"</Link>" atom, leave out "
                    <Code inline=true>"color"</Code>": the swatch shows the picker\u{2019}s color."
                </p>
                <Section title="Props" id="color-swatch-props">
                    <ApiTable kind=ApiKind::Props of="ColorSwatch">
                        <ApiRow name="color" ty="Option<ColorProp>" default="None">
                            "The color to show: any color value or a signal of one. "<Code inline=true>"None"</Code>": the color "
                            "of the "<Code inline=true>"ColorSwatchPickerItem"</Code>" or else the "<Code inline=true>"ColorPicker"</Code>
                            " around it (black, with a warning, outside both)."
                        </ApiRow>
                        <ApiRow name="color_name" ty="MaybeProp<String>" default="None">
                            "Replaces the color\u{2019}s generated name, e.g. \u{201c}Ocean\u{201d} instead of \u{201c}dark "
                            "vibrant cyan blue\u{201d}."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                            "Added to the color\u{2019}s name: \u{201c}vibrant red, Background\u{201d}."
                        </ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the swatch as well."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the swatch."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"Content rendered inside the swatch."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <p>
                    "The atom renders no "<Code inline=true>"data-*"</Code>" attributes: it has no state. Select it through "
                    "your own class."
                </p>
            </Section>

            <Section title="Styling">
                <p>
                    "The atom renders a "<Code inline=true>"<div role=\"img\">"</Code>" with the class "<Code inline=true>"leptonic-ColorSwatch"</Code>" and the "<Code inline=true>"classes"</Code>" "
                    "you pass, and sets only its "<Code inline=true>"background-color"</Code>" and "<Code inline=true>"forced-color-adjust: none"</Code>", so Windows high contrast "
                    "mode keeps the color. Its size and shape are yours; the demo above uses this CSS:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .demo-color-atoms-palette { display: flex; flex-wrap: wrap; gap: 1rem; margin: 0; padding: 0; list-style: none; }
                        .demo-color-atoms-palette-item { display: flex; flex-direction: column; align-items: center; gap: 0.25rem; width: 7em; }
                        .demo-color-atoms-palette-swatch { width: 3em; height: 3em; border-radius: 50%; box-shadow: inset 0 0 0 1px var(--border); }
                    ")}
                </Code>
                <p>
                    "Apps that don\u{2019}t want to style from scratch can load leptonic\u{2019}s optional atom theme, "
                    <Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>", which styles the default classes."
                </p>
            </Section>

            <Section title="Composition">
                <p>
                    "Let users pick one of several swatches with a "
                    <Link href=routes::doc::ColorSwatchPicker.materialize()>"ColorSwatchPicker"</Link>
                    ", whose items each hold a "<Code inline=true>"ColorSwatch"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch"</Link></li>
                <li><Link href=routes::doc::color_swatch::Hook.materialize()>"use_color_swatch"</Link></li>
                <li><Link href=routes::doc::ColorSwatchPicker.materialize()>"Color Swatch Picker Atoms"</Link></li>
                <li><Link href=routes::doc::color_area::Atom.materialize()>"Color Area Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
