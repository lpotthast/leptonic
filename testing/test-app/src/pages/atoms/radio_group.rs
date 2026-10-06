use leptonic::{
    atoms::{
        field::{Description, FieldError, Label},
        radio::{Radio, RadioGroup},
    },
    hooks::{Orientation, ValidationBehavior, collections::Key},
};
use leptos::prelude::*;

/// Radio groups (react-aria-components' `RadioGroup.test.js` setup: dogs, cats, dragons).
#[component]
pub fn PageAtomRadioGroup() -> impl IntoView {
    let selected = RwSignal::new(String::new());
    let bound = RwSignal::new(Some(Key::from("b")));

    view! {
        <h1>"Radio Group"</h1>
        <button id="test-rg-before">"Before"</button>
        <RadioGroup on_change={move |value: Option<Key>| {
            selected.set(value.map(|v| v.to_string()).unwrap_or_default());
        }}>
            <Label>"Favorite pet"</Label>
            <Radio value="dogs">"Dogs"</Radio>
            <Radio value="cats">"Cats"</Radio>
            <Radio value="dragons">"Dragons"</Radio>
            <Description>"Pick one."</Description>
        </RadioGroup>
        // A label outside any field: the group's label context must not reach it.
        <Label>"Standalone label"</Label>
        <div>"Selected: " <span id="test-rg-value">{selected}</span></div>
        <button id="test-rg-after">"After"</button>

        <RadioGroup aria_label="Horizontal" orientation=Orientation::Horizontal default_value="b">
            <Radio value="a">"Horizontal A"</Radio>
            <Radio value="b">"Horizontal B"</Radio>
            <Radio value="c">"Horizontal C"</Radio>
        </RadioGroup>

        <RadioGroup aria_label="Disabled radio">
            <Radio value="a">"Skip A"</Radio>
            <Radio value="b" is_disabled=true>"Skip B"</Radio>
            <Radio value="c">"Skip C"</Radio>
        </RadioGroup>

        <RadioGroup aria_label="Disabled group" is_disabled=true>
            <Radio value="a">"Disabled group A"</Radio>
        </RadioGroup>

        <RadioGroup aria_label="Read-only group" is_read_only=true default_value="a">
            <Radio value="a">"Read-only A"</Radio>
            <Radio value="b">"Read-only B"</Radio>
        </RadioGroup>

        // Controlled: bound to a signal, which a button changes from outside.
        <RadioGroup aria_label="Bound" value=bound set_value=bound>
            <Radio value="a">"Bound A"</Radio>
            <Radio value="b">"Bound B"</Radio>
            <Radio value="c">"Bound C"</Radio>
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
        <RadioGroup aria_label="Fixed" value=Some(Key::from("a"))>
            <Radio value="a">"Fixed A"</Radio>
            <Radio value="b">"Fixed B"</Radio>
        </RadioGroup>

        <form id="test-rg-form">
            <RadioGroup is_required=true validation_behavior=ValidationBehavior::Native>
                <Label>"Required"</Label>
                <Radio value="a">"Required A"</Radio>
                <Radio value="b" is_disabled=true>"Required B"</Radio>
                <FieldError />
            </RadioGroup>
        </form>
    }
}
