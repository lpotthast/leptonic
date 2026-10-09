use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use leptonic::{
    atoms::{
        button::Button,
        field::{Description, FieldError, Label},
        form::Form,
        listbox::{
            ListBox, ListBoxItem, ListBoxItemDescription, ListBoxItemLabel, ListBoxItems,
            ListBoxSection, ListBoxSectionHeading,
        },
        select::{HiddenSelect, Select, SelectPopover, SelectTrigger, SelectValue},
    },
    hooks::{
        collections::{
            CloseOnSelect, CollectionMemo, Key, UseListCollectionInput, use_collection,
            use_list_collection,
        },
        form::{ValidateFn, ValidationBehavior},
    },
};
use leptos::prelude::*;

use crate::pages::Section;

const ANIMALS: [(&str, &str); 3] = [("cat", "Cat"), ("dog", "Dog"), ("kangaroo", "Kangaroo")];

fn animals() -> CollectionMemo {
    use_list_collection(UseListCollectionInput {
        items: Signal::stored(ANIMALS.to_vec()),
        key: |(key, _): &(&str, &str)| Key::from(*key),
        text_value: |(_, text): &(&str, &str)| (*text).to_owned(),
    })
}

/// The trigger with the value, and the popover with one option per item.
#[component]
fn Parts() -> impl IntoView {
    view! {
        <SelectTrigger>
            <SelectValue />
        </SelectTrigger>
        <SelectPopover>
            <ListBox>
                <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
            </ListBox>
        </SelectPopover>
    }
}

/// A log of values, shown in `#<id>`: each change's value, changes separated by `|`.
fn log<T: Send + Sync + 'static>(
    id: &'static str,
    describe: impl Fn(&T) -> String + Send + Sync + 'static,
) -> (Callback<T>, impl IntoView) {
    let entries = RwSignal::new(Vec::<String>::new());
    let on_change = Callback::new(move |value: T| entries.update(|e| e.push(describe(&value))));
    let view = view! { <div>"Log: " <span id=id>{move || entries.get().join("|")}</span></div> };
    (on_change, view)
}

fn describe_key(key: &Option<Key>) -> String {
    key.as_ref().map(ToString::to_string).unwrap_or_default()
}

fn describe_keys(keys: &HashSet<Key>) -> String {
    let mut keys: Vec<String> = keys.iter().map(ToString::to_string).collect();
    keys.sort();
    keys.join(",")
}

