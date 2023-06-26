use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageStack(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Stack"</H1>

        <H2>"Vertical stacks"</H2>

        <P>"Use a stack to create a container displaying its list of children one after the other while spacing them out by a predefined distance."</P>

        <Code>
            {indoc!(r#"
                <Stack spacing=Size::Em(0.6)>
                    <Skeleton animated=false>"Item 1"</Skeleton>
                    <Skeleton animated=false>"Item 2"</Skeleton>
                    <Skeleton animated=false>"Item 3"</Skeleton>
                </Stack>
            "#)}
        </Code>

        <Stack spacing=Size::Em(0.6)>
            <Skeleton animated=false>"Item 1"</Skeleton>
            <Skeleton animated=false>"Item 2"</Skeleton>
            <Skeleton animated=false>"Item 3"</Skeleton>
        </Stack>

        <H2>"Horizontal stacks"</H2>

        <P>
            "A stacks default orientation is "<Code inline=true>"StackOrientation::Vertical"</Code>". "
            "You can explicitly set the orientation to be "<Code inline=true>"StackOrientation::Horizontal"</Code>" "
            "to let the stack display its children horizontally."
        </P>

        <Code>
            {indoc!(r#"
                <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
                    <Skeleton animated=false>"Item 1"</Skeleton>
                    <Skeleton animated=false>"Item 2"</Skeleton>
                    <Skeleton animated=false>"Item 3"</Skeleton>
                </Stack>
            "#)}
        </Code>

        <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.6)>
            <Skeleton animated=false>"Item 1"</Skeleton>
            <Skeleton animated=false>"Item 2"</Skeleton>
            <Skeleton animated=false>"Item 3"</Skeleton>
        </Stack>
    }
}
