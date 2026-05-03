use leptonic::{components::prelude::*, utils::color::HSV};
use leptos::prelude::*;

#[component]
pub fn ColorPreviewDemo() -> impl IntoView {
    let (hsv, _set_hsv) = signal(HSV::new());
    let rgb = Signal::derive(move || hsv.get().into_rgb8());

    view! {
        <ColorPreview rgb=rgb attr:style="width: 5em%; height: 5em;"/>
    }
}
