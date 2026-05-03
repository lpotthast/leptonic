use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn ButtonColorsDemo() -> impl IntoView {
    view! {
        <Stack orientation=StackOrientation::Horizontal spacing=em(0.6) attr:style="justify-content: flex-start;">
            <Button on_press=move |_| {} color=ButtonColor::Primary>"Primary"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Secondary>"Secondary"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Warn>"Warn"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Danger>"Danger"</Button>
            <Button on_press=move |_| {} color=ButtonColor::Info>"Info"</Button>
        </Stack>
    }
}
