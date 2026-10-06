use leptonic::{
    atoms::prelude::ColorSwatch,
    components::prelude::*,
    hooks::Placement,
    utils::color::{Color, ColorValue, HSV, RGB8},
};
use leptos::prelude::*;

#[component]
pub fn ColorPopoverDemo() -> impl IntoView {
    let color = RwSignal::new(Color::from(HSV {
        hue: 212.0,
        saturation: 0.84,
        value: 0.9,
    }));
    let rgb = Signal::derive(move || color.get().to::<RGB8>());
    let description = move || format!("{}, {}", rgb.get(), rgb.get().color_name());

    view! {
        <Popover placement=Placement::Bottom aria_label="Accent color">
            <PopoverTrigger slot>
                // The button's label names the color, so the swatch inside it is decoration.
                <Button
                    variant=ButtonVariant::Outlined
                    attr:aria-label=move || format!("Accent color: {}", description())
                >
                    <ColorSwatch color=rgb classes="demo-color-trigger-swatch" attr:aria-hidden="true"/>
                    "Accent color"
                </Button>
            </PopoverTrigger>
            <ColorPicker value=color set_value=color/>
        </Popover>
        <p class="demo-status">{move || format!("Accent color: {}", description())}</p>
    }
}
