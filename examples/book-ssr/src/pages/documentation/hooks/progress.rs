use indoc::indoc;
use leptos::prelude::*;

use super::demos::progress_bar::ProgressBarHookDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageUseProgressBar() -> impl IntoView {
    view! {
        <DocPage title="use_progress_bar">
            <p>
                "The "<Code inline=true>"use_progress_bar"</Code>" hook makes an element announce the progress of a task, "
                "either as a value in a range (determinate) or as ongoing activity (indeterminate). "
                "See the "<Link href=routes::doc::ProgressBar.materialize()>"Progress Bar overview"</Link>" for concept guidance."
            </p>

            <ReactAria hook="useProgressBar"/>

            <Section title="Input">
                <p>
                    <Code inline=true>"UseProgressBarInput"</Code>" implements "<Code inline=true>"Default"</Code>
                    ", so you only name the fields you care about."
                </p>

                <ApiTable kind=ApiKind::Input of="UseProgressBarInput">
                    <ApiRow name="value" ty="Signal<Option<T>>" default="Some(0)">
                        "The progress, clamped to the range; "<Code inline=true>"None"</Code>" while it isn\u{2019}t known (indeterminate)."
                    </ApiRow>
                    <ApiRow name="min_value" ty="Signal<T>" default="0">"The start of the range."</ApiRow>
                    <ApiRow name="max_value" ty="Signal<T>" default="100">"The end of the range."</ApiRow>
                    <ApiRow name="format_options" ty="Signal<NumberFormatOptions>" default="percent">
                        "How the value text is formatted: a percent style formats the percentage, other styles the value."
                    </ApiRow>
                    <ApiRow name="value_label" ty="MaybeProp<String>" default="None">"Replaces the formatted value text (e.g. \u{201c}1 of 4\u{201d})."</ApiRow>
                    <ApiRow name="id" ty="Option<String>" default="None">"The element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                    <ApiRow name="has_label" ty="Signal<bool>" default="false">"Whether a visible label is rendered (with "<Code inline=true>"label_props"</Code>")."</ApiRow>
                    <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names it when there is no visible label."</ApiRow>
                    <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of further elements naming it."</ApiRow>
                    <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Ids of elements describing it."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return">
                <ApiTable kind=ApiKind::Return of="UseProgressBarReturn">
                    <ApiRow name="props" ty="UseProgressBarProps">
                        "Spread on the progress bar element: "<Code inline=true>"role"</Code>", "<Code inline=true>"aria-valuenow"</Code>", "<Code inline=true>"aria-valuemin"</Code>", "<Code inline=true>"aria-valuemax"</Code>", "
                        <Code inline=true>"aria-valuetext"</Code>", id and labelling."
                    </ApiRow>
                    <ApiRow name="label_props" ty="UseLabelProps">"Spread on the visible label, a "<Code inline=true>"<span>"</Code>"."</ApiRow>
                    <ApiRow name="percentage" ty="Signal<Option<Fraction>>">
                        "The value as a share of the range, a "<Code inline=true>"Fraction"</Code>" from 0 to 1 ("
                        <Code inline=true>"as_percent()"</Code>": 0 to 100); "<Code inline=true>"None"</Code>" while indeterminate."
                    </ApiRow>
                    <ApiRow name="value_text" ty="Signal<Option<String>>">"The formatted value (or "<Code inline=true>"value_label"</Code>"); "<Code inline=true>"None"</Code>" while indeterminate."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            computed_pct,
                            computed_size,
                            hooks::progress::{UseProgressBarInput, use_progress_bar},
                            leptos_styles::{Styles, property::WidthProperty},
                        };

                        let progress = use_progress_bar(UseProgressBarInput {
                            value: Signal::stored(Some(45.0)),
                            has_label: true.into(),
                            ..UseProgressBarInput::default()
                        });
                        let percentage = progress.percentage;
                        let fill = Styles::new().add_optional(move || {
                            percentage.get().map(|p| WidthProperty.declare(computed_size(computed_pct(p.as_percent()))))
                        });

                        view! {
                            <span {..progress.label_props.into_attrs()}>"Uploading"</span>
                            <span>{progress.value_text}</span>
                            <div {..progress.props.into_attrs()} class="track">
                                <div class="fill" style=fill></div>
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "The fill width follows "<Code inline=true>"percentage"</Code>". With \u{201c}Duration unknown\u{201d}, "
                    "the value becomes "<Code inline=true>"None"</Code>": the value text disappears and the fill slides "
                    "back and forth."
                </p>

                <Demo description="Progress bar with buttons to change the value and an indeterminate toggle" source=include_str!("demos/progress_bar.rs")>
                    <ProgressBarHookDemo/>
                </Demo>
            </Section>

            <Section title="Indeterminate Progress">
                <p>
                    "For tasks of unknown duration, pass "<Code inline=true>"None"</Code>" as the value. "
                    <Code inline=true>"percentage"</Code>" and "<Code inline=true>"value_text"</Code>" become "
                    <Code inline=true>"None"</Code>", and the element loses "<Code inline=true>"aria-valuenow"</Code>
                    " and "<Code inline=true>"aria-valuetext"</Code>", so screen readers announce a busy bar without a value. "
                    "Style that state through the missing attribute; without motion, stripe the whole track so that it "
                    "can\u{2019}t be mistaken for a partial fill:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .track { position: relative; overflow: hidden; }
                        .track:not([aria-valuenow]) .fill { position: absolute; width: 40%; animation: slide 1.5s infinite ease-in-out; }
                        @keyframes slide { from { left: -40%; } to { left: 100%; } }

                        @media (prefers-reduced-motion: reduce) {
                            .track:not([aria-valuenow]) .fill {
                                left: 0; width: 100%; animation: none;
                                background: repeating-linear-gradient(-45deg, var(--accent) 0 0.5em, var(--surface) 0.5em 1em);
                            }
                        }
                    ")}
                </Code>
            </Section>

            <Section title="ARIA Attributes">
                <p>"The progress bar element gets:"</p>

                <ul>
                    <li><Code inline=true>"role=\"progressbar\""</Code></li>
                    <li><Code inline=true>"aria-valuenow"</Code>" \u{2014} the current value, absent while indeterminate"</li>
                    <li><Code inline=true>"aria-valuemin"</Code>" / "<Code inline=true>"aria-valuemax"</Code>" \u{2014} the range"</li>
                    <li><Code inline=true>"aria-valuetext"</Code>" \u{2014} "<Code inline=true>"value_text"</Code>", absent while indeterminate"</li>
                    <li><Code inline=true>"aria-labelledby"</Code>" \u{2014} the label, with "<Code inline=true>"has_label"</Code></li>
                </ul>

                <p>
                    "Without a visible label, name the progress bar with "<Code inline=true>"aria_label"</Code>" or "
                    <Code inline=true>"aria_labelledby"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ProgressBar.materialize()>"Progress Bar overview"</Link></li>
                <li><Link href=routes::doc::progress_bar::Atom.materialize()>"Progress Bar Atoms"</Link></li>
                <li><Link href=routes::doc::meter::Hook.materialize()>"use_meter"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
