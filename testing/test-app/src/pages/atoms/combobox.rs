use std::collections::HashSet;

use leptonic::{
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover, ComboBoxValue},
        dialog::Dialog,
        field::{Description, Label},
        input::Input,
        listbox::{ListBox, ListBoxItems},
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::{
        collections::{CollectionMemo, Key, UseListCollectionInput, use_list_collection},
        combobox::{ComboBoxMenuTrigger, ComboBoxOpenChange, use_contains_filter},
    },
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// A combo box over fruits, filtering by "contains". "Cherry" is disabled. The value is shown in
/// `#test-cb-value`.
#[component]
pub fn PageAtomComboBox() -> impl IntoView {
    let fruits = use_list_collection(UseListCollectionInput {
        items: Signal::stored(FRUITS.to_vec()),
        key: |fruit| Key::from(*fruit),
        text_value: |fruit| (*fruit).to_owned(),
    });
    let value = RwSignal::new(String::new());

    view! {
        <div id="test-page-atom-combobox">
            <h1>"ComboBox"</h1>
            <ComboBox
                collection=fruits
                filter=use_contains_filter()
                disabled_keys=Signal::stored(HashSet::from([Key::from("Cherry")]))
                on_change=Callback::new(move |key: Option<Key>| {
                    value.set(key.map(|k| k.to_string()).unwrap_or_default());
                })
            >
                <Label>"Fruit"</Label>
                <Input />
                <ComboBoxButton>"▼"</ComboBoxButton>
                <ComboBoxPopover>
                    <ListBox>
                        <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                    </ListBox>
                </ComboBoxPopover>
            </ComboBox>
            <button id="test-cb-after">"After"</button>
            <div>"Value: " <span id="test-cb-value">{value}</span></div>
            <ModalComboBox />
            <ControlledComboBox />
            <DerivedComboBox />
            <ReadOnlyComboBox />
            <DisabledComboBox />
            <ServerFilteredComboBox />
            <ContextsComboBox />
            <MultipleComboBox />
        </div>
    }
}

