use leptonic::{
    atoms::prelude::Pressable,
    components::prelude::*,
    hooks::{PlacementY, PressEvent},
    utils::color::{HSV, RGB8},
};
use leptos::prelude::*;

#[component]
fn ColorSwatchPicker(label: &'static str, hsv: RwSignal<HSV>) -> impl IntoView {
    let rgb = Signal::derive(move || RGB8::from(hsv.get()));

    view! {
        <div style="display: flex; flex-direction: column; align-items: center; gap: 0.5em;">
            <span style="font-weight: 600; font-size: 0.85em;">{label}</span>

            <Popover placement_y=PlacementY::Below>
                <PopoverTrigger slot>
                    // ColorPreview is not a Button, so wrap in Pressable to receive
                    // press events. PressResponder context from Popover automatically
                    // merges the toggle callback into Pressable's use_press.
                    <Pressable disabled=Signal::stored(false) on_press=Callback::new(|_: PressEvent| {})>
                        <ColorPreview rgb=rgb classes="demo-color-swatch" />
                    </Pressable>
                </PopoverTrigger>
                <div style="padding: 0.75em; background: var(--surface); border-radius: 0.5em; box-shadow: 0 4px 16px rgba(0,0,0,0.18);">
                    <ColorPicker hsv=hsv set_hsv=hsv />
                </div>
            </Popover>

            <code style="font-size: 0.8em;">{move || format!("{}", rgb.get())}</code>
            <span style="font-size: 0.7em; opacity: 0.6;">
                {move || {
                    let c = rgb.get();
                    format!("rgb({}, {}, {})", c.r, c.g, c.b)
                }}
            </span>
        </div>
    }
}

#[component]
pub fn ColorConceptDemo() -> impl IntoView {
    let primary = RwSignal::new(HSV {
        hue: 0.0,
        saturation: 0.85,
        value: 0.95,
    });
    let secondary = RwSignal::new(HSV {
        hue: 150.0,
        saturation: 0.78,
        value: 0.82,
    });
    let accent = RwSignal::new(HSV {
        hue: 212.0,
        saturation: 0.84,
        value: 1.0,
    });

    view! {
        <div style="display: flex; flex-direction: row; gap: 2.5em; padding: 1.5em; justify-content: center; flex-wrap: wrap;">
            <ColorSwatchPicker label="Primary" hsv=primary />
            <ColorSwatchPicker label="Secondary" hsv=secondary />
            <ColorSwatchPicker label="Accent" hsv=accent />
        </div>
    }
}
