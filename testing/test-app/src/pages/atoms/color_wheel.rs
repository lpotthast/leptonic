use leptonic::{
    atoms::{
        color_thumb::ColorThumb,
        color_wheel::{ColorWheel, ColorWheelTrack},
    },
    utils::{
        color::{HSV, HsvChannel},
        styles::Styles,
    },
};
use leptos::prelude::*;

/// `ColorWheel` atoms (react-spectrum's `ColorWheel.test.tsx`): wheels of radius 100 (track ring
/// from 74); hues are logged as `change:<hue>`/`end:<hue>` to `#test-cw-log`.
#[component]
pub fn PageAtomColorWheel() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let hsv = |hue: f64| HSV {
        hue,
        saturation: 1.0,
        value: 1.0,
    };
    let thumb = || {
        Styles::new()
            .add_unchecked("width", "20px")
            .add_unchecked("height", "20px")
    };
    view! {
        <div id="test-page-atom-color-wheel">
            <h1>"Color wheel"</h1>
            <button id="test-cw-before">"Before"</button>
            <div id="test-cw-default">
                <ColorWheel
                    channel=HsvChannel::Hue
                    outer_radius=100.0
                    inner_radius=74.0
                    default_value=hsv(0.0)
                    on_change=move |c: HSV| log.update(|l| l.push(format!("change:{}", c.hue)))
                    on_change_end=move |c: HSV| log.update(|l| l.push(format!("end:{}", c.hue)))
                >
                    <ColorWheelTrack />
                    <ColorThumb styles=thumb() />
                </ColorWheel>
            </div>
            <button id="test-cw-a">"A"</button>
            <div id="test-cw-disabled">
                <ColorWheel
                    channel=HsvChannel::Hue
                    outer_radius=50.0
                    inner_radius=30.0
                    default_value=hsv(0.0)
                    is_disabled=true
                >
                    <ColorWheelTrack />
                    <ColorThumb />
                </ColorWheel>
            </div>
            <button id="test-cw-b">"B"</button>
            <form id="test-cw-form">
                <ColorWheel
                    channel=HsvChannel::Hue
                    outer_radius=50.0
                    inner_radius=30.0
                    default_value=hsv(10.0)
                    name="hue"
                    aria_label="Tint"
                >
                    <ColorWheelTrack />
                    <ColorThumb />
                </ColorWheel>
                <button type="reset" id="test-cw-reset">"Reset"</button>
            </form>
            <button id="test-cw-clear" on:click=move |_| log.set(Vec::new())>"Clear log"</button>
            <div>"Log: " <span id="test-cw-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
