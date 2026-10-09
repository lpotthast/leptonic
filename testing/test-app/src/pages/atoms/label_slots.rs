use leptonic::{
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        field::Label,
        input::Input,
        listbox::{ListBox, ListBoxItem, ListBoxItems},
        number_field::{
            NumberField, NumberFieldDecrementButton, NumberFieldGroup, NumberFieldIncrementButton,
        },
        select::{Select, SelectPopover, SelectTrigger, SelectValue},
        slider::{Slider, SliderThumb, SliderTrack},
    },
    hooks::collections::{Key, UseListCollectionInput, use_list_collection},
};
use leptos::prelude::*;

const FRUITS: [&str; 3] = ["Apple", "Banana", "Cherry"];

/// Fields whose label is rendered only after a toggle: the parts naming themselves after the
/// label (the select's trigger, the combo box's button, the number field's steppers, the slider's
/// thumb) follow its presence.
#[component]
pub fn PageAtomLabelSlots() -> impl IntoView {
    let shown = RwSignal::new(false);
    let fruits = move || {
        use_list_collection(UseListCollectionInput {
            items: Signal::stored(FRUITS.to_vec()),
            key: |fruit| Key::from(*fruit),
            text_value: |fruit| (*fruit).to_owned(),
        })
    };
    let label = move |text: &'static str| {
        view! {
            <Show when=move || shown.get()>
                <Label>{text}</Label>
            </Show>
        }
    };

    view! {
        <div id="test-page-atom-label-slots">
            <h1>"Label slots"</h1>
            <button id="test-label-slots-toggle" on:click=move |_| shown.update(|s| *s = !*s)>
                "Toggle labels"
            </button>
            <div id="test-ls-select">
                <Select<Option<Key>> collection=fruits()>
                    {label("Fruit")}
                    <SelectTrigger>
                        <SelectValue placeholder="Pick a fruit" />
                    </SelectTrigger>
                    <SelectPopover>
                        <ListBox>
                            {FRUITS
                                .map(|fruit| view! { <ListBoxItem key=fruit>{fruit}</ListBoxItem> })
                                .collect_view()}
                        </ListBox>
                    </SelectPopover>
                </Select<Option<Key>>>
            </div>
            <div id="test-ls-combobox">
                <ComboBox<Option<Key>> collection=fruits()>
                    {label("Fruit")}
                    <Input />
                    <ComboBoxButton>"▼"</ComboBoxButton>
                    <ComboBoxPopover>
                        <ListBox>
                            <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                        </ListBox>
                    </ComboBoxPopover>
                </ComboBox<Option<Key>>>
            </div>
            <div id="test-ls-number-field">
                <NumberField default_value=1_i32>
                    {label("Width")}
                    <NumberFieldGroup>
                        <NumberFieldDecrementButton>"-"</NumberFieldDecrementButton>
                        <Input />
                        <NumberFieldIncrementButton>"+"</NumberFieldIncrementButton>
                    </NumberFieldGroup>
                </NumberField>
            </div>
            <div id="test-ls-slider">
                <Slider min_value=0 max_value=10 default_values=vec![5_i32]>
                    {label("Volume")}
                    <SliderTrack>
                        <SliderThumb />
                    </SliderTrack>
                </Slider>
            </div>
        </div>
    }
}
