use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::progress_determinate::ProgressDeterminateDemo;
use super::demos::progress_indeterminate::ProgressIndeterminateDemo;

#[component]
pub fn PageUseProgressBar() -> impl IntoView {
    view! {
        <Article>
            <h1 id="use_progress_bar" class="anchor">
                "use_progress_bar"
                <AnchorLink href="#use_progress_bar" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible progress indicators with support for determinate and indeterminate states. "
               "See the "<Link href=crate::routes::doc::Progress.materialize()>"Progress overview"</Link>" for concept guidance."</p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useProgressBar.html" target=LinkTarget::_Blank>
                    "useProgressBar"
                </LinkExt>
                "."
            </p>

            <h2 id="determinate" class="anchor">
                "Determinate Progress"
                <AnchorLink href="#determinate" description="Direct link to determinate"/>
            </h2>

            <p>"Shows a specific progress value:"</p>

            <DemoShell source=include_str!("demos/progress_determinate.rs")>
                <ProgressDeterminateDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseProgressBarReturn { progress_props, label_props, .. } = use_progress_bar(
                        UseProgressBarInput {
                            label: Some("Loading progress".into()),
                            value: Signal::derive(|| Some(65.0)),
                            min_value: 0.0,
                            max_value: 100.0,
                            show_value_label: true,
                            is_indeterminate: false,
                        }
                    );
                "#)}
            </Code>

            <h2 id="indeterminate" class="anchor">
                "Indeterminate Progress"
                <AnchorLink href="#indeterminate" description="Direct link to indeterminate"/>
            </h2>

            <p>"For unknown progress duration:"</p>

            <DemoShell source=include_str!("demos/progress_indeterminate.rs")>
                <ProgressIndeterminateDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let UseProgressBarReturn { progress_props, .. } = use_progress_bar(
                        UseProgressBarInput {
                            label: Some("Loading".into()),
                            value: Signal::derive(|| None), // No specific value
                            min_value: 0.0,
                            max_value: 100.0,
                            show_value_label: false,
                            is_indeterminate: true,
                        }
                    );
                "#)}
            </Code>

            <h2 id="aria-attributes" class="anchor">
                "ARIA Attributes"
                <AnchorLink href="#aria-attributes" description="Direct link to ARIA attributes"/>
            </h2>

            <p>"The hook automatically sets:"</p>
            <ul>
                <li><code>"role=\"progressbar\""</code></li>
                <li><code>"aria-valuenow"</code> " (current value)"</li>
                <li><code>"aria-valuemin"</code> " (minimum value)"</li>
                <li><code>"aria-valuemax"</code> " (maximum value)"</li>
                <li><code>"aria-valuetext"</code> " (human-readable value)"</li>
                <li><code>"aria-label"</code> " or " <code>"aria-labelledby"</code></li>
            </ul>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Determinate progress with specific value"</li>
                <li>"Indeterminate progress for unknown duration"</li>
                <li>"Customizable min/max values"</li>
                <li>"Label association"</li>
                <li>"Proper ARIA progressbar attributes"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Progress.materialize()>"Progress overview"</Link></li>
                <li><Link href=crate::routes::doc::progress::Component.materialize()>"Progress component"</Link></li>
                <li><Link href=crate::routes::doc::hooks::UseMeter.materialize()>"use_meter"</Link></li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_progress_bar", link: "#use_progress_bar" },
                Toc::Leaf { title: "Determinate Progress", link: "#determinate" },
                Toc::Leaf { title: "Indeterminate Progress", link: "#indeterminate" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
