use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageColorPicker(cx: Scope) -> impl IntoView {
    let (hsv, set_hsv) = create_signal(cx, HSV::new());

    let (hsv_test, set_hsv_test) = create_signal(cx, HSV::new());
    let hsv_test_rgb_preview = Signal::derive(cx, move || hsv_test.get().into_rgb8());

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

        <H2>"Parts"</H2>

        <P>"The "<Code inline=true>"<ColorPicker>"</Code>" build on top of a few other components build to help work with colors. You may use them directly and build your own color picker."</P>

        <P>"Let's define a HSV color with a derived RGB representation. We will use them for the next component on this page."</P>

        <Code>
            {indoc!(r#"
                let (hsv_test, set_hsv_test) = create_signal(cx, HSV::new());
                let hsv_test_rgb_preview = Signal::derive(cx, move || hsv_test.get().into_rgb8());
            "#)}
        </Code>

        <H3>ColorPreview</H3>

        <P>"The "<Code inline=true>"<ColorPreview>"</Code>" component simply displays a reactive color patch based on the given RGB color signal."</P>

        <Code>
            {indoc!(r#"
                view! {cx,
                    <ColorPreview rgb=hsv_test_rgb_preview style="width: 5em%; height: 5em;"/>
                }
            "#)}
        </Code>

        <ColorPreview rgb=hsv_test_rgb_preview style="width: 5em%; height: 5em;"/>

        <H3>ColorPalette</H3>

        <P>
            "The "<Code inline=true>"<ColorPalette>"</Code>" component works on an HSV color signal, "
            "displays the color-gradient field for any given hue value and allows selecting new values for "
            "saturation (S, x-axis) and value (V, y-axis) of the HSV color by dragging a handle on the displayed surface."
        </P>

        <Code>
            {indoc!(r#"
                view! {cx,
                    <ColorPalette hsv=hsv_test
                        set_saturation=create_callback(cx, move |s| set_hsv_test.update(|hsv| hsv.saturation = s))
                        set_value=create_callback(cx, move |v| set_hsv_test.update(|hsv| hsv.value = v))
                        style="width: 10em; height: 5em;"
                    />
                }
            "#)}
        </Code>

        <ColorPalette hsv=hsv_test
            set_saturation=create_callback(cx, move |s| set_hsv_test.update(|hsv| hsv.saturation = s))
            set_value=create_callback(cx, move |v| set_hsv_test.update(|hsv| hsv.value = v))
            style="width: 10em; height: 5em;"
        />

        <H3>HueSlider</H3>

        <P>
            "The "<Code inline=true>"<HueSlider>"</Code>" component renders a specialized "<Code inline=true>"<Slider>"</Code>", "
            "allowing you to pick a hue, a floating-point value between 0° and 360°. "
            "The slider background displays the hue range as a color band, the knob displays the currently selected hue value at maximum saturation and value."
        </P>

        <Code>
            {indoc!(r#"
                view! {cx,
                    <HueSlider
                        hue=Signal::derive(cx, move || hsv_test.get().hue)
                        set_hue=create_callback(cx, move |h| set_hsv_test.update(|hsv| hsv.hue = h))
                    />
                }
            "#)}
        </Code>

        <HueSlider
            hue=Signal::derive(cx, move || hsv_test.get().hue)
            set_hue=create_callback(cx, move |h| set_hsv_test.update(|hsv| hsv.hue = h))
        />

        <P>"If you look at the source of Leptonic's <ColorPicker>, you will see that there is not much more to it as what you saw here!"</P>

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
