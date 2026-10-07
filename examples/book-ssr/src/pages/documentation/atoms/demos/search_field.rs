use leptonic::atoms::{
    checkbox::Checkbox,
    field::Label,
    input::Input,
    search_field::{SearchField, SearchFieldClearButton},
};
use leptos::prelude::*;

#[component]
pub fn SearchFieldAtomDemo() -> impl IntoView {
    let (submitted, set_submitted) = signal(None::<String>);
    let (cleared, set_cleared) = signal(0_u32);
    let disabled = RwSignal::new(false);

    let status = move || {
        let submitted = submitted.get().map_or_else(
            || "Nothing submitted yet.".to_owned(),
            |query| format!("Submitted: \u{201c}{query}\u{201d}."),
        );
        let cleared = match cleared.get() {
            0 => "Never cleared.".to_owned(),
            1 => "Cleared 1 time.".to_owned(),
            n => format!("Cleared {n} times."),
        };
        format!("{submitted} {cleared}")
    };

    view! {
        // `data-empty` on the field hides the clear button while there is nothing to clear.
        <SearchField
            on_submit=move |query| set_submitted.set(Some(query))
            on_clear=move |()| set_cleared.update(|n| *n += 1)
            placeholder="Search recipes\u{2026}"
            is_disabled=disabled
            classes=["demo-field", "demo-search-field"]
        >
            <Label classes="demo-field-label">"Search"</Label>
            <div class="demo-input-row">
                <Input classes="demo-atom-input"/>
                // The button is labelled "Clear search"; the glyph is decoration.
                <SearchFieldClearButton classes=["demo-btn", "demo-search-field-clear"]>
                    <span aria-hidden="true">"\u{2715}"</span>
                </SearchFieldClearButton>
            </div>
        </SearchField>

        <p class="demo-status">{status}</p>

        <div class="demo-controls">
            <Checkbox is_selected=disabled set_selected=disabled classes="demo-check">
                <span class="demo-check-box" aria-hidden="true"></span>
                "Disabled"
            </Checkbox>
        </div>
    }
}