/// Select behavior, as react-aria-components' `Select` and React Spectrum's `Picker` tests render
/// them (animals cat, dog, kangaroo). Each select is a section of its own (`?only=<name>`); value
/// changes are logged in `#<name>-log`.
/// - `contexts`: default cat, labelled "Favorite animal"; its popover holds a label "Hello", a
///   button "Yo" and a description "hi" besides the options.
/// - `aria-label`: labelled by `aria_label` "Pick an animal" only; `aria-labelledby`: by
///   `#sb-external-label` ("External label") only; `both`: by both.
/// - `described`: a label "Described animal", a description "Pick wisely", invalid with the
///   error "Invalid animal".
/// - `empty`: no value; a button `#empty-before` before it.
/// - `stay-open`: single selection that stays open on select; `close-multiple`: multiple
///   selection that closes on select.
/// - `controlled`: the value fixed to dog; `controlled-multiple`: fixed to dog and kangaroo.
/// - `focus`: focus changes logged in `#focus-log` (`true`/`false`); buttons `#focus-before` and
///   `#focus-after` around it.
/// - `form`: name "animal", `form="sb-outer-form"` (a form after it), autocomplete "off".
/// - `validate`: in a native-validation form `#validate-form`, name "validated", dogs are
///   invalid ("Dogs are not allowed"), a submit button `#validate-submit` and a reset button
///   `#validate-reset`. `validate-aria`: the same in an ARIA-validation form.
/// - `server`: in a form with the server error "Server says no" for "server-animal".
/// - `default-open`: opens when it mounts. `fixed-open`: always open (the open state is never
///   written); open changes logged in `#fixed-open-log`.
/// - `sections`: "Mammals" (Cat, Dog) and "Marsupials" (Kangaroo with the description "Jumps").
/// - `scroll`: 50 options ("Option 0" ...), default "Option 40", in a popover at most 100px
///   high.
/// - `typeahead`: options "Foo Bar", "Foo Baz", "Bar", "Zoo".
#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomSelectBehavior() -> impl IntoView {
    let (stay_open_change, stay_open_log) = log("stay-open-log", describe_key);
    let (close_multiple_change, close_multiple_log) = log("close-multiple-log", describe_keys);
    let (controlled_change, controlled_log) = log("controlled-log", describe_key);
    let (controlled_multiple_change, controlled_multiple_log) =
        log("controlled-multiple-log", describe_keys);
    let (focus_change, focus_log) = log("focus-log", |focused: &bool| focused.to_string());
    let (empty_change, empty_log) = log("empty-log", describe_key);
    let (fixed_open_change, fixed_open_log) = log("fixed-open-log", |open: &bool| open.to_string());
    let (sections_change, sections_log) = log("sections-log", describe_key);
    let (typeahead_change, typeahead_log) = log("typeahead-log", describe_key);

    let no_dogs: ValidateFn<Option<Key>> = Arc::new(|value: &Option<Key>| {
        if *value == Some(Key::from("dog")) {
            Err(vec!["Dogs are not allowed".to_owned()])
        } else {
            Ok(())
        }
    });
    let server_errors = HashMap::from([(
        "server-animal".to_owned(),
        vec!["Server says no".to_owned()],
    )]);
    let sections = use_collection(|b| {
        b.section("mammals", |s| {
            s.header("mammals-header", "Mammals");
            s.item("cat", "Cat");
            s.item("dog", "Dog");
        });
        b.section("marsupials", |s| {
            s.header("marsupials-header", "Marsupials");
            s.item("kangaroo", "Kangaroo");
        });
    });
    let scroll = use_list_collection(UseListCollectionInput {
        items: Signal::stored((0..50).collect::<Vec<u32>>()),
        key: |i: &u32| Key::from(*i),
        text_value: |i: &u32| format!("Option {i}"),
    });
    let typeahead = use_list_collection(UseListCollectionInput {
        items: Signal::stored(vec!["Foo Bar", "Foo Baz", "Bar", "Zoo"]),
        key: |item: &&str| Key::from(*item),
        text_value: |item: &&str| (*item).to_owned(),
    });

    let no_dogs_aria = no_dogs.clone();
    view! {
        <h1>"Select behavior"</h1>

        <Section name="contexts">
            <div id="contexts">
                <Select<Option<Key>> collection=animals() default_value=Some(Key::from("cat"))>
                    <Label>"Favorite animal"</Label>
                    <SelectTrigger>
                        <SelectValue />
                    </SelectTrigger>
                    <SelectPopover classes="test-sb-contexts-popover">
                        <Label>"Hello"</Label>
                        <Button>"Yo"</Button>
                        <Description>"hi"</Description>
                        <ListBox>
                            <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                        </ListBox>
                    </SelectPopover>
                </Select<Option<Key>>>
            </div>
        </Section>
        <Section name="aria-label">
            <div id="aria-label">
                <Select<Option<Key>> collection=animals() aria_label="Pick an animal">
                    <Parts />
                </Select<Option<Key>>>
            </div>
        </Section>
        <Section name="aria-labelledby">
            <span id="sb-external-label">"External label"</span>
            <div id="aria-labelledby">
                <Select<Option<Key>> collection=animals() aria_labelledby="sb-external-label">
                    <Parts />
                </Select<Option<Key>>>
            </div>
        </Section>
        <Section name="both">
            <span id="sb-both-label">"Both label"</span>
            <div id="both">
                <Select<Option<Key>>
                    collection=animals()
                    aria_label="Pick an animal"
                    aria_labelledby="sb-both-label"
                >
                    <Parts />
                </Select<Option<Key>>>
            </div>
        </Section>
        <Section name="described">
            <div id="described">
                <Select<Option<Key>> collection=animals() is_invalid=true>
                    <Label>"Described animal"</Label>
                    <Parts />
                    <Description>"Pick wisely"</Description>
                    <FieldError>"Invalid animal"</FieldError>
                </Select<Option<Key>>>
            </div>
        </Section>
        <Section name="empty">
            <button id="empty-before">"Before"</button>
            <div id="empty">
                <Select<Option<Key>> collection=animals() on_change=empty_change>
                    <Label>"Empty animal"</Label>
                    <Parts />
                </Select<Option<Key>>>
            </div>
            {empty_log}
        </Section>
        <Section name="stay-open">
            <div id="stay-open">
                <Select<Option<Key>>
                    collection=animals()
                    should_close_on_select=CloseOnSelect::Never
                    on_change=stay_open_change
                >
                    <Label>"Stays open"</Label>
                    <Parts />
                </Select<Option<Key>>>
            </div>
            {stay_open_log}
        </Section>
        <Section name="close-multiple">
            <div id="close-multiple">
                <Select<HashSet<Key>>
                    collection=animals()
                    should_close_on_select=CloseOnSelect::Always
                    on_change=close_multiple_change
                >
                    <Label>"Closes"</Label>
                    <Parts />
                </Select<HashSet<Key>>>
            </div>
            {close_multiple_log}
        </Section>
        <Section name="controlled">
            <div id="controlled">
                <Select<Option<Key>>
                    collection=animals()
                    value=Some(Key::from("dog"))
                    on_change=controlled_change
                >
                    <Label>"Controlled animal"</Label>
                    <Parts />
                </Select<Option<Key>>>
            </div>
            {controlled_log}
        </Section>
        <Section name="controlled-multiple">
            <div id="controlled-multiple">
                <Select<HashSet<Key>>
                    collection=animals()
                    value=HashSet::from([Key::from("dog"), Key::from("kangaroo")])
                    on_change=controlled_multiple_change
                >
                    <Label>"Controlled animals"</Label>
                    <Parts />
                </Select<HashSet<Key>>>
            </div>
            {controlled_multiple_log}
        </Section>
        <Section name="focus">
            <button id="focus-before">"Before"</button>
            <div id="focus">
                <Select<Option<Key>> collection=animals() on_focus_change=focus_change>
                    <Label>"Focus animal"</Label>
                    <Parts />
                </Select<Option<Key>>>
            </div>
            <button id="focus-after">"After"</button>
            {focus_log}
        </Section>
        <Section name="form">
            <div id="form">
                <Select<Option<Key>>
                    collection=animals()
                    name="animal"
                    form="sb-outer-form"
                    default_value=Some(Key::from("cat"))
                >
                    <Label>"Form animal"</Label>
                    <Parts />
                    <HiddenSelect auto_complete="off" />
                </Select<Option<Key>>>
            </div>
            <form id="sb-outer-form"></form>
        </Section>
        <Section name="validate">
            <Form attr:id="validate-form">
                <Select<Option<Key>>
                    collection=animals()
                    name="validated"
                    default_value=Some(Key::from("dog"))
                    validate=no_dogs_aria
                >
                    <Label>"Validated animal"</Label>
                    <Parts />
                    <FieldError />
                    <HiddenSelect />
                </Select<Option<Key>>>
                <button type="submit" id="validate-submit">"Submit"</button>
                <input type="reset" id="validate-reset" />
            </Form>
        </Section>
        <Section name="validate-aria">
            <Form attr:id="validate-aria-form" validation_behavior=ValidationBehavior::Aria>
                <Select<Option<Key>>
                    collection=animals()
                    name="validated-aria"
                    default_value=Some(Key::from("dog"))
                    validate=no_dogs
                >
                    <Label>"Validated animal (ARIA)"</Label>
                    <Parts />
                    <FieldError />
                    <HiddenSelect />
                </Select<Option<Key>>>
            </Form>
        </Section>
        <Section name="server">
            <Form attr:id="server-form" validation_errors=server_errors>
                <Select<Option<Key>> collection=animals() name="server-animal">
                    <Label>"Server animal"</Label>
                    <Parts />
                    <FieldError />
                    <HiddenSelect />
                </Select<Option<Key>>>
            </Form>
        </Section>
        <Section name="default-open">
            <div id="default-open">
                <Select<Option<Key>> collection=animals() default_open=true>
                    <Label>"Opened animal"</Label>
                    <Parts />
                </Select<Option<Key>>>
            </div>
        </Section>
        <Section name="fixed-open">
            <div id="fixed-open">
                <Select<Option<Key>>
                    collection=animals()
                    is_open=true
                    on_open_change=fixed_open_change
                >
                    <Label>"Always open"</Label>
                    <Parts />
                </Select<Option<Key>>>
            </div>
            {fixed_open_log}
        </Section>
        <Section name="sections">
            <div id="sections">
                <Select<Option<Key>> collection=sections on_change=sections_change>
                    <Label>"Grouped animal"</Label>
                    <SelectTrigger>
                        <SelectValue />
                    </SelectTrigger>
                    <SelectPopover>
                        <ListBox>
                            <ListBoxSection key="mammals">
                                <ListBoxSectionHeading />
                                <ListBoxItem key="cat">"Cat"</ListBoxItem>
                                <ListBoxItem key="dog">"Dog"</ListBoxItem>
                            </ListBoxSection>
                            <ListBoxSection key="marsupials">
                                <ListBoxSectionHeading />
                                <ListBoxItem key="kangaroo">
                                    <ListBoxItemLabel>"Kangaroo"</ListBoxItemLabel>
                                    <ListBoxItemDescription>"Jumps"</ListBoxItemDescription>
                                </ListBoxItem>
                            </ListBoxSection>
                        </ListBox>
                    </SelectPopover>
                </Select<Option<Key>>>
            </div>
            {sections_log}
        </Section>
        <Section name="scroll">
            <div id="scroll">
                <Select<Option<Key>> collection=scroll default_value=Some(Key::from(40_u32))>
                    <Label>"Far option"</Label>
                    <SelectTrigger>
                        <SelectValue />
                    </SelectTrigger>
                    <SelectPopover max_height=Some(100.0)>
                        <ListBox>
                            <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                        </ListBox>
                    </SelectPopover>
                </Select<Option<Key>>>
            </div>
        </Section>
        <Section name="typeahead">
            <div id="typeahead">
                <Select<Option<Key>> collection=typeahead on_change=typeahead_change>
                    <Label>"Type-ahead"</Label>
                    <Parts />
                </Select<Option<Key>>>
            </div>
            {typeahead_log}
        </Section>
    }
}
