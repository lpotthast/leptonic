use leptonic::{components::prelude::*, utils::use_description::use_description};
use leptos::prelude::*;

#[component]
pub fn UseDescriptionDemo() -> impl IntoView {
    let is_described = RwSignal::new(true);
    let deleted = RwSignal::new(0u32);

    // The id of a hidden element containing the text, while there is a text.
    let description_id = use_description(Signal::derive(move || {
        is_described
            .get()
            .then(|| String::from("Deletes the draft permanently. You can\u{2019}t undo this."))
    }));

    view! {
        <Button
            on_press=move |_| deleted.update(|deleted| *deleted += 1)
            attr:aria-describedby=move || description_id.get()
        >
            "Delete draft"
        </Button>
        <div class="demo-controls">
            <Checkbox is_selected=is_described set_selected=is_described>"Describe the button"</Checkbox>
        </div>
        <p class="demo-status">
            {move || match description_id.get() {
                Some(id) => format!("aria-describedby=\"{id}\""),
                None => String::from("No aria-describedby."),
            }}
            " "
            {move || match deleted.get() {
                1 => String::from("Pressed 1 time."),
                n => format!("Pressed {n} times."),
            }}
        </p>
    }
}
