use leptonic::{
    atoms::{
        field::{FieldError, Label},
        form::Form,
        listbox::{ListBox, ListBoxItems},
        select::{HiddenSelect, Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::collections::{CollectionMemo, Key, use_list_collection},
};
use leptos::{ev::SubmitEvent, prelude::*};

const ANIMALS: [(&str, &str); 3] = [("cat", "Cat"), ("dog", "Dog"), ("kangaroo", "Kangaroo")];

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

/// The trigger with the value and the popover with one option per item (and the empty state
/// "No results" with `with_empty_state`).
#[component]
fn Parts(#[prop(optional)] with_empty_state: bool) -> impl IntoView {
    view! {
        <SelectTrigger>
            <SelectValue />
        </SelectTrigger>
        <SelectPopover>
            {if with_empty_state {
                view! {
                    <ListBox empty_state=|| "No results">
                        <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                    </ListBox>
                }
                    .into_any()
            } else {
                view! {
                    <ListBox>
                        <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                    </ListBox>
                }
                    .into_any()
            }}
        </SelectPopover>
    }
}

/// Selects in forms, as react-aria-components' `Select` tests render them (animals cat, dog,
/// kangaroo):
/// - `#sf-multiple`: multiple selection, name "select"; `on_change` logged in
///   `#sf-multiple-changes` (`[keys]`, separated by `|`).
/// - `#sf-required`: required, name "required-select", a `FieldError`.
/// - `#sf-submit`: required, name "submitted", value bound to app state (`#sf-submit-clear`
///   clears it), a submit button; submissions counted in `#sf-submits`.
/// - `#sf-disabled`: disabled, name "disabled-select".
/// - `#sf-empty`: no options; `#sf-empty-allowed`: no options, `allows_empty_collection`, an
///   empty state "No results".
/// - `#sf-many`: 320 options ("item0" ... with the keys 0 ...), multiple selection, required, name
///   "many", a submit button and a reset button.
/// - `#sf-open`: the open state bound to app state (`#sf-open-toggle` toggles it, shown in
///   `#sf-open-state`).
#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomSelectForms() -> impl IntoView {
    let multiple_changes = RwSignal::new(Vec::<String>::new());
    let submitted = RwSignal::new(None::<Key>);
    let submits = RwSignal::new(0_u32);
    let many_submits = RwSignal::new(0_u32);
    let empty = use_list_collection(
        Signal::stored(Vec::<&str>::new()),
        |key| Key::from(*key),
        |key| (*key).to_owned(),
    );
    let items: Vec<usize> = (0..320).collect();
    let many = use_list_collection(
        Signal::stored(items),
        |i| Key::from(i.to_string()),
        |i| format!("item{i}"),
    );
    let is_open = RwSignal::new(false);

    view! {
        <h1>"Select forms"</h1>

        <Form attr:id="sf-multiple">
            <Select<Vec<Key>>
                collection=animals()
                name="select"
                on_change={move |keys: Vec<Key>| {
                    multiple_changes.update(|c| c.push(format!("[{}]", join(&keys))));
                }}
            >
                <Label>"Animals"</Label>
                <Parts />
                <HiddenSelect />
            </Select<Vec<Key>>>
        </Form>
        <div>"Changes: " <span id="sf-multiple-changes">{move || multiple_changes.get().join("|")}</span></div>

        <form id="sf-required">
            <Select<Option<Key>> collection=animals() name="required-select" is_required=true>
                <Label>"Required animal"</Label>
                <Parts />
                <FieldError />
                <HiddenSelect />
            </Select<Option<Key>>>
        </form>

        <Form
            attr:id="sf-submit"
            on:submit=move |e: SubmitEvent| {
                e.prevent_default();
                submits.update(|n| *n += 1);
            }
        >
            <Select
                collection=animals()
                name="submitted"
                is_required=true
                value=submitted
                set_value=submitted
            >
                <Label>"Submitted animal"</Label>
                <Parts />
                <HiddenSelect />
            </Select>
            <button type="submit" id="sf-submit-button">"Submit"</button>
            <button type="button" id="sf-submit-clear" on:click=move |_| submitted.set(None)>
                "Clear"
            </button>
        </Form>
        <div>"Submits: " <span id="sf-submits">{submits}</span></div>

        <form id="sf-disabled">
            <Select<Option<Key>> collection=animals() name="disabled-select" is_disabled=true>
                <Label>"Disabled animal"</Label>
                <Parts />
                <HiddenSelect />
            </Select<Option<Key>>>
        </form>

        <div id="sf-empty">
            <Select collection=empty default_value=Some(Key::from("cat"))>
                <Label>"Empty"</Label>
                <Parts />
            </Select>
        </div>
        <div id="sf-empty-allowed">
            <Select<Option<Key>> collection=empty allows_empty_collection=true>
                <Label>"Empty allowed"</Label>
                <Parts with_empty_state=true />
            </Select<Option<Key>>>
        </div>

        <Form
            attr:id="sf-many"
            on:submit=move |e: SubmitEvent| {
                e.prevent_default();
                many_submits.update(|n| *n += 1);
            }
        >
            <Select<Vec<Key>> collection=many is_required=true name="many">
                <Label>"Many"</Label>
                <Parts />
                <FieldError />
                <HiddenSelect />
            </Select<Vec<Key>>>
            <button type="submit" id="sf-many-submit">"Submit"</button>
            <input type="reset" id="sf-many-reset" />
        </Form>
        <div>"Submits: " <span id="sf-many-submits">{many_submits}</span></div>

        <div id="sf-open">
            <button id="sf-open-toggle" on:click=move |_| is_open.update(|open| *open = !*open)>
                "Toggle"
            </button>
            <Select<Option<Key>> collection=animals() is_open=is_open set_open=is_open>
                <Label>"Controlled open"</Label>
                <Parts />
            </Select<Option<Key>>>
            <div>"Open: " <span id="sf-open-state">{move || is_open.get().to_string()}</span></div>
        </div>
    }
}
