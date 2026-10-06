use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::separator::SeparatorDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSeparator() -> impl IntoView {
    view! {
        <DocPage title="Separator component">
            <p>
                "The "<Code inline=true>"Separator"</Code>" component renders a horizontal "<Code inline=true>"<hr>"</Code>
                " between two blocks of content. See the "
                <Link href=routes::doc::Separator.materialize()>"Separator overview"</Link>
                " for concept guidance."
            </p>

            <Demo description="Separator between two paragraphs" source=include_str!("demos/separator.rs")>
                <SeparatorDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Separator">
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The "<Code inline=true>"<hr>"</Code>" carries the classes "<Code inline=true>"leptonic-separator"</Code>
                    " and "<Code inline=true>"solid"</Code>". The theme doesn\u{2019}t style it, so it looks like the "
                    "browser\u{2019}s default "<Code inline=true>"<hr>"</Code>". Target these classes, or pass your own, to "
                    "change its appearance."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Separator.materialize()>"Separator overview"</Link></li>
                <li><Link href=routes::doc::separator::Hook.materialize()>"use_separator"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
