use leptonic::hooks::*;
use leptos::prelude::*;

#[component]
pub fn ToolbarVerticalDemo() -> impl IntoView {
    let vertical_toolbar = use_toolbar(UseToolbarInput {
        label: Some("Actions".to_string()),
        orientation: ToolbarOrientation::Vertical,
        ..Default::default()
    });

    view! {
        <div
            {..vertical_toolbar.toolbar_props.into_attrs()}
            style="display: flex; flex-direction: column; gap: 0.5em; padding: 0.5em; background: #f5f5f5; border-radius: 4px; width: fit-content;"
        >
            <button style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; background: white;">
                "New"
            </button>
            <button style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; background: white;">
                "Save"
            </button>
            <button style="padding: 0.5em 1em; border: 1px solid #ccc; border-radius: 4px; cursor: pointer; background: white;">
                "Export"
            </button>
        </div>
    }
}
