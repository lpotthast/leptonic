use leptonic::components::select::{Multiselect, OptionalSelect, Select};
use leptos::prelude::*;

const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

/// The styled select components: a labelled `Select` bound to app state (changed from outside
/// too), a disabled `OptionalSelect` named by `aria_label`, a clearable one, and a named `Multiselect`
/// with two chosen fruits.
#[component]
pub fn PageComponentSelect() -> impl IntoView {
    let options = Signal::stored(FRUITS.map(str::to_owned).to_vec());
    let fruit = RwSignal::new("Banana".to_owned());
    let optional = RwSignal::new(None::<String>);
    let many = RwSignal::new(vec!["Apple".to_owned(), "Cherry".to_owned()]);
    let clearable = RwSignal::new(Some("Banana".to_owned()));
    let text = |o: String| o;

    view! {
        <div id="test-page-component-select">
            <h1>"Select components"</h1>
            <div id="test-csel-single">
                <Select
                    options=options
                    selected=fruit
                    set_selected=fruit
                    search_text_provider=text
                    render_option=|o: String| o
                    label="Fruit"
                    autofocus_search=false
                />
            </div>
            <button id="test-csel-set-cherry" on:click=move |_| fruit.set("Cherry".to_owned())>
                "Set Cherry"
            </button>
            <div>"Fruit: " <span id="test-csel-fruit">{fruit}</span></div>
            <div id="test-csel-optional">
                <OptionalSelect
                    options=options
                    selected=optional
                    set_selected=optional
                    search_text_provider=text
                    render_option=|o: String| o
                    allow_deselect=true
                    aria_label="Optional fruit"
                    is_disabled=true
                    autofocus_search=false
                />
            </div>
            <div id="test-csel-clearable">
                <OptionalSelect
                    options=options
                    selected=clearable
                    set_selected=clearable
                    search_text_provider=text
                    render_option=|o: String| o
                    allow_deselect=true
                    aria_label="Clearable fruit"
                    autofocus_search=false
                />
            </div>
            <div>"Clearable: " <span id="test-csel-clearable-value">{move || clearable.get().unwrap_or_default()}</span></div>
            <form id="test-csel-form">
                <Multiselect
                    options=options
                    selected=many
                    set_selected=many
                    search_text_provider=text
                    render_option=|o: String| o
                    aria_label="Fruits"
                    name="fruits"
                    autofocus_search=false
                />
            </form>
            <div>"Many: " <span id="test-csel-many">{move || many.get().join(",")}</span></div>
        </div>
    }
}
