use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::{kit::*, routes};

#[component]
pub fn PageCollections() -> impl IntoView {
    view! {
        <DocPage title="Collections">
            <p>
                "Listboxes, selects, menus, combo boxes and grid lists all show a "<b>"collection"</b>": a list of items, "
                "optionally grouped in sections, of which some can be focused, selected or disabled. Leptonic builds the "
                "collection from your data and keeps the state about it (selection, focus) in one place, so every widget "
                "gets the same keyboard navigation, type-ahead, selection and disabled handling."
            </p>

            <Section title="Overview">
                <DocTable headers=&["Building block", "Purpose"]>
                    <TableRow>
                        <TableCell><Code inline=true>"use_collection"</Code>", "<Code inline=true>"use_list_collection"</Code></TableCell>
                        <TableCell>"Build the items (and sections) from your data, as a reactive "<Code inline=true>"CollectionMemo"</Code>"."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Key"</Code></TableCell>
                        <TableCell>"Identifies an item. Created from your own ids."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_list_state"</Code></TableCell>
                        <TableCell>"Holds the selection and focus of a list (listbox, menu, grid list)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_single_select_list_state"</Code></TableCell>
                        <TableCell>"A list in which exactly one item is selected (a select\u{2019}s options)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"use_filtered_list_state"</Code></TableCell>
                        <TableCell>"The same state, showing only the items matching a filter (a combo box)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"CollectionOptions"</Code></TableCell>
                        <TableCell>"Keyboard and focus behavior of the element showing the collection."</TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "The widget hooks take a list state: "<Link href=routes::doc::listbox::Hook.materialize()>"use_listbox"</Link>", "
                    <Link href=routes::doc::menu::Hook.materialize()>"use_menu"</Link>", "
                    <Link href=routes::doc::grid::Hook.materialize()>"use_grid_list"</Link>", "
                    <Link href=routes::doc::select::Hook.materialize()>"use_select"</Link>" and "
                    <Link href=routes::doc::combobox::Hook.materialize()>"use_combobox"</Link>
                    ". Their atoms build the state for you from a "<Code inline=true>"collection"</Code>" prop."
                </p>
            </Section>

            <Section title="use_collection">
                <p>
                    "Builds a collection with a "<Code inline=true>"CollectionBuilder"</Code>". The builder function runs again "
                    "whenever a signal it reads changes; dependents are only notified when the result differs."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
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

                <DocTable headers=&["Builder method", "Adds"]>
                    <TableRow>
                        <TableCell><Code inline=true>"item(key, text_value)"</Code></TableCell>
                        <TableCell>
                            "An item. The text value is used for type-ahead, filtering and as accessible text. Chain "
                            <Code inline=true>".disabled(bool)"</Code>", "<Code inline=true>".aria_label(..)"</Code>", "
                            <Code inline=true>".link(..)"</Code>" (items that navigate) or "<Code inline=true>".children(..)"</Code>
                            " (tree items)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"section(key, |s| ..)"</Code></TableCell>
                        <TableCell>"A section; chain "<Code inline=true>".aria_label(..)"</Code>" for sections without a visible header."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"header(key, text)"</Code></TableCell>
                        <TableCell>"The heading of a section, at its start."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"separator(key)"</Code>", "<Code inline=true>"loader(key)"</Code></TableCell>
                        <TableCell>"A visual separator, or a placeholder while more items load."</TableCell>
                    </TableRow>
                </DocTable>

                <p>
                    "The collection is built from data, not from rendered elements, so it exists during server-side rendering "
                    "too. Render your items from the same data, in the same order. Look nodes up with "
                    <Code inline=true>"collection.with(|c| c.get(&key))"</Code>"; each "<Code inline=true>"Node"</Code>" knows its "
                    <Code inline=true>"kind"</Code>", "<Code inline=true>"text_value"</Code>", position and neighbors."
                </p>

                <Section title="use_list_collection">
                    <p>"For flat lists (no sections), build the collection from a signal of values:"</p>
                    <Code language=Language::Rust>
                        {indoc!(r"
                            let collection = use_list_collection(
                                fruits,                     // Signal<Vec<Fruit>>
                                |fruit| Key::from(fruit.id),
                                |fruit| fruit.name.clone(),
                            );
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="Keys">
                <p>
                    "A "<Code inline=true>"Key"</Code>" is a cheap-to-clone identifier. Integers and strings convert with "
                    <Code inline=true>"Key::from"</Code>" / "<Code inline=true>".into()"</Code>"; keys from different kinds of "
                    "values never compare equal ("<Code inline=true>"Key::from(1)"</Code>" is not "<Code inline=true>"Key::from(\"1\")"</Code>
                    "). Atoms and components with typed APIs convert your values with the "<Code inline=true>"ToKey"</Code>" trait."
                </p>
                <p>
                    "The hooks use this one key type instead of being generic over your item type: they only need identity and "
                    "order, and a concrete type keeps the large hook bodies from being compiled once per item type."
                </p>
            </Section>

            <Section title="use_list_state">
                <p>
                    "Holds the selection and focus of a list. When the focused item disappears (deleted, filtered out), focus "
                    "moves to the next remaining item, or else the previous one. To keep the selection in your app\u{2019}s "
                    "state, bind it to a signal with "<Code inline=true>"selection"</Code>": the list shows the signal\u{2019}s "
                    "selection, the user\u{2019}s selecting writes it, and setting it from your code changes the selection. "
                    "The collection atoms ("<Code inline=true>"ListBox"</Code>", "<Code inline=true>"Grid"</Code>", "
                    <Code inline=true>"Table"</Code>", \u{2026}) take the same binding as their "<Code inline=true>"selection"</Code>" prop."
                </p>

                <Code language=Language::Rust>
                    {indoc!(r#"
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

                <Section title="SelectionOptions">
                    <ApiTable kind=ApiKind::Input of="SelectionOptions">
                        <ApiRow name="selection_mode" ty="Signal<SelectionMode>" default="None">
                            <Code inline=true>"None"</Code>", "<Code inline=true>"Single"</Code>" or "<Code inline=true>"Multiple"</Code>"."
                        </ApiRow>
                        <ApiRow name="selection_behavior" ty="SelectionBehavior" default="Toggle">
                            "Whether a press toggles the item, or replaces the selection with it."
                        </ApiRow>
                        <ApiRow name="default_selection" ty="Selection" default="no keys">"The initial selection."</ApiRow>
                        <ApiRow name="selection" ty="Option<ValueBinding<Selection>>" default="None">
                            "The selection as app state, replacing "<Code inline=true>"default_selection"</Code>": the collection shows it, and selecting writes it."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Selection>>" default="None">
                            "Called with the new selection whenever it changes."
                        </ApiRow>
                        <ApiRow name="disallow_empty_selection" ty="Signal<bool>" default="false">"Prevent deselecting the last selected item."</ApiRow>
                        <ApiRow name="disabled_keys" ty="Signal<HashSet<Key>>" default="empty">
                            "Items that can\u{2019}t be selected, in addition to items built with "<Code inline=true>".disabled(true)"</Code>"."
                        </ApiRow>
                        <ApiRow name="disabled_behavior" ty="DisabledBehavior" default="All">
                            <Code inline=true>"All"</Code>": disabled items can\u{2019}t be focused or used either. "
                            <Code inline=true>"Selection"</Code>": they only can\u{2019}t be selected."
                        </ApiRow>
                        <ApiRow name="allow_duplicate_selection_events" ty="bool" default="false">
                            "Report a selection even when an interaction leaves it unchanged."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="ListState">
                    <ApiTable kind=ApiKind::Fields of="ListState">
                        <ApiRow name="collection" ty="CollectionMemo">"The items."</ApiRow>
                        <ApiRow name="selection" ty="SelectionManager">
                            "Selection and focus: queries like "<Code inline=true>"is_selected"</Code>", "
                            <Code inline=true>"selected_keys"</Code>", "<Code inline=true>"focused_key"</Code>" (tracked), and "
                            "mutations like "<Code inline=true>"select"</Code>", "<Code inline=true>"toggle_selection"</Code>", "
                            <Code inline=true>"set_selected_keys"</Code>", "<Code inline=true>"select_all"</Code>", "
                            <Code inline=true>"clear_selection"</Code>"."
                        </ApiRow>
                        <ApiRow name="item_elements" ty="ItemElements">"The rendered item elements, registered by the item hooks."</ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"Selection"</Code>" is either "<Code inline=true>"Selection::All"</Code>" (every selectable "
                        "item, without listing their keys) or "<Code inline=true>"Selection::Keys(..)"</Code>"."
                    </p>
                </Section>
            </Section>

            <Section title="use_single_select_list_state">
                <p>
                    "A list in which exactly one item is selected, like the options of a select. It takes a "
                    <Code inline=true>"collection"</Code>", "<Code inline=true>"default_selected_key"</Code>", "
                    <Code inline=true>"disabled_keys"</Code>" and an "<Code inline=true>"on_selection_change"</Code>
                    " callback receiving the selected key. Selecting the selected item again keeps it selected, but is still "
                    "reported (so a select can close its popover). Read and change the selection with "
                    <Code inline=true>"selected_key()"</Code>", "<Code inline=true>"selected_item()"</Code>" and "
                    <Code inline=true>"set_selected_key(..)"</Code>"."
                </p>
            </Section>

            <Section title="use_filtered_list_state">
                <p>
                    "Views a list state through a filter, e.g. the text typed into a combo box. Selection and focus are shared "
                    "with the unfiltered state, and \u{201c}select all\u{201d} still covers every item."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        let visible = use_filtered_list_state(state, move |text, _node| {
                            text.to_lowercase().contains(&query.get().to_lowercase())
                        });
                    ")}
                </Code>
            </Section>

            <Section title="CollectionOptions">
                <p>
                    "Configures the keyboard and focus behavior of the element showing a collection. Widget hooks take it as "
                    <Code inline=true>"options"</Code>"."
                </p>
                <ApiTable kind=ApiKind::Input of="CollectionOptions">
                    <ApiRow name="auto_focus" ty="Signal<Option<AutoFocus>>" default="None">
                        "Move focus into the collection when it mounts: to the "<Code inline=true>"Selected"</Code>" item, the "
                        <Code inline=true>"First"</Code>" or the "<Code inline=true>"Last"</Code>" one."
                    </ApiRow>
                    <ApiRow name="should_focus_wrap" ty="bool" default="false">"Arrow keys wrap around at the ends."</ApiRow>
                    <ApiRow name="disallow_empty_selection" ty="bool" default="false">"Keep Escape from clearing the selection."</ApiRow>
                    <ApiRow name="disallow_select_all" ty="bool" default="false">"Disable "<Keys keys="Control + A"/>"."</ApiRow>
                    <ApiRow name="escape_key_behavior" ty="EscapeKeyBehavior" default="ClearSelection">"What Escape does."</ApiRow>
                    <ApiRow name="select_on_focus" ty="Option<bool>" default="None">
                        "Select items as keyboard focus moves to them; by default with the "<Code inline=true>"Replace"</Code>
                        " selection behavior."
                    </ApiRow>
                    <ApiRow name="disallow_type_ahead" ty="bool" default="false">"Disable jumping to items by typing."</ApiRow>
                    <ApiRow name="allows_tab_navigation" ty="bool" default="false">
                        "Let Tab move between focusable elements inside items instead of leaving the collection."
                    </ApiRow>
                    <ApiRow name="link_behavior" ty="LinkBehavior" default="Action">"How link items behave when activated."</ApiRow>
                    <ApiRow name="should_use_virtual_focus" ty="bool" default="false">
                        "Keep DOM focus elsewhere (e.g. in a combo box\u{2019}s input) and point to the focused item with "
                        <Code inline=true>"aria-activedescendant"</Code>"."
                    </ApiRow>
                </ApiTable>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox"</Link></li>
                <li><Link href=routes::doc::Select.materialize()>"Select"</Link></li>
                <li><Link href=routes::doc::Menu.materialize()>"Menu"</Link></li>
                <li><Link href=routes::doc::Combobox.materialize()>"Combobox"</Link></li>
                <li><Link href=routes::doc::Grid.materialize()>"Grid"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
