use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageStack(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Stack"</H1>

        <H2>"Vertical stacks"</H2>

        <Code>
            {indoc!(r#"
                <Stack spacing=6>
                    <Skeleton animated=false>"Item 1"</Skeleton>
                    <Skeleton animated=false>"Item 2"</Skeleton>
                    <Skeleton animated=false>"Item 3"</Skeleton>
                </Stack>
            "#)}
        </Code>

        <Stack spacing=6>
            <Skeleton animated=false>"Item 1"</Skeleton>
            <Skeleton animated=false>"Item 2"</Skeleton>
            <Skeleton animated=false>"Item 3"</Skeleton>
        </Stack>

        <H2>"Horizontal stacks"</H2>

        <Code>
            {indoc!(r#"
                <Stack orientation=StackOrientation::Horizontal spacing=6>
                    <Skeleton animated=false>"Item 1"</Skeleton>
                    <Skeleton animated=false>"Item 2"</Skeleton>
                    <Skeleton animated=false>"Item 3"</Skeleton>
                </Stack>
            "#)}
        </Code>

        <Stack orientation=StackOrientation::Horizontal spacing=6>
            <Skeleton animated=false>"Item 1"</Skeleton>
            <Skeleton animated=false>"Item 2"</Skeleton>
            <Skeleton animated=false>"Item 3"</Skeleton>
        </Stack>
    }
}
