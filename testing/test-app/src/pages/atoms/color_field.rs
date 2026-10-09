use leptonic::{
    HSV, HsvChannel, RGB8,
    atoms::{
        color_field::{ColorChannelField, ColorField},
        field::Label,
        input::Input,
    },
};
use leptos::prelude::*;

/// `ColorField`/`ColorChannelField` atoms (react-spectrum's `ColorField.test.js`): committed
/// colors are logged as `<field>:<hex or none>` to `#test-cf-log`.
#[component]
pub fn PageAtomColorField() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let push = move |field: &'static str, entry: String| {
        log.update(|l| l.push(format!("{field}:{entry}")));
    };
    let hex = move |field: &'static str| {
        move |color: Option<RGB8>| {
            push(
                field,
                color.map_or_else(|| "none".to_owned(), |c| format!("{c:X}")),
            );
        }
    };
    let hsv = |hue: f64| HSV {
        hue,
        saturation: 0.5,
        brightness: 1.0,
    };
    let hue_change = move |c: Option<HSV>| {
        push(
            "hue",
            c.map_or_else(|| "none".to_owned(), |c| c.hue.to_string()),
        );
    };
    view! {
        <div id="test-page-atom-color-field">
            <h1>"Color field"</h1>
            <button id="test-cf-before">"Before"</button>
            <div id="test-cf-primary">
                <ColorField default_value=RGB8::from_hex_int(0xAA_BB_CC) on_change=hex("primary")>
                    <Label>"Primary Color"</Label>
                    <Input />
                </ColorField>
            </div>
            <div id="test-cf-empty">
                <ColorField on_change=hex("empty") aria_label="Empty color">
                    <Input />
                </ColorField>
            </div>
            <div id="test-cf-max">
                <ColorField default_value=RGB8::from_hex_int(0xFF_FF_FE) on_change=hex("max") aria_label="Max">
                    <Input />
                </ColorField>
            </div>
            <div id="test-cf-flags">
                <ColorField is_read_only=true is_required=true aria_label="Flags">
                    <Input />
                </ColorField>
            </div>
            <form id="test-cf-form">
                <ColorField default_value=RGB8::from_hex_int(0x12_34_56) name="color" aria_label="Form color">
                    <Input />
                </ColorField>
                <button type="reset" id="test-cf-reset">"Reset"</button>
            </form>
            <div id="test-cf-hue">
                <ColorChannelField
                    channel=HsvChannel::Hue
                    default_value=hsv(10.0)
                    on_change=hue_change
                >
                    <Input />
                </ColorChannelField>
            </div>
            <div id="test-cf-saturation">
                <ColorChannelField channel=HsvChannel::Saturation default_value=hsv(10.0)>
                    <Label>"Saturation"</Label>
                    <Input />
                </ColorChannelField>
            </div>
            <button id="test-cf-clear" on:click=move |_| log.set(Vec::new())>"Clear log"</button>
            <div>"Log: " <span id="test-cf-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
