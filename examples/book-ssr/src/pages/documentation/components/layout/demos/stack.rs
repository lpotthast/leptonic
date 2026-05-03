use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn StackDemo() -> impl IntoView {
    view! {
        <Stack spacing=em(0.6)>
            <Skeleton animated=false>"Item 1"</Skeleton>
            <Skeleton animated=false>"Item 2"</Skeleton>
            <Skeleton animated=false>"Item 3"</Skeleton>
        </Stack>

        <Stack orientation=StackOrientation::Horizontal spacing=em(0.6)>
            <Skeleton animated=false>"Item 1"</Skeleton>
            <Skeleton animated=false>"Item 2"</Skeleton>
            <Skeleton animated=false>"Item 3"</Skeleton>
        </Stack>
    }
}
