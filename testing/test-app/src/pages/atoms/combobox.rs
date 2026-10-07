use std::collections::HashSet;

use leptonic::{
    atoms::{
        combobox::{ComboBox, ComboBoxButton, ComboBoxPopover},
        dialog::Dialog,
        field::Label,
        input::Input,
        listbox::{ListBox, ListBoxItems},
        modal::{ModalBackdrop, ModalContent},
    },
    hooks::{
        collections::{Key, use_list_collection},
        use_contains_filter,
    },
};
use leptos::prelude::*;

const FRUITS: [&str; 5] = ["Apple", "Banana", "Cherry", "Durian", "Elderberry"];

/// A combo box over fruits, filtering by "contains". "Cherry" is disabled. The value is shown in
/// `#test-cb-value`.
#[component]
pub fn PageAtomComboBox() -> impl IntoView {
    let fruits = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |fruit| Key::from(*fruit),
        |fruit| (*fruit).to_owned(),
    );
    let value = RwSignal::new(String::new());

    view! {
        <div id="test-page-atom-combobox">
            <h1>"ComboBox"</h1>
            <button id="test-cb-before">"Before"</button>
            <ComboBox
                collection=fruits
                filter=use_contains_filter()
                disabled_keys=Signal::stored(HashSet::from([Key::from("Cherry")]))
                on_change=Callback::new(move |keys: Vec<Key>| {
                    let keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
                    value.set(keys.join(","));
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
            <TimedComboBox />
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
    let fruits = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |fruit| Key::from(*fruit),
        |fruit| (*fruit).to_owned(),
    );
    let is_open = RwSignal::new(false);
    let value = RwSignal::new(Vec::<Key>::new());
    view! {
        <button id="test-cb-modal-open" on:click=move |_| is_open.set(true)>"Open modal"</button>
        <div>"Modal value: " <span id="test-cb-modal-value">{move || join(&value.get())}</span></div>
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
#[component]
fn ControlledComboBox() -> impl IntoView {
    let items = RwSignal::new(FRUITS.map(str::to_owned).to_vec());
    let fruits = use_list_collection(
        Signal::from(items),
        |fruit: &String| Key::from(fruit.as_str()),
        |fruit: &String| fruit.clone(),
    );
    let value = RwSignal::new(vec![Key::from("Apple")]);
    view! {
        <button id="test-cb-controlled-set" on:click=move |_| value.set(vec![Key::from("Banana")])>
            "Select Banana"
        </button>
        <button
            id="test-cb-controlled-add"
            on:click=move |_| {
                items.update(|items| items.push("Fig".to_owned()));
                value.set(vec![Key::from("Fig")]);
            }
        >
            "Add and select Fig"
        </button>
        <button
            id="test-cb-controlled-select-then-add"
            on:click=move |_| {
                value.set(vec![Key::from("Grape")]);
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
    let options = use_list_collection(
        Signal::from(sources),
        |source: &String| Key::from(source.as_str()),
        |source: &String| source.clone(),
    );
    let value = Signal::derive(move || vec![Key::from(label(source.get()).as_str())]);
    let set_value = move |keys: Vec<Key>| {
        let process = keys
            .first()
            .and_then(|key| key.to_string().strip_prefix("Process ")?.parse().ok());
        selected.set(process);
    };
    view! {
        <button id="test-cb-derived-1" on:click=move |_| selected.set(Some(1))>"Process 1"</button>
        <button id="test-cb-derived-2" on:click=move |_| selected.set(Some(2))>"Process 2"</button>
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

/// A controlled combo box over a fixed collection whose value changes from a timer, with no
/// event or focus change around it (`#test-cb-timed-start`: "Banana" after 300 ms), or from an
/// effect (`#test-cb-timed-effect`: "Cherry", written while an effect runs).
#[component]
fn TimedComboBox() -> impl IntoView {
    let fruits = use_list_collection(
        Signal::stored(FRUITS.to_vec()),
        |fruit| Key::from(*fruit),
        |fruit| (*fruit).to_owned(),
    );
    let value = RwSignal::new(vec![Key::from("Apple")]);
    let start = move |_| {
        set_timeout(
            move || value.set(vec![Key::from("Banana")]),
            std::time::Duration::from_millis(300),
        );
    };
    let trigger = RwSignal::new(false);
    Effect::new(move |_| {
        if trigger.get() {
            value.set(vec![Key::from("Cherry")]);
        }
    });
    view! {
        <button id="test-cb-timed-start" on:click=start>"Select Banana later"</button>
        <button id="test-cb-timed-effect" on:click=move |_| trigger.set(true)>"Select Cherry from an effect"</button>
        <ComboBox collection=fruits value=value set_value=value>
            <Label>"Timed fruit"</Label>
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
