use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::progress_controlled::ProgressControlledDemo;
use super::demos::progress_indeterminate::ProgressIndeterminateDemo;
use crate::pages::documentation::demo_shell::DemoShell;
use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageProgress() -> impl IntoView {
    view! {
        <Article>
            <h1 id="progress" class="anchor">
                "Progress"
                <AnchorLink href="#progress" description="Direct link to article header"/>
            </h1>

            <p>
                "Display how much work of an operation is already completed using the "<Code inline=true>"<ProgressBar>"</Code>" component."
            </p>

            <DemoShell source=include_str!("demos/progress_controlled.rs")>
                <ProgressControlledDemo />
            </DemoShell>

            <h2 id="indeterminate-state" class="anchor">
                "Indeterminate state"
                <AnchorLink href="#indeterminate-state" description="Direct link to section: Indeterminate state"/>
            </h2>

            <p>
                "As you have probably spotted in the above example, progress is stored as "<Code inline=true>"Option<T>"</Code>". "
                "In our earlier example, we always had "<Code inline=true>"Some(progress)"</Code>" which the progress bar displayed for us. "
                "Whenever the signal stores a "<Code inline=true>"None"</Code>" value, the progress bar is in the "<Code inline=true>"indeterminate"</Code>" state, "
                "telling the user that something is going on, but we cannot exactly say how much of the total work already completed."
            </p>

            <DemoShell source=include_str!("demos/progress_indeterminate.rs")>
                <ProgressIndeterminateDemo />
            </DemoShell>

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code language=Language::Rust>
                {indoc!(r"
                    --progress-bar-height
                    --progress-bar-border-radius
                    --progress-bar-background-color
                    --progress-bar-background-color-transparent
                    --progress-bar-background-box-shadow
                    --progress-bar-fill-background-color
                    --progress-bar-fill-transition
                    --progress-bar-color
                ")}
            </Code>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Progress", link: "#progress" },
                Toc::Leaf { title: "Indeterminate state", link: "#indeterminate-state" },
                Toc::Leaf { title: "Styling", link: "#styling" },
            ]
        }/>
    }
}
