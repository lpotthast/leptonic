use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::skeleton::SkeletonDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageSkeleton() -> impl IntoView {
    view! {
        <DocPage title="Skeleton">
            <p>
                "A skeleton is a placeholder of a specific shape and size, shown in place of content that can\u{2019}t be "
                "displayed yet, for example because its data is still being fetched. It reduces layout shifts and shows "
                "your users where the content will appear. The "<Code inline=true>"Skeleton"</Code>
                " component renders such a placeholder, animated by default."
            </p>

            <Demo
                description="Animated skeleton, static skeleton and skeleton with content"
                source=include_str!("demos/skeleton.rs")
            >
                <SkeletonDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="Skeleton">
                    <ApiRow name="width" ty="Option<CssDimension>" default="None">
                        "The width. Without it, the skeleton spans the full width ("<Code inline=true>"100%"</Code>")."
                    </ApiRow>
                    <ApiRow name="height" ty="Option<CssDimension>" default="None">
                        "The height. Without it, the skeleton is as high as its content."
                    </ApiRow>
                    <ApiRow name="animated" ty="bool" default="true">
                        "Whether a highlight sweeps across the skeleton. Sets the "<Code inline=true>"data-animated"</Code>
                        " attribute."
                    </ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                    <ApiRow name="children" ty="Option<Children>
                        " default="None">"Content shown centered in the skeleton."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="When to Use">
                <p>
                    "A skeleton only mitigates slowly loading content. Often you can avoid the wait altogether, by "
                    "preloading resources or by serving data fast, so don\u{2019}t reach for skeletons by default. "
                    "Keep in mind, though, that even fast services can\u{2019}t make up for a slow network connection "
                    "of your users."
                </p>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt skeletons to your design:"</p>
                <CssVariables prefix="--skeleton-" scss=theme_scss!("skeleton")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::components::Stack.materialize()>"Stack"</Link></li>
                <li><Link href=routes::doc::Progress.materialize()>"Progress"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
