use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{
    progress_determinate::ProgressDeterminateDemo,
    progress_indeterminate::ProgressIndeterminateDemo,
};
use crate::{kit::*, routes};

#[component]
pub fn PageUseProgressBar() -> impl IntoView {
    view! {
        <DocPage title="use_progress_bar">
            <p>
                "The "<Code inline=true>"use_progress_bar"</Code>" hook makes an element announce the progress of a task, "
                "either as a value in a range (determinate) or as ongoing activity (indeterminate). "
                "See the "<Link href=routes::doc::Progress.materialize()>"Progress overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useProgressBar"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseProgressBarInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ", so you only name the fields you care about."
                </p>

                <ApiTable kind=ApiKind::Input of="UseProgressBarInput">
                    <ApiRow name="value" ty="Signal<Option<f64>>" default="Some(0.0)">
                        "The current value. "<Code inline=true>"None"</Code>" omits "<Code inline=true>"aria-valuenow"</Code>"."
                    </ApiRow>
                    <ApiRow name="min_value" ty="f64" default="0.0">"The value at which no progress has been made."</ApiRow>
                    <ApiRow name="max_value" ty="f64" default="100.0">"The value at which the task is complete."</ApiRow>
                    <ApiRow name="label" ty="Option<String>" default="None">
                        "Whether the progress bar has a visible label. When set, the progress bar gets "
                        <Code inline=true>"aria-labelledby"</Code>" pointing to "<Code inline=true>"label_props.id"</Code>
                        "; the text itself is not used, so render it in your label element."
                    </ApiRow>
                    <ApiRow name="show_value_label" ty="bool" default="true">
                        "Currently has no effect: "<Code inline=true>"value_label"</Code>" is always computed."
                    </ApiRow>
                    <ApiRow name="is_indeterminate" ty="bool" default="false">
                        "Whether progress is ongoing without a known value. Removes "<Code inline=true>"aria-valuenow"</Code>
                        " and "<Code inline=true>"aria-valuetext"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseProgressBarReturn">
                    <ApiRow name="progress_props" ty="UseProgressBarProps">
                        "Attributes for the progress bar element, see "<a href="#aria-attributes">"ARIA attributes"</a>
                        ". Spread with "<Code inline=true>"{..progress_props.into_attrs()}"</Code>"."
                    </ApiRow>
                    <ApiRow name="label_props" ty="UseProgressBarLabelProps">
                        "The "<Code inline=true>"id"</Code>" for your label element."
                    </ApiRow>
                    <ApiRow name="percentage" ty="Signal<Option<f64>>">
                        "The value as a percentage of the range (0 to 100), for sizing a fill. "
                        <Code inline=true>"None"</Code>" while indeterminate or without a value."
                    </ApiRow>
                    <ApiRow name="value_label" ty="Signal<String>">
                        "The rounded percentage as text, e.g. \u{201c}65%\u{201d}. Empty while indeterminate."
                    </ApiRow>
                    <ApiRow name="is_indeterminate" ty="bool">"The "<Code inline=true>"is_indeterminate"</Code>" input."</ApiRow>
                    <ApiRow name="progress_id" ty="String">"The generated id of the progress bar."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let progress = use_progress_bar(UseProgressBarInput {
                            value: Signal::derive(|| Some(45.0)),
                            label: Some("Uploading".to_string()),
                            ..Default::default()
                        });

                        view! {
                            <label id=progress.label_props.id>"Uploading"</label>
                            <div {..progress.progress_props.into_attrs()} class="track">
                                // Size the fill from `progress.percentage`.
                                <div class="fill"></div>
                            </div>
                            <span>{move || progress.value_label.get()}</span>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Determinate Progress">
                <p>"A known value within a range. The fill width follows "<Code inline=true>"percentage"</Code>":"</p>

                <Demo description="Determinate progress bar with buttons to change the value" source=include_str!("demos/progress_determinate.rs")>
                    <ProgressDeterminateDemo/>
                </Demo>
            </Section>

            <Section title="Indeterminate Progress">
                <p>"For tasks of unknown duration, set "<Code inline=true>"is_indeterminate"</Code>" and animate the fill with CSS:"</p>

                <Demo description="Indeterminate progress bar with an animated fill" source=include_str!("demos/progress_indeterminate.rs")>
                    <ProgressIndeterminateDemo/>
                </Demo>
            </Section>

            <Section title="ARIA attributes">
                <p>"The progress bar element gets:"</p>

                <ul>
                    <li><Code inline=true>"role=\"progressbar\""</Code></li>
                    <li><Code inline=true>"aria-valuenow"</Code>" \u{2014} the current value, absent while indeterminate"</li>
                    <li><Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>" \u{2014} the range"</li>
                    <li><Code inline=true>"aria-valuetext"</Code>" \u{2014} "<Code inline=true>"value_label"</Code>", absent while indeterminate"</li>
                    <li><Code inline=true>"aria-labelledby"</Code>" \u{2014} the label, if "<Code inline=true>"label"</Code>" is set"</li>
                </ul>

                <p>"Without a "<Code inline=true>"label"</Code>", give the element an "<Code inline=true>"aria-label"</Code>" yourself."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Progress.materialize()>"Progress overview"</Link></li>
                <li><Link href=routes::doc::progress::Component.materialize()>"Progress component"</Link></li>
                <li><Link href=routes::doc::hooks::UseMeter.materialize()>"use_meter"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
