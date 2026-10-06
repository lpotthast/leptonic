use leptonic::{
    atoms::prelude::{Label, Meter, MeterFill, MeterValueText},
    components::prelude::Button,
    utils::{
        classes::Classes,
        css::{CssColor, CssColorName, css_custom_property, var},
        style::BackgroundColorProperty,
        styles::Styles,
    },
};
use leptos::prelude::*;

// The theme's status colors, as typed custom properties: only `CssColor` values fit them.
css_custom_property!(SUCCESS_COLOR: CssColor = "--success-color");
css_custom_property!(WARN_COLOR: CssColor = "--warn-color");

#[component]
pub fn ClassesAndStylesMeterDemo() -> impl IntoView {
    // Used storage in GB, of 100 GB.
    let (used, set_used) = signal(40_u32);
    let is_almost_full = move || used.get() >= 80;

    // The look lives in CSS; a class follows the state reactively.
    let fill_classes = Classes::from("demo-storage-meter-fill").add_reactive("demo-storage-meter-fill-high", is_almost_full);

    // A typed, reactive declaration: the fill color follows the state, too.
    let fill_styles = Styles::new().add_reactive(move || {
        BackgroundColorProperty.declare(if is_almost_full() {
            var(&WARN_COLOR, CssColor::Named(CssColorName::Olive))
        } else {
            var(&SUCCESS_COLOR, CssColor::Named(CssColorName::Green))
        })
    });

    view! {
        <Meter value=used classes="demo-storage-meter">
            <div class="demo-storage-meter-header">
                <Label>"Storage"</Label>
                <MeterValueText/>
            </div>
            <div class="demo-storage-meter-track">
                <MeterFill classes=fill_classes styles=fill_styles/>
            </div>
        </Meter>

        <div class="demo-controls">
            <Button
                on_press=move |_| set_used.update(|used| *used = used.saturating_sub(10))
                is_disabled=Signal::derive(move || used.get() == 0)
            >
                "Free 10 GB"
            </Button>
            <Button
                on_press=move |_| set_used.update(|used| *used = (*used + 10).min(100))
                is_disabled=Signal::derive(move || used.get() == 100)
            >
                "Use 10 GB"
            </Button>
        </div>
    }
}
