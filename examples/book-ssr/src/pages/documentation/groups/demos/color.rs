use leptonic::{
    Color, ColorValue, HSV, HsvChannel, RGB8,
    atoms::{
        button::Button,
        color_area::ColorArea,
        color_field::ColorField,
        color_picker::ColorPicker,
        color_slider::{ColorSlider, ColorSliderTrack},
        color_swatch::ColorSwatch,
        color_thumb::ColorThumb,
        dialog::{Dialog, DialogTrigger},
        field::Label,
        input::Input,
        popover::Popover,
    },
    hooks::overlay::Placement,
    use_locale,
};
use leptos::prelude::*;

#[component]
pub fn ColorPopoverDemo() -> impl IntoView {
    let locale = use_locale();
    let color = RwSignal::new(Color::from(HSV {
        hue: 212.0,
        saturation: 0.84,
        brightness: 0.9,
    }));
    let rgb = Signal::derive(move || color.get().to::<RGB8>());
    let description = move || format!("{}, {}", rgb.get(), rgb.get().color_name(&locale.get()));

    view! {
        <DialogTrigger>
            // The button's label names the color, so the swatch inside it is decoration.
            <Button aria_label=Signal::derive(move || format!("Accent color: {}", description())) classes="demo-btn">
                <ColorSwatch color=rgb classes="demo-color-trigger-swatch" attr:aria-hidden="true"/>
                "Accent color"
            </Button>
            <Popover placement=Placement::Bottom classes="demo-popover">
                <Dialog aria_label="Accent color">
                    // The atoms inside the picker show and change its color.
                    <ColorPicker value=color set_value=color>
                        <div class="demo-color-atoms-sliders">
                            <ColorArea<HSV>
                                x_channel=HsvChannel::Saturation
                                y_channel=HsvChannel::Brightness
                                aria_label="Saturation and brightness"
                                classes="demo-color-atoms-area"
                            >
                                <ColorThumb classes="demo-color-atoms-thumb"/>
                            </ColorArea<HSV>>
                            <ColorSlider channel=HsvChannel::Hue classes="demo-color-atoms-slider">
                                <Label>"Hue"</Label>
                                <ColorSliderTrack classes="demo-color-atoms-slider-track">
                                    <ColorThumb classes="demo-color-atoms-thumb"/>
                                </ColorSliderTrack>
                            </ColorSlider>
                            <ColorField classes="demo-field">
                                <Label classes="demo-field-label">"Hex"</Label>
                                <Input classes="demo-color-atoms-input"/>
                            </ColorField>
                        </div>
                    </ColorPicker>
                </Dialog>
            </Popover>
        </DialogTrigger>
        <p class="demo-status">{move || format!("Accent color: {}", description())}</p>
    }
}
