use leptonic::{components::prelude::*, utils::css::em};
use leptos::prelude::*;

#[component]
pub fn SkeletonDemo() -> impl IntoView {
    view! {
        <Stack spacing=em(1.0)>
            <Skeleton height=em(5.0)/>

            <Skeleton animated=false height=em(5.0)/>

            <Skeleton animated=false>
                "I am a skeleton!"
            </Skeleton>
        </Stack>
    }
}
