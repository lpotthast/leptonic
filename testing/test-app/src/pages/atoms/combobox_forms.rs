use std::{collections::HashSet, sync::Arc};

use leptonic::{
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        field::{FieldError, Label},
        form::Form,
        input::Input,
        listbox::{ListBox, ListBoxItem, ListBoxItems, ListBoxSection, ListBoxSectionHeading},
    },
    hooks::{
        ComboBoxFormValue, ComboBoxMenuTrigger, ComboBoxValue, SelectMode, ValidationBehavior,
        collections::{CollectionMemo, Key, use_collection, use_list_collection},
        use_contains_filter,
    },
};
use leptos::{ev::SubmitEvent, prelude::*};

const ANIMALS: [(&str, &str); 3] = [("1", "Cat"), ("2", "Dog"), ("3", "Kangaroo")];

fn animals() -> CollectionMemo {
    use_list_collection(
        Signal::stored(ANIMALS.to_vec()),
        |(key, _)| Key::from(*key),
        |(_, text)| (*text).to_owned(),
    )
}

fn join(keys: &[Key]) -> String {
    keys.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// The popover with one option per item, shared by the combo boxes below.
#[component]
fn Options() -> impl IntoView {
    view! {
        <ComboBoxButton>"▼"</ComboBoxButton>
        <ComboBoxPopover>
            <ListBox>
                <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
            </ListBox>
        </ComboBoxPopover>
    }
}

/// Combo boxes in forms, as react-aria-components' `ComboBox` tests render them (animals Cat,
/// Dog, Kangaroo with the keys 1, 2, 3):
/// - `#cbf-custom`: `allows_custom_value`, name "animal"; every `on_change` is logged in
///   `#cbf-custom-changes` (`[keys]`, separated by `|`).
/// - `#cbf-required`: required, native validation, a `FieldError`.
/// - `#cbf-validate`: ARIA validation rejecting Dog ("Dogs are not allowed").
/// - `#cbf-multiple`: multiple selection, name "animals", a reset button; `on_change` logged in
///   `#cbf-multiple-changes`.
/// - `#cbf-multiple-required`: multiple selection, required, name "required-animals".
/// - `#cbf-key`: name "key-animal", default value 2 (the key is submitted); `#cbf-text`: the same
///   with `form_value` text.
/// - `#cbf-focus` / `#cbf-manual`: `menu_trigger` focus / manual, with `on_open_change` logged
///   in `#cbf-focus-open` / `#cbf-manual-open`.
/// - `#cbf-sections`: sections (Animals: Cat, Dog; Birds: Owl, Parrot), Dog disabled.
/// - `#cbf-submit`: Enter in the input; submissions counted in `#cbf-submits`.
#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomComboBoxForms() -> impl IntoView {
    let custom_changes = RwSignal::new(Vec::<String>::new());
    let multiple_changes = RwSignal::new(Vec::<String>::new());
    let focus_open = RwSignal::new(Vec::<String>::new());
    let manual_open = RwSignal::new(Vec::<String>::new());
    let submits = RwSignal::new(0_u32);
    let sections = use_collection(|b| {
        b.section("animals", |s| {
            s.header("animals-header", "Animals");
            s.item("cat", "Cat");
            s.item("dog", "Dog");
        });
        b.section("birds", |s| {
            s.header("birds-header", "Birds");
            s.item("owl", "Owl");
            s.item("parrot", "Parrot");
        });
    });
    let no_dogs: leptonic::hooks::ValidateFn<ComboBoxValue> =
        Arc::new(|value: &ComboBoxValue| {
            if value.value.contains(&Key::from("2")) {
                Err(vec!["Dogs are not allowed".to_owned()])
            } else {
                Ok(())
            }
        });
    let log_open = |log: RwSignal<Vec<String>>| {
        Callback::new(move |change: leptonic::hooks::ComboBoxOpenChange| {
            log.update(|log| {
                log.push(format!("{}:{:?}", change.is_open, change.trigger));
            });
        })
    };

    view! {
        <h1>"ComboBox forms"</h1>

        <form id="cbf-custom" on:submit=|e: SubmitEvent| e.prevent_default()>
            <ComboBox
                collection=animals()
                filter=use_contains_filter()
                allows_custom_value=true
                name="animal"
                on_change={move |keys: Vec<Key>| {
                    custom_changes.update(|c| c.push(format!("[{}]", join(&keys))));
                }}
            >
                <Label>"Custom animal"</Label>
                <Input />
                <Options />
            </ComboBox>
            <button type="button" id="cbf-custom-after">"After"</button>
        </form>
        <div>"Changes: " <span id="cbf-custom-changes">{move || custom_changes.get().join("|")}</span></div>

        <form id="cbf-required">
            <ComboBox collection=animals() filter=use_contains_filter() is_required=true>
                <Label>"Required animal"</Label>
                <Input />
                <Options />
                <FieldError />
            </ComboBox>
        </form>

        <Form attr:id="cbf-validate" validation_behavior=ValidationBehavior::Aria>
            <ComboBox collection=animals() filter=use_contains_filter() validate=no_dogs>
                <Label>"Validated animal"</Label>
                <Input />
                <Options />
                <FieldError />
            </ComboBox>
        </Form>

        <Form attr:id="cbf-multiple">
            <ComboBox
                collection=animals()
                filter=use_contains_filter()
                selection_mode=SelectMode::Multiple
                name="animals"
                default_input_value=""
                on_change={move |keys: Vec<Key>| {
                    multiple_changes.update(|c| c.push(format!("[{}]", join(&keys))));
                }}
            >
                <Label>"Animals"</Label>
                <Input />
                <Options />
            </ComboBox>
            <input type="reset" id="cbf-multiple-reset" />
        </Form>
        <div>"Changes: " <span id="cbf-multiple-changes">{move || multiple_changes.get().join("|")}</span></div>

        <Form attr:id="cbf-multiple-required">
            <ComboBox
                collection=animals()
                filter=use_contains_filter()
                selection_mode=SelectMode::Multiple
                is_required=true
                name="required-animals"
            >
                <Label>"Required animals"</Label>
                <Input />
                <Options />
                <FieldError />
            </ComboBox>
        </Form>

        <form id="cbf-key">
            <ComboBox collection=animals() name="key-animal" default_value=vec![Key::from("2")]>
                <Label>"Key animal"</Label>
                <Input />
                <Options />
            </ComboBox>
        </form>
        <form id="cbf-text">
            <ComboBox
                collection=animals()
                name="text-animal"
                default_value=vec![Key::from("2")]
                form_value=ComboBoxFormValue::Text
            >
                <Label>"Text animal"</Label>
                <Input />
                <Options />
            </ComboBox>
        </form>

        <div id="cbf-focus">
            <ComboBox
                collection=animals()
                filter=use_contains_filter()
                menu_trigger=ComboBoxMenuTrigger::Focus
                default_input_value="Do"
                on_open_change=log_open(focus_open)
            >
                <Label>"Focus animal"</Label>
                <Input />
                <Options />
            </ComboBox>
        </div>
        <div>"Open changes: " <span id="cbf-focus-open">{move || focus_open.get().join("|")}</span></div>
        <div id="cbf-manual">
            <ComboBox
                collection=animals()
                filter=use_contains_filter()
                menu_trigger=ComboBoxMenuTrigger::Manual
                on_open_change=log_open(manual_open)
            >
                <Label>"Manual animal"</Label>
                <Input />
                <Options />
            </ComboBox>
        </div>
        <div>"Open changes: " <span id="cbf-manual-open">{move || manual_open.get().join("|")}</span></div>

        <div id="cbf-sections">
            <ComboBox
                collection=sections
                filter=use_contains_filter()
                disabled_keys=Signal::stored(HashSet::from([Key::from("dog")]))
            >
                <Label>"Sectioned animal"</Label>
                <Input />
                <ComboBoxButton>"▼"</ComboBoxButton>
                <ComboBoxPopover>
                    <ListBox>
                        <SectionedOptions />
                    </ListBox>
                </ComboBoxPopover>
            </ComboBox>
        </div>

        <form
            id="cbf-submit"
            on:submit=move |e: SubmitEvent| {
                e.prevent_default();
                submits.update(|n| *n += 1);
            }
        >
            <ComboBox collection=animals() filter=use_contains_filter()>
                <Label>"Submitted animal"</Label>
                <Input />
                <Options />
            </ComboBox>
        </form>
        <div>"Submits: " <span id="cbf-submits">{submits}</span></div>
    }
}

/// The sections of the combo box's (filtered) collection, with their headings and options.
#[component]
fn SectionedOptions() -> impl IntoView {
    let list = expect_context::<leptonic::hooks::ListBoxData>();
    let collection = list.state.collection;
    move || {
        collection.with(|c| {
            c.iter()
                .map(|section| {
                    let items: Vec<(Key, String)> = c
                        .children(&section.key)
                        .filter(|node| node.is_item())
                        .map(|node| (node.key.clone(), node.text_value.to_string()))
                        .collect();
                    view! {
                        <ListBoxSection key=section.key.clone()>
                            <ListBoxSectionHeading />
                            {items
                                .into_iter()
                                .map(|(key, text)| view! { <ListBoxItem key=key>{text}</ListBoxItem> })
                                .collect_view()}
                        </ListBoxSection>
                    }
                })
                .collect_view()
        })
    }
}
