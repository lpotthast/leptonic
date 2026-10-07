use leptonic::{
    atoms::{
        checkbox::{Checkbox, CheckboxButton, CheckboxField, CheckboxGroup},
        field::{Description, FieldError, Label},
        radio::{Radio, RadioButton, RadioField, RadioGroup},
        switch::{Switch, SwitchButton, SwitchField},
    },
    hooks::{Orientation, ValidationBehavior, collections::Key},
    utils::i18n::{I18nProvider, Locale},
};
use leptos::{ev::SubmitEvent, prelude::*};

/// Checkboxes, checkbox groups, radio groups and switches in forms: form reset (also a canceled
/// one), implicit submission with Enter, right-to-left arrow keys, realtime re-validation, and
/// the field atoms with their own description and error message (react-aria-components'
/// `Checkbox`/`RadioGroup`/`Switch` tests, react-aria's `useFormReset` and `useCheckboxGroup`
/// tests).
#[component]
pub fn PageAtomForms() -> impl IntoView {
    let submits = RwSignal::new(0_u32);
    let on_submit = move |e: SubmitEvent| {
        e.prevent_default();
        submits.update(|n| *n += 1);
    };
    let rtl = "ar-AE".parse::<Locale>().expect("a locale");

    view! {
        <h1>"Forms"</h1>

        // Every control changes from its default; resetting the form restores them.
        <form id="fm-reset">
            <Checkbox name="terms">"Terms"</Checkbox>
            <CheckboxGroup name="pets" default_value=vec![Key::from("cats")]>
                <Label>"Pets"</Label>
                <Checkbox value="dogs">"Dogs"</Checkbox>
                <Checkbox value="cats">"Cats"</Checkbox>
            </CheckboxGroup>
            <RadioGroup name="size" default_value="m">
                <Label>"Size"</Label>
                <Radio value="s">"Small"</Radio>
                <Radio value="m">"Medium"</Radio>
            </RadioGroup>
            <Switch name="wifi" default_selected=true>"Wi-Fi"</Switch>
            <button id="fm-reset-button" type="reset">"Reset"</button>
        </form>

        // A reset canceled by the form's own listener changes nothing.
        <form id="fm-reset-canceled" on:reset=|e| e.prevent_default()>
            <Checkbox name="kept">"Kept"</Checkbox>
            <Switch name="kept-switch">"Kept switch"</Switch>
            <button id="fm-reset-canceled-button" type="reset">"Reset"</button>
        </form>

        // Enter on a focused control submits the form (implicit submission).
        <form id="fm-submit" on:submit=on_submit>
            <Checkbox name="submit-checkbox">"Submit checkbox"</Checkbox>
            <RadioGroup name="submit-radio" aria_label="Submit radios">
                <Radio value="a">"Submit radio"</Radio>
            </RadioGroup>
            <Switch name="submit-switch">"Submit switch"</Switch>
            <button type="submit">"Submit"</button>
        </form>
        <div>"Submits: " <span id="fm-submits">{submits}</span></div>

        // Right to left: horizontal groups swap ArrowLeft and ArrowRight, vertical ones don't.
        <I18nProvider locale=rtl>
            <RadioGroup aria_label="RTL horizontal" orientation=Orientation::Horizontal>
                <Radio value="a">"RTL horizontal A"</Radio>
                <Radio value="b">"RTL horizontal B"</Radio>
                <Radio value="c">"RTL horizontal C"</Radio>
            </RadioGroup>
            <RadioGroup aria_label="RTL vertical">
                <Radio value="a">"RTL vertical A"</Radio>
                <Radio value="b">"RTL vertical B"</Radio>
                <Radio value="c">"RTL vertical C"</Radio>
            </RadioGroup>
        </I18nProvider>

        // Fields: a checkbox, switch and radio with their own description (and error message).
        <form id="fm-fields">
            <CheckboxField is_required=true attr:data-foo="bar">
                <CheckboxButton>"Field checkbox"</CheckboxButton>
                <Description>"Checkbox help"</Description>
                <FieldError />
            </CheckboxField>
            <SwitchField is_required=true>
                <SwitchButton>"Field switch"</SwitchButton>
                <Description>"Switch help"</Description>
                <FieldError />
            </SwitchField>
            <RadioGroup is_required=true>
                <Label>"Field radios"</Label>
                <RadioField value="a">
                    <RadioButton>"Field radio A"</RadioButton>
                    <Description>"Radio A help"</Description>
                </RadioField>
                <FieldError />
            </RadioGroup>
            <CheckboxGroup>
                <Label>"Field group"</Label>
                <CheckboxField value="x">
                    <CheckboxButton>"Field group X"</CheckboxButton>
                    <Description>"X help"</Description>
                </CheckboxField>
            </CheckboxGroup>
        </form>

        // A required group validated in realtime.
        <CheckboxGroup is_required=true validation_behavior=ValidationBehavior::Aria>
            <Label>"Favorite pet"</Label>
            <Checkbox value="dogs">"Realtime dogs"</Checkbox>
            <Checkbox value="cats">"Realtime cats"</Checkbox>
            <Checkbox value="dragons">"Realtime dragons"</Checkbox>
            <FieldError />
        </CheckboxGroup>
    }
}
