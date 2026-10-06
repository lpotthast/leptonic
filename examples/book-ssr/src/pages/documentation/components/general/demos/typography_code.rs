use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TypographyCodeDemo() -> impl IntoView {
    let (copied, set_copied) = signal(None::<Result<(), ()>>);

    view! {
        <Code language=Language::Rust on_copy=move |result| set_copied.set(Some(result))>
            "let answer = 42;"
        </Code>
        <p>"A paragraph with " <Code inline=true>"inline"</Code> " code."</p>
        <p class="demo-status">
            {move || match copied.get() {
                None => "Not copied yet.",
                Some(Ok(())) => "Copied to the clipboard.",
                Some(Err(())) => "Copying failed.",
            }}
        </p>
    }
}
