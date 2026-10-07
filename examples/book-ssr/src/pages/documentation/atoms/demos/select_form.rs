use leptonic::{
    atoms::{
        button::Button,
        field::Label,
        form::Form,
        listbox::{ListBox, ListBoxItems},
        select::{HiddenSelect, Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::{
        ButtonType,
        collections::{Key, use_list_collection},
    },
};
use leptos::{ev::SubmitEvent, prelude::*, wasm_bindgen::JsCast};

const SIZES: [(&str, &str); 5] = [
    ("s", "Small"),
    ("m", "Medium"),
    ("l", "Large"),
    ("xl", "X-Large"),
    ("xxl", "XX-Large"),
];

#[component]
pub fn SelectFormDemo() -> impl IntoView {
    let sizes = use_list_collection(
        Signal::stored(SIZES.to_vec()),
        |(key, _)| Key::from(*key),
        |(_, label)| (*label).to_owned(),
    );
    let submitted = RwSignal::new(None::<String>);

    // Reads the submitted value the way a server would receive it: from the form data.
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        let Some(form) = e
            .current_target()
            .and_then(|target| target.dyn_into::<web_sys::HtmlFormElement>().ok())
        else {
            return;
        };
        let Ok(data) = web_sys::FormData::new_with_form(&form) else {
            return;
        };
        submitted.set(Some(data.get("size").as_string().unwrap_or_default()));
    };

    view! {
        <Form classes="demo-sel-form" on:submit=on_submit on:reset=move |_| submitted.set(None)>
            // `name` makes the select a form field; `HiddenSelect` holds its value.
            <Select collection=sizes name="size" default_value=vec![Key::from("m")] classes="demo-sel">
                <Label classes="demo-sel-label">"T-shirt size"</Label>
                <SelectTrigger classes="demo-sel-trigger">
                    <SelectValue classes="demo-sel-value"/>
                    <span class="demo-sel-caret" aria-hidden="true">"\u{25bc}"</span>
                </SelectTrigger>
                <SelectPopover classes="demo-sel-popover">
                    <ListBox classes="demo-sel-listbox">
                        <ListBoxItems classes="demo-sel-item" let:node>{node.text_value.to_string()}</ListBoxItems>
                    </ListBox>
                </SelectPopover>
                <HiddenSelect/>
            </Select>
            <div class="demo-flex-center-row">
                <Button button_type=ButtonType::Submit classes="demo-btn-primary">"Submit"</Button>
                <Button button_type=ButtonType::Reset classes="demo-btn">"Reset"</Button>
            </div>
        </Form>

        <p class="demo-status">
            {move || submitted.get().map_or_else(|| "Not submitted yet.".to_owned(), |size| format!("Submitted: size={size}."))}
        </p>
    }
}
