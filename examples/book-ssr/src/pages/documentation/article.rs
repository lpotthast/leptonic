use leptos::prelude::*;
use leptos_router::hooks::use_location;

#[component]
pub fn Article(
    children: Children,
    #[prop(optional)] documents: Option<Vec<&'static str>>,
) -> impl IntoView {
    let data_docs = documents.map(|d| d.join(","));
    view! {
        <article style="width: 100%; height: 100%; position: relative;"
                 data-documents=data_docs>
            <CopyAsMarkdownButton/>
            { children() }
        </article>
    }
}

#[component]
fn CopyAsMarkdownButton() -> impl IntoView {
    let location = use_location();
    let (copied, set_copied) = signal(false);

    let on_click = move |_| {
        let path = location.pathname.get_untracked();
        let md_url = format!("{path}.md");
        #[cfg(not(feature = "ssr"))]
        {
            use wasm_bindgen::prelude::*;
            use wasm_bindgen_futures::JsFuture;

            leptos::task::spawn_local(async move {
                let window = web_sys::window().expect("window should be available on client");
                let Ok(resp) = JsFuture::from(window.fetch_with_str(&md_url)).await else {
                    return;
                };
                let resp: web_sys::Response = resp.unchecked_into();
                let text = match resp.text() {
                    Ok(p) => match JsFuture::from(p).await {
                        Ok(t) => t,
                        Err(_) => return,
                    },
                    Err(_) => return,
                };
                if let Some(text) = text.as_string() {
                    let clipboard = window.navigator().clipboard();
                    let _ = JsFuture::from(clipboard.write_text(&text)).await;
                    set_copied.set(true);
                    set_timeout(
                        move || set_copied.set(false),
                        std::time::Duration::from_millis(2000),
                    );
                }
            });
        }
        #[cfg(feature = "ssr")]
        {
            let _ = (md_url, set_copied);
        }
    };

    view! {
        <button
            on:click=on_click
            title="Copy page as Markdown"
            class="copy-md-button"
        >
            {move || if copied.get() { "Copied!" } else { "Copy as MD" }}
        </button>
    }
}
