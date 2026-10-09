use leptonic::{
    Alpha, HSL, HSV, HslChannel, HsvChannel, I18nProvider, Locale, RGB8, RgbChannel,
    atoms::{color_area::ColorArea, color_thumb::ColorThumb},
    leptos_styles::Styles,
};
use leptos::prelude::*;

/// `ColorArea`/`ColorThumb` atoms (react-spectrum's `ColorArea.test.tsx`): RGB areas of 200 × 200
/// pixels whose changes are logged as `change:<hex>`/`end:<hex>` to `#test-ca-log`. Further
/// (200 × 200): "hsv" (saturation × brightness of `hsb(0, 50%, 50%)`), "hsv-swapped" (brightness
/// × saturation), "hsl-swapped" (lightness × saturation), "rtl" (RGB, in Hebrew), "input" (RGB,
/// logged as `input:<hex>` to `#test-ca-input-log`), "alpha" (an `Alpha<RGB8>` at 50%) and
/// "show" (thumb inside a `<Show>` toggled by `#test-ca-toggle`). "white": an RGB area without a
/// default value; "blue-green": cyan with blue on x, green on y; "red": red, for held keys;
/// "rtl-log": in Hebrew at `#f000f0`, logged as `rtl:<hex>` to `#test-ca-rtl-log-entries`;
/// "props": a custom class, `data-foo`/`data-bar` attributes, the form `test-ca-other-form`, details
/// `#test-ca-details` and an x step of 5.
#[component]
pub fn PageAtomColorArea() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let input_log = RwSignal::new(Vec::<String>::new());
    let rtl_log = RwSignal::new(Vec::<String>::new());
    let hebrew_log: Locale = "he".parse().expect("a valid locale");
    let shown = RwSignal::new(true);
    let hebrew: Locale = "he".parse().expect("a valid locale");
    let size = || {
        Styles::new()
            .add_unchecked("width", "200px")
            .add_unchecked("height", "200px")
    };
    let hsv = HSV {
        hue: 0.0,
        saturation: 0.5,
        brightness: 0.5,
    };
    let area = move |id: &'static str, color: RGB8| {
        view! {
            <div id=id>
                <ColorArea
                    default_value=color
                    on_change=move |c: RGB8| log.update(|l| l.push(format!("change:{c:X}")))
                    on_change_end=move |c: RGB8| log.update(|l| l.push(format!("end:{c:X}")))
                    styles=Styles::new().add_unchecked("width", "200px").add_unchecked("height", "200px")
                >
                    <ColorThumb styles=Styles::new().add_unchecked("width", "10px").add_unchecked("height", "10px") />
                </ColorArea>
            </div>
        }
    };
    view! {
        <div id="test-page-atom-color-area">
            <h1>"Color area"</h1>
            <button id="test-ca-before">"Before"</button>
            {area("test-ca-default", RGB8 { r: 255, g: 0, b: 255 })}
            {area("test-ca-shift", RGB8 { r: 240, g: 0, b: 240 })}
            <button id="test-ca-a">"A"</button>
            <div id="test-ca-disabled">
                <ColorArea default_value=RGB8 { r: 255, g: 0, b: 255 } is_disabled=true>
                    <ColorThumb />
                </ColorArea>
            </div>
            <button id="test-ca-b">"B"</button>
            <div id="test-ca-label">
                <ColorArea default_value=RGB8::default() aria_label="Color hue">
                    <ColorThumb />
                </ColorArea>
            </div>
            <span id="test-ca-label-id">"Label"</span>
            <div id="test-ca-labelledby">
                <ColorArea default_value=RGB8::default() aria_labelledby="test-ca-label-id">
                    <ColorThumb />
                </ColorArea>
            </div>
            <form id="test-ca-form">
                <ColorArea default_value=RGB8 { r: 10, g: 20, b: 30 } x_name="red" y_name="green">
                    <ColorThumb />
                </ColorArea>
                <button type="reset" id="test-ca-reset">"Reset"</button>
            </form>
            <div id="test-ca-hsv">
                <ColorArea
                    default_value=hsv
                    x_channel=HsvChannel::Saturation
                    y_channel=HsvChannel::Brightness
                    styles=size()
                >
                    <ColorThumb />
                </ColorArea>
            </div>
            <div id="test-ca-hsv-swapped">
                <ColorArea
                    default_value=hsv
                    x_channel=HsvChannel::Brightness
                    y_channel=HsvChannel::Saturation
                    styles=size()
                >
                    <ColorThumb />
                </ColorArea>
            </div>
            <div id="test-ca-hsl-swapped">
                <ColorArea
                    default_value=HSL::new()
                    x_channel=HslChannel::Lightness
                    y_channel=HslChannel::Saturation
                    styles=size()
                >
                    <ColorThumb />
                </ColorArea>
            </div>
            <div id="test-ca-rtl">
                <I18nProvider locale=hebrew>
                    <ColorArea default_value=RGB8 { r: 0, g: 0, b: 0 } styles=size()>
                        <ColorThumb />
                    </ColorArea>
                </I18nProvider>
            </div>
            <div id="test-ca-input">
                <ColorArea
                    default_value=RGB8 { r: 0, g: 0, b: 0 }
                    on_change=move |c: RGB8| input_log.update(|l| l.push(format!("input:{c:X}")))
                    styles=size()
                >
                    <ColorThumb />
                </ColorArea>
            </div>
            <div>"Input log: " <span id="test-ca-input-log">{move || input_log.get().join(",")}</span></div>
            <div id="test-ca-alpha">
                <ColorArea default_value=Alpha::new(RGB8 { r: 255, g: 0, b: 255 }).with_alpha(0.5) styles=size()>
                    <ColorThumb />
                </ColorArea>
            </div>
            <div id="test-ca-white">
                <ColorArea<RGB8> styles=size()>
                    <ColorThumb />
                </ColorArea<RGB8>>
            </div>
            <div id="test-ca-blue-green">
                <ColorArea
                    default_value=RGB8 { r: 0, g: 255, b: 255 }
                    x_channel=RgbChannel::Blue
                    y_channel=RgbChannel::Green
                    styles=size()
                >
                    <ColorThumb />
                </ColorArea>
            </div>
            <div id="test-ca-red">
                <ColorArea default_value=RGB8 { r: 255, g: 0, b: 0 } styles=size()>
                    <ColorThumb />
                </ColorArea>
            </div>
            <div id="test-ca-rtl-log">
                <I18nProvider locale=hebrew_log>
                    <ColorArea
                        default_value=RGB8 { r: 240, g: 0, b: 240 }
                        on_change=move |c: RGB8| rtl_log.update(|l| l.push(format!("rtl:{c:X}")))
                        styles=size()
                    >
                        <ColorThumb />
                    </ColorArea>
                </I18nProvider>
            </div>
            <div>"RTL log: " <span id="test-ca-rtl-log-entries">{move || rtl_log.get().join(",")}</span></div>
            <form id="test-ca-other-form"></form>
            <span id="test-ca-details">"Details"</span>
            <div id="test-ca-props">
                <ColorArea
                    default_value=RGB8 { r: 0, g: 0, b: 0 }
                    classes="custom-area"
                    form="test-ca-other-form"
                    aria_details="test-ca-details"
                    x_channel_step=5.0
                    attr:data-foo="area"
                    styles=size()
                >
                    <ColorThumb attr:data-bar="thumb" />
                </ColorArea>
            </div>
            <button id="test-ca-toggle" on:click=move |_| shown.update(|s| *s = !*s)>"Toggle"</button>
            <div id="test-ca-show">
                <ColorArea default_value=RGB8 { r: 10, g: 20, b: 30 } styles=size()>
                    <Show when=move || shown.get()>
                        <ColorThumb />
                    </Show>
                </ColorArea>
            </div>
            <button id="test-ca-clear" on:click=move |_| log.set(Vec::new())>"Clear log"</button>
            <div>"Log: " <span id="test-ca-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
