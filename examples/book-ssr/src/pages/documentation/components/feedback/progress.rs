use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    progress_controlled::ProgressControlledDemo, progress_indeterminate::ProgressIndeterminateDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageProgress() -> impl IntoView {
    view! {
        <DocPage title="Progress component">
            <p>
                "The themed "<Code inline=true>"ProgressBar"</Code>" shows how much of an operation is already "
                "completed, as a filled bar and a percentage. See the "
                <Link href=routes::doc::Progress.materialize()>"Progress overview"</Link>" for concept guidance."
            </p>

            <Demo
                description="Progress bar controlled by a number input and a slider"
                source=include_str!("demos/progress_controlled.rs")
            >
                <ProgressControlledDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="ProgressBar">
                    <ApiRow name="progress" ty="Signal<Option<f64>>">
                        "The completed amount, between "<Code inline=true>"0"</Code>" and "<Code inline=true>"max"</Code>
                        ". "<Code inline=true>"None"</Code>" means indeterminate. Required."
                    </ApiRow>
                    <ApiRow name="max" ty="Signal<f64>" default="100.0">"The amount at which the bar is full."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
                <p>
                    "Values outside "<Code inline=true>"0..=max"</Code>" are clamped. The bar shows the percentage with "
                    "two decimal places."
                </p>
            </Section>

            <Section title="Indeterminate State">
                <p>
                    "Progress is an "<Code inline=true>"Option<f64>"</Code>". As long as it is "<Code inline=true>"Some"</Code>
                    ", the bar shows that value. When it is "<Code inline=true>"None"</Code>", the bar is "
                    "indeterminate: it tells your users that something is going on, without saying how much of the work "
                    "is done. It then hides the percentage and sets "<Code inline=true>"data-indeterminate"</Code>"."
                </p>

                <Demo description="Indeterminate progress bar" source=include_str!("demos/progress_indeterminate.rs")>
                    <ProgressIndeterminateDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt progress bars to your design:"</p>
                <CssVariables prefix="--progress-bar-" scss=theme_scss!("progress_bar")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Progress.materialize()>"Progress overview"</Link></li>
                <li><Link href=routes::doc::progress::Hook.materialize()>"use_progress_bar"</Link></li>
                <li><Link href=routes::doc::components::Skeleton.materialize()>"Skeleton"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
