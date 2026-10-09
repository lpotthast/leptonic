use indoc::indoc;
use leptos::prelude::*;

use super::demos::color_swatch::ColorSwatchDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseColorSwatch() -> impl IntoView {
    view! {
        <DocPage title="use_color_swatch">
            <p>
                "The "<Code inline=true>"use_color_swatch"</Code>" hook gives a display-only color preview its accessible "
                "semantics: an image of a color with a name screen readers can announce. See the "
                <Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch overview"</Link>" for the concept."
            </p>

            <ReactAria hook="useColorSwatch"/>

            <Section title="Input">
                <ApiTable kind=ApiKind::Input of="UseColorSwatchInput">
                    <ApiRow name="color" ty="Signal<Color>">
                        "The color to display. Required. Convert a color value with "<Code inline=true>"Color::from"</Code>
                        ", or a signal of one with "<Code inline=true>"ColorProp::from(signal).0"</Code>" (see "
                        <Link href=format!("{}#colorvalue", routes::doc::Color.materialize())><Code inline=true>"Color Values"</Code></Link>")."
                    </ApiRow>
                    <ApiRow name="color_name" ty="MaybeProp<String>" default="None">
                        "Replaces the color\u{2019}s generated name, e.g. \u{201c}Ocean\u{201d} instead of \u{201c}dark "
                        "vibrant cyan blue\u{201d}."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">
                        "Added to the color\u{2019}s name: the swatch is announced as \u{201c}vibrant red, Background\u{201d}."
                    </ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the swatch as well."</ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The swatch\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseColorSwatchReturn">
                    <ApiRow name="color_swatch_props" ty="PropsWithStyles<UseColorSwatchProps>">
                        <Code inline=true>"role=\"img\""</Code>", "<Code inline=true>"aria-roledescription=\"color swatch\""</Code>
                        ", the id and the "<Code inline=true>"aria-label"</Code>", plus the swatch\u{2019}s background color as a "
                        "style. "<Code inline=true>"into_inner()"</Code>" splits them into props and "<Code inline=true>"Styles"</Code>
                        "; spread "<Code inline=true>"{..props.into_attrs()}"</Code>" and pass "<Code inline=true>"style=styles"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            Color,
                            RGB8,
                            hooks::color::{UseColorSwatchInput, use_color_swatch},
                        };
                        use leptos::prelude::*;

                        let swatch = use_color_swatch(UseColorSwatchInput {
                            color: Signal::stored(Color::from(RGB8 { r: 66, g: 135, b: 245 })),
                            color_name: "Ocean blue".into(),
                            aria_label: MaybeProp::default(),
                            aria_labelledby: None,
                            id: None,
                        });

                        let (props, styles) = swatch.color_swatch_props.into_inner();

                        view! { <div {..props.into_attrs()} class="swatch" style=styles></div> }
                    "#)}
                </Code>

                <p>
                    "The styles set the background color and "<Code inline=true>"forced-color-adjust: none"</Code>
                    ", so Windows high contrast mode doesn\u{2019}t replace the color. The "
                    <Link href=routes::doc::color_swatch::Atom.materialize()>"Color Swatch Atom"</Link>" renders the element for you."
                </p>
            </Section>

            <Section title="Demo">
                <p>"The first swatch is announced with its generated color name, the second one with its "<Code inline=true>"color_name"</Code>"."</p>

                <Demo description="Two color swatches and their accessible names" source=include_str!("demos/color_swatch.rs")>
                    <ColorSwatchDemo/>
                </Demo>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ColorSwatch.materialize()>"Color Swatch"</Link></li>
                <li><Link href=routes::doc::color_swatch::Atom.materialize()>"Color Swatch Atom"</Link></li>
                <li><Link href=routes::doc::color_picker::Hook.materialize()>"use_color_picker_state"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
