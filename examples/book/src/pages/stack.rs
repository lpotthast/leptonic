use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageStack(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Stack"</h2>

        <h2>"Stack - Vertically"</h2>
        <Stack spacing=6>
            <Skeleton>"Item 1"</Skeleton>
            <Skeleton>"Item 2"</Skeleton>
            <Skeleton>"Item 3"</Skeleton>
        </Stack>

        <h2>"Stack - Horizontally"</h2>
        <Stack orientation=StackOrientation::Horizontal spacing=6>
            <Skeleton>"Item 1"</Skeleton>
            <Skeleton>"Item 2"</Skeleton>
            <Skeleton>"Item 3"</Skeleton>
        </Stack>
    }
}
