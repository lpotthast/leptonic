use leptonic::components::tiptap_editor::TiptapEditor;
use leptos::prelude::*;

/// The styled `TiptapEditor`: initial content, edits counted in `#test-ctip-changes`; and one bound
/// to app state, replaced from outside by `#test-ctip-replace`.
#[component]
pub fn PageComponentTiptapEditor() -> impl IntoView {
    let changes = RwSignal::new(0_u32);
    let html = RwSignal::new(String::new());
    let bound = RwSignal::new("<p>Bound</p>".to_owned());
    view! {
        <div id="test-page-component-tiptap-editor">
            <h1>"Tiptap editor component"</h1>
            <div id="test-ctip">
                <TiptapEditor
                    default_value="<p>Hello world</p>"
                    aria_label="Notes"
                    on_change=move |content: String| {
                        html.set(content);
                        changes.update(|c| *c += 1);
                    }
                />
            </div>
            <div>"Changes: " <span id="test-ctip-changes">{changes}</span></div>
            <div>"HTML: " <span id="test-ctip-html">{html}</span></div>
            <div id="test-ctip-bound">
                <TiptapEditor value=bound set_value=bound aria_label="Bound" />
            </div>
            <button id="test-ctip-replace" on:click=move |_| bound.set("<p>Replaced</p>".to_owned())>
                "Replace"
            </button>
            <div>"Bound: " <span id="test-ctip-bound-html">{bound}</span></div>
        </div>
    }
}
