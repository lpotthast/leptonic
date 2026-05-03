use leptonic::{components::prelude::*, prelude::*, utils::css::CssDimension};
use leptos::prelude::*;

#[component]
pub fn SliderVolumeDemo() -> impl IntoView {
    let (value, set_value) = signal(0.5);

    view! {
        <Stack orientation=StackOrientation::Horizontal spacing=CssDimension::Zero>
            <Icon icon=icondata::BsVolumeDownFill attr:style="font-size: 2.5em;"/>
            <Slider min=0.0 max=1.0 value=value set_value=set_value attr:style="width: 10em"
                value_display=move |v| format!("{:.0}%", v * 100.0)/>
            <Icon icon=icondata::BsVolumeUpFill attr:style="font-size: 2.5em; margin-left: 0.25em;"/>
        </Stack>
    }
}
