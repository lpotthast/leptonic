use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    progress_controlled::ProgressControlledDemo, progress_indeterminate::ProgressIndeterminateDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageProgress() -> impl IntoView {
    view! {
        <DocPage title="Progress Bar Component">
            <p>
                "The themed "<Code inline=true>"ProgressBar"</Code>" component shows how much of an operation is already "
                "completed, as a filled bar with the value text on it. See the "
                <Link href=routes::doc::ProgressBar.materialize()>"Progress Bar overview"</Link>" for concept guidance."
            </p>

            <Demo
                description="Progress bar with a label, controlled by a number field and a slider"
                source=include_str!("demos/progress_controlled.rs")
            >
                <ProgressControlledDemo/>
            </Demo>

            <Section title="Props">
                <ApiTable kind=ApiKind::Props of="components::progress_bar::ProgressBar">
                    <ApiRow name="value" ty="OptionalNumberSignal<T>">
                        "The progress: a number, an "<Code inline=true>"Option"</Code>", or any signal of them, clamped to the range. "
                        <Code inline=true>"None"</Code>" means indeterminate. Required."
                    </ApiRow>
                    <ApiRow name="min_value" ty="Option<Signal<T>>" default="0">"The start of the range."</ApiRow>
                    <ApiRow name="max_value" ty="Option<Signal<T>>" default="100">"The end of the range: the bar is full."</ApiRow>
                    <ApiRow name="format_options" ty="Option<Signal<NumberFormatOptions>>" default="percent">
                        "How the value text is formatted: a percent style formats the percentage, other styles the value."
                    </ApiRow>
                    <ApiRow name="value_label" ty="MaybeProp<String>" default="None">"Replaces the value text (e.g. \u{201c}1 of 4\u{201d})."</ApiRow>
                    <ApiRow name="label" ty="Option<String>" default="None">"The visible label above the bar. Without it, set "<Code inline=true>"aria_label"</Code>"."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the bar when it has no visible label."</ApiRow>
                    <ApiRow name="show_value" ty="bool" default="true">"Whether the value text shows in the bar."</ApiRow>
                    <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                        "Additional classes and styles."
                    </ApiRow>
                </ApiTable>
                <p>
                    "The value can be of any number type; values outside "<Code inline=true>"min_value"</Code>" to "
                    <Code inline=true>"max_value"</Code>" are clamped. The value text is the percentage in whole percent, "
                    "formatted for the current locale. Format it differently with "<Code inline=true>"format_options"</Code>
                    " (a style other than percent formats the value itself), replace it with "
                    <Code inline=true>"value_label"</Code>", or hide it with "<Code inline=true>"show_value=false"</Code>"."
                </p>
            </Section>

            <Section title="Indeterminate State">
                <p>
                    "The value may be an "<Code inline=true>"Option"</Code>". As long as it is "<Code inline=true>"Some"</Code>
                    ", the bar shows that value. When it is "<Code inline=true>"None"</Code>", the bar is "
                    "indeterminate: it tells your users that something is going on, without saying how much of the work "
                    "is done. It then shows no value text, and the bar and its fill carry "
                    <Code inline=true>"data-indeterminate"</Code>"."
                </p>

                <Demo description="Indeterminate progress bar labelled Connecting" source=include_str!("demos/progress_indeterminate.rs")>
                    <ProgressIndeterminateDemo/>
                </Demo>
            </Section>

            <Section title="Styling">
                <p>"Override any of these CSS variables to adapt progress bars to your design:"</p>
                <CssVariables prefix="--progress-bar-" scss=theme_scss!("progress_bar")/>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ProgressBar.materialize()>"Progress Bar overview"</Link></li>
                <li><Link href=routes::doc::progress_bar::Hook.materialize()>"use_progress_bar"</Link></li>
                <li><Link href=routes::doc::progress_bar::Atom.materialize()>"Progress Bar Atoms"</Link></li>
                <li><Link href=routes::doc::meter::Component.materialize()>"Meter Component"</Link></li>
                <li><Link href=routes::doc::Skeleton.materialize()>"Skeleton"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
