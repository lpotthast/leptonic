use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageUsage(cx: Scope) -> impl IntoView {
    view! { cx,
        <H1>"Usage"</H1>

        <P>"Similar to Leptos, this crate comes with a prelude module."</P>
        <P>
            "Just "
            <Code inline=true>"use leptonic::prelude::*;"</Code>
            " and you are ready to go."
        </P>
    }
}
