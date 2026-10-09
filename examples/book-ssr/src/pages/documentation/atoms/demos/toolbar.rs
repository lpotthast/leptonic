use leptonic::{Orientation, atoms};
use leptos::prelude::*;

#[component]
pub fn ToolbarAtomDemo() -> impl IntoView {
    let bold = RwSignal::new(false);
    let italic = RwSignal::new(false);

    view! {
        <atoms::toolbar::Toolbar aria_label="Text formatting" classes="demo-toolbar">
            <atoms::toggle_button::ToggleButton is_selected=bold set_selected=bold classes="demo-toolbar-atom-button">
                "Bold"
            </atoms::toggle_button::ToggleButton>
            <atoms::toggle_button::ToggleButton is_selected=italic set_selected=italic classes="demo-toolbar-atom-button">
                "Italic"
            </atoms::toggle_button::ToggleButton>
            // A horizontal toolbar divides its groups with vertical separators.
            <atoms::separator::Separator orientation=Orientation::Vertical classes="demo-toolbar-separator"/>
            <atoms::button::Button
                classes="demo-toolbar-atom-button"
                on_press=move |_| {
                    bold.set(false);
                    italic.set(false);
                }
            >
                "Clear"
            </atoms::button::Button>
        </atoms::toolbar::Toolbar>

        <p class="demo-toolbar-preview" data-bold=bold data-italic=italic>
            "The quick brown fox jumps over the lazy dog."
        </p>
        <p class="demo-status">
            {move || match (bold.get(), italic.get()) {
                (false, false) => "No formatting.",
                (true, false) => "Formatting: bold.",
                (false, true) => "Formatting: italic.",
                (true, true) => "Formatting: bold, italic.",
            }}
        </p>
    }
}
