use leptonic::{components::prelude::*, hooks::ToggleState};
use leptos::prelude::*;

const FILES: [&str; 3] = ["notes.txt", "photo.jpg", "report.pdf"];

#[component]
pub fn CheckboxIndeterminateDemo() -> impl IntoView {
    let selected = FILES.map(|_| RwSignal::new(false));
    let count = move || selected.iter().filter(|file| file.get()).count();

    // "Select all" is checked when all files are, indeterminate when some are.
    let all = ToggleState::new(
        Signal::derive(move || count() == FILES.len()),
        false,
        Callback::new(move |select: bool| selected.iter().for_each(|file| file.set(select))),
    );

    view! {
        <div class="demo-control-stack">
            <Checkbox state=all is_indeterminate=Signal::derive(move || (1..FILES.len()).contains(&count()))>
                "Select all"
            </Checkbox>
            {FILES
                .into_iter()
                .zip(selected)
                .map(|(file, state)| view! { <Checkbox state>{file}</Checkbox> })
                .collect_view()}
        </div>
        <p class="demo-status">{move || format!("{} of {} selected", count(), FILES.len())}</p>
    }
}
