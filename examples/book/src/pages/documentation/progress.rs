use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageProgress(cx: Scope) -> impl IntoView {
    let (progress, set_progress) = create_signal(cx, Some(34.0));

    view! { cx,
        <H1>"Progress"</H1>

        <P>
            "Display how much work of an operation is already completed using the "<Code inline=true>"<ProgressBar>"</Code>" component."
        </P>

        <Code>
            {indoc!(r#"
                let (progress, set_progress) = create_signal(cx, Some(34.0));

                view! {cx,
                    <ProgressBar progress=progress/>
                }
            "#)}
        </Code>

        <ProgressBar progress=progress/>

        <NumberInput
            get=Signal::derive(cx, move || progress.get().unwrap_or_default())
            set=create_callback(cx, move |v| set_progress.set(Some(v)))
            style="margin-top: 1em;"
        />

        <Slider
            value=Signal::derive(cx, move || progress.get().unwrap_or(0.0))
            set_value=create_callback(cx, move |v: f64| set_progress.set(Some((v * 100.0).round() / 100.0)))
            min=0.0
            max=100.0
            step=0.01
        />

        <H2>"Indeterminate state"</H2>

        <P>
            "As you have probably spotted in the above example, progress is stored as "<Code inline=true>"Option<T>"</Code>". "
            "In our earlier example, we always had "<Code inline=true>"Some(progress)"</Code>" which the progress bar displayed for us. "
            "Whenever the signal stores a "<Code inline=true>"None"</Code>" value, the progress bar is in the "<Code inline=true>"indeterminate"</Code>" state, "
            "telling the user that something is going on, but we cannot exactly say how much of the total work already completed."
        </P>

        <Code>
            {indoc!(r#"
                <ProgressBar progress=create_signal(cx, None).0 />
            "#)}
        </Code>

        <ProgressBar progress=create_signal(cx, None).0 />

        <H2>"Styling"</H2>

        <P>"You may overwrite any of the following CSS variables to meet your styling needs."</P>

        <Code>
            {indoc!(r#"
                --progress-bar-height
                --progress-bar-border-radius
                --progress-bar-background-color
                --progress-bar-background-color-transparent
                --progress-bar-background-box-shadow
                --progress-bar-fill-background-color
                --progress-bar-fill-transition
                --progress-bar-color
            "#)}
        </Code>
    }
}
