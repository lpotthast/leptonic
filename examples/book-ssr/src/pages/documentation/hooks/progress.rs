use crate::pages::documentation::article::Article;
use crate::pages::documentation::toc::Toc;
use leptonic::atoms::link::AnchorLink;
use leptonic::components::prelude::*;
use leptonic::hooks::*;
use leptonic::prelude::Size;
use leptos::prelude::*;

#[component]
pub fn PageUseProgressBar() -> impl IntoView {
    let (value, set_value) = signal(65.0);

    let UseProgressBarReturn {
        progress_props,
        label_props,
        percentage: _,
        value_label: _,
        is_indeterminate: _,
        progress_id: _,
    } = use_progress_bar(UseProgressBarInput {
        label: Some("Loading progress".into()),
        value: Signal::derive(move || Some(value.get())),
        min_value: 0.0,
        max_value: 100.0,
        show_value_label: true,
        is_indeterminate: false,
    });

    let UseProgressBarReturn {
        progress_props: indeterminate_props,
        ..
    } = use_progress_bar(UseProgressBarInput {
        label: Some("Loading".into()),
        value: Signal::derive(|| None),
        min_value: 0.0,
        max_value: 100.0,
        show_value_label: false,
        is_indeterminate: true,
    });

    view! {
        <Article>
            <h1 id="use_progress_bar" class="anchor">
                "use_progress_bar"
                <AnchorLink href="#use_progress_bar" description="Direct link to article header"/>
            </h1>

            <p>"Hook for creating accessible progress indicators with support for determinate and indeterminate states."</p>

            <h2 id="determinate" class="anchor">
                "Determinate Progress"
                <AnchorLink href="#determinate" description="Direct link to determinate"/>
            </h2>

            <p>"Shows a specific progress value:"</p>

            <div style="margin: 1em 0; max-width: 400px;">
                <div style="display: flex; justify-content: space-between; margin-bottom: 0.5em;">
                    <label id=label_props.id.clone()>"Loading progress"</label>
                    <span>{ move || format!("{}%", value.get() as i32) }</span>
                </div>
                <div
                    {..progress_props}
                    style="height: 8px; background: #ddd; border-radius: 4px; overflow: hidden;"
                >
                    <div style=move || format!(
                        "height: 100%; background: var(--brand-color); transition: width 0.3s; width: {}%;",
                        value.get()
                    )></div>
                </div>
            </div>

            <Stack orientation=StackOrientation::Horizontal spacing=Size::Em(0.5)>
                <button
                    on:click=move |_| set_value.update(|v| *v = (*v - 10.0).max(0.0))
                    style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                >
                    "-10%"
                </button>
                <button
                    on:click=move |_| set_value.update(|v| *v = (*v + 10.0).min(100.0))
                    style="padding: 0.5em 1em; border-radius: 4px; cursor: pointer; border: 1px solid #ccc;"
                >
                    "+10%"
                </button>
            </Stack>

            <Code>
                {r#"let UseProgressBarReturn { progress_props, label_props, .. } = use_progress_bar(
    UseProgressBarInput {
        label: Some("Loading progress".into()),
        value: Signal::derive(|| Some(65.0)),
        min_value: 0.0,
        max_value: 100.0,
        show_value_label: true,
        is_indeterminate: false,
    }
);"#}
            </Code>

            <h2 id="indeterminate" class="anchor">
                "Indeterminate Progress"
                <AnchorLink href="#indeterminate" description="Direct link to indeterminate"/>
            </h2>

            <p>"For unknown progress duration:"</p>

            <div style="margin: 1em 0; max-width: 400px;">
                <div
                    {..indeterminate_props}
                    style="height: 8px; background: #ddd; border-radius: 4px; overflow: hidden; position: relative;"
                >
                    <div style="
                        position: absolute;
                        width: 40%;
                        height: 100%;
                        background: var(--brand-color);
                        animation: indeterminate 1.5s infinite ease-in-out;
                    "></div>
                </div>
            </div>

            <style>
                r"@keyframes indeterminate {
                    0% { left: -40%; }
                    100% { left: 100%; }
                }"
            </style>

            <Code>
                {r#"let UseProgressBarReturn { progress_props, .. } = use_progress_bar(
    UseProgressBarInput {
        label: Some("Loading".into()),
        value: Signal::derive(|| None), // No specific value
        min_value: 0.0,
        max_value: 100.0,
        show_value_label: false,
        is_indeterminate: true,
    }
);"#}
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
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "use_progress_bar", link: "#use_progress_bar" },
                Toc::Leaf { title: "Determinate Progress", link: "#determinate" },
                Toc::Leaf { title: "Indeterminate Progress", link: "#indeterminate" },
                Toc::Leaf { title: "ARIA Attributes", link: "#aria-attributes" },
                Toc::Leaf { title: "Features", link: "#features" },
            ]
        }/>
    }
}
