use indoc::indoc;
use leptos::prelude::*;

use super::demos::virtualizer::VirtualizerHooksDemo;
use crate::{kit::*, routes};

#[component]
#[allow(clippy::too_many_lines)]
pub fn PageUseVirtualizerState() -> impl IntoView {
    view! {
        <DocPage title="use_virtualizer_state">
            <p>
                "The virtualizer hooks render only the visible items of a collection in markup of your own: "
                <Code inline=true>"use_virtualizer_state"</Code>" lays the collection out and tracks which items are "
                "visible, "<Code inline=true>"use_scroll_view"</Code>" reports what the scrolling element shows, and "
                <Code inline=true>"use_virtualizer_item"</Code>" positions and measures each rendered item. To virtualize a "
                <Code inline=true>"ListBox"</Code>", use the "
                <Link href=routes::doc::collection_state::Virtualizer.materialize()>"Virtualizer"</Link>
                " atom instead, which explains when virtualizing pays off. See the "
                <Link href=routes::doc::CollectionState.materialize()>"Collection State overview"</Link>
                " for the other building blocks of collections."
            </p>

            <Section title="Example">
                <p>
                    "The three hooks connect through the state: the scroll view tells it the visible area and its size, "
                    "the state answers with the layout infos of the visible items, and each rendered item reports its "
                    "measured size back:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        let state = use_virtualizer_state(UseVirtualizerStateInput {
                            layout: ListLayout::new(ListLayoutOptions { row_size: Some(32.0), ..ListLayoutOptions::default() }),
                            collection: rows.into(),
                            persisted_keys: Signal::stored(HashSet::new()),
                            layout_options: Signal::stored(None),
                            on_visible_rect_change: Callback::new(|_| {}),
                        });
                        let element = CapturedElement::new();
                        let scroll_view = use_scroll_view(
                            UseScrollViewInput {
                                content_size: state.content_size.into(),
                                on_visible_rect_change: Callback::new(move |rect| state.set_visible_rect(rect)),
                                on_size_change: Some(Callback::new(move |size| state.set_size(size))),
                                on_scroll_start: Some(Callback::new(move |()| state.start_scrolling())),
                                on_scroll_end: Some(Callback::new(move |()| state.end_scrolling())),
                                scroll_direction: ScrollDirection::Vertical,
                                allows_window_scrolling: false,
                            },
                            element,
                        );

                        view! {
                            <div {..element.attr()} style=scroll_view.scroll_view_styles>
                                <div style=scroll_view.content_styles>
                                    <For each=move || state.visible.get() key=|info| info.key.clone() let:info>
                                        // A Leptos component of yours calling `use_virtualizer_item`.
                                        <VirtualRow info=info state=state/>
                                    </For>
                                </div>
                            </div>
                        }
                    ")}
                </Code>
            </Section>

            <Section title="Demo">
                <p>
                    "100,000 rows in a scrolling "<Code inline=true>"<div>"</Code>", built from the three hooks. Scroll "
                    "it with the mouse or, once focused, with the arrow and page keys: only the rows in view, and a few "
                    "more in the direction you scroll, are rendered."
                </p>
                <Demo
                    description="List of 100,000 rows virtualized with the virtualizer hooks"
                    source=include_str!("demos/virtualizer.rs")
                >
                    <VirtualizerHooksDemo/>
                </Demo>
            </Section>

            <Section title="Layout">
                <p>
                    "A layout computes where the items of a collection go: a "<Code inline=true>"LayoutInfo"</Code>
                    " per item, and the size of the whole content. "
                    <Link href=format!("{}#listlayout", routes::doc::collection_state::Virtualizer.materialize())>"ListLayout"</Link>
                    " stacks items in rows. For another arrangement, implement the "<Code inline=true>"Layout"</Code>
                    " trait and pass your layout to "<Code inline=true>"use_virtualizer_state"</Code>" or to the "
                    <Code inline=true>"Virtualizer"</Code>" atom. Each method receives a "
                    <Code inline=true>"VirtualizerContext"</Code>" with the collection, the persisted keys, the scroll "
                    "view\u{2019}s size, the visible rectangle and the previous content size."
                </p>
                <DocTable headers=&["Item", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"type Options"</Code></TableCell>
                        <TableCell>
                            "The layout\u{2019}s options, which "<Code inline=true>"layout_options"</Code>" can replace at runtime."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"update(ctx, invalidation)"</Code></TableCell>
                        <TableCell>
                            "Lays out the collection. Called before the visible items are read, whenever the collection, the "
                            "scroll view\u{2019}s size, a measured item size or the options changed; the "
                            <Code inline=true>"InvalidationContext"</Code>" says which."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"visible_layout_infos(ctx, rect)"</Code></TableCell>
                        <TableCell>"The layout infos inside "<Code inline=true>"rect"</Code>", plus those of the persisted keys."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"layout_info(ctx, key)"</Code></TableCell>
                        <TableCell>
                            "The layout info of one item, laying out more of the collection if needed (keyboard navigation "
                            "asks for items that aren\u{2019}t visible)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"content_size()"</Code></TableCell>
                        <TableCell>"The size of the whole content."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"update_item_size(ctx, key, size)"</Code></TableCell>
                        <TableCell>
                            "Takes the measured size of a rendered item and returns whether it changed the layout. Default: "
                            "ignores it."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"should_invalidate(new_rect, old_rect)"</Code></TableCell>
                        <TableCell>
                            "Whether a change of the visible rectangle needs a new layout. Default: when its size changes."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"should_invalidate_layout_options(new, old)"</Code></TableCell>
                        <TableCell>"Whether new options need a new layout. Default: when they differ."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"scroll_anchor_info(options)"</Code></TableCell>
                        <TableCell>
                            "The edge the view stays anchored to while the content changes. Default: none."
                        </TableCell>
                    </TableRow>
                </DocTable>
                <p>
                    "A layout of fixed-size tiles that wrap into rows as wide as the scroll view:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        use leptonic::hooks::{
                            collections::{Key, NodeKind, Rect, Size},
                            virtualizer::{InvalidationContext, Layout, LayoutInfo, VirtualizerContext},
                        };

                        #[derive(Clone)]
                        struct TileLayout {
                            tile: Size,
                            infos: Vec<LayoutInfo>,
                            content_size: Size,
                        }

                        impl Layout for TileLayout {
                            type Options = ();

                            fn update(&mut self, ctx: &VirtualizerContext<'_>, _: &InvalidationContext<()>) {
                                let columns = ((ctx.size.width / self.tile.width).floor() as usize).max(1);
                                self.infos = ctx
                                    .collection
                                    .items()
                                    .enumerate()
                                    .map(|(index, node)| {
                                        let x = (index % columns) as f64 * self.tile.width;
                                        let y = (index / columns) as f64 * self.tile.height;
                                        let rect = Rect::new(x, y, self.tile.width, self.tile.height);
                                        LayoutInfo::new(NodeKind::Item, node.key.clone(), rect)
                                    })
                                    .collect();
                                let rows = self.infos.len().div_ceil(columns);
                                self.content_size = Size::new(ctx.size.width, rows as f64 * self.tile.height);
                            }

                            fn visible_layout_infos(&mut self, ctx: &VirtualizerContext<'_>, rect: Rect) -> Vec<LayoutInfo> {
                                self.infos
                                    .iter()
                                    .filter(|info| info.rect.intersects(&rect) || ctx.persisted_keys.contains(&info.key))
                                    .cloned()
                                    .collect()
                            }

                            fn layout_info(&mut self, _: &VirtualizerContext<'_>, key: &Key) -> Option<LayoutInfo> {
                                self.infos.iter().find(|info| &info.key == key).cloned()
                            }

                            fn content_size(&self) -> Size {
                                self.content_size
                            }
                        }
                    ")}
                </Code>
                <Section title="LayoutInfo">
                    <p>
                        "Where an element goes and how it is shown. "<Code inline=true>"LayoutInfo::new(kind, key, rect)"</Code>
                        " creates one with the defaults below."
                    </p>
                    <ApiTable kind=ApiKind::Fields of="LayoutInfo">
                        <ApiRow name="kind" ty="NodeKind">"What the element represents: the collection node\u{2019}s kind."</ApiRow>
                        <ApiRow name="key" ty="Key">"The collection node\u{2019}s key."</ApiRow>
                        <ApiRow name="parent_key" ty="Option<Key>" default="None">
                            "The key of the parent layout info, e.g. the section of an item."
                        </ApiRow>
                        <ApiRow name="rect" ty="Rect">"The element\u{2019}s position and size in the content."</ApiRow>
                        <ApiRow name="estimated_size" ty="bool" default="false">
                            "The size is an estimate: the element is measured once rendered."
                        </ApiRow>
                        <ApiRow name="is_sticky" ty="bool" default="false">"The element sticks to the view while scrolling."</ApiRow>
                        <ApiRow name="opacity" ty="f64" default="1.0">"The element\u{2019}s opacity."</ApiRow>
                        <ApiRow name="transform" ty="Option<Arc<str>>" default="None">"A CSS transform for the element."</ApiRow>
                        <ApiRow name="z_index" ty="i32" default="0">"The element\u{2019}s z-index."</ApiRow>
                        <ApiRow name="allow_overflow" ty="bool" default="false">"The element\u{2019}s content may overflow it."</ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_virtualizer_state">
                <p>
                    "Lays out a collection with a layout and tracks which items are visible. It lays out again when the "
                    "collection, the visible area, a measured size or the options change, and publishes the layout infos "
                    "to render as a signal."
                </p>
                <ReactAriaSource path="virtualizer/useVirtualizerState.ts" package=UpstreamPackage::ReactStately/>
                <Section title="Input" id="use-virtualizer-state-input">
                    <ApiTable kind=ApiKind::Input of="UseVirtualizerStateInput">
                        <ApiRow name="layout" ty="L: Layout">
                            "Positions the items, e.g. a "<Code inline=true>"ListLayout"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="collection" ty="Signal<Arc<Collection>>">
                            "The items, e.g. from "<Code inline=true>"use_list_collection"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="persisted_keys" ty="Signal<HashSet<Key>>">
                            "Items that stay rendered while out of view, e.g. the focused item. Required (an empty set)."
                        </ApiRow>
                        <ApiRow name="layout_options" ty="Signal<Option<L::Options>>">
                            "Options replacing the ones the layout was created with. Required ("
                            <Code inline=true>"None"</Code>" keeps the layout\u{2019}s)."
                        </ApiRow>
                        <ApiRow name="on_visible_rect_change" ty="Callback<Rect>">
                            "Called when the virtualizer moves the view, e.g. to keep an anchored layout at its end: scroll "
                            "the element there with "<Code inline=true>"use_scroll_view"</Code>"\u{2019}s "
                            <Code inline=true>"scroll_to"</Code>". Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-virtualizer-state-return">
                    <p>
                        "A "<Code inline=true>"VirtualizerState"</Code>" (it is "<Code inline=true>"Copy"</Code>"):"
                    </p>
                    <ApiTable kind=ApiKind::Return of="VirtualizerState">
                        <ApiRow name="visible" ty="RwSignal<Vec<LayoutInfo>>">
                            "The layout infos to render, parents before children. Empty during server-side rendering."
                        </ApiRow>
                        <ApiRow name="content_size" ty="RwSignal<Size>">"The size of the whole content."</ApiRow>
                    </ApiTable>
                    <DocTable headers=&["Method", "Description"]>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"set_visible_rect(rect)"</Code>", "<Code inline=true>"set_size(size)"</Code>
                            </TableCell>
                            <TableCell>"The scroll view moved or resized: from "<Code inline=true>"use_scroll_view"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"start_scrolling()"</Code>", "<Code inline=true>"end_scrolling()"</Code>
                            </TableCell>
                            <TableCell>
                                "The user started or stopped scrolling. While scrolling, rendered items keep their DOM order "
                                "and new ones go last."
                            </TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell>
                                <Code inline=true>"is_scrolling()"</Code>", "<Code inline=true>"visible_rect()"</Code>
                            </TableCell>
                            <TableCell>"Whether the user is scrolling, and the visible area of the content, as signals."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"update_item_size(key, size)"</Code></TableCell>
                            <TableCell>"The measured size of a rendered item: from "<Code inline=true>"use_virtualizer_item"</Code>"."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"layout_info(key)"</Code></TableCell>
                            <TableCell>"Where an item is, rendered or not."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"key_at_point(point)"</Code></TableCell>
                            <TableCell>"The item at a point of the content."</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell><Code inline=true>"layout_delegate()"</Code></TableCell>
                            <TableCell>
                                "The layout as a "<Code inline=true>"LayoutDelegate"</Code>", for the "
                                <Code inline=true>"layout_delegate"</Code>" of collection hooks: keyboard navigation then "
                                "reaches items that aren\u{2019}t rendered."
                            </TableCell>
                        </TableRow>
                    </DocTable>
                </Section>
            </Section>

            <Section title="use_scroll_view">
                <p>
                    "Makes the captured element the scroll view of a virtualized collection: it reports the visible area "
                    "as the element scrolls and resizes, and returns the styles of the element and of the content box "
                    "inside it, which is as large as the whole content."
                </p>
                <ReactAriaSource path="virtualizer/ScrollView.tsx"/>
                <Section title="Input" id="use-scroll-view-input">
                    <ApiTable kind=ApiKind::Input of="UseScrollViewInput">
                        <ApiRow name="content_size" ty="Signal<Size>">
                            "The size of the content: the state\u{2019}s "<Code inline=true>"content_size"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="on_visible_rect_change" ty="Callback<Rect>">
                            "Called with the visible area of the content. Required."
                        </ApiRow>
                        <ApiRow name="on_size_change" ty="Option<Callback<Size>>">
                            "Called with the element\u{2019}s new size. Required ("<Code inline=true>"None"</Code>" to ignore it)."
                        </ApiRow>
                        <ApiRow name="on_scroll_start, on_scroll_end" ty="Option<Callback<()>>">
                            "Called when the user starts and stops scrolling. Required ("<Code inline=true>"None"</Code>
                            " to ignore them)."
                        </ApiRow>
                        <ApiRow name="scroll_direction" ty="ScrollDirection" default="Both">
                            "The axes the element scrolls along: "<Code inline=true>"Horizontal"</Code>", "
                            <Code inline=true>"Vertical"</Code>" or "<Code inline=true>"Both"</Code>"."
                        </ApiRow>
                        <ApiRow name="allows_window_scrolling" ty="bool">
                            "The visible area is also bounded by the window, for an element that grows with its content and "
                            "scrolls with the page. Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-scroll-view-return">
                    <ApiTable kind=ApiKind::Return of="UseScrollViewReturn">
                        <ApiRow name="scroll_view_styles" ty="Styles">
                            "Styles of the scrolling element: the overflow, and no padding (the layout pads)."
                        </ApiRow>
                        <ApiRow name="content_styles" ty="Styles">
                            "Styles of the content box inside it: the content size, "<Code inline=true>"position: relative"</Code>
                            ", and no pointer events while scrolling."
                        </ApiRow>
                        <ApiRow name="is_scrolling" ty="Signal<bool>">"Whether the user is scrolling."</ApiRow>
                        <ApiRow name="scroll_to" ty="Callback<Rect>">
                            "Scrolls the element so that the visible area starts at the rectangle\u{2019}s position."
                        </ApiRow>
                    </ApiTable>
                </Section>
            </Section>

            <Section title="use_virtualizer_item">
                <p>
                    "Positions a rendered item at its layout info, and measures items of estimated size once they are "
                    "rendered (with "<Code inline=true>"should_observe_item_size"</Code>", whenever their content resizes). "
                    "Render a wrapper "<Code inline=true>"<div>"</Code>" with the returned styles around the item; give it "
                    <Code inline=true>"role=\"presentation\""</Code>" when the item has a role of its own."
                </p>
                <ReactAriaSource path="virtualizer/useVirtualizerItem.ts"/>
                <Section title="Input" id="use-virtualizer-item-input">
                    <ApiTable kind=ApiKind::Input of="UseVirtualizerItemInput">
                        <ApiRow name="layout_info" ty="Signal<LayoutInfo>">
                            "The item\u{2019}s current layout info, from the state\u{2019}s "<Code inline=true>"visible"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="parent" ty="Signal<Option<LayoutInfo>>">
                            "The layout info of the element the wrapper is placed in (e.g. a section), for positions relative "
                            "to it. Required ("<Code inline=true>"None"</Code>": the content box)."
                        </ApiRow>
                        <ApiRow name="update_item_size" ty="Callback<(Key, Size)>">
                            "Receives the measured size: call the state\u{2019}s "<Code inline=true>"update_item_size"</Code>". Required."
                        </ApiRow>
                        <ApiRow name="should_observe_item_size" ty="bool">
                            "Measure again whenever the item\u{2019}s content resizes. Required."
                        </ApiRow>
                    </ApiTable>
                </Section>
                <Section title="Return" id="use-virtualizer-item-return">
                    <ApiTable kind=ApiKind::Return of="UseVirtualizerItemReturn">
                        <ApiRow name="styles" ty="Signal<Styles>">
                            "The wrapper\u{2019}s styles: absolutely positioned (sticky for sticky layout infos) at its "
                            "layout info, in the writing direction of the locale."
                        </ApiRow>
                    </ApiTable>
                    <p>
                        <Code inline=true>"layout_info_styles(info, direction, parent)"</Code>" returns the same styles for "
                        "a layout info without the hook, e.g. for elements you don\u{2019}t measure."
                    </p>
                </Section>
            </Section>

            <Section title="Virtualizing a Collection Hook">
                <p>
                    "Collection hooks such as "
                    <Link href=format!("{}#use-listbox-input", routes::doc::listbox::Hook.materialize())>"use_listbox"</Link>
                    " and "
                    <Link href=format!("{}#use-selectable-list-input", routes::doc::CollectionState.materialize())>"use_selectable_list"</Link>
                    " find items through the rendered elements by default. When only the visible items are rendered, pass "
                    "the state\u{2019}s "<Code inline=true>"layout_delegate()"</Code>" as their "
                    <Code inline=true>"layout_delegate"</Code>": the page keys then find items through the layout. "
                    <Code inline=true>"use_listbox"</Code>" also takes "<Code inline=true>"is_virtualized"</Code>
                    ", which makes options announce their position. Persist the focused key, so that the focused item "
                    "stays rendered:"
                </p>
                <Code language=Language::Rust>
                    {indoc!(r"
                        // In `UseVirtualizerStateInput`:
                        persisted_keys: Signal::derive(move || {
                            list_state.selection.focused_key().into_iter().collect()
                        }),

                        // In `UseListBoxInput`:
                        layout_delegate: Some(state.layout_delegate()),
                        is_virtualized: true,
                    ")}
                </Code>
            </Section>

            <SeeAlso>
                <li><Link href=routes::doc::collection_state::Virtualizer.materialize()>"Virtualizer"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link></li>
                <li><Link href=routes::doc::listbox::Hook.materialize()>"Listbox Hooks"</Link></li>
            </SeeAlso>
        </DocPage>
    }
}
