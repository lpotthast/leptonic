use leptonic::components::prelude::*;
use leptos::prelude::*;

const FILES: [&str; 3] = ["notes.txt", "photo.jpg", "report.pdf"];

#[component]
pub fn CheckboxIndeterminateDemo() -> impl IntoView {
    let selected = FILES.map(|_| RwSignal::new(false));
    let count = move || selected.iter().filter(|file| file.get()).count();

    // "Select all" is checked when all files are, indeterminate when some are.
    let all = Signal::derive(move || count() == FILES.len());
    let select_all = move |select: bool| selected.iter().for_each(|file| file.set(select));

    view! {
        <div class="demo-control-stack">
            <Checkbox is_selected=all set_selected=select_all is_indeterminate=Signal::derive(move || (1..FILES.len()).contains(&count()))>
                "Select all"
            </Checkbox>
            {FILES
                .into_iter()
                .zip(selected)
                .map(|(file, is_selected)| view! { <Checkbox is_selected set_selected=is_selected>{file}</Checkbox> })
                .collect_view()}
        </div>
        <p class="demo-status">{move || format!("{} of {} selected.", count(), FILES.len())}</p>
    }
}
