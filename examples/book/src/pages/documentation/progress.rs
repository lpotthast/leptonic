use indoc::indoc;
use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageProgress(cx: Scope) -> impl IntoView {
    let (progress, set_progress) = create_signal(cx, Some(34.0));

    let progress_str = Signal::derive(cx, move || {
        progress
            .get()
            .map(|it| it.to_string())
            .unwrap_or_else(|| "0.0".to_owned())
    });

    view! { cx,
        <H1>"Progress"</H1>

        <Code>
            {indoc!(r#"
                let (progress, set_progress) = create_signal(cx, Some(34.0));

                view! {cx,
                    <ProgressBar progress=progress/>
                }
            "#)}
        </Code>

        <ProgressBar progress=progress/>

        <Input ty=InputType::Number get=progress_str set=move |v| set_progress.set(str::parse::<f64>(v.as_str()).ok()) style="margin-top: 1em;"/>

        <H2>"Indeterminate state"</H2>

        <P>
            "As already seen, the progress is stored as "<Code inline=true>"Option<T>"</Code>". "
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
                --progress-bar-color
            "#)}
        </Code>
    }
}
