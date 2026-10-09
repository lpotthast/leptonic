use leptonic::{
    Orientation,
    atoms::{
        field::{Description, FieldError, Label},
        radio::{RadioButton, RadioField, RadioGroup},
    },
    hooks::{collections::Key, form::ValidationBehavior},
};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Size {
    Small,
    Large,
}

leptonic::selection_value!(Size { Small = "s", Large = "l" });

/// Radio groups (react-aria-components' `RadioGroup.test.js` setup: dogs, cats, dragons).
#[component]
pub fn PageAtomRadioGroup() -> impl IntoView {
    let selected = RwSignal::new(String::new());
    let size = RwSignal::new(Some(Size::Small));
    let bound = RwSignal::new(Some(Key::from("b")));

    view! {
        <h1>"Radio Group"</h1>
        <button id="test-rg-before">"Before"</button>
        <RadioGroup on_change={move |value: Option<Key>| {
            selected.set(value.map(|v| v.to_string()).unwrap_or_default());
        }}>
            <Label>"Favorite pet"</Label>
            <RadioField value="dogs"><RadioButton>"Dogs"</RadioButton></RadioField>
            <RadioField value="cats"><RadioButton>"Cats"</RadioButton></RadioField>
            <RadioField value="dragons"><RadioButton>"Dragons"</RadioButton></RadioField>
            <Description>"Pick one."</Description>
        </RadioGroup>
        // A label outside any field: the group's label context must not reach it.
        <Label>"Standalone label"</Label>
        <div>"Selected: " <span id="test-rg-value">{selected}</span></div>
        <button id="test-rg-after">"After"</button>

        <RadioGroup aria_label="Horizontal" orientation=Orientation::Horizontal default_value=Key::from("b")>
            <RadioField value="a"><RadioButton>"Horizontal A"</RadioButton></RadioField>
            <RadioField value="b"><RadioButton>"Horizontal B"</RadioButton></RadioField>
            <RadioField value="c"><RadioButton>"Horizontal C"</RadioButton></RadioField>
        </RadioGroup>

        <RadioGroup<Key> aria_label="Disabled radio">
            <RadioField value="a"><RadioButton>"Skip A"</RadioButton></RadioField>
            <RadioField value="b" is_disabled=true><RadioButton>"Skip B"</RadioButton></RadioField>
            <RadioField value="c"><RadioButton>"Skip C"</RadioButton></RadioField>
        </RadioGroup<Key>>

        <RadioGroup<Key> aria_label="Disabled group" is_disabled=true>
            <RadioField value="a"><RadioButton>"Disabled group A"</RadioButton></RadioField>
        </RadioGroup<Key>>

        <RadioGroup aria_label="Read-only group" is_read_only=true default_value=Key::from("a")>
            <RadioField value="a"><RadioButton>"Read-only A"</RadioButton></RadioField>
            <RadioField value="b"><RadioButton>"Read-only B"</RadioButton></RadioField>
        </RadioGroup>

        // Controlled: bound to a signal, which a button changes from outside.
        <RadioGroup aria_label="Bound" value=bound set_value=bound>
            <RadioField value="a"><RadioButton>"Bound A"</RadioButton></RadioField>
            <RadioField value="b"><RadioButton>"Bound B"</RadioButton></RadioField>
            <RadioField value="c"><RadioButton>"Bound C"</RadioButton></RadioField>
        </RadioGroup>
        <div>
            "Bound: "
            <span id="test-rg-bound-value">
                {move || bound.get().map(|v| v.to_string()).unwrap_or_default()}
            </span>
        </div>
        <button id="test-rg-bound-set-c" on:click=move |_| bound.set(Some(Key::from("c")))>
            "Select C"
        </button>
        // A value without a setter: selecting changes nothing.
        <RadioGroup<Key> aria_label="Fixed" value=Some(Key::from("a"))>
            <RadioField value="a"><RadioButton>"Fixed A"</RadioButton></RadioField>
            <RadioField value="b"><RadioButton>"Fixed B"</RadioButton></RadioField>
        </RadioGroup<Key>>

        // Typed values: an enum (`selection_value!`), its key submitted with the form.
        <form id="test-rg-typed-form">
            <RadioGroup aria_label="Typed" name="size" value=size set_value=size>
                <RadioField value=Size::Small><RadioButton>"Typed small"</RadioButton></RadioField>
                <RadioField value=Size::Large><RadioButton>"Typed large"</RadioButton></RadioField>
            </RadioGroup>
        </form>
        <div>"Typed: " <span id="test-rg-typed-value">{move || format!("{:?}", size.get())}</span></div>

        <form id="test-rg-form">
            <RadioGroup<Key> is_required=true validation_behavior=ValidationBehavior::Native>
                <Label>"Required"</Label>
                <RadioField value="a"><RadioButton>"Required A"</RadioButton></RadioField>
                <RadioField value="b" is_disabled=true><RadioButton>"Required B"</RadioButton></RadioField>
                <FieldError />
            </RadioGroup<Key>>
        </form>
        <form id="test-rg-keyboard-form">
            <RadioGroup<Key> aria_label="Keyboard pets" is_required=true validation_behavior=ValidationBehavior::Native>
                <RadioField value="dogs"><RadioButton>"Keyboard dogs"</RadioButton></RadioField>
                <RadioField value="cats"><RadioButton>"Keyboard cats"</RadioButton></RadioField>
                <RadioField value="dragons"><RadioButton>"Keyboard dragons"</RadioButton></RadioField>
                <FieldError />
            </RadioGroup<Key>>
        </form>
    }
}
