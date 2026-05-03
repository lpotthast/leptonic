use leptonic::{components::prelude::*, utils::color::HSV};
use leptos::prelude::*;

#[component]
pub fn ColorPickerFullDemo() -> impl IntoView {
    let (hsv, set_hsv) = signal(HSV::new());

    view! {
        <ColorPicker hsv=hsv set_hsv=set_hsv/>
    }
}