fn join(keys: &[Key]) -> String {
    keys.iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// A combo box in a modal dialog (opened by `#test-cb-modal-open`): its popover, portaled next
/// to the modal, must stay interactive. The value is shown in `#test-cb-modal-value`.
#[component]
fn ModalComboBox() -> impl IntoView {
    let fruits = use_list_collection(UseListCollectionInput {
        items: Signal::stored(FRUITS.to_vec()),
        key: |fruit| Key::from(*fruit),
        text_value: |fruit| (*fruit).to_owned(),
    });
    let is_open = RwSignal::new(false);
    let value = RwSignal::new(None::<Key>);
    view! {
        <button id="test-cb-modal-open" on:click=move |_| is_open.set(true)>
            "Open modal"
        </button>
        <div>
            "Modal value: "
            <span id="test-cb-modal-value">{move || join(value.get().as_slice())}</span>
        </div>
        <ModalBackdrop is_open=is_open set_open=is_open is_dismissable=true>
            <ModalContent>
                <Dialog aria_label="Modal with combo box">
                    <ComboBox collection=fruits value=value set_value=value>
                        <Label>"Modal fruit"</Label>
                        <Input />
                        <ComboBoxButton>"▼"</ComboBoxButton>
                        <ComboBoxPopover>
                            <ListBox>
                                <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                            </ListBox>
                        </ComboBoxPopover>
                    </ComboBox>
                </Dialog>
            </ModalContent>
        </ModalBackdrop>
    }
}

/// A controlled combo box whose value changes from outside: `#test-cb-controlled-set` selects
/// "Banana" (an existing item), `#test-cb-controlled-add` adds "Fig" to the collection and
/// selects it in the same update, `#test-cb-controlled-select-then-add` selects "Grape" before
/// adding it.
/// `#test-cb-timed-start` selects "Banana" after 300 ms, with no event or focus change around it,
/// and `#test-cb-timed-effect` selects "Cherry" from an effect.
#[component]
fn ControlledComboBox() -> impl IntoView {
    let items = RwSignal::new(FRUITS.map(str::to_owned).to_vec());
    let fruits = use_list_collection(UseListCollectionInput {
        items: Signal::from(items),
        key: |fruit: &String| Key::from(fruit.as_str()),
        text_value: |fruit: &String| fruit.clone(),
    });
    let value = RwSignal::new(Some(Key::from("Apple")));
    let start = move |_| {
        set_timeout(
            move || value.set(Some(Key::from("Banana"))),
            std::time::Duration::from_millis(300),
        );
    };
    let trigger = RwSignal::new(false);
    Effect::new(move |_| {
        if trigger.get() {
            value.set(Some(Key::from("Cherry")));
        }
    });
    view! {
        <button id="test-cb-timed-start" on:click=start>
            "Select Banana later"
        </button>
        <button id="test-cb-timed-effect" on:click=move |_| trigger.set(true)>
            "Select Cherry from an effect"
        </button>
        <button id="test-cb-controlled-set" on:click=move |_| value.set(Some(Key::from("Banana")))>
            "Select Banana"
        </button>
        <button
            id="test-cb-controlled-add"
            on:click=move |_| {
                items.update(|items| items.push("Fig".to_owned()));
                value.set(Some(Key::from("Fig")));
            }
        >
            "Add and select Fig"
        </button>
        <button
            id="test-cb-controlled-select-then-add"
            on:click=move |_| {
                value.set(Some(Key::from("Grape")));
                items.update(|items| items.push("Grape".to_owned()));
            }
        >
            "Select Grape, then add it"
        </button>
        <ComboBox collection=fruits value=value set_value=value>
            <Label>"Controlled fruit"</Label>
            <Input />
            <ComboBoxButton>"▼"</ComboBoxButton>
            <ComboBoxPopover>
                <ListBox>
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </ComboBoxPopover>
        </ComboBox>
    }
}

/// As agnite dev-ui's log source picker: value and items derive from one app signal (memos),
/// changed from outside (`#test-cb-derived-<n>` selects process n). The items are "Orchestration"
/// and the selected process, so selecting another process removes the previous one.
#[component]
fn DerivedComboBox() -> impl IntoView {
    let selected = RwSignal::new(None::<u32>);
    // As dev-ui: items and value both derive from a memo over the app signal.
    let source = Memo::new(move |_| selected.get());
    let label = |process: Option<u32>| {
        process.map_or_else(
            || "Orchestration".to_owned(),
            |process| format!("Process {process}"),
        )
    };
    let sources = Memo::new(move |_| {
        let mut sources = vec!["Orchestration".to_owned()];
        if source.get().is_some() {
            sources.push(label(source.get()));
        }
        sources
    });
    let options = use_list_collection(UseListCollectionInput {
        items: Signal::from(sources),
        key: |source: &String| Key::from(source.as_str()),
        text_value: |source: &String| source.clone(),
    });
    let value = Signal::derive(move || Some(Key::from(label(source.get()).as_str())));
    let set_value = move |key: Option<Key>| {
        let process = key.and_then(|key| key.to_string().strip_prefix("Process ")?.parse().ok());
        selected.set(process);
    };
    view! {
        <button id="test-cb-derived-1" on:click=move |_| selected.set(Some(1))>
            "Process 1"
        </button>
        <button id="test-cb-derived-2" on:click=move |_| selected.set(Some(2))>
            "Process 2"
        </button>
        <ComboBox
            collection=options
            filter=use_contains_filter()
            value=value
            set_value=set_value
            placeholder="—"
        >
            <Label>"Logs of"</Label>
            <div>
                <Input />
                <ComboBoxButton>"▾"</ComboBoxButton>
            </div>
            <ComboBoxPopover>
                <ListBox>
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </ComboBoxPopover>
        </ComboBox>
    }
}

/// A fruit combo box's button and popover with all options.
#[component]
fn FruitOptions() -> impl IntoView {
    view! {
        <ComboBoxButton>"▼"</ComboBoxButton>
        <ComboBoxPopover>
            <ListBox>
                <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
            </ListBox>
        </ComboBoxPopover>
    }
}

fn fruits() -> CollectionMemo {
    use_list_collection(UseListCollectionInput {
        items: Signal::stored(FRUITS.to_vec()),
        key: |fruit| Key::from(*fruit),
        text_value: |fruit| (*fruit).to_owned(),
    })
}

/// A read-only combo box ("Read-only fruit", "Apple" selected) that would open on focus.
#[component]
fn ReadOnlyComboBox() -> impl IntoView {
    view! {
        <ComboBox
            collection=fruits()
            default_value=Some(Key::from("Apple"))
            is_read_only=true
            menu_trigger=ComboBoxMenuTrigger::Focus
        >
            <Label>"Read-only fruit"</Label>
            <Input />
            <FruitOptions />
        </ComboBox>
    }
}

/// A disabled combo box ("Disabled fruit").
#[component]
fn DisabledComboBox() -> impl IntoView {
    view! {
        <ComboBox<Option<Key>> collection=fruits() filter=use_contains_filter() is_disabled=true>
            <Label>"Disabled fruit"</Label>
            <Input />
            <FruitOptions />
        </ComboBox<Option<Key>>>
    }
}

const CHARACTERS: [&str; 4] = [
    "Luke Skywalker",
    "Leia Organa",
    "Han Solo",
    "Lando Calrissian",
];

/// A combo box filtered by the app ("Character", no `filter`), as an async list: 50 ms after its
/// text changed, the options are the characters containing the text (ignoring case). Opening it
/// without options loads them.
#[component]
fn ServerFilteredComboBox() -> impl IntoView {
    let items = RwSignal::new(Vec::<&'static str>::new());
    let load = move |text: String| {
        set_timeout(
            move || {
                let text = text.to_lowercase();
                items.set(
                    CHARACTERS
                        .into_iter()
                        .filter(|name| name.to_lowercase().contains(&text))
                        .collect(),
                );
            },
            std::time::Duration::from_millis(50),
        );
    };
    let characters = use_list_collection(UseListCollectionInput {
        items: Signal::from(items),
        key: |name| Key::from(*name),
        text_value: |name| (*name).to_owned(),
    });
    let text = RwSignal::new(String::new());
    let set_text = move |value: String| {
        text.set(value.clone());
        load(value);
    };
    let on_open_change = move |change: ComboBoxOpenChange| {
        if change.is_open && items.with_untracked(Vec::is_empty) {
            load(text.get_untracked());
        }
    };
    view! {
        <ComboBox<Option<Key>>
            collection=characters
            input_value=text
            set_input_value=set_text
            on_open_change=on_open_change
        >
            <Label>"Character"</Label>
            <Input />
            <ComboBoxButton>"▼"</ComboBoxButton>
            <ComboBoxPopover>
                <ListBox>
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </ComboBoxPopover>
        </ComboBox<Option<Key>>>
    }
}

/// A combo box ("Context fruit") whose popover holds a `Label`, an `Input` and a `Description`
/// of its own besides the options: none of them belongs to the combo box.
#[component]
fn ContextsComboBox() -> impl IntoView {
    view! {
        <ComboBox<Option<Key>> collection=fruits()>
            <Label>"Context fruit"</Label>
            <Input />
            <ComboBoxButton>"▼"</ComboBoxButton>
            <ComboBoxPopover classes="test-cb-contexts-popover">
                <Label>"Hello"</Label>
                <Input />
                <Description>"hi"</Description>
                <ListBox>
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </ComboBoxPopover>
        </ComboBox<Option<Key>>>
    }
}

/// A combo box selecting several fruits ("Fruits"), with a `ComboBoxValue` listing them
/// (placeholder "No fruits selected").
#[component]
fn MultipleComboBox() -> impl IntoView {
    view! {
        <ComboBox<HashSet<Key>> collection=fruits() filter=use_contains_filter()>
            <Label>"Fruits"</Label>
            <ComboBoxValue placeholder="No fruits selected" classes="test-cb-multiple-value" />
            <Input />
            <FruitOptions />
        </ComboBox<HashSet<Key>>>
    }
}
