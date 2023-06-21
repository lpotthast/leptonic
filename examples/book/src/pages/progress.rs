use leptonic::prelude::*;
use leptos::*;

#[component]
pub fn PageProgress(cx: Scope) -> impl IntoView {
    let (progress, set_progress) = create_signal(cx, Some(34.0));
    let (progress2, set_progres2s) = create_signal(cx, None);

    let progress_str = Signal::derive(cx, move || {
        progress
            .get()
            .map(|it| it.to_string())
            .unwrap_or_else(|| "0.0".to_owned())
    });

    view! { cx,
        <H1>"Progress"</H1>



        <ProgressBar progress=progress/>

        <Input ty=InputType::Number get=progress_str set=move |v| set_progress.set(str::parse::<f64>(v.as_str()).ok()) />

        <P>"Indeterminate"</P>

        <ProgressBar progress=progress2/>
    }
}
