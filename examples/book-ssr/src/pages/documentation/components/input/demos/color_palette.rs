use leptonic::{components::prelude::*, utils::color::HSV};
use leptos::prelude::*;

#[component]
pub fn ColorPaletteDemo() -> impl IntoView {
    let (hsv, set_hsv) = signal(HSV::new());

    view! {
        <ColorPalette
            hsv=hsv
            set_saturation=move |s| set_hsv.update(|hsv| hsv.saturation = s)
            set_value=move |v| set_hsv.update(|hsv| hsv.value = v)
            attr:style="width: 10em; height: 5em;"
        />
    }
}
