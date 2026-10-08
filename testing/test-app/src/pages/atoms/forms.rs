use leptonic::{
    atoms::{
        checkbox::{CheckboxButton, CheckboxField, CheckboxGroup},
        field::{Description, FieldError, Label},
        radio::{RadioButton, RadioField, RadioGroup},
        switch::{SwitchButton, SwitchField},
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
            <CheckboxField name="terms"><CheckboxButton>"Terms"</CheckboxButton></CheckboxField>
            <CheckboxGroup name="pets" default_value=vec![Key::from("cats")]>
                <Label>"Pets"</Label>
                <CheckboxField value="dogs"><CheckboxButton>"Dogs"</CheckboxButton></CheckboxField>
                <CheckboxField value="cats"><CheckboxButton>"Cats"</CheckboxButton></CheckboxField>
            </CheckboxGroup>
            <RadioGroup name="size" default_value=Key::from("m")>
                <Label>"Size"</Label>
                <RadioField value="s"><RadioButton>"Small"</RadioButton></RadioField>
                <RadioField value="m"><RadioButton>"Medium"</RadioButton></RadioField>
            </RadioGroup>
            <SwitchField name="wifi" default_selected=true><SwitchButton>"Wi-Fi"</SwitchButton></SwitchField>
            <button id="fm-reset-button" type="reset">"Reset"</button>
        </form>

        // A reset canceled by the form's own listener changes nothing.
        <form id="fm-reset-canceled" on:reset=|e| e.prevent_default()>
            <CheckboxField name="kept"><CheckboxButton>"Kept"</CheckboxButton></CheckboxField>
            <SwitchField name="kept-switch"><SwitchButton>"Kept switch"</SwitchButton></SwitchField>
            <button id="fm-reset-canceled-button" type="reset">"Reset"</button>
        </form>

        // Enter on a focused control submits the form (implicit submission).
        <form id="fm-submit" on:submit=on_submit>
            <CheckboxField name="submit-checkbox"><CheckboxButton>"Submit checkbox"</CheckboxButton></CheckboxField>
            <RadioGroup<Key> name="submit-radio" aria_label="Submit radios">
                <RadioField value="a"><RadioButton>"Submit radio"</RadioButton></RadioField>
            </RadioGroup<Key>>
            <SwitchField name="submit-switch"><SwitchButton>"Submit switch"</SwitchButton></SwitchField>
            <button type="submit">"Submit"</button>
        </form>
        <div>"Submits: " <span id="fm-submits">{submits}</span></div>

        // Right to left: horizontal groups swap ArrowLeft and ArrowRight, vertical ones don't.
        <I18nProvider locale=rtl>
            <RadioGroup<Key> aria_label="RTL horizontal" orientation=Orientation::Horizontal>
                <RadioField value="a"><RadioButton>"RTL horizontal A"</RadioButton></RadioField>
                <RadioField value="b"><RadioButton>"RTL horizontal B"</RadioButton></RadioField>
                <RadioField value="c"><RadioButton>"RTL horizontal C"</RadioButton></RadioField>
            </RadioGroup<Key>>
            <RadioGroup<Key> aria_label="RTL vertical">
                <RadioField value="a"><RadioButton>"RTL vertical A"</RadioButton></RadioField>
                <RadioField value="b"><RadioButton>"RTL vertical B"</RadioButton></RadioField>
                <RadioField value="c"><RadioButton>"RTL vertical C"</RadioButton></RadioField>
            </RadioGroup<Key>>
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
            <RadioGroup<Key> is_required=true>
                <Label>"Field radios"</Label>
                <RadioField value="a">
                    <RadioButton>"Field radio A"</RadioButton>
                    <Description>"Radio A help"</Description>
                </RadioField>
                <FieldError />
            </RadioGroup<Key>>
            <CheckboxGroup<Key>>
                <Label>"Field group"</Label>
                <CheckboxField value="x">
                    <CheckboxButton>"Field group X"</CheckboxButton>
                    <Description>"X help"</Description>
                </CheckboxField>
            </CheckboxGroup<Key>>
        </form>

        // A required group validated in realtime.
        <CheckboxGroup<Key> is_required=true validation_behavior=ValidationBehavior::Aria>
            <Label>"Favorite pet"</Label>
            <CheckboxField value="dogs"><CheckboxButton>"Realtime dogs"</CheckboxButton></CheckboxField>
            <CheckboxField value="cats"><CheckboxButton>"Realtime cats"</CheckboxButton></CheckboxField>
            <CheckboxField value="dragons"><CheckboxButton>"Realtime dragons"</CheckboxButton></CheckboxField>
            <FieldError />
        </CheckboxGroup<Key>>
    }
}
