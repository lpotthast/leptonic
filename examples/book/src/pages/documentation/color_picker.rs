use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageColorPicker(cx: Scope) -> impl IntoView {
    let (hsv, set_hsv) = create_signal(cx, HSV::new());

    view! { cx,
        <H1>"Color Picker"</H1>

        <P>"Select colors using the "<Code inline=true>"<ColorPicker>"</Code>" component."</P>

        <Code>
            {indoc!(r#"
                let (hsv, set_hsv) = create_signal(cx, HSV::new());
                view! {cx,
                    <ColorPicker hsv=hsv set_hsv=set_hsv/>
                }
            "#)}
        </Code>

        <ColorPicker hsv=hsv set_hsv=set_hsv/>

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --color-palette-knob-size
                --color-palette-knob-border-width
                --color-palette-knob-border-color
                --color-palette-knob-border-style
                --color-palette-knob-background-color
                --color-palette-knob-halo-size
                --color-palette-knob-halo-size-while-dragged
                --color-palette-knob-halo-opacity
                --color-palette-knob-halo-background-color
                --color-palette-knob-transition-speed
                --color-palette-knob-box-shadow
            "#)}
        </Code>
    }
}
