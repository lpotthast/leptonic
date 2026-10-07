use indoc::indoc;
use leptos::prelude::*;

use super::demos::{virtual_list::VirtualListDemo, virtualizer::VirtualizerDemo};
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageAtomVirtualizer() -> impl IntoView {
    view! {
        <DocPage title="Virtualizer">
            <p>
                "The "<Code inline=true>"Virtualizer"</Code>" atom renders only the options of a "
                <Link href=routes::doc::listbox::Atom.materialize()>"ListBox"</Link>" that are in view, and the "
                <Code inline=true>"VirtualList"</Code>" atom only the visible rows of a plain list, such as a log. See the "
                <Link href=routes::doc::CollectionState.materialize()>"Collection State overview"</Link>
                " for the other building blocks of collections."
            </p>
            <ReactAria hook="Virtualizer"/>
            <p>
                "Virtualize a list when it holds thousands of items. Every rendered item costs DOM nodes, memory and "
                "rendering time, so a list of 10,000 options is slow to mount, update and scroll. A virtualized list "
                "renders the few dozen items in view, positions them where they would be in the full list, and swaps "
                "them as you scroll. A list of a few hundred items doesn\u{2019}t need it."
            </p>

            <Section title="Hooks Used">
                <DocTable headers=&["Atom", "Hooks"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Virtualizer"</Code>", "<Code inline=true>"VirtualList"</Code></TableCell>
                        <TableCell>
                            <Link href=hook_section("use-virtualizer-state")>"use_virtualizer_state"</Link>", "
                            <Link href=hook_section("use-scroll-view")>"use_scroll_view"</Link>", "
                            <Link href=hook_section("use-virtualizer-item")>"use_virtualizer_item"</Link>
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell>"The virtualized "<Code inline=true>"ListBox"</Code></TableCell>
                        <TableCell>
                            <Link href=format!("{}#use-listbox-input", routes::doc::listbox::Hook.materialize())>"use_listbox"</Link>
                            " with "<Code inline=true>"layout_delegate"</Code>" and "<Code inline=true>"is_virtualized"</Code>
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="Example">
                <p>
                    "Wrap a "<Code inline=true>"ListBox"</Code>" in a "<Code inline=true>"Virtualizer"</Code>" and render its "
                    "options with "<Code inline=true>"ListBoxItems"</Code>". The layout decides where each option goes: a "
                    <Code inline=true>"ListLayout"</Code>" stacks them in rows. The listbox becomes the scrolling element, "
                    "so give it a height:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{
                            atoms::{
                                listbox::{ListBox, ListBoxItems},
                                virtualizer::Virtualizer,
                            },
                            hooks::{
                                collections::{Key, use_list_collection},
                                virtualizer::{ListLayout, ListLayoutOptions},
                            },
                        };
                        use leptos::prelude::*;

                        let orders = use_list_collection(
                            Signal::stored((1..=10_000).collect::<Vec<usize>>()),
                            |number| Key::from(*number),
                            |number| format!("Order {number}"),
                        );

                        view! {
                            <Virtualizer layout=ListLayout::new(ListLayoutOptions {
                                row_size: Some(36.0),
                                ..ListLayoutOptions::default()
                            })>
                                // `.orders { height: 20em; }` in your stylesheet.
                                <ListBox collection=orders aria_label="Orders" classes="orders">
                                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                                </ListBox>
                            </Virtualizer>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "10,000 orders in a listbox with single selection. Scroll and watch the status: only the options in "
                    "view, and a few more in the direction you scroll, are rendered. Tab into the list and move with the "
                    "arrow keys. Each option counts itself while it is rendered and reports when it has focus; the "
                    "focused option stays rendered when you scroll it out of view."
                </p>
                <Demo
                    description="Listbox of 10,000 orders that renders only the options in view"
                    source=include_str!("demos/virtualizer.rs")
                >
                    <VirtualizerDemo/>
                </Demo>
            </Section>

            <Section title="Virtualizer">
                <p>
                    "Renders its children and makes the "<Code inline=true>"ListBox"</Code>" inside virtualized: the listbox "
                    "scrolls, and its "<Code inline=true>"ListBoxItems"</Code>" render only the options whose rows "
                    "intersect the visible area (extended by a third of the view in the direction you scroll). Each option "
                    "sits in a "<Code inline=true>"<div role=\"presentation\">"</Code>" wrapper, absolutely positioned "
                    "inside a content box as large as the whole list. The atom renders no element of its own."
                </p>
                <p>
                    "Rows of a fixed size ("<Code inline=true>"row_size"</Code>") are fastest: the layout knows where every "
                    "option is without rendering it. With "<Code inline=true>"estimated_row_size"</Code>" instead, rows "
                    "start at the estimate and are measured once rendered; the scrollbar adjusts as more rows are measured."
                </p>
                <Section title="Props" id="virtualizer-props">
                    <ApiTable kind=ApiKind::Props of="atoms::virtualizer::Virtualizer">
                        <ApiRow name="layout" ty="L: Layout + Clone">
                            "Positions the options, e.g. a "<AnchorLink href="#listlayout">"ListLayout"</AnchorLink>
                            ". Required."
                        </ApiRow>
                        <ApiRow name="layout_options" ty="Option<Signal<L::Options>>" default="None">
                            "Replaces the options the layout was created with, e.g. to change the row size at runtime."
                        </ApiRow>
                        <ApiRow name="should_observe_item_size" ty="bool" default="false">
                            "Measure an option again whenever its content resizes, for options whose size changes after "
                            "they were first rendered (e.g. images loading). Options of estimated size are measured once "
                            "rendered either way."
                        </ApiRow>
                        <ApiRow name="children" ty="Children">
                            "A "<Code inline=true>"ListBox"</Code>" that renders its options with "
                            <Code inline=true>"ListBoxItems"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="ListLayout">
                <p>
                    "Stacks the items in rows (or, with a horizontal orientation, in columns) of a fixed or measured size. "
                    "Create it with "<Code inline=true>"ListLayout::new(ListLayoutOptions { .. })"</Code>". It lays out "
                    "lazily: rows far from the visible area are only estimated until you scroll near them or jump there "
                    "with "<Keys keys="Home"/>" or "<Keys keys="End"/>"."
                </p>
                <p>
                    "Not to be confused with "<Code inline=true>"collections::ListLayout"</Code>" ("
                    <Code inline=true>"Stack"</Code>" or "<Code inline=true>"Grid"</Code>"), the "
                    <Code inline=true>"layout"</Code>" prop of "<Code inline=true>"ListBox"</Code>" that decides how the "
                    "arrow keys move."
                </p>
                <Section title="ListLayoutOptions">
                    <ApiTable kind=ApiKind::Fields of="ListLayoutOptions">
                        <ApiRow name="orientation" ty="Orientation" default="Vertical">
                            "The direction the items stack in."
                        </ApiRow>
                        <ApiRow name="row_size" ty="Option<f64>" default="None">
                            "The fixed size of every row along the orientation, in pixels. "<Code inline=true>"None"</Code>
                            ": rows start at "<Code inline=true>"estimated_row_size"</Code>" and are measured once rendered."
                        </ApiRow>
                        <ApiRow name="estimated_row_size" ty="Option<f64>" default="None">
                            "The size of a row that wasn\u{2019}t measured yet. "<Code inline=true>"None"</Code>": 48."
                        </ApiRow>
                        <ApiRow name="heading_size, estimated_heading_size" ty="Option<f64>, Option<f64>" default="None">
                            "The fixed or estimated size of section headings. Virtualized listboxes don\u{2019}t render "
                            "sections yet."
                        </ApiRow>
                        <ApiRow name="loader_size" ty="Option<f64>" default="None">
                            "The size of a loader node (\u{201c}load more\u{201d}). "<Code inline=true>"None"</Code>
                            ": the row size, else 48."
                        </ApiRow>
                        <ApiRow name="drop_indicator_thickness" ty="f64" default="2.0">
                            "The thickness of drop indicators, for drag and drop."
                        </ApiRow>
                        <ApiRow name="gap" ty="f64" default="0.0">"The space between rows."</ApiRow>
                        <ApiRow name="padding" ty="f64" default="0.0">
                            "The space around the list. Use it instead of CSS padding on the scrolling element, which "
                            "the virtualizer sets to 0."
                        </ApiRow>
                        <ApiRow name="anchor_to" ty="Option<ScrollAnchorEdge>" default="None">
                            <Code inline=true>"Some(ScrollAnchorEdge::End)"</Code>": while the view is at the end, it stays "
                            "there as items are added or measured, e.g. in a chat or a log. Vertical lists only; "
                            <Code inline=true>"Start"</Code>" has no effect."
                        </ApiRow>
                        <ApiRow name="scroll_end_threshold" ty="f64" default="0.0">
                            "Within this distance from the end (in pixels), the view counts as being at the end."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="VirtualList">
                <p>
                    "A virtualized list of plain rows, e.g. a log or a feed: only the visible rows are rendered, "
                    "positioned by a "<AnchorLink href="#listlayout">"ListLayout"</AnchorLink>". Unlike a virtualized "
                    "listbox, it has no roles and no focus or selection handling: a row is whatever its children render "
                    "for an item, and its text stays selectable. Rows holding the text selection stay rendered while "
                    "scrolled away, so copying works across rows."
                </p>
                <p>
                    "The list renders a "<Code inline=true>"<div>"</Code>" that scrolls: give it a height. "
                    <Code inline=true>"is_focusable"</Code>" puts it in the tab order, so that keyboard users can scroll "
                    "it; a focusable list needs a role and a name, which you pass as attributes ("
                    <Code inline=true>"attr:role"</Code>", "<Code inline=true>"attr:aria-label"</Code>"). Each item needs a "
                    "unique, stable key; a row is rendered once per key, so give a changed item a new key."
                </p>
                <p>
                    <Code inline=true>"is_anchored_to_end"</Code>" makes the list follow its end as items are added. "
                    "Scrolling away from the end turns following off, scrolling back to the end (within "
                    <Code inline=true>"scroll_end_threshold"</Code>") turns it on again, and turning it on scrolls to the end."
                </p>
                <Demo
                    description="Build log of 1,000 lines of varying height that follows its end as lines are added"
                    source=include_str!("demos/virtual_list.rs")
                >
                    <VirtualListDemo/>
                </Demo>
                <Section title="Props" id="virtual-list-props">
                    <ApiTable kind=ApiKind::Props of="VirtualList">
                        <ApiRow name="items" ty="Signal<Vec<T>>">"The items, in order. Required."</ApiRow>
                        <ApiRow name="key" ty="Fn(&T) -> Key">"The unique, stable key of an item. Required."</ApiRow>
                        <ApiRow name="layout_options" ty="Signal<ListLayoutOptions>" default="ListLayoutOptions::default()">
                            "Row sizes, gap and padding of the list. Its "<Code inline=true>"anchor_to"</Code>" is replaced by "
                            <Code inline=true>"is_anchored_to_end"</Code>"."
                        </ApiRow>
                        <ApiRow name="should_observe_item_size" ty="bool" default="false">
                            "Measure a row again whenever its content resizes. Rows of estimated size are measured once "
                            "rendered either way."
                        </ApiRow>
                        <ApiRow name="default_anchored_to_end" ty="bool" default="false">
                            "Whether the list starts at its end and follows it, when "<Code inline=true>"is_anchored_to_end"</Code>
                            " isn\u{2019}t given."
                        </ApiRow>
                        <ApiRow name="is_anchored_to_end" ty="Option<Signal<bool>>" default="None">
                            "Whether the list follows its end (controlled): a value or any signal."
                        </ApiRow>
                        <ApiRow name="set_anchored_to_end" ty="Option<Out<bool>>" default="None">
                            "Receives the new state when scrolling turns following on or off: an "<Code inline=true>"RwSignal"</Code>", "
                            <Code inline=true>"WriteSignal"</Code>", closure, "<Code inline=true>"Callback"</Code>", \u{2026}"
                        </ApiRow>
                        <ApiRow name="on_anchored_to_end_change" ty="Option<Callback<bool>>" default="None">
                            "Called when following turns on or off."
                        </ApiRow>
                        <ApiRow name="is_focusable" ty="bool" default="false">
                            "Puts the list in the tab order ("<Code inline=true>"tabindex=\"0\""</Code>"), to scroll it with "
                            "the keyboard. It then has "<Code inline=true>"data-focused"</Code>" and "
                            <Code inline=true>"data-focus-visible"</Code>"."
                        </ApiRow>
                        <ApiRow name="classes, styles" ty="Classes, Styles" default="empty">
                            "Classes and styles of the list "<Code inline=true>"<div>"</Code>". Default class: "
                            <Code inline=true>"leptonic-VirtualList"</Code>"."
                        </ApiRow>
                        <ApiRow name="children" ty="Fn(T) -> impl IntoView">
                            "Renders an item\u{2019}s row ("<Code inline=true>"let:item"</Code>"). Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="Styling">
                <p>
                    "Style the listbox and its options as without the virtualizer (see "
                    <Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link>"), with three differences. "
                    "The scrolling element needs a height. Its padding is set to 0, as it would shift the positioned rows: "
                    "use the layout\u{2019}s "<Code inline=true>"padding"</Code>" and "<Code inline=true>"gap"</Code>
                    ". And each option\u{2019}s wrapper is as large as its row and clips what overflows it, so let the "
                    "option fill it and draw focus rings inside:"
                </p>
                <Code language=Language::Css>
                    {indoc!(r"
                        .orders { height: 20em; border: 1px solid var(--border); }
                        .orders [role=option] { box-sizing: border-box; height: 100%; padding: 0 1em; }
                        .orders [role=option][data-focus-visible] { outline: 2px solid var(--focus); outline-offset: -2px; }
                    ")}
                </Code>
            </Section>

            <Section title="Accessibility">
                <p>
                    "A virtualized listbox behaves like any other for keyboard and screen reader users:"
                </p>
                <ul>
                    <li>
                        "Each rendered option tells its position and the number of options ("
                        <Code inline=true>"aria-posinset"</Code>", "<Code inline=true>"aria-setsize"</Code>"), as most "
                        "options aren\u{2019}t in the DOM for assistive technology to count."
                    </li>
                    <li>
                        "The focused option stays rendered when you scroll it out of view, so focus is never lost and "
                        <Keys keys="Tab"/>" returns to it."
                    </li>
                    <li>
                        "The arrow keys, "<Keys keys="PageUp"/>", "<Keys keys="PageDown"/>", "<Keys keys="Home"/>", "
                        <Keys keys="End"/>" and type-ahead reach options that aren\u{2019}t rendered: the layout knows where "
                        "every option is, and the option is rendered as it gets focus and scrolled into view. Page keys move "
                        "by the height of the view."
                    </li>
                    <li>
                        "Selecting all ("<Keys keys="Control + A"/>", "<Keys keys="Meta + A"/>" on macOS) selects every "
                        "option, rendered or not."
                    </li>
                </ul>
            </Section>

            <Section title="Limitations">
                <ul>
                    <li>
                        "Only a "<Code inline=true>"ListBox"</Code>" rendering its options through "
                        <Code inline=true>"ListBoxItems"</Code>" is virtualized. Options written out one by one with "
                        <Code inline=true>"ListBoxItem"</Code>" are all rendered, and sections aren\u{2019}t rendered."
                    </li>
                    <li>
                        "Grid lists, tables, trees, menus and the listboxes in select and combobox popovers can\u{2019}t be "
                        "virtualized yet."
                    </li>
                    <li>
                        "Nothing is rendered during server-side rendering, as the size of the view is unknown there: the "
                        "list fills once the page is hydrated."
                    </li>
                    <li>
                        "The browser\u{2019}s find in page only finds the text of rendered items."
                    </li>
                </ul>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::collection_state::UseVirtualizerState.materialize()>"use_virtualizer_state"</Link></li>
                <li><Link href=routes::doc::Listbox.materialize()>"Listbox overview"</Link></li>
                <li><Link href=routes::doc::listbox::Atom.materialize()>"Listbox Atoms"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}

/// A link target on the virtualizer hooks page.
fn hook_section(id: &str) -> String {
    format!("{}#{id}", routes::doc::collection_state::UseVirtualizerState.materialize())
}
