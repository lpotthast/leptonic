use leptonic::{
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        field::Label,
        input::Input,
        listbox::{ListBox, ListBoxItems},
    },
    components::prelude::{Button, ButtonColor, Checkbox},
    hooks::{
        collections::{Key, use_collection},
        use_contains_filter,
    },
};
use leptos::prelude::*;

const COUNTRIES: [(&str, &str); 8] = [
    ("at", "Austria"),
    ("be", "Belgium"),
    ("dk", "Denmark"),
    ("fi", "Finland"),
    ("fr", "France"),
    ("de", "Germany"),
    ("nl", "Netherlands"),
    ("se", "Sweden"),
];

#[component]
pub fn ComboBoxAtomDemo() -> impl IntoView {
    // The options. Keys identify them, texts are shown and matched against the input. Shipping to Finland is paused.
    let countries = use_collection(|b| {
        for (key, name) in COUNTRIES {
            b.item(key, name).disabled(key == "fi");
        }
    });
    // App state: the selected country and the text in the input. The combo box shows both, and the user\u{2019}s
    // typing and choosing writes them.
    let value = RwSignal::new(vec![Key::from("de")]);
    let input_value = RwSignal::new(String::from("Germany"));
    let disabled = RwSignal::new(false);

    view! {
        <ComboBox
            collection=countries
            filter=use_contains_filter()
            value=value
            input_value=input_value
            placeholder="Search countries\u{2026}"
            is_disabled=disabled
            classes="demo-combo"
        >
            <Label classes="demo-combo-label">"Ship to"</Label>
            <div class="demo-combo-field">
                <Input classes="demo-combo-input"/>
                <ComboBoxButton classes="demo-combo-button">
                    <span aria-hidden="true">"\u{25bc}"</span>
                </ComboBoxButton>
            </div>
            <ComboBoxPopover classes="demo-combo-popover">
                <ListBox classes="demo-combo-listbox">
                    <ListBoxItems classes="demo-combo-item" let:node>
                        {node.text_value.to_string()}
                    </ListBoxItems>
                </ListBox>
            </ComboBoxPopover>
        </ComboBox>

        <div class="demo-flex-center-row">
            <Checkbox state=disabled>"Disabled"</Checkbox>
            // The app changes the selection by writing its state; the input then shows the country\u{2019}s name.
            <Button on_press=move |_| value.set(vec![Key::from("se")]) color=ButtonColor::Secondary>
                "Ship to Sweden"
            </Button>
        </div>

        <p>
            "Selected: "
            <strong>{move || value.with(|keys| keys.first().map_or_else(|| "none".to_owned(), ToString::to_string))}</strong>
            ", typed: "
            <strong>{move || format!("\u{201c}{}\u{201d}", input_value.get())}</strong>
        </p>
    }
}
