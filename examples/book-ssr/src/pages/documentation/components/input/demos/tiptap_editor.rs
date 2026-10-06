use leptonic::components::prelude::*;
use leptos::prelude::*;

const NOTES: &str = "<h2>Trip to the coast</h2>\
<p>We leave on <strong>Friday at 8\u{a0}am</strong> and are back on Sunday evening. Pack a warm jacket: the \
forecast says <mark>wind and rain</mark> for Saturday.</p>\
<blockquote><p>The best view is from the lighthouse, and it\u{2019}s open until sunset.</p></blockquote>";

#[component]
pub fn TiptapEditorDemo() -> impl IntoView {
    // The latest content the editor reported. Without `value`, the editor keeps its content itself: `default_value` is
    // only where it starts.
    let (html, set_html) = signal(None::<String>);
    let (changes, set_changes) = signal(0_u32);
    let disabled = RwSignal::new(false);

    view! {
        <TiptapEditor
            default_value=NOTES
            on_change=move |content: String| {
                set_html.set(Some(content));
                set_changes.update(|changes| *changes += 1);
            }
            is_disabled=disabled
        />

        <p class="demo-status">
            {move || match (changes.get(), html.with(|html| html.as_ref().map(String::len))) {
                (1, Some(length)) => format!("Edited 1 time; the HTML has {length} characters."),
                (changes, Some(length)) => format!("Edited {changes} times; the HTML has {length} characters."),
                (_, None) => "Not edited yet.".to_owned(),
            }}
        </p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled>"Disabled"</Checkbox>
        </div>
    }
}
