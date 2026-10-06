use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use super::demos::{tabs::TabsDemo, tabs_manual::TabsManualDemo, tabs_vertical::TabsVerticalDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseTabsHook() -> impl IntoView {
    view! {
        <DocPage title="Tabs Hooks">
            <p>
                "The tab hooks build accessible tabs from your own markup: "<Code inline=true>"use_tab_list_state"</Code>
                " holds the tabs and the selected tab, "<Code inline=true>"use_tab_list"</Code>" turns an element into the "
                "tab list, "<Code inline=true>"use_tab"</Code>" renders each tab and "<Code inline=true>"use_tab_panel"</Code>
                " the content of the selected tab. See the "<Link href=routes::doc::Tabs.materialize()>"Tabs overview"</Link>
                " for concept guidance."
            </p>

            <ReactAria hook="useTabList"/>

            <Section title="Demo">
                <p>
                    "Tabs built from the hooks. Click a tab, or tab into the tab list and use the arrow keys: they select "
                    "the tab they move to and skip the disabled \u{201c}Reviews\u{201d} tab. "<Keys keys="Tab"/>" moves on to the "
                    "panel, or to the checkbox in the \u{201c}Shipping\u{201d} panel."
                </p>
                <Demo
                    description="Horizontal tabs with a disabled tab, one panel showing the selected tab, and a toggle disabling all tabs"
                    source=include_str!("demos/tabs.rs")
                >
                    <TabsDemo/>
                </Demo>
            </Section>

            <Section title="Collections">
                <p>
                    "The tabs are a "<Link href=routes::doc::Collections.materialize()>"collection"</Link>": every tab has a "
                    <Code inline=true>"Key"</Code>" and a text. The keyboard navigation and the selection work on the "
                    "collection, not on what you render, so render one tab per collection item, in collection order. A tab "
                    "disabled in the collection ("<Code inline=true>"ItemBuilder::disabled"</Code>") or through "
                    <Code inline=true>"disabled_keys"</Code>" can\u{2019}t be selected, and the arrow keys skip it."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        let collection = use_collection(|b| {
                            b.item("details", "Details");
                            b.item("reviews", "Reviews").disabled(true);
                            b.item("shipping", "Shipping");
                        });
                    "#)}
                </Code>
            </Section>

            <Section title="use_tab_list_state">
                <p>
                    "Creates the state: the tabs, the selected tab and keyboard focus. One tab is always selected \u{2014} the "
                    "default, or the first enabled tab. If the selected tab disappears from the collection, the first enabled "
                    "tab is selected. The hook owns the selection: you get notified of changes and can select a tab through "
                    "the state, but you don\u{2019}t pass in a selected key."
                </p>

                <Section title="Input" id="use-tab-list-state-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseTabListStateInput::new(collection)"</Code>
                        " and change the fields you need with struct update syntax."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseTabListStateInput">
                        <ApiRow name="collection" ty="CollectionMemo">"The tabs. Required."</ApiRow>
                        <ApiRow name="default_selected_key" ty="Option<Key>" default="None">
                            "The initially selected tab. "<Code inline=true>"None"</Code>": the first enabled tab (the first tab "
                            "if all are disabled)."
                        </ApiRow>
                        <ApiRow name="selected_key" ty="Option<ValueBinding<Key>>" default="None">
                            "The selected tab as app state ("<Code inline=true>"Some(rw_signal.into())"</Code>"), replacing "<Code inline=true>"default_selected_key"</Code>". A key that isn\u{2019}t an enabled tab is replaced by the first enabled tab."
                        </ApiRow>
                        <ApiRow name="on_selection_change" ty="Option<Callback<Key>>" default="None">
                            "Called with the key of the tab the user selects, even if it was selected already."
                        </ApiRow>
                        <ApiRow name="disabled_keys" ty="Signal<HashSet<Key>>" default="empty">
                            "Tabs that can\u{2019}t be selected, besides those disabled in the collection."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Disables all tabs. They leave the tab order; the selected tab and its panel stay."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tab-list-state-return">
                    <p>"A "<Code inline=true>"TabListState"</Code>". It is "<Code inline=true>"Copy"</Code>"."</p>
                    <ApiTable kind=ApiKind::Return of="TabListState">
                        <ApiRow name="list" ty="SingleSelectListState">
                            "The underlying single-selection list: the collection, the "<Code inline=true>"SelectionManager"</Code>
                            " (focused key, selection) and "<Code inline=true>"set_selected_key"</Code>" to select a tab from code."
                        </ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">"Whether all tabs are disabled."</ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"selected_key()"</Code>" returns the selected tab (reactive). It is "
                        <Code inline=true>"None"</Code>" only while the collection is empty."
                    </p>
                </Section>

                <Section title="Example" id="use-tab-list-state-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let state = use_tab_list_state(UseTabListStateInput {
                                default_selected_key: Some(Key::from("shipping")),
                                on_selection_change: Some(Callback::new(|key: Key| log!("selected {key}"))),
                                ..UseTabListStateInput::new(collection)
                            });

                            // Select a tab from code:
                            state.list.set_selected_key(Some(Key::from("details")));
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_tab_list">
                <p>
                    "Turns an element into the tab list. The tab list itself is not a tab stop: the selected tab is (more "
                    "precisely, the focused one, which follows the selection while the tab list doesn\u{2019}t have focus). "
                    "The arrow keys move between the tabs, see "<a href="#keyboard">"Keyboard"</a>"."
                </p>
                <p>
                    "Tabs and panels learn about their tab list through a "<Code inline=true>"TabListData"</Code>
                    ". Create it from the state with "<Code inline=true>"TabListData::new(state)"</Code>" and hand a clone to "
                    <Code inline=true>"use_tab_list"</Code>" and to "<Code inline=true>"use_tab_panel"</Code>". It holds the "
                    <Code inline=true>"state"</Code>" and a generated "<Code inline=true>"id"</Code>", from which "
                    <Code inline=true>"tab_id(&key)"</Code>" and "<Code inline=true>"tab_panel_id(&key)"</Code>
                    " derive the ids of a tab and its panel."
                </p>

                <Section title="Input" id="use-tab-list-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseTabListInput::new(tabs, element)"</Code>
                        " and change the fields you need with struct update syntax."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseTabListInput">
                        <ApiRow name="tabs" ty="TabListData">"The tab list, from "<Code inline=true>"TabListData::new(state)"</Code>". Required."</ApiRow>
                        <ApiRow name="element" ty="CapturedElement">"The tab list element; the hook\u{2019}s props capture it. Required."</ApiRow>
                        <ApiRow name="orientation" ty="Orientation" default="Horizontal">
                            "Which arrow keys move between the tabs, see "<a href="#orientation">"Orientation"</a>
                            "; set as "<Code inline=true>"aria-orientation"</Code>"."
                        </ApiRow>
                        <ApiRow name="keyboard_activation" ty="KeyboardActivation" default="Automatic">
                            "Whether moving focus with the keyboard selects the tab, see "
                            <a href="#keyboard-activation">"Keyboard Activation"</a>"."
                        </ApiRow>
                        <ApiRow name="aria_label" ty="MaybeProp<String>" default="None">"An accessible name for the tab list."</ApiRow>
                        <ApiRow name="aria_labelledby" ty="Option<String>" default="None">"The id of a visible label."</ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tab-list-return">
                    <ApiTable kind=ApiKind::Return of="UseTabListReturn">
                        <ApiRow name="props" ty="UseTabListProps">
                            "The id, "<Code inline=true>"role=\"tablist\""</Code>", "<Code inline=true>"aria-orientation"</Code>
                            ", the labels, and the keyboard and focus handling. Spread "<Code inline=true>"{..props.into_attrs()}"</Code>"."
                        </ApiRow>
                        <ApiRow name="data" ty="TabListItemData">
                            "What the tabs need to know about the tab list. Pass a clone to each "<Code inline=true>"use_tab"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-tab-list-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let tabs = TabListData::new(state);
                            let UseTabListReturn { props, data } = use_tab_list(UseTabListInput {
                                aria_label: "Product".into(),
                                ..UseTabListInput::new(tabs.clone(), CapturedElement::new())
                            });

                            view! {
                                <div {..props.into_attrs()}>
                                    // One tab per collection item, in collection order.
                                </div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="use_tab">
                <p>
                    "Renders one tab. It needs the tab list data and the tab\u{2019}s key; whether it is selected, focused or "
                    "disabled comes from the state. A press selects the tab, and focus follows."
                </p>

                <Section title="Input" id="use-tab-input">
                    <p>
                        "Create the input with "<Code inline=true>"UseTabInput::new(data, key)"</Code>"."
                    </p>
                    <ApiTable kind=ApiKind::Input of="UseTabInput">
                        <ApiRow name="list" ty="TabListItemData">"The tab list, from "<Code inline=true>"use_tab_list"</Code>". Required."</ApiRow>
                        <ApiRow name="key" ty="Key">"The tab\u{2019}s key in the collection. Required."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>" default="false">
                            "Disables this tab. The arrow keys only skip tabs disabled in the collection or through "
                            <Code inline=true>"disabled_keys"</Code>", so prefer those."
                        </ApiRow>
                        <ApiRow name="should_select_on_press_up" ty="Option<bool>" default="None">
                            "Select when the press ends instead of when it starts. "<Code inline=true>"None"</Code>
                            ": only for tabs that are links in the collection."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tab-return">
                    <ApiTable kind=ApiKind::Return of="UseTabReturn">
                        <ApiRow name="tab_props" ty="PropsWithStyles<UseTabProps>">
                            "The id, "<Code inline=true>"role=\"tab\""</Code>", "<Code inline=true>"aria-selected"</Code>", "
                            <Code inline=true>"aria-disabled"</Code>", "<Code inline=true>"aria-controls"</Code>" (on the selected "
                            "tab, pointing to its panel), "<Code inline=true>"tabindex"</Code>" ("<Code inline=true>"0"</Code>
                            " on the focused tab, "<Code inline=true>"-1"</Code>" on the others, none on disabled tabs), press and "
                            "focus handling. Call "<Code inline=true>"tab_props.into_parts()"</Code>" to get "
                            <Code inline=true>"(attrs, styles)"</Code>"."
                        </ApiRow>
                        <ApiRow name="is_selected" ty="Signal<bool>">"Whether the tab is selected."</ApiRow>
                        <ApiRow name="is_disabled" ty="Signal<bool>">
                            "Whether the tab is disabled: by its input, the collection, "<Code inline=true>"disabled_keys"</Code>
                            " or the whole tab list."
                        </ApiRow>
                        <ApiRow name="is_pressed" ty="Signal<bool>">"Whether the tab is being pressed."</ApiRow>
                        <ApiRow name="is_focused" ty="Signal<bool>">
                            "Whether the tab has focus, from the keyboard or a click. For a focus ring, use "
                            <Code inline=true>":focus-visible"</Code>" in CSS."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-tab-example">
                    <Code language=Language::Rust>
                        {indoc!(r"
                            #[component]
                            fn MyTab(list: TabListItemData, key: &'static str, label: &'static str) -> impl IntoView {
                                let UseTabReturn { tab_props, is_selected, .. } = use_tab(UseTabInput::new(list, Key::from(key)));
                                let (attrs, styles) = tab_props.into_parts();
                                view! {
                                    <div {..attrs} style=styles class:selected=is_selected>{label}</div>
                                }
                            }
                        ")}
                    </Code>
                </Section>
            </Section>

            <Section title="use_tab_panel">
                <p>
                    "Renders the content of a tab, labelled by the tab. The panel is a tab stop, so keyboard users reach "
                    "its content right after the tab list \u{2014} unless it contains tabbable elements, which then take "
                    "focus instead. Render either one panel for whatever tab is selected ("<Code inline=true>"key: None"</Code>
                    ", as in the demo above) or one panel per tab ("<Code inline=true>"key: Some(..)"</Code>
                    "), rendered only while its tab is selected (as in the "<a href="#orientation">"vertical demo"</a>")."
                </p>

                <Section title="Input" id="use-tab-panel-input">
                    <ApiTable kind=ApiKind::Input of="UseTabPanelInput">
                        <ApiRow name="tabs" ty="TabListData">"The tab list. Required."</ApiRow>
                        <ApiRow name="key" ty="Option<Key>">
                            "The panel\u{2019}s tab. "<Code inline=true>"None"</Code>": the selected tab; id and label follow the "
                            "selection. Required."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Return" id="use-tab-panel-return">
                    <ApiTable kind=ApiKind::Return of="UseTabPanelReturn">
                        <ApiRow name="tab_panel_props" ty="UseTabPanelProps">
                            "The id the tab\u{2019}s "<Code inline=true>"aria-controls"</Code>" points to, "
                            <Code inline=true>"role=\"tabpanel\""</Code>", "<Code inline=true>"aria-labelledby"</Code>
                            " (the tab), and "<Code inline=true>"tabindex=\"0\""</Code>" while the panel has no tabbable "
                            "content. Spread "<Code inline=true>"{..tab_panel_props.into_attrs()}"</Code>"."
                        </ApiRow>
                    </ApiTable>
                </Section>

                <Section title="Example" id="use-tab-panel-example">
                    <Code language=Language::Rust>
                        {indoc!(r#"
                            let panel = use_tab_panel(UseTabPanelInput { tabs, key: None });
                            let text = move || match state.selected_key().map(|key| key.to_string()).as_deref() {
                                Some("shipping") => "Ships within two days.",
                                _ => "A foldable reading lamp.",
                            };

                            view! {
                                <div {..panel.tab_panel_props.into_attrs()}>{text}</div>
                            }
                        "#)}
                    </Code>
                </Section>
            </Section>

            <Section title="Keyboard Activation">
                <p>
                    "With "<Code inline=true>"KeyboardActivation::Automatic"</Code>" (the default), moving focus to a tab "
                    "with the arrow keys selects it. Use "<Code inline=true>"KeyboardActivation::Manual"</Code>" when "
                    "showing a panel is expensive, e.g. because it loads data: the arrow keys only move focus, and "
                    <Keys keys="Enter"/>" or "<Keys keys="Space"/>" selects the focused tab. Clicks select in both modes."
                </p>
                <Demo
                    description="Tabs with manual activation, showing the focused and the selected tab"
                    source=include_str!("demos/tabs_manual.rs")
                >
                    <TabsManualDemo/>
                </Demo>
            </Section>

            <Section title="Orientation">
                <p>
                    "Vertical tab lists ("<Code inline=true>"Orientation::Vertical"</Code>") add "<Keys keys="ArrowUp"/>" and "
                    <Keys keys="ArrowDown"/>" to the keys moving between the tabs; "<Keys keys="ArrowLeft"/>" and "<Keys keys="ArrowRight"/>
                    " keep working. Lay the tab list out as a column yourself. This demo also renders one panel per tab."
                </p>
                <Demo
                    description="Vertical tabs with one panel per tab"
                    source=include_str!("demos/tabs_vertical.rs")
                >
                    <TabsVerticalDemo/>
                </Demo>
            </Section>

            <Section title="Keyboard">
                <p>
                    "The tab list is a single tab stop: "<Keys keys="Tab"/>" moves focus to the selected tab (with manual activation: "
                    "the tab focused last), and on to the panel. "
                    "Disabled tabs are skipped, and navigation wraps around at the ends."
                </p>
                <KeyboardTable>
                    <KeyRow keys="ArrowRight / ArrowLeft">
                        "Move focus to the next or previous tab. In right-to-left locales, "<Keys keys="ArrowLeft"/>
                        " moves to the next tab. Works in both orientations."
                    </KeyRow>
                    <KeyRow keys="ArrowDown / ArrowUp">"Move focus to the next or previous tab (vertical tab lists only)."</KeyRow>
                    <KeyRow keys="Home / End">"Move focus to the first or last enabled tab."</KeyRow>
                    <KeyRow keys="Enter / Space">"Select the focused tab (manual activation)."</KeyRow>
                    <KeyRow keys="Tab">"Move focus from the tab list into the panel, or to the panel\u{2019}s first tabbable element."</KeyRow>
                </KeyboardTable>
                <p>"With automatic activation, every move also selects the tab."</p>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::Tabs.materialize()>"Tabs overview"</Link></li>
                <li><Link href=routes::doc::tabs::Atom.materialize()>"Tabs atoms"</Link></li>
                <li><Link href=routes::doc::Collections.materialize()>"Collections"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
