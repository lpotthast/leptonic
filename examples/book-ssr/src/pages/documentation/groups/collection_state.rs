use indoc::indoc;
use leptos::prelude::*;

use super::demos::collection_state::CollectionStateDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageCollectionState() -> impl IntoView {
    view! {
        <DocPage title="Collection State">
            <p>
                "Listboxes, selects, comboboxes, menus, grid lists, grids, tables, tag groups and trees all show a "
                <b>"collection"</b>": items, optionally grouped in sections, of which some can be focused, selected or "
                "disabled. Leptonic builds the collection from your data and keeps its selection and focus in one place, "
                "so all of these concepts share the same keyboard navigation, type-ahead, selection and disabled handling."
            </p>
            <p>
                "The hooks of this area are used together: you need them when you build a collection concept of your "
                "own, or when you want to drive one of leptonic\u{2019}s from your app state. The concepts\u{2019} hooks "
                "and atoms call them for you."
            </p>

            <Section title="Pages">
                <SectionMembers overview=routes::doc::CollectionState.materialize()/>
            </Section>

            <Section title="Relationships">
                <DocTable headers=&["Building block", "Role"]>
                    <TableRow>
                        <TableCell>
                            <AnchorLink href="#use-collection">"use_collection"</AnchorLink>", "
                            <AnchorLink href="#use-list-collection">"use_list_collection"</AnchorLink>", "
                            <AnchorLink href="#key">"Key"</AnchorLink>
                        </TableCell>
                        <TableCell>"The items, built from your data and identified by keys."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <AnchorLink href="#use-list-state">"use_list_state"</AnchorLink>", "
                            <AnchorLink href="#use-single-select-list-state">"use_single_select_list_state"</AnchorLink>", "
                            <AnchorLink href="#use-list-state-view">"use_list_state_view"</AnchorLink>
                        </TableCell>
                        <TableCell>
                            "The selection and focus of a list. Grids, tables and trees build their own states on it ("
                            <Link href=routes::doc::grid::Hook.materialize()>"Grid Hooks"</Link>", "
                            <Link href=routes::doc::table::Hook.materialize()>"Table Hooks"</Link>", "
                            <Link href=routes::doc::Tree.materialize()>"Tree Hooks"</Link>")."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <AnchorLink href="#use-selectable-collection">"use_selectable_collection"</AnchorLink>", "
                            <AnchorLink href="#use-selectable-list">"use_selectable_list"</AnchorLink>", "
                            <AnchorLink href="#collectionoptions">"CollectionOptions"</AnchorLink>
                        </TableCell>
                        <TableCell>"The keyboard and focus behavior of the element showing the collection."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><AnchorLink href="#use-selectable-item">"use_selectable_item"</AnchorLink></TableCell>
                        <TableCell>"Selection, focus and actions of one item."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <AnchorLink href="#keyboard-delegates">"Keyboard delegates"</AnchorLink>", "
                            <AnchorLink href="#use-type-select">"use_type_select"</AnchorLink>
                        </TableCell>
                        <TableCell>"Which item the arrow keys, page keys and typed text lead to, per layout."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>
                            <Link href=routes::doc::collection_state::Virtualizer.materialize()>"Virtualizer"</Link>", "
                            <Link href=routes::doc::collection_state::UseVirtualizerState.materialize()>"use_virtualizer_state"</Link>
                        </TableCell>
                        <TableCell>"Renders only the visible items of long collections."</TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "The hooks of the collection concepts take a list state: "
                    <Link href=routes::doc::listbox::Hook.materialize()>"use_listbox"</Link>", "
                    <Link href=routes::doc::menu::Hook.materialize()>"use_menu"</Link>", "
                    <Link href=routes::doc::grid_list::Hook.materialize()>"use_grid_list"</Link>" and "
                    <Link href=routes::doc::TagGroup.materialize()>"use_tag_group"</Link>"; "
                    <Link href=routes::doc::select::Hook.materialize()>"use_select"</Link>" and "
                    <Link href=routes::doc::combobox::Hook.materialize()>"use_combobox"</Link>
                    " create one in their own state hooks. Their atoms take a "<Code inline=true>"collection"</Code>
                    " prop and build the state for you."
                </p>
            </Section>

            <Section title="Quick Start">
                <p>
                    "A list of fruits from the hooks alone: "<Code inline=true>"use_collection"</Code>" builds the items, "
                    <Code inline=true>"use_list_state"</Code>" holds the selection (bound to app state), "
                    <Code inline=true>"use_selectable_list"</Code>" makes the list one tab stop with arrow-key navigation "
                    "and type-ahead, and "<Code inline=true>"use_selectable_item"</Code>" makes each item selectable. The "
                    "hooks bring behavior only, so the demo adds the listbox roles and states itself. To build a listbox, "
                    "use the "<Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link>", which do that "
                    "for you."
                </p>
                <Demo
                    description="Fruit list with multiple selection and a disabled item, built from the collection state hooks"
                    source=include_str!("demos/collection_state.rs")
                    source_open=true
                >
                    <CollectionStateDemo/>
                </Demo>
            </Section>

            <Section title="use_collection">
                <p>
                    "Builds a "<Code inline=true>"CollectionMemo"</Code>" with a "<Code inline=true>"CollectionBuilder"</Code>
                    ". The builder function runs again whenever a signal it reads changes; dependents are only notified "
                    "when the result differs."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::collections::use_collection;

                        let collection = use_collection(move |b| {
                            for fruit in fruits.read().iter() {
                                b.item(fruit.id, fruit.name.clone()).disabled(fruit.sold_out);
                            }
                            b.section("exotic", |s| {
                                s.header("exotic-header", "Exotic");
                                s.item("durian", "Durian");
                            });
                        });
                    "#)}
                </Code>
                <p>
                    "The collection is built from data, not from rendered elements, so it exists during server-side "
                    "rendering too. Render your items from the same data, in the same order. Look nodes up with "
                    <Code inline=true>"collection.with(|c| c.get(&key))"</Code>"; each "<Code inline=true>"Node"</Code>
                    " knows its "<Code inline=true>"kind"</Code>", "<Code inline=true>"text_value"</Code>", level, "
                    "position and neighbors. "<Code inline=true>"c.items()"</Code>" iterates the items in order, "
                    "descending into sections."
                </p>

                <Section title="CollectionBuilder">
                    <DocTable headers=&["Method", "Adds"]>
                        <TableRow>
                            <TableCell><Code inline=true>"item(key, text_value) -> ItemBuilder"</Code></TableCell>
                            <TableCell>"An item. Its text is used for type-ahead, filtering and as its accessible text."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"section(key, |s| ..) -> SectionBuilder"</Code></TableCell>
                            <TableCell>"A section, filled by the closure through a builder of its own."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"header(key, text)"</Code></TableCell>
                            <TableCell>"The heading of a section, at its start."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"row(key, text_value, |r| ..) -> ItemBuilder"</Code></TableCell>
                            <TableCell>
                                "A grid row with cells: "<Code inline=true>"r.cell(text_value)"</Code>" adds one, with "
                                <Code inline=true>".col_span(n)"</Code>" and "<Code inline=true>".aria_label(..)"</Code>
                                ". Cell keys are generated: "<Code inline=true>"Key::cell(&row, column)"</Code>"."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"separator(key)"</Code>", "<Code inline=true>"loader(key)"</Code></TableCell>
                            <TableCell>"A visual separator, or a placeholder while more items load."</TableCell>
                        </TableRow>
                    </DocTable>
                    <p>
                        "A key used twice is kept for its first node only (with a warning in development builds). "
                        "Tables have a builder of their own, "<Code inline=true>"TableCollection::build"</Code>", see the "
                        <Link href=routes::doc::table::Hook.materialize()>"Table Hooks"</Link>"."
                    </p>
                </Section>

                <Section title="ItemBuilder">
                    <p>"Configures the item (or row) just added. Chaining is optional: the builder writes through."</p>
                    <DocTable headers=&["Method", "Effect"]>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled(bool)"</Code></TableCell>
                            <TableCell>"Disables the item: it can\u{2019}t be selected, and by default not focused either."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"disabled_behavior(DisabledBehavior)"</Code></TableCell>
                            <TableCell>
                                "How this item behaves while disabled, overriding the selection\u{2019}s "
                                <AnchorLink href="#selectionoptions">"disabled_behavior"</AnchorLink>": with "
                                <Code inline=true>"Selection"</Code>", it can still be focused and have actions."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"aria_label(text)"</Code></TableCell>
                            <TableCell>"An accessible name for an item whose content isn\u{2019}t text."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"link(ItemLink)"</Code></TableCell>
                            <TableCell>
                                "Makes the item a link ("<Code inline=true>"ItemLink::new(href)"</Code>", with optional "
                                <Code inline=true>"target"</Code>" and "<Code inline=true>"rel"</Code>"); "
                                <Code inline=true>"LinkBehavior"</Code>" decides what activating it does."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"children(|b| ..)"</Code></TableCell>
                            <TableCell>"Child items one level deeper, for "<Link href=routes::doc::Tree.materialize()>"trees"</Link>"."</TableCell>
                        </TableRow>
                    </DocTable>
                </Section>

                <Section title="SectionBuilder">
                    <p>
                        "Configures the section just added: "<Code inline=true>"aria_label(text)"</Code>" names a section "
                        "without a visible header."
                    </p>
                </Section>
            </Section>

            <Section title="use_list_collection">
                <p>
                    "Builds a flat collection (no sections) from a signal of values: "<Code inline=true>"key"</Code>
                    " identifies a value, "<Code inline=true>"text_value"</Code>" describes it in plain text."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::hooks::collections::{Key, UseListCollectionInput, use_list_collection};

                        let collection = use_list_collection(UseListCollectionInput {
                            items: fruits,
                            key: // Signal<Vec<Fruit>>
                            |fruit| Key::from(fruit.id),
                            text_value: |fruit| fruit.name.clone(),
                        });
                    ")}
                </Code>
            </Section>

            <Section title="Key">
                <p>
                    "Identifies an item, section, header or cell. A "<Code inline=true>"Key"</Code>" is cheap to clone. "
                    "Integers and strings convert with "<Code inline=true>"Key::from"</Code>" / "<Code inline=true>".into()"</Code>
                    "; keys from different kinds of values never compare equal ("<Code inline=true>"Key::from(1)"</Code>
                    " is not "<Code inline=true>"Key::from(\"1\")"</Code>"). Read them back with "
                    <Code inline=true>"as_str()"</Code>" or "<Code inline=true>"as_i64()"</Code>"; "
                    <Code inline=true>"Display"</Code>" prints the value. Integer keys also accept "<Code inline=true>"u64"</Code>
                    ". For a DOM id use "<Code inline=true>"id_fragment()"</Code>" instead: it escapes whitespace and keeps "
                    "integer and string keys distinct."
                </p>
                <p>
                    "The hooks use this one key type instead of being generic over your item type: they only need identity "
                    "and order, and a concrete type keeps their large bodies from being compiled once per item type."
                </p>

                <Section title="SelectionValue">
                    <p>
                        "The atoms that hold a value-like selection are generic over the type of their values: "
                        <Code inline=true>"RadioGroup<V>"</Code>" ("<Code inline=true>"Option<V>"</Code>"), "
                        <Code inline=true>"CheckboxGroup<V>"</Code>" ("<Code inline=true>"Vec<V>"</Code>"), "
                        <Code inline=true>"ToggleButtonGroup<V>"</Code>" ("<Code inline=true>"HashSet<V>"</Code>"). "
                        <Code inline=true>"Select<S>"</Code>" and "<Code inline=true>"ComboBox<S>"</Code>" take the shape of "
                        "their value as their type, which is their selection mode: "<Code inline=true>"Option<V>"</Code>
                        " for one value, "<Code inline=true>"Vec<V>"</Code>" for several. "
                        <Code inline=true>"V: SelectionValue"</Code>" converts between a value and the "
                        <Code inline=true>"Key"</Code>" that identifies its item in the collection: "
                        <Code inline=true>"to_key(&self) -> Key"</Code>" and "<Code inline=true>"from_key(&Key) -> Option<Self>"</Code>
                        ". It is implemented for "<Code inline=true>"Key"</Code>", "<Code inline=true>"String"</Code>" and the "
                        "integers. The items (radio and checkbox fields, listbox items, toggle buttons) keep taking a "
                        <Code inline=true>"Key"</Code>", into which a value converts."
                    </p>
                    <p>
                        "The atom learns "<Code inline=true>"V"</Code>" from any typed prop ("<Code inline=true>"value"</Code>", "
                        <Code inline=true>"default_value"</Code>", "<Code inline=true>"on_change"</Code>", \u{2026}). A group "
                        "without one (only a "<Code inline=true>"name"</Code>", or nothing) names it: "
                        <Code inline=true>"<RadioGroup<Key> name=\"plan\">"</Code>", "<Code inline=true>"<Select<Option<Key>>>"</Code>
                        ". "<Code inline=true>"default_value"</Code>
                        " takes no "<Code inline=true>"into"</Code>", so that it fixes the type: write "
                        <Code inline=true>"default_value=Key::from(\"pro\")"</Code>" or an enum value, not "<Code inline=true>"\"pro\""</Code>"."
                    </p>
                    <p>
                        "Selecting an item whose key is no value of "<Code inline=true>"V"</Code>" ("<Code inline=true>"from_key"</Code>
                        " returns "<Code inline=true>"None"</Code>") doesn\u{2019}t reach your state: "
                        <Code inline=true>"value=\"small\""</Code>" in a "<Code inline=true>"RadioGroup<Size>"</Code>" whose keys "
                        "are "<Code inline=true>"\"s\""</Code>", "<Code inline=true>"\"m\""</Code>", \u{2026} The atom drops such "
                        "keys and logs a warning in debug builds. Pass values ("<Code inline=true>"value=Size::Small"</Code>
                        ") rather than keys, and the compiler catches the mismatch instead. Keys of different types differ, "
                        "too: a "<Code inline=true>"ListBoxItem"</Code>" keyed "<Code inline=true>"\"10\""</Code>" in a collection "
                        "keyed "<Code inline=true>"10"</Code>" isn\u{2019}t found, which also warns in debug builds."
                    </p>
                </Section>

                <Section title="selection_value!">
                    <p>
                        "Implements "<Code inline=true>"SelectionValue"</Code>" (and "<Code inline=true>"From<T> for Key"</Code>
                        ") for a fieldless enum, naming each variant\u{2019}s key. The keys are what forms submit. Derive "
                        <Code inline=true>"Clone"</Code>", "<Code inline=true>"PartialEq"</Code>", "<Code inline=true>"Eq"</Code>
                        " and "<Code inline=true>"Hash"</Code>" yourself. Two variants with the same key are a compile error:"
                    </p>
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::{
                                atoms::{
                                    field::Label,
                                    radio::{RadioButton, RadioField, RadioGroup},
                                },
                                selection_value,
                            };
                            use leptos::prelude::*;

                            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
                            enum Plan { Free, Pro }

                            selection_value!(Plan { Free = "free", Pro = "pro" });

                            // App state of the enum's type: the group reads and writes `Option<Plan>`.
                            let plan = RwSignal::new(Some(Plan::Free));

                            view! {
                                <RadioGroup value=plan set_value=plan name="plan">
                                    <Label>"Plan"</Label>
                                    <RadioField value=Plan::Free><RadioButton>"Free"</RadioButton></RadioField>
                                    <RadioField value=Plan::Pro><RadioButton>"Pro"</RadioButton></RadioField>
                                </RadioGroup>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_list_state">
                <p>
                    "Holds the selection and focus of a list (listbox, menu, grid list, tag group). When the focused item "
                    "disappears (deleted, filtered out), focus moves to the next remaining item, or else the previous one."
                </p>
                <p>
                    "To keep the selection in your app\u{2019}s state, bind it with "<Code inline=true>"selection"</Code>
                    " (a "<Code inline=true>"ValueBinding"</Code>", e.g. from an "<Code inline=true>"RwSignal"</Code>
                    "): the list shows the signal\u{2019}s selection, the user\u{2019}s selecting writes it, and setting it "
                    "from your code changes the selection. The collection atoms take the two sides as separate props, "
                    <Code inline=true>"selection"</Code>" and "<Code inline=true>"set_selection"</Code>"."
                </p>

                <Section title="Example" id="use-list-state-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            use leptonic::hooks::collections::{Key, Selection, SelectionMode, SelectionOptions, UseListStateInput, use_list_state};
                            use leptos::prelude::*;

                            // App state: the selected fruits.
                            let selected = RwSignal::new(Selection::keys([Key::from("apple")]));

                            let state = use_list_state(UseListStateInput {
                                collection,
                                selection: SelectionOptions {
                                    selection_mode: Signal::stored(SelectionMode::Multiple),
                                    selection: Some(selected.into()),
                                    ..SelectionOptions::default()
                                },
                            });

                            // Queries track signals, so they work in views:
                            let is_selected = move || state.selection.is_selected(&Key::from("apple"));
                        "#)}
                    </Code>
                </Section>

                <Section title="Input" id="use-list-state-input">
                    <ApiTable kind=ApiKind::Input of="UseListStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">"The items. Required."</ApiRow>
                        <ApiRow name="selection" ty="SelectionOptions">
                            "How items are selected, see "<AnchorLink href="#selectionoptions">"SelectionOptions"</AnchorLink>
                            ". Required; "<Code inline=true>"SelectionOptions::default()"</Code>" selects nothing."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="SelectionOptions">
                    <ApiTable kind=ApiKind::Input of="SelectionOptions">
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                            <Code inline=true>"None"</Code>", "<Code inline=true>"Single"</Code>" or "<Code inline=true>"Multiple"</Code>"."
                        </ApiRow>
                        <ApiRow name="selection_behavior" ty="Signal<SelectionBehavior>" default="Toggle">
                            <Code inline=true>"Toggle"</Code>": a press toggles the item. "<Code inline=true>"Replace"</Code>
                            ": a press replaces the selection with the item, and keyboard focus selects (as in a file manager)."
                        </ApiRow>
                        <ApiRow name="default_selection" ty="Selection" default="no keys">
                            "The initial selection. Ignored when "<Code inline=true>"selection"</Code>" is bound."
                        </ApiRow>
                        <ApiRow name="selection" ty="Option<ValueBinding<Selection>>" default="None">
                            "The selection as app state, replacing "<Code inline=true>"default_selection"</Code>": the "
                            "collection shows it, and selecting writes it."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">
                            "Called with the new selection whenever it changes."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="Signal<bool>" default="false">
                            "Prevent deselecting the last selected item."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Signal<HashSet<Key>>" default="empty">
                            "Items that can\u{2019}t be selected, in addition to items built with "
                            <Code inline=true>".disabled(true)"</Code>"."
                        </ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">
                            <Code inline=true>"All"</Code>": disabled items can\u{2019}t be focused or used either. "
                            <Code inline=true>"Selection"</Code>": they only can\u{2019}t be selected."
                        </ApiRow>
                        <ApiRow name="allow_duplicate_selection_events" ty="bool" default="false">
                            "Report a selection even when an interaction leaves it unchanged."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"Selection"</Code>" is either "<Code inline=true>"Selection::All"</Code>" (every "
                        "selectable item, without listing their keys, after "<Keys keys="Control + A"/>" ("
                        <Keys keys="Meta + A"/>" on macOS)) or "<Code inline=true>"Selection::Keys(..)"</Code>
                        "; build one with "<Code inline=true>"Selection::keys([..])"</Code>". "
                        <Code inline=true>"Selection"</Code>", "<Code inline=true>"SelectionOptions"</Code>" and the input "
                        "structs of this page are in "<Code inline=true>"leptonic::hooks::collections"</Code>"."
                    </p>
                </Section>

                <Section title="ListState">
                    <ApiTable kind=ApiKind::Fields of="ListState">
                        <ApiRow name="collection" ty="CollectionMemo">"The items."</ApiRow>
                        <ApiRow name="selection" ty="SelectionManager">
                            "Selection and focus: queries like "<Code inline=true>"is_selected"</Code>", "
                            <Code inline=true>"selected_keys"</Code>", "<Code inline=true>"focused_key"</Code>", "
                            <Code inline=true>"has_focused_key"</Code>" and "
                            <Code inline=true>"is_disabled"</Code>" (tracked; disabled for interaction, by the disabled behavior), "
                            <Code inline=true>"is_item_disabled"</Code>" (disabled at all, whatever the behavior), and mutations like "
                            <Code inline=true>"select"</Code>", "<Code inline=true>"toggle_selection"</Code>", "
                            <Code inline=true>"replace_selection"</Code>", "<Code inline=true>"set_selected_keys"</Code>", "
                            <Code inline=true>"select_all"</Code>", "<Code inline=true>"clear_selection"</Code>" and "
                            <Code inline=true>"set_focused_key"</Code>"."
                        </ApiRow>
                        <ApiRow name="item_elements" ty="ItemElements">
                            "The rendered item elements, registered by "<Code inline=true>"use_selectable_item"</Code>
                            ". Empty during server-side rendering."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_single_select_list_state">
                <p>
                    "A list in which one item at most is selected, like the options of a select or the tabs of a tab list. "
                    "Selecting the selected item again keeps it selected, but is still reported (so a select can close its "
                    "popover)."
                </p>
                <Section title="Input" id="use-single-select-list-state-input">
                    <ApiTable kind=ApiKind::Input of="UseSingleSelectListStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">"The items. Required."</ApiRow>
                        <ApiRow name="default_selected_key" ty="Option<Key>">
                            "The initially selected item. Ignored when "<Code inline=true>"selected_key"</Code>" is bound. Required."
                        </ApiRow>
                        <ApiRow name="selected_key" ty="Option<ValueBinding<Option<Key>>>">
                            "The selected key as app state, replacing "<Code inline=true>"default_selected_key"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Option<Key>>>">
                            "Called whenever the user selects an item, even if it was selected already. Required."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Signal<HashSet<Key>>">"Items that can\u{2019}t be selected. Required."</ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-single-select-list-state-return">
                    <ApiTable kind=ApiKind::Return of="SingleSelectListState">
                        <ApiRow name="list" ty="ListState">"The underlying list state, for the hooks rendering the list."</ApiRow>
                    </ApiTable>
                    <p>
                        "Read and change the selection with "<Code inline=true>"selected_key()"</Code>", "
                        <Code inline=true>"selected_item()"</Code>" (the selected "<Code inline=true>"Node"</Code>") and "
                        <Code inline=true>"set_selected_key(Option<Key>)"</Code>"."
                    </p>
                </Section>
            </Section>

            <Section title="use_list_state_view">
                <p>
                    "Shows a list state through another collection, a subset of the state\u{2019}s collection that you "
                    "compute yourself (e.g. the items of the current page, or a server-side search result). Selection and "
                    "focus are shared with the state; when the focused item leaves the view, focus moves to a neighbor. "
                    "To filter a list, e.g. by the text typed into a combobox, build the view\u{2019}s collection with "
                    <Code inline=true>"Collection::filter"</Code>": it receives each item\u{2019}s text and node, keeps the "
                    "children of kept items and drops sections left empty. \u{201c}Select all\u{201d} still covers every item "
                    "of the state."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use std::sync::Arc;

                        use leptonic::hooks::collections::{UseListStateViewInput, use_list_state_view};
                        use leptos::prelude::*;


                        // The first ten items of the state's collection.
                        let source = state.collection;
                        let first_page = Memo::new(move |_| {
                            source.with(|c| {
                                let keys: Vec<_> = c.items().take(10).map(|node| node.key.clone()).collect();
                                Arc::new(c.filter(|_, node| keys.contains(&node.key)))
                            })
                        });
                        let page = use_list_state_view(UseListStateViewInput { state, collection: first_page });
                    ")}
                </Code>
            </Section>

            <Section title="CollectionOptions">
                <p>
                    "Configures the keyboard and focus behavior of the element showing a collection. "
                    <Code inline=true>"use_selectable_collection"</Code>", "<Code inline=true>"use_selectable_list"</Code>
                    " and the hooks of the collection concepts take it as "<Code inline=true>"options"</Code>"."
                </p>
                <ApiTable kind=ApiKind::Input of="CollectionOptions">
                    <ApiRow name="auto_focus" ty="Signal<Option<AutoFocus>>" default="None">
                        "Move focus into the collection when it mounts: to the "<Code inline=true>"Selected"</Code>" item, "
                        "the "<Code inline=true>"First"</Code>" or the "<Code inline=true>"Last"</Code>" one. A selected item "
                        "always takes precedence."
                    </ApiRow>
                    <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                    <ApiRow name="disallow_empty_selection" ty="bool" default="false">
                        "Keep "<Keys keys="Escape"/>" from clearing the selection."
                    </ApiRow>
                    <ApiRow name="disallow_select_all" ty="bool" default="false">
                        "Disable "<Keys keys="Control + A"/>" ("<Keys keys="Meta + A"/>" on macOS)."
                    </ApiRow>
                    <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">
                        <Code inline=true>"ClearSelection"</Code>": "<Keys keys="Escape"/>" clears the selection. "
                        <Code inline=true>"None"</Code>": the key press bubbles, e.g. to close a popover."
                    </ApiRow>
                    <ApiRow name="select_on_focus" ty="SelectOnFocus" default="Auto">
                        "Select items as keyboard focus moves to them: "<Code inline=true>"Always"</Code>", "
                        <Code inline=true>"Never"</Code>", or "<Code inline=true>"Auto"</Code>" (with the "<Code inline=true>"Replace"</Code>" selection behavior)."
                    </ApiRow>
                    <ApiRow name="disallow_type_ahead" ty="bool" default="false">"Disable jumping to items by typing."</ApiRow>
                    <ApiRow name="allows_tab_navigation" ty="bool" default="false">
                        "Let "<Keys keys="Tab"/>" move between focusable elements inside items instead of leaving the collection."
                    </ApiRow>
                    <ApiRow name="should_use_virtual_focus" ty="bool" default="false">
                        "Keep DOM focus elsewhere (e.g. in a combobox\u{2019}s input) and point to the focused item with "
                        <Code inline=true>"aria-activedescendant"</Code>" (see "
                        <Link href=routes::doc::focus::VirtualFocus.materialize()>"virtual_focus"</Link>")."
                    </ApiRow>
                    <ApiRow name="link_behavior" ty="Signal<LinkBehavior>" default="Action">
                        "How link items behave: "<Code inline=true>"Action"</Code>" (a press opens the link), "
                        <Code inline=true>"Selection"</Code>" (selecting opens it), "<Code inline=true>"Override"</Code>
                        " (a press opens it and never selects) or "<Code inline=true>"None"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="use_selectable_collection">
                <p>
                    "Keyboard navigation, selection and focus management for the element showing a collection: the arrow, "
                    "page, Home and End keys, type-ahead, select all, "<Keys keys="Escape"/>", and moving focus into and out "
                    "of the collection as a single tab stop. Where the keys lead is up to the "
                    <AnchorLink href="#keyboard-delegates">"keyboard delegate"</AnchorLink>". For lists, use "
                    <AnchorLink href="#use-selectable-list">"use_selectable_list"</AnchorLink>", which brings the list delegate."
                </p>
                <Section title="Input" id="use-selectable-collection-input">
                    <ApiTable kind=ApiKind::Input of="UseSelectableCollectionInput">
                        <ApiRow name="selection" ty="SelectionManager">"The state\u{2019}s selection. Required."</ApiRow>
                        <ApiRow name="item_elements" ty="ItemElements">"The state\u{2019}s item elements. Required."</ApiRow>
                        <ApiRow name="delegate" ty="Signal<Arc<dyn KeyboardDelegate>>">
                            "Keyboard navigation. A signal, as delegates depend on the locale (reading direction, collation). Required."
                        </ApiRow>
                        <ApiRow name="element" ty="CapturedElement">
                            "The collection element; the returned props capture it. Required."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions">
                            "See "<AnchorLink href="#collectionoptions">"CollectionOptions"</AnchorLink>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-selectable-collection-return">
                    <ApiTable kind=ApiKind::Return of="UseSelectableCollectionReturn">
                        <ApiRow name="props" ty="UseSelectableCollectionProps">
                            "Spread onto the collection element with "<Code inline=true>"{..props.into_attrs()}"</Code>": the "
                            "tab index ("<Code inline=true>"0"</Code>" while no item is focused, so "<Keys keys="Tab"/>
                            " reaches the collection), the key, focus, mouse and scroll handlers, and "
                            <Code inline=true>"data-collection"</Code>". Its "<Code inline=true>"collection_id"</Code>
                            " goes to "<Code inline=true>"use_selectable_item"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Keyboard" id="use-selectable-collection-keyboard">
                    <KeyboardTable>
                        <KeyRow keys="Arrow keys">"Move focus to the next item in that direction, as the delegate decides."</KeyRow>
                        <KeyRow keys="Shift + Arrow keys">"With multiple selection: extend the selection."</KeyRow>
                        <KeyRow keys="Control + Arrow keys">
                            "Move focus without selecting, with the "<Code inline=true>"Replace"</Code>" behavior ("
                            <Keys keys="Option + ArrowDown"/>" and so on, on macOS)."
                        </KeyRow>
                        <KeyRow keys="Home / End">"Focus the first or last item (in a grid row: the first or last cell)."</KeyRow>
                        <KeyRow keys="Control + Home / Control + End">
                            "In a grid: focus the first or last cell of the grid ("<Keys keys="Meta + Home"/>" / "
                            <Keys keys="Meta + End"/>" on macOS)."
                        </KeyRow>
                        <KeyRow keys="PageDown / PageUp">"Move focus by the visible height."</KeyRow>
                        <KeyRow keys="Control + A">
                            "With multiple selection: select all ("<Keys keys="Meta + A"/>" on macOS)."
                        </KeyRow>
                        <KeyRow keys="Escape">"Clear the selection (see "<Code inline=true>"escape_key_behavior"</Code>")."</KeyRow>
                        <KeyRow keys="Any character">"Type-ahead, see "<AnchorLink href="#use-type-select">"use_type_select"</AnchorLink>"."</KeyRow>
                        <KeyRow keys="Tab">"Leave the collection (unless "<Code inline=true>"allows_tab_navigation"</Code>")."</KeyRow>
                    </KeyboardTable>
                    <p>
                        <Keys keys="Space"/>" and "<Keys keys="Enter"/>" on an item are handled by "
                        <AnchorLink href="#use-selectable-item">"use_selectable_item"</AnchorLink>"."
                    </p>
                </Section>
            </Section>

            <Section title="use_selectable_list">
                <p>
                    <Code inline=true>"use_selectable_collection"</Code>" for lists: it creates the list keyboard delegate "
                    "("<AnchorLink href="#use-list-keyboard-delegate">"use_list_keyboard_delegate"</AnchorLink>") for the "
                    "list\u{2019}s orientation and layout, with the current locale\u{2019}s reading direction and collation. "
                    "It returns the props of "<Code inline=true>"use_selectable_collection"</Code>"."
                </p>
                <Section title="Input" id="use-selectable-list-input">
                    <ApiTable kind=ApiKind::Input of="UseSelectableListInput">
                        <ApiRow name="state" ty="ListState">"From "<Code inline=true>"use_list_state"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The list element; the returned props capture it. Required."</ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>">
                            "Whether the items are stacked vertically (up and down arrows) or horizontally (left and right). Required."
                        </ApiRow>
                        <ApiRow name="layout" ty="ListLayout">
                            <Code inline=true>"Stack"</Code>" (one item per row) or "<Code inline=true>"Grid"</Code>
                            " (items wrap into rows; up and down find the item in the same column). Required."
                        </ApiRow>
                        <ApiRow name="keyboard_delegate" ty="Option<Signal<Arc<dyn KeyboardDelegate>>>">
                            "Replaces the list keyboard delegate. Required ("<Code inline=true>"None"</Code>" for the default)."
                        </ApiRow>
                        <ApiRow name="layout_delegate" ty="Option<Arc<dyn LayoutDelegate>>">
                            "Where items are on screen when only the visible ones are rendered (virtualized), for paging and "
                            "scrolling to the focused item: the "<Code inline=true>"layout_delegate()"</Code>" of "
                            <Link href=routes::doc::collection_state::UseVirtualizerState.materialize()>"use_virtualizer_state"</Link>
                            ". Required ("<Code inline=true>"None"</Code>": the rendered item elements)."
                        </ApiRow>
                        <ApiRow name="options" ty="CollectionOptions">
                            "See "<AnchorLink href="#collectionoptions">"CollectionOptions"</AnchorLink>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_selectable_item">
                <p>
                    "Selection, focus and actions for one item: pressing selects it (respecting the selection mode, "
                    "behavior and modifier keys), the focused item gets DOM focus and is the tab stop, and items with an "
                    "action or link perform it on press, "<Keys keys="Enter"/>" or double-click. "<Keys keys="Space"/>
                    " selects the focused item."
                </p>
                <Section title="Input" id="use-selectable-item-input">
                    <ApiTable kind=ApiKind::Input of="UseSelectableItemInput">
                        <ApiRow name="selection" ty="SelectionManager">"The state\u{2019}s selection. Required."</ApiRow>
                        <ApiRow name="item_elements" ty="ItemElements">"The state\u{2019}s item elements, where the item registers. Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The item\u{2019}s key in the collection. Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The item element; the returned props capture it. Required."</ApiRow>
                        <ApiRow name="id" ty="Option<String>">"The element id. Required; "<Code inline=true>"None"</Code>" generates one."</ApiRow>
                        <ApiRow name="collection_id" ty="String">
                            "The "<Code inline=true>"collection_id"</Code>" of the collection\u{2019}s props. Required."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">
                            "Disables the item, in addition to the selection\u{2019}s disabled keys. Required."
                        </ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="bool">
                            "Select when the press ends instead of when it starts (menus, select popovers). Required."
                        </ApiRow>
                        <ApiRow name="allows_different_press_origin" ty="bool">
                            "Let a press that started elsewhere (e.g. on a menu trigger) select the item when it ends on it. Required."
                        </ApiRow>
                        <ApiRow name="on_action" ty="Signal<Option<Callback<()>>>">
                            "The item\u{2019}s action, e.g. opening a detail view. Without a selection mode, pressing performs "
                            "it; with one, "<Keys keys="Enter"/>" or a double-click does (with the "<Code inline=true>"Replace"</Code>
                            " behavior). A signal, so that an item can gain or lose its action (a tree row that gets children "
                            "becomes expandable). Required; "<Code inline=true>"Signal::stored(None)"</Code>" for none."
                        </ApiRow>
                        <ApiRow name="on_context_menu" ty="Option<Callback<ContextMenuEvent>>">
                            "Called when a context menu is requested on the item (right click, "<Keys keys="Shift + F10"/>", the context menu key, a long press on iOS); the item\u{2019}s menu then replaces the browser\u{2019}s. Required; "<Code inline=true>"None"</Code>" keeps the browser\u{2019}s menu."
                        </ApiRow>
                        <ApiRow name="link_behavior" ty="Signal<LinkBehavior>">
                            "See "<AnchorLink href="#collectionoptions">"CollectionOptions"</AnchorLink>". Required."
                        </ApiRow>
                        <ApiRow name="focus" ty="Option<FocusItem>">
                            "Moves DOM focus when the item becomes the focused key, in place of focusing the item element, "
                            "e.g. to a grid row\u{2019}s cell: "<Code inline=true>"FocusItem::new(|| ..)"</Code>". Required; "
                            <Code inline=true>"None"</Code>" focuses the element."
                        </ApiRow>
                        <ApiRow name="should_use_virtual_focus" ty="bool">
                            "DOM focus stays elsewhere; the item is focused virtually. Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-selectable-item-return">
                    <ApiTable kind=ApiKind::Return of="UseSelectableItemReturn">
                        <ApiRow name="props" ty="PropsWithStyles<UseSelectableItemProps>">
                            "Spread onto the item: "<Code inline=true>"let (attrs, styles) = props.into_parts();"</Code>
                            " The id, the tab index ("<Code inline=true>"0"</Code>" for the focused item), the press and focus "
                            "handlers. No role or ARIA states: those are up to the concept."
                        </ApiRow>
                        <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the item is being pressed."</ApiRow>
                        <ApiRow name="is_selected" ty="Signal<bool>">"Whether the item is selected."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">"Whether the item has keyboard focus within the focused collection."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether the item is disabled."</ApiRow>
                        <ApiRow name="allows_selection" ty="Signal<bool>">"Whether pressing the item can select it."</ApiRow>
                        <ApiRow name="has_action" ty="Signal<bool>">"Whether the item has an action (or link) to perform."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_type_select">
                <p>
                    "Type-ahead: typing characters moves focus to the next item whose text starts with what was typed. "
                    "Typing within a second continues the search (\u{201c}ap\u{201d} finds \u{201c}Apple\u{201d} after "
                    "\u{201c}Avocado\u{201d}), and "<Keys keys="Space"/>" then belongs to the search instead of selecting. "
                    <Code inline=true>"use_selectable_collection"</Code>" includes it; use it alone for elements that "
                    "aren\u{2019}t a collection themselves, like a select\u{2019}s closed trigger."
                </p>
                <Section title="Input" id="use-type-select-input">
                    <ApiTable kind=ApiKind::Input of="UseTypeSelectInput">
                        <ApiRow name="delegate" ty="Signal<Arc<dyn KeyboardDelegate>>">
                            "Finds items by their text ("<Code inline=true>"key_for_search"</Code>"). Required."
                        </ApiRow>
                        <ApiRow name="selection" ty="SelectionManager">"The state\u{2019}s selection, whose focus moves. Required."</ApiRow>
                        <ApiRow name="on_type_select" ty="Option<Callback<Key>>">
                            "Called with the item type-ahead moved to, e.g. to select it. Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-type-select-return">
                    <ApiTable kind=ApiKind::Return of="UseTypeSelectReturn">
                        <ApiRow name="props" ty="UseTypeSelectProps">
                            "Two key handlers for the element: attach "<Code inline=true>"on_keydown_capture"</Code>" with "
                            <Code inline=true>"into_on(ev::capture(ev::keydown))"</Code>" and "<Code inline=true>"on_keydown"</Code>
                            " with "<Code inline=true>"into_on(ev::keydown)"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Keyboard Delegates">
                <p>
                    "A "<Code inline=true>"KeyboardDelegate"</Code>" answers \u{201c}which item is next?\u{201d} for a "
                    "layout: "<Code inline=true>"key_below"</Code>", "<Code inline=true>"key_above"</Code>", "
                    <Code inline=true>"key_left_of"</Code>", "<Code inline=true>"key_right_of"</Code>", "
                    <Code inline=true>"key_page_below"</Code>", "<Code inline=true>"key_page_above"</Code>", "
                    <Code inline=true>"first_key"</Code>", "<Code inline=true>"last_key"</Code>" and "
                    <Code inline=true>"key_for_search"</Code>" (all returning "<Code inline=true>"None"</Code>" by default). "
                    "The hooks below create the delegates leptonic\u{2019}s concepts use, measuring the rendered items and "
                    "following the locale. Pass one (or your own implementation, e.g. wrapping one of them) as "
                    <Code inline=true>"keyboard_delegate"</Code>" to replace a concept\u{2019}s navigation, or use one "
                    "for keyboard navigation elsewhere, as drag and drop does for its drop targets."
                </p>

                <Section title="use_list_keyboard_delegate">
                    <p>
                        <Code inline=true>"use_list_keyboard_delegate(input: UseListKeyboardDelegateInput) -> Signal<Arc<dyn KeyboardDelegate>>"</Code>
                    </p>
                    <p>
                        "Lists: listboxes, menus, grid lists, tag groups, selects and comboboxes. Moves in collection order, "
                        "skipping disabled items; in a "<Code inline=true>"Grid"</Code>" layout, up and down find the item "
                        "in the same column. Type-ahead compares with the locale\u{2019}s collation."
                    </p>
                    <p>"The input has no defaults: set every field."</p>
                    <ApiTable kind=ApiKind::Input of="UseListKeyboardDelegateInput">
                        <ApiRow name="state" ty="ListState">"The list\u{2019}s collection and selection. Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">
                            "The list element, in which the rendered items are measured. Required."
                        </ApiRow>
                        <ApiRow name="orientation" ty="Signal<Orientation>">
                            "The direction the items follow each other: "<Code inline=true>"Vertical"</Code>" or "
                            <Code inline=true>"Horizontal"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="layout" ty="ListLayout">
                            <Code inline=true>"Stack"</Code>" (one item per row) or "<Code inline=true>"Grid"</Code>
                            " (items wrap into rows). Required."
                        </ApiRow>
                        <ApiRow name="layout_delegate" ty="Option<Arc<dyn LayoutDelegate>>">
                            "Where the items are. "<Code inline=true>"None"</Code>" measures the rendered items; pass a "
                            "virtualizer\u{2019}s "<Code inline=true>"layout_delegate()"</Code>" so that page keys reach items "
                            "that aren\u{2019}t rendered. Required."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="use_grid_keyboard_delegate">
                    <p>
                        <Code inline=true>"use_grid_keyboard_delegate(UseGridKeyboardDelegateInput { state, element }) -> Signal<Arc<dyn KeyboardDelegate>>"</Code>
                    </p>
                    <p>
                        "Grids: up and down move between rows (or the cells of the same column), left and right between a "
                        "row and its cells, mirrored in right-to-left locales. See the "
                        <Link href=routes::doc::grid::Hook.materialize()>"Grid Hooks"</Link>"."
                    </p>
                </Section>

                <Section title="use_table_keyboard_delegate">
                    <p>
                        <Code inline=true>"use_table_keyboard_delegate(UseTableKeyboardDelegateInput { state, element }) -> Signal<Arc<dyn KeyboardDelegate>>"</Code>
                    </p>
                    <p>
                        "Tables: the grid navigation, plus the column headers above the first row. See the "
                        <Link href=routes::doc::table::Hook.materialize()>"Table Hooks"</Link>"."
                    </p>
                </Section>
            </Section>
        </DocPage>
    }
}
