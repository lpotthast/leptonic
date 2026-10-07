use indoc::indoc;
use leptos::prelude::*;

use super::demos::progress_bar::ProgressBarAtomDemo;
use crate::{kit::*, routes};

#[component]
pub fn PageAtomProgressBar() -> impl IntoView {
    view! {
        <DocPage title="Progress Bar Atoms">
            <p>
                "The unstyled "<Code inline=true>"ProgressBar"</Code>" atom shows the progress of an operation over time: "
                "known ("<Code inline=true>"Some(value)"</Code>") or not ("<Code inline=true>"None"</Code>"). Its parts "
                <Code inline=true>"ProgressBarFill"</Code>" and "<Code inline=true>"ProgressBarValueText"</Code>
                " show the progress; you draw the track. See the "
                <Link href=routes::doc::ProgressBar.materialize()>"Progress Bar overview"</Link>" for concept guidance."
            </p>

            <Section title="Hooks Used">
                <p>
                    <Code inline=true>"ProgressBar"</Code>" calls "
                    <Link href=routes::doc::progress_bar::Hook.materialize()>"use_progress_bar"</Link>" for "
                    <Code inline=true>"role=\"progressbar\""</Code>", the value attributes and the label, and provides the "
                    "percentage and the value text to its parts. A "<Link href=routes::doc::field::Atom.materialize()>"Label"</Link>
                    " inside it names the bar."
                </p>
            </Section>

            <Section title="Example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::atoms::prelude::*;

                        let total_bytes = 4_000_000_u64;
                        // `None` while the progress isn't known.
                        let uploaded_bytes = RwSignal::new(Some(0_u64));

                        view! {
                            <ProgressBar value=uploaded_bytes max_value=total_bytes classes="progress">
                                <Label>"Uploading"</Label>
                                <ProgressBarValueText />
                                <div class="progress-track">
                                    <ProgressBarFill classes="progress-fill" />
                                </div>
                            </ProgressBar>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "With \u{201c}Size unknown\u{201d}, the value becomes "<Code inline=true>"None"</Code>": the bar and its "
                    "fill get "<Code inline=true>"data-indeterminate"</Code>", which the demo\u{2019}s styles animate."
                </p>
                <Demo description="Upload progress in bytes that can become indeterminate" source=include_str!("demos/progress_bar.rs")>
                    <ProgressBarAtomDemo/>
                </Demo>
            </Section>

            <Section title="ProgressBar">
                <p>"The progress bar element, a "<Code inline=true>"<div>"</Code>" holding the label, the parts and your track."</p>

                <Section title="Props" id="progress-bar-props">
                    <ApiTable kind=ApiKind::Props of="atoms::progress_bar::ProgressBar">
                        <ApiRow name="value" ty="OptionalNumberSignal<T>">
                            "The progress: a number, an "<Code inline=true>"Option"</Code>", or any signal of them, clamped to the range. "
                            <Code inline=true>"None"</Code>" is indeterminate. The number type "<Code inline=true>"T"</Code>
                            " is inferred from it. Required."
                        </ApiRow>
                        <ApiRow name="min_value" ty="Option<Signal<T>>" default="0">"The start of the range."</ApiRow>
                        <ApiRow name="max_value" ty="Option<Signal<T>>" default="100">"The end of the range."</ApiRow>
                        <ApiRow name="format_options" ty="Option<Signal<NumberFormatOptions>>" default="percent">
                            "How the value text is formatted: a percent style formats the percentage, other styles the value."
                        </ApiRow>
                        <ApiRow name="value_label" ty="MaybeProp<String>" default="None">"Replaces the value text (e.g. \u{201c}1 of 4\u{201d})."</ApiRow>
                        <ApiRow name="id" ty="Option<String>" default="None">"The element\u{2019}s id. Generated when "<Code inline=true>"None"</Code>"."</ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"Names the bar when it has no "<Code inline=true>"Label"</Code>"."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"Ids of elements naming the bar."</ApiRow>
                        <ApiRow name="aria_describedby" ty="Option<String>" default="None">"Ids of elements describing the bar."</ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the bar element."</ApiRow>
                        <ApiRow name="children" ty="Option<Children>" default="None">"The label, the parts and your track."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ProgressBarFill">
                <p>
                    "A "<Code inline=true>"<div>"</Code>" as wide as the progress, in percent of its container: put it into the "
                    "track you draw. While the bar is indeterminate, it has no width of its own; give it one on "
                    <Code inline=true>"[data-indeterminate]"</Code>"."
                </p>

                <Section title="Props" id="progress-bar-fill-props">
                    <ApiTable kind=ApiKind::Props of="ProgressBarFill">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and further styles of the fill."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ProgressBarValueText">
                <p>
                    "A "<Code inline=true>"<span>"</Code>" with the formatted value, the same text screen readers announce; "
                    "empty while the bar is indeterminate."
                </p>

                <Section title="Props" id="progress-bar-value-text-props">
                    <ApiTable kind=ApiKind::Props of="ProgressBarValueText">
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">"Classes and styles of the value text."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Data Attributes">
                <ApiTable kind=ApiKind::DataAttributes>
                    <ApiRow name="data-indeterminate" ty="true">
                        "On "<Code inline=true>"ProgressBar"</Code>" and "<Code inline=true>"ProgressBarFill"</Code>
                        ": the value is "<Code inline=true>"None"</Code>", the progress isn\u{2019}t known."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Styling">
                <p>
                    "The atoms bring no styles. They render the classes "<Code inline=true>"leptonic-ProgressBar"</Code>", "
                    <Code inline=true>"leptonic-ProgressBarFill"</Code>" and "<Code inline=true>"leptonic-ProgressBarValueText"</Code>
                    ", each followed by the "<Code inline=true>"classes"</Code>" you pass. The track is your own markup "
                    "around the fill, which the atom sizes to the progress. While indeterminate, the fill has no width: "
                    "give it one and an animation through "<Code inline=true>"data-indeterminate"</Code>". Without motion, "
                    "show something that can\u{2019}t be mistaken for a partial fill, such as a striped track. The "
                    "book\u{2019}s demos use these rules:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .my-progress-track { position: relative; height: 8px; overflow: hidden; border-radius: 4px; background: var(--border); }
                        .my-progress-fill { height: 100%; background: var(--accent); }
                        .my-progress-fill[data-indeterminate] { position: absolute; width: 40%; animation: sweep 1.5s infinite ease-in-out; }
                        @keyframes sweep { from { left: -40%; } to { left: 100%; } }

                        @media (prefers-reduced-motion: reduce) {
                            .my-progress-fill[data-indeterminate] {
                                left: 0; width: 100%; animation: none;
                                background: repeating-linear-gradient(-45deg, var(--accent) 0 0.5em, var(--surface) 0.5em 1em);
                            }
                        }
                    ")}
                </Code>
                <p>
                    "Leptonic also ships an optional atom theme that styles the default classes, for apps that don\u{2019}t "
                    "want to start from scratch: "<Code inline=true>"@use \"leptonic/leptonic-atoms\";"</Code>"."
                </p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::ProgressBar.materialize()>"Progress Bar overview"</Link></li>
                <li><Link href=routes::doc::progress_bar::Hook.materialize()>"use_progress_bar"</Link></li>
                <li><Link href=routes::doc::meter::Atom.materialize()>"Meter Atoms"</Link></li>
                <li><Link href=routes::doc::field::Atom.materialize()>"Field Atoms"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
