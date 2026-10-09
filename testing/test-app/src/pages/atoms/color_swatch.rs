use leptonic::{
    Color, RGB8,
    atoms::{
        color_picker::ColorPicker,
        color_swatch::ColorSwatch,
        color_swatch_picker::{ColorSwatchPicker, ColorSwatchPickerItem, ColorSwatchPickerItems},
    },
};
use leptos::prelude::*;

/// `ColorSwatch` and `ColorSwatchPicker` atoms (react-aria-components' `ColorSwatch.test.js`,
/// `ColorSwatchPicker.test.js`). Picked colors are logged to `#test-csw-log`.
#[component]
pub fn PageAtomColorSwatch() -> impl IntoView {
    let log = RwSignal::new(Vec::<String>::new());
    let red = RGB8 { r: 255, g: 0, b: 0 };
    let palette: Signal<Vec<RGB8>> = Signal::stored(
        [0xFF_00_00, 0x00_FF_00, 0x00_FF_FF, 0x00_00_FF]
            .map(RGB8::from_hex_int)
            .to_vec(),
    );
    let picked = move |c: RGB8| log.update(|l| l.push(format!("{c:X}")));
    view! {
        <div id="test-page-atom-color-swatch">
            <h1>"Color swatch"</h1>
            <div id="test-csw-plain"><ColorSwatch color=red /></div>
            <div id="test-csw-label"><ColorSwatch color=red aria_label="Background" /></div>
            <span id="test-csw-label-id">"Label"</span>
            <div id="test-csw-labelledby">
                <ColorSwatch color=red aria_labelledby="test-csw-label-id" />
            </div>
            <div id="test-csw-name"><ColorSwatch color=red color_name="Fire truck red" /></div>
            <div id="test-csw-default">
                <ColorSwatchPicker<RGB8> colors=palette default_value=RGB8::from_hex_int(0x00_FF_FF)>
                    <ColorSwatchPickerItems />
                </ColorSwatchPicker<RGB8>>
            </div>
            <button id="test-csw-before">"Before"</button>
            <div id="test-csw-keyboard">
                <ColorSwatchPicker<RGB8> colors=palette on_change=picked>
                    <ColorSwatchPickerItems />
                </ColorSwatchPicker<RGB8>>
            </div>
            // Items given one by one, the middle one disabled.
            <div id="test-csw-disabled">
                <ColorSwatchPicker<RGB8> colors=palette>
                    <ColorSwatchPickerItem color=RGB8::from_hex_int(0xFF_00_00)><ColorSwatch /></ColorSwatchPickerItem>
                    <ColorSwatchPickerItem color=RGB8::from_hex_int(0x00_FF_00) is_disabled=true>
                        <ColorSwatch />
                    </ColorSwatchPickerItem>
                    <ColorSwatchPickerItem color=RGB8::from_hex_int(0x00_FF_FF)><ColorSwatch /></ColorSwatchPickerItem>
                    <ColorSwatchPickerItem color=RGB8::from_hex_int(0x00_00_FF)><ColorSwatch /></ColorSwatchPickerItem>
                </ColorSwatchPicker<RGB8>>
            </div>
            // Inside a picker of red, the items' swatches show their own colors.
            <div id="test-csw-in-picker">
                <ColorPicker default_value=Color::from(red)>
                    <ColorSwatchPicker<RGB8> colors=palette>
                        <ColorSwatchPickerItems />
                    </ColorSwatchPicker<RGB8>>
                </ColorPicker>
            </div>
            <div>"Log: " <span id="test-csw-log">{move || log.get().join(",")}</span></div>
        </div>
    }
}
