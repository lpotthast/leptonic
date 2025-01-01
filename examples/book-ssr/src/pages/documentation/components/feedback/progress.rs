use indoc::indoc;
use leptonic::{atoms::link::AnchorLink, components::prelude::*};
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, toc::Toc};

#[component]
pub fn PageProgress() -> impl IntoView {
    let (progress, set_progress) = signal(Some(34.0));

    view! {
        <Article>
            <h1 id="progress" class="anchor">
                "Progress"
                <AnchorLink href="#progress" description="Direct link to article header"/>
            </h1>

            <p>
                "Display how much work of an operation is already completed using the "<Code inline=true>"<ProgressBar>"</Code>" component."
            </p>

            <Code>
                {indoc!(r"
                    let (progress, set_progress) = signal(Some(34.0));

                    view! {
                        <ProgressBar progress=progress/>
                    }
                ")}
            </Code>

            <ProgressBar progress=progress/>

            <NumberInput
                get=Signal::derive(move || progress.get().unwrap_or_default())
                set=move |v| set_progress.set(Some(v))
                attr:style="margin-top: 1em;"
            />

            <Slider
                value=Signal::derive(move || progress.get().unwrap_or(0.0))
                set_value=move |v: f64| set_progress.set(Some((v * 100.0).round() / 100.0))
                min=0.0
                max=100.0
                step=0.01
            />

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

            <Code>
                {indoc!(r"
                    <ProgressBar progress=signal(None).0 />
                ")}
            </Code>

            <ProgressBar progress=signal(None).0 />

            <h2 id="styling" class="anchor">
                "Styling"
                <AnchorLink href="#styling" description="Direct link to section: Styling"/>
            </h2>

            <p>"You may overwrite any of the following CSS variables to meet your styling needs."</p>

            <Code>
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
