use leptonic::{
    atoms::prelude::{ColorField, Input, Label},
    utils::color::{ColorValue, RGB8},
};
use leptos::prelude::*;

#[component]
pub fn ColorFieldConceptDemo() -> impl IntoView {
    let color = RwSignal::new(Some(RGB8 {
        r: 30,
        g: 110,
        b: 200,
    }));

    view! {
        <ColorField value=color set_value=color classes="demo-field">
            <Label classes="demo-field-label">"Accent color"</Label>
            <Input classes="demo-color-atoms-input"/>
        </ColorField>
        <p class="demo-status">
            {move || color.get().map_or_else(|| "No color".to_owned(), |c| format!("Color: {c}, {}", c.color_name()))}
        </p>
    }
}
