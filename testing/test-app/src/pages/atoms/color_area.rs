use leptonic::utils::styles::Styles;
use leptonic::{
    atoms::{color_area::ColorArea, color_thumb::ColorThumb},
    utils::color::RGB8,
};
use leptos::prelude::*;

/// `ColorArea`/`ColorThumb` atoms (react-spectrum's `ColorArea.test.tsx`): RGB areas of 200 × 200
/// pixels whose changes are logged as `change:<hex>`/`end:<hex>` to `#test-ca-log`.
#[component]
pub fn PageAtomColorArea() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
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
            <button id="test-ca-clear" on:click=move |_| log.set(Vec::new())>"Clear log"</button>
            <div>"Log: " <span id="test-ca-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
