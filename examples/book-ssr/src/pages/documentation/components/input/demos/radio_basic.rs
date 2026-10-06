use leptonic::{components::prelude::*, hooks::Key};
use leptos::prelude::*;

#[component]
pub fn RadioBasicDemo() -> impl IntoView {
    let answer = RwSignal::new(None::<Key>);

    view! {
        <RadioGroup aria_label="Answer" value=answer set_value=answer>
            <Radio value="yes">"Yes"</Radio>
            <Radio value="no">"No"</Radio>
        </RadioGroup>
        <p class="demo-status">
            {move || answer.get().map_or_else(|| "Nothing selected.".to_owned(), |answer| format!("Answer: {answer}."))}
        </p>
    }
}
