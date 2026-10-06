use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::color_swatch::ColorSwatchDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseColorSwatch() -> impl IntoView {
    view! {
        <DocPage title="use_color_swatch">
            <p>
                "The "<Code inline=true>"use_color_swatch"</Code>" hook gives a display-only color preview its accessible "
                "semantics: an image of a color with a name screen readers can announce. It works with any "
                <Link href=routes::doc::color::Hooks.materialize()>"color type"</Link>". See the "
                <Link href=routes::doc::Color.materialize()>"Color overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useColorSwatch"/>

            <Section title="Demo">
                <p>"The first swatch is named after its CSS color, the second one has a "<Code inline=true>"color_name"</Code>"."</p>

                <Demo description="Two color swatches and their accessible names" source=include_str!("demos/color_swatch.rs")>
                    <ColorSwatchDemo/>
                </Demo>
            </Section>

            <Section title="Input">
                <p>"The input has no "<Code inline=true>"Default"</Code>"; set every field."</p>

                <ApiTable kind=ApiKind::Input of="UseColorSwatchInput">
                    <ApiRow name="color" ty="Signal<C>">"The color to display. "<Code inline=true>"C"</Code>" is any "<Code inline=true>"ColorValue"</Code>"."</ApiRow>
                    <ApiRow name="color_name" ty="Option<Signal<String>>">
                        "A name for the color, used as the "<Code inline=true>"aria-label"</Code>". "<Code inline=true>"None"</Code>
                        " uses the CSS color string, e.g. "<Code inline=true>"rgb(66, 135, 245)"</Code>"."
                    </ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>">"Replaces the "<Code inline=true>"aria-label"</Code>" entirely."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseColorSwatchReturn">
                    <ApiRow name="props" ty="UseColorSwatchProps">
                        <Code inline=true>"role=\"img\""</Code>", "<Code inline=true>"aria-roledescription=\"color swatch\""</Code>
                        " and the "<Code inline=true>"aria-label"</Code>". Spread "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="background_color" ty="Signal<String>">
                        "The color as a CSS string (e.g. "<Code inline=true>"rgb(128, 64, 32)"</Code>") for the background."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let swatch = use_color_swatch(UseColorSwatchInput {
                            color: Signal::stored(RGB8 { r: 66, g: 135, b: 245 }),
                            color_name: Some(Signal::stored("Ocean blue".to_owned())),
                            aria_label: MaybeProp::default(),
                        });

                        view! { <div {..swatch.props.into_attrs()} class="swatch"></div> }
                    "#)}
                </Code>

                <p>
                    "Set the background from "<Code inline=true>"background_color"</Code>" and add "
                    <Code inline=true>"forced-color-adjust: none"</Code>
                    ", so Windows high contrast mode doesn\u{2019}t replace the color. The "<Code inline=true>"ColorSwatch"</Code>
                    " atom does both for you."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Color.materialize()>"Color overview"</Link></li>
                <li><Link href=routes::doc::color::Hooks.materialize()>"Color hooks"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorField.materialize()>"use_color_field"</Link></li>
                <li><Link href=routes::doc::hooks::UseColorArea.materialize()>"use_color_area"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
