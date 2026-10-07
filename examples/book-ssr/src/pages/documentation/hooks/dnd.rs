use indoc::indoc;
use leptos::prelude::*;

use super::demos::{
    dnd_drag_to_drop::DragToDropDemo, dnd_quick_start::DndQuickStartDemo, dnd_reorder::ReorderDemo,
};
use crate::{kit::*, routes};

/// The Drag & Drop area: an overview of hooks that are used together, documented on this one page.
#[component]
pub fn PageUseDnd() -> impl IntoView {
    view! {
        <DocPage title="Drag & Drop">
            <p>
                "The drag and drop hooks move data between elements: "<Code inline=true>"use_drag"</Code>" makes an element "
                "draggable, "<Code inline=true>"use_drop"</Code>" makes one a drop target, and the collection hooks let users "
                "reorder a list or drop on, between and into its items. Every drag works three ways: with the mouse or touch "
                "(the browser\u{2019}s native drag and drop), with the keyboard, and with a screen reader, which hears how to "
                "start, where it can drop and what happened. See the "
                <Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for related hooks."
            </p>

            <HowDragsWork/>
            <Relationships/>
            <QuickStart/>
            <UseDragSection/>
            <UseDropSection/>
            <CollectionsSection/>
            <UseAutoScrollSection/>
            <UseVirtualDropSection/>

            <Section title="Clipboard">
                <p>
                    "Cut, copy and paste use the same data: "
                    <Link href=routes::doc::utilities::UseClipboard.materialize()><Code inline=true>"use_clipboard"</Code></Link>
                    " puts "<Code inline=true>"DragItem"</Code>"s on the clipboard and hands pasted data to you as "
                    <Code inline=true>"DropItem"</Code>"s."
                </p>
            </Section>

            <DataModel/>
            <KeyboardAndScreenReaders/>

            <SeeAlso>
                <li><Link href=routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link>" \u{2014} collections, list state and selection"</li>
                <li><Link href=routes::doc::grid_list::Hook.materialize()>"Grid List Hooks"</Link>" \u{2014} "<Code inline=true>"use_grid_list"</Code>" and "<Code inline=true>"use_grid_list_item"</Code></li>
                <li><Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" \u{2014} for drag and drop buttons"</li>
                <li><Link href=routes::doc::utilities::UseClipboard.materialize()>"use_clipboard"</Link>" \u{2014} cut, copy and paste with the same data"</li>
                <li>
                    <Link href=routes::doc::interactions::UseMove.materialize()>"use_move"</Link>
                    " \u{2014} pointer and keyboard movement without data transfer"
                </li>
            </SeeAlso>
        </DocPage>
    }
}

#[component]
fn HowDragsWork() -> impl IntoView {
    view! {
        <Section title="How Drags Work">
            <p>
                "A mouse or touch drag is a native HTML drag: the hooks write the dragged data into the drag\u{2019}s "
                <Code inline=true>"DataTransfer"</Code>", so it can also leave the page, and drop targets read data dragged "
                "in from other applications (text, links, files and folders)."
            </p>
            <p>
                "A keyboard or screen reader drag starts a drag session instead. While it lasts, focus moves only between "
                "the drop targets that accept the dragged data, everything else is hidden from assistive technology, and "
                "the session announces each step. Focusing a drop target is the keyboard\u{2019}s equivalent of dragging "
                "over it: it fires the same enter and exit events. The data stays in the page."
            </p>
            <p>
                "Each drag allows some drop operations ("<AnchorLink href="#dropoperation">"DropOperation"</AnchorLink>
                ": move, copy, link), and each drop target picks one of them, or "<Code inline=true>"Cancel"</Code>
                " to refuse the drop. The drag source learns the outcome from "<Code inline=true>"on_drag_end"</Code>
                ", for example to remove moved data."
            </p>
        </Section>
    }
}

#[component]
fn Relationships() -> impl IntoView {
    view! {
        <Section title="Relationships">
            <p>
                "Pick the hooks by what users drag and where they drop it. Single elements and collections combine "
                "freely: a card made draggable with "<Code inline=true>"use_drag"</Code>" can be dropped into a "
                "droppable list, and list items onto a "<Code inline=true>"use_drop"</Code>" target."
            </p>
            <DocTable headers=&["Hooks", "Use them for"]>
                <TableRow>
                    <TableCell>
                        <AnchorLink href="#use-drag">"use_drag"</AnchorLink>", "
                        <AnchorLink href="#use-drop">"use_drop"</AnchorLink>
                    </TableCell>
                    <TableCell>"A single draggable element, and a single drop target (an inbox, a trash can, a file upload area)."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#collections">"The collection hooks"</AnchorLink></TableCell>
                    <TableCell>
                        "Lists and grids whose items are dragged, reordered or dropped on, between and into, built on a "
                        <Code inline=true>"ListState"</Code>"."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>
                        <AnchorLink href="#use-auto-scroll">"use_auto_scroll"</AnchorLink>", "
                        <AnchorLink href="#use-virtual-drop">"use_virtual_drop"</AnchorLink>
                    </TableCell>
                    <TableCell>
                        "Parts of the hooks above, for drop targets you build yourself: scrolling while a drag nears the "
                        "edges, and the description of a drop target during keyboard drags."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell>
                        <AnchorLink href="#use-drag-session">"use_drag_session"</AnchorLink>", "
                        <AnchorLink href="#use-drag-modality">"use_drag_modality"</AnchorLink>
                    </TableCell>
                    <TableCell>"Rendering that depends on a running keyboard drag, or on how the user drags."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Link href=routes::doc::utilities::UseClipboard.materialize()>"use_clipboard"</Link></TableCell>
                    <TableCell>"Cut, copy and paste of the same data."</TableCell>
                </TableRow>
            </DocTable>
        </Section>
    }
}

#[component]
fn QuickStart() -> impl IntoView {
    view! {
        <Section title="Quick Start">
            <p>
                "A draggable card and a drop target. Drag the card with the mouse, or focus it, press "<Keys keys="Enter"/>
                " to start a keyboard drag and "<Keys keys="Enter"/>" again on the drop target to drop."
            </p>
            <Demo description="A card dragged onto a drop target" source=include_str!("demos/dnd_quick_start.rs") source_open=true>
                <DndQuickStartDemo/>
            </Demo>
        </Section>
    }
}

#[component]
fn UseDragSection() -> impl IntoView {
    view! {
        <Section title="use_drag">
            <ReactAria hook="useDrag"/>
            <p>
                "Makes an element draggable. Spread "<Code inline=true>"drag_props"</Code>" on a focusable element: it "
                "sets "<Code inline=true>"draggable"</Code>", describes how to start a drag, and starts keyboard drags on "
                <Keys keys="Enter"/>" and screen reader drags on a click. With "<Code inline=true>"has_drag_button"</Code>
                ", the element only handles pointer drags, and a separate button (built with "
                <Link href=routes::doc::button::Hook.materialize()>"use_button"</Link>" from "
                <Code inline=true>"drag_button"</Code>") starts the others."
            </p>

            <Section title="Input" id="use-drag-input">
                <p>"Pass a "<Code inline=true>"UseDragInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                <ApiTable kind=ApiKind::Input of="UseDragInput">
                    <ApiRow name="get_items" ty="Callback<(), Vec<DragItem>>">"The dragged data, read when a drag starts. Required."</ApiRow>
                    <ApiRow name="get_allowed_drop_operations" ty="Option<Callback<(), Vec<DropOperation>>>" default="None">
                        "The operations the drag allows, in order of preference. "<Code inline=true>"None"</Code>
                        " allows move, copy and link."
                    </ApiRow>
                    <ApiRow name="preview" ty="Option<Callback<Vec<DragItem>, Option<DragPreview>>>" default="None">
                        "An element the browser shows under the pointer instead of a snapshot of the dragged element, and "
                        "the pointer\u{2019}s offset in it. Pointer drags only."
                    </ApiRow>
                    <ApiRow name="on_drag_start" ty="Option<Callback<DragStartEvent>>" default="None">
                        "A drag starts. Viewport coordinates of the pointer (the element\u{2019}s center for keyboard drags)."
                    </ApiRow>
                    <ApiRow name="on_drag_move" ty="Option<Callback<DragMoveEvent>>" default="None">"The pointer moves during a pointer drag."</ApiRow>
                    <ApiRow name="on_drag_end" ty="Option<Callback<DragEndEvent>>" default="None">
                        "The drag ends, with the "<Code inline=true>"drop_operation"</Code>" the drop target chose, or "
                        <Code inline=true>"Cancel"</Code>"."
                    </ApiRow>
                    <ApiRow name="has_drag_button" ty="bool" default="false">
                        "Keyboard and screen reader drags start from "<Code inline=true>"drag_button"</Code>" instead of the element."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Prevents all drags."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-drag-return">
                <ApiTable kind=ApiKind::Return of="UseDragReturn">
                    <ApiRow name="drag_props" ty="UseDragProps">
                        "For the draggable element: "<Code inline=true>"draggable"</Code>" (\u{201c}false\u{201d} while disabled), "
                        <Code inline=true>"aria-describedby"</Code>" (\u{201c}Press Enter to start dragging.\u{201d}), the native "
                        "drag events, and the key and click handlers starting keyboard and screen reader drags (the latter two "
                        "only without a drag button)."
                    </ApiRow>
                    <ApiRow name="drag_button" ty="UseButtonInput">
                        "For "<Code inline=true>"use_button"</Code>": starts keyboard and screen reader drags and is described like "
                        "the element. Give the button a label."
                    </ApiRow>
                    <ApiRow name="is_dragging" ty="Signal<bool>">"Whether the element is being dragged."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example" id="use-drag-example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::hooks::*;

                        let UseDragReturn { drag_props, is_dragging, .. } = use_drag(UseDragInput {
                            get_items: Callback::new(|()| vec![DragItem::text("Hello")]),
                            get_allowed_drop_operations: None,
                            preview: None,
                            on_drag_start: None,
                            on_drag_move: None,
                            on_drag_end: Some(Callback::new(|e: DragEndEvent| {
                                if e.drop_operation == DropOperation::Move {
                                    // Remove the moved data.
                                }
                            })),
                            has_drag_button: false,
                            is_disabled: Signal::stored(false),
                        });

                        view! {
                            <div {..drag_props.into_attrs()} role="button" tabindex="0"
                                data-dragging=move || is_dragging.get().then_some("")>
                                "Drag me"
                            </div>
                        }
                    "#)}
                </Code>
            </Section>
        </Section>
    }
}

#[component]
fn UseDropSection() -> impl IntoView {
    view! {
        <Section title="use_drop">
            <ReactAria hook="useDrop"/>
            <p>
                "Makes an element a drop target. Spread "<Code inline=true>"drop_props"</Code>" on it; they capture the "
                "element, which keyboard drags focus, so make it focusable. By default the target accepts any data with "
                "the first operation the drag allows; "<Code inline=true>"get_drop_operation"</Code>" decides per drag, "
                "and a target returning "<Code inline=true>"Cancel"</Code>" is skipped by keyboard drags."
            </p>

            <Section title="Input" id="use-drop-input">
                <p>"Pass a "<Code inline=true>"UseDropInput"</Code>" with every field named; the Default column gives the value for fields you don\u{2019}t need."</p>
                <ApiTable kind=ApiKind::Input of="UseDropInput">
                    <ApiRow name="element" ty="CapturedElement">"The drop target element. The props capture it. Required."</ApiRow>
                    <ApiRow name="get_drop_operation" ty="Option<Callback<DropOperationQuery, DropOperation>>" default="None">
                        "The operation for a drag, from its "<Code inline=true>"types"</Code>" and "
                        <Code inline=true>"allowed_operations"</Code>". Return one of the allowed operations, or "
                        <Code inline=true>"Cancel"</Code>" to refuse. "<Code inline=true>"None"</Code>" takes the first allowed operation."
                    </ApiRow>
                    <ApiRow name="get_drop_operation_for_point" ty="Option<Callback<DropOperationPointQuery, DropOperation>>" default="None">
                        "The same at a point (relative to the element), for targets with differing areas. Pointer drags only."
                    </ApiRow>
                    <ApiRow name="on_drop_enter" ty="Option<Callback<DropEnterEvent>>" default="None">
                        "A drag that the target accepts enters it (for keyboard drags: the target gains focus)."
                    </ApiRow>
                    <ApiRow name="on_drop_exit" ty="Option<Callback<DropExitEvent>>" default="None">
                        "The drag leaves the target again (for keyboard drags: the target loses focus)."
                    </ApiRow>
                    <ApiRow name="on_drop_move" ty="Option<Callback<DropMoveEvent>>" default="None">"A pointer drag moves over the target."</ApiRow>
                    <ApiRow name="on_drop_activate" ty="Option<Callback<DropActivateEvent>>" default="None">
                        "A pointer drag rested on the target for 800 ms, or "<Keys keys="Alt + Enter"/>" was pressed on it during a "
                        "keyboard drag. Use it to open the target, e.g. a folder or a tab."
                    </ApiRow>
                    <ApiRow name="on_drop" ty="Option<Callback<DropEvent>>" default="None">
                        "Data was dropped: its "<Code inline=true>"items"</Code>" and the "<Code inline=true>"drop_operation"</Code>"."
                    </ApiRow>
                    <ApiRow name="has_drop_button" ty="bool" default="false">
                        "Keyboard and screen reader drops go to a button inside the target (built from "
                        <Code inline=true>"drop_button"</Code>"), which then carries the description."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Refuses all drops; keyboard drags skip the target."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-drop-return">
                <ApiTable kind=ApiKind::Return of="UseDropReturn">
                    <ApiRow name="drop_props" ty="UseDropProps">
                        "For the drop target: the element capture, the native drag events, and during keyboard drags "
                        <Code inline=true>"aria-describedby"</Code>" (\u{201c}Press Enter to drop. Press Escape to cancel drag.\u{201d})."
                    </ApiRow>
                    <ApiRow name="drop_button" ty="UseButtonInput">"For "<Code inline=true>"use_button"</Code>", with "<Code inline=true>"has_drop_button"</Code>"."</ApiRow>
                    <ApiRow name="is_drop_target" ty="Signal<bool>">"Whether an accepted drag is over the target (or the target has focus during a keyboard drag)."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Example" id="use-drop-example">
                <Code language=Language::Rust>
                    {indoc!(r#"
                        use leptonic::{hooks::*, utils::CapturedElement};

                        let UseDropReturn { drop_props, is_drop_target, .. } = use_drop(UseDropInput {
                            element: CapturedElement::new(),
                            // Accept images only, as copies.
                            get_drop_operation: Some(Callback::new(|q: DropOperationQuery| {
                                if q.types.has(&DragType::from("image/*")) && q.allowed_operations.contains(&DropOperation::Copy) {
                                    DropOperation::Copy
                                } else {
                                    DropOperation::Cancel
                                }
                            })),
                            get_drop_operation_for_point: None,
                            on_drop_enter: None,
                            on_drop_move: None,
                            on_drop_activate: None,
                            on_drop_exit: None,
                            on_drop: Some(Callback::new(|e: DropEvent| {
                                for item in e.items {
                                    if let DropItem::File(file) = item {
                                        // Upload `file.file()`.
                                    }
                                }
                            })),
                            has_drop_button: false,
                            is_disabled: Signal::stored(false),
                        });

                        view! {
                            <div {..drop_props.into_attrs()} role="button" tabindex="0"
                                data-drop-target=move || is_drop_target.get().then_some("")>
                                "Drop images here"
                            </div>
                        }
                    "#)}
                </Code>
            </Section>

            <Section title="Demo" id="use-drop-demo">
                <p>
                    "Drag the cards onto the drop targets. The inbox takes any data and uses the first operation the drag "
                    "allows: the note is moved, the link (which can\u{2019}t be moved) is copied. The bookmarks take links only, "
                    "and link them. With the keyboard, focus a card and press "<Keys keys="Enter"/>": focus moves to the nearest "
                    "drop target, "<Keys keys="Tab"/>" moves to the next one (the note skips the bookmarks), "<Keys keys="Enter"/>
                    " drops and "<Keys keys="Escape"/>" cancels. You can also drop text, links or files from other applications "
                    "onto the inbox."
                </p>
                <Demo description="Two draggable cards and two drop targets with different accepted data and operations" source=include_str!("demos/dnd_drag_to_drop.rs")>
                    <DragToDropDemo/>
                </Demo>
            </Section>
        </Section>
    }
}

#[component]
fn CollectionsSection() -> impl IntoView {
    view! {
        <Section title="Collections">
            <ReactAria hook="useDraggableCollection"/>
            <ReactAria hook="useDroppableCollection"/>
            <p>
                "The collection hooks add drag and drop to a collection built on a "<Code inline=true>"ListState"</Code>
                ", such as a "<Link href=routes::doc::GridList.materialize()>"grid list"</Link>" (see "
                <Link href=routes::doc::CollectionState.materialize()>"Collection State"</Link>"). Dragging a selected item drags "
                "all selected items. Drops land on the collection itself, on an item, or between two items, shown by drop "
                "indicators; during keyboard drags, the collection\u{2019}s navigation keys move between these positions."
            </p>
            <DocTable headers=&["Hook", "Called", "Role"]>
                <TableRow>
                    <TableCell><AnchorLink href="#use-draggable-collection-state">"use_draggable_collection_state"</AnchorLink></TableCell>
                    <TableCell>"Once"</TableCell>
                    <TableCell>"The dragged keys and their data, drag events."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-draggable-collection">"use_draggable_collection"</AnchorLink></TableCell>
                    <TableCell>"Once"</TableCell>
                    <TableCell>"Marks the collection element as the source, so drops back into it count as internal (reorders)."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-draggable-item">"use_draggable_item"</AnchorLink></TableCell>
                    <TableCell>"Per item"</TableCell>
                    <TableCell>"Makes an item draggable."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-droppable-collection-state">"use_droppable_collection_state"</AnchorLink></TableCell>
                    <TableCell>"Once"</TableCell>
                    <TableCell>"The drop handlers and the current drop target; decides which drops are valid."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-droppable-collection">"use_droppable_collection"</AnchorLink></TableCell>
                    <TableCell>"Once"</TableCell>
                    <TableCell>"Native drops on the collection element, keyboard navigation between drop targets, focus after drops."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-droppable-item">"use_droppable_item"</AnchorLink></TableCell>
                    <TableCell>"Per item"</TableCell>
                    <TableCell>"Makes an item a target of keyboard drags (dropping on it)."</TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><AnchorLink href="#use-drop-indicator">"use_drop_indicator"</AnchorLink></TableCell>
                    <TableCell>"Per position"</TableCell>
                    <TableCell>"A drop position between items (or on the collection), focusable during keyboard drags."</TableCell>
                </TableRow>
            </DocTable>

            <Section title="Demo" id="collections-demo">
                <p>
                    "Drag the rows to reorder them; a line shows where they land. Click rows (or press "<Keys keys="Space"/>
                    ") to select several and drag them together. With the keyboard, focus a row and press "<Keys keys="Enter"/>
                    ": the arrow keys move between the positions between rows, "<Keys keys="Enter"/>" drops and "
                    <Keys keys="Escape"/>" cancels. The rows only accept reorders, so dropping on a row is not possible."
                </p>
                <Demo description="A grid list reordered by dragging with the mouse or keyboard, with drop indicators and multiple selection" source=include_str!("demos/dnd_reorder.rs")>
                    <ReorderDemo/>
                </Demo>
            </Section>

            <CollectionsExample/>
            <DraggableCollectionHooks/>
            <DroppableCollectionStateSection/>
            <DroppableCollectionHooks/>
        </Section>
    }
}

#[component]
fn CollectionsExample() -> impl IntoView {
    view! {
        <Section title="Example" id="collections-example">
            <p>
                "The demo\u{2019}s structure, reduced to the drag and drop parts. Rows render a drop indicator before "
                "themselves; the last row also one after itself."
            </p>
            <Code language=Language::Rust>
                {indoc!(r#"
                    use std::{collections::HashSet, sync::Arc};

                    use leptonic::{
                        hooks::{collections::*, *},
                        utils::CapturedElement,
                    };

                    // Once, for the collection (`list` from `use_list_state`, `element` captured by the grid list's props):
                    let drag_state = use_draggable_collection_state(UseDraggableCollectionStateInput {
                        list,
                        get_items: Callback::new(|keys: HashSet<Key>| keys.iter().map(|k| DragItem::text(k.to_string())).collect()),
                        preview: None,
                        get_allowed_drop_operations: None,
                        on_drag_start: None,
                        on_drag_move: None,
                        on_drag_end: None,
                        is_disabled: Signal::stored(false),
                    });
                    use_draggable_collection(drag_state, element);
                    let drop_state = use_droppable_collection_state(UseDroppableCollectionStateInput {
                        list,
                        options: DroppableCollectionOptions { on_reorder: Some(on_reorder), ..Default::default() },
                        is_disabled: Signal::stored(false),
                    });
                    let UseDroppableCollectionReturn { collection_props, data: drop } =
                        use_droppable_collection(UseDroppableCollectionInput {
                            state: drop_state,
                            element,
                            collection_id: props.id.clone(),
                            keyboard_delegate: use_list_keyboard_delegate(list, element, Orientation::Vertical, ListLayout::Stack),
                            drop_target_delegate: Arc::new(ListDropTargetDelegate::new(list.collection, list.item_elements, element)),
                            on_key_down: None,
                        });
                    // <div {..props.into_attrs()} {..collection_props.into_attrs()}>

                    // Per item, on the row element:
                    let drag = use_draggable_item(UseDraggableItemInput { state: drag_state, key: key.clone(), has_drag_button: false, has_action: false });
                    let row_element = CapturedElement::new();
                    let drop_item = use_droppable_item(UseDroppableItemInput {
                        collection: drop.clone(),
                        target: DropTarget::item(key.clone(), DropPosition::On),
                        element: row_element,
                        activate_button: None,
                    });

                    // Per position between items:
                    let indicator = use_drop_indicator(UseDropIndicatorInput {
                        collection: drop.clone(),
                        target: DropTarget::item(key, DropPosition::Before),
                        activate_button: None,
                    });
                    // <div role="row"><div role="gridcell" {..indicator.drop_indicator_props.into_attrs()}></div></div>
                "#)}
            </Code>
            <p>
                "Both "<Code inline=true>"use_draggable_item"</Code>" and "<Code inline=true>"use_droppable_item"</Code>
                " describe the row: join their "<Code inline=true>"aria_describedby"</Code>" ids into one attribute, as "
                "the demo does. Set the grid list\u{2019}s "<Code inline=true>"should_select_on_press_up"</Code>
                " too, so that dragging a row doesn\u{2019}t select it."
            </p>
        </Section>
    }
}

#[component]
fn DraggableCollectionHooks() -> impl IntoView {
    view! {
        <Section title="use_draggable_collection_state">
            <Section title="Input" id="use-draggable-collection-state-input">
                <p>
                    "Pass a "<Code inline=true>"UseDraggableCollectionStateInput"</Code>" with every field named; the Default "
                    "column gives the value for fields you don\u{2019}t need."
                </p>
                <ApiTable kind=ApiKind::Input of="UseDraggableCollectionStateInput">
                    <ApiRow name="list" ty="ListState">"The collection and its selection. Required."</ApiRow>
                    <ApiRow name="get_items" ty="Callback<HashSet<Key>, Vec<DragItem>>">"The data of the dragged items. Required."</ApiRow>
                    <ApiRow name="get_allowed_drop_operations" ty="Option<Callback<(), Vec<DropOperation>>>" default="None">
                        "As for "<AnchorLink href="#use-drag-input">"use_drag"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="preview" ty="Option<Callback<Vec<DragItem>, Option<DragPreview>>>" default="None">
                        "As for "<AnchorLink href="#use-drag-input">"use_drag"</AnchorLink>"."
                    </ApiRow>
                    <ApiRow name="on_drag_start" ty="Option<Callback<DraggableCollectionStartEvent>>" default="None">
                        "A drag of items starts, with the dragged "<Code inline=true>"keys"</Code>"."
                    </ApiRow>
                    <ApiRow name="on_drag_move" ty="Option<Callback<DraggableCollectionMoveEvent>>" default="None">
                        "A pointer drag of items moves."
                    </ApiRow>
                    <ApiRow name="on_drag_end" ty="Option<Callback<DraggableCollectionEndEvent>>" default="None">
                        "The drag ends: the "<Code inline=true>"keys"</Code>", the "<Code inline=true>"drop_operation"</Code>
                        " and "<Code inline=true>"is_internal"</Code>" (dropped into the same collection). Remove moved items here "
                        "when they were dropped elsewhere."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>" default="false">"Prevents dragging items."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-draggable-collection-state-return">
                <p>"Returns a "<Code inline=true>"DraggableCollectionState"</Code>" ("<Code inline=true>"Copy"</Code>")."</p>
                <ApiTable kind=ApiKind::Fields of="DraggableCollectionState">
                    <ApiRow name="list" ty="ListState">"The collection."</ApiRow>
                    <ApiRow name="dragged_key" ty="Signal<Option<Key>>">"The item the drag started from."</ApiRow>
                    <ApiRow name="dragging_keys" ty="Signal<HashSet<Key>>">"All dragged items."</ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">"The input\u{2019}s "<Code inline=true>"is_disabled"</Code>"."</ApiRow>
                </ApiTable>

                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"is_dragging(&Key) -> bool"</Code></TableCell>
                        <TableCell>"Whether an item is being dragged."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"keys_for_drag(&Key) -> HashSet<Key>"</Code></TableCell>
                        <TableCell>
                            "What a drag starting at an item drags: the selection if the item is selected (without items whose "
                            "parent is selected too), else the item alone."
                        </TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </Section>

        <Section title="use_draggable_collection">
            <p>
                <Code inline=true>"use_draggable_collection(state, element)"</Code>" returns nothing. While the "
                "collection\u{2019}s items are dragged, it records the collection element as the drag source, so that "
                "droppable collections tell reorders and moves (internal drops) from inserts."
            </p>
        </Section>

        <Section title="use_draggable_item">
            <Section title="Input" id="use-draggable-item-input">
                <p>"The input has no defaults: set every field."</p>
                <ApiTable kind=ApiKind::Input of="UseDraggableItemInput">
                    <ApiRow name="state" ty="DraggableCollectionState">"From "<Code inline=true>"use_draggable_collection_state"</Code>". Required."</ApiRow>
                    <ApiRow name="key" ty="Key">"The item\u{2019}s key. Required."</ApiRow>
                    <ApiRow name="has_drag_button" ty="bool">"Keyboard and screen reader drags start from a drag button. Required."</ApiRow>
                    <ApiRow name="has_action" ty="bool">
                        "The item has an action on "<Keys keys="Enter"/>", so keyboard drags start with "<Keys keys="Alt + Enter"/>". Required."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-draggable-item-return">
                <ApiTable kind=ApiKind::Return of="UseDraggableItemReturn">
                    <ApiRow name="drag_props" ty="UseDragProps">
                        "For the item, as for "<Code inline=true>"use_drag"</Code>". In collections with selection (and "
                        "without drag button) it describes the drag: \u{201c}Press Enter to drag 2 selected items.\u{201d}"
                    </ApiRow>
                    <ApiRow name="drag_button" ty="UseButtonInput">"For "<Code inline=true>"use_button"</Code>", with "<Code inline=true>"has_drag_button"</Code>"."</ApiRow>
                    <ApiRow name="drag_button_label" ty="Signal<String>">
                        "The drag button\u{2019}s "<Code inline=true>"aria-label"</Code>": \u{201c}Drag Plan\u{201d}, or "
                        "\u{201c}Drag 3 selected items\u{201d}."
                    </ApiRow>
                    <ApiRow name="is_dragging" ty="Signal<bool>">
                        "Whether a drag started from this item. To mark all dragged items, use the state\u{2019}s "
                        <Code inline=true>"is_dragging(&key)"</Code>", as the demo does."
                    </ApiRow>
                </ApiTable>
                <p>"Disabled items (see the selection options) can\u{2019}t be dragged."</p>
            </Section>
        </Section>
    }
}

#[component]
fn DroppableCollectionStateSection() -> impl IntoView {
    view! {
        <Section title="use_droppable_collection_state">
            <Section title="Input" id="use-droppable-collection-state-input">
                <p>"The input has no defaults: set every field."</p>
                <ApiTable kind=ApiKind::Input of="UseDroppableCollectionStateInput">
                    <ApiRow name="list" ty="ListState">"The collection and its selection. Required."</ApiRow>
                    <ApiRow name="options" ty="DroppableCollectionOptions">
                        "The drop handlers and accepted data, see "<AnchorLink href="#droppablecollectionoptions">"DroppableCollectionOptions"</AnchorLink>". Required."
                    </ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">"Refuses all drops. Required."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="DroppableCollectionOptions">
                <p>
                    "A drop target is valid when a handler takes the drop there; set only the handlers you need, and take "
                    "the rest from "<Code inline=true>"DroppableCollectionOptions::default()"</Code>". A drag whose data "
                    "doesn\u{2019}t match "<Code inline=true>"accepted_drag_types"</Code>" is refused."
                </p>
                <ApiTable kind=ApiKind::Input of="DroppableCollectionOptions">
                    <ApiRow name="accepted_drag_types" ty="AcceptedDragTypes" default="All">"The data types the collection accepts."</ApiRow>
                    <ApiRow name="on_reorder" ty="Option<Callback<DroppableCollectionReorderEvent>>" default="None">
                        "Items of this collection dropped between items with the same parent: the "<Code inline=true>"keys"</Code>
                        " and the "<Code inline=true>"target"</Code>" position."
                    </ApiRow>
                    <ApiRow name="on_move" ty="Option<Callback<DroppableCollectionReorderEvent>>" default="None">
                        "Items of this collection dropped on or between any of its items, also into other parents."
                    </ApiRow>
                    <ApiRow name="on_insert" ty="Option<Callback<DroppableCollectionInsertDropEvent>>" default="None">"Data from elsewhere dropped between items."</ApiRow>
                    <ApiRow name="on_root_drop" ty="Option<Callback<DroppableCollectionRootDropEvent>>" default="None">"Data from elsewhere dropped on the collection itself."</ApiRow>
                    <ApiRow name="on_item_drop" ty="Option<Callback<DroppableCollectionOnItemDropEvent>>" default="None">
                        "Data dropped on an item (not one of the dragged items); "<Code inline=true>"is_internal"</Code>
                        " tells whether it comes from this collection."
                    </ApiRow>
                    <ApiRow name="should_accept_item_drop" ty="Option<Callback<ItemDropQuery, bool>>" default="None">"Whether an item accepts drops on it."</ApiRow>
                    <ApiRow name="on_drop" ty="Option<Callback<DroppableCollectionDropEvent>>" default="None">
                        "Handles every drop itself, instead of the handlers above; all targets are valid."
                    </ApiRow>
                    <ApiRow name="get_drop_operation" ty="Option<Callback<CollectionDropOperationQuery, DropOperation>>" default="None">
                        "The operation at a valid target. "<Code inline=true>"None"</Code>" takes the first allowed operation."
                    </ApiRow>
                    <ApiRow name="on_drop_enter" ty="Option<Callback<DroppableCollectionEnterEvent>>" default="None">
                        "A drag enters a drop target: the collection, an item or a position between items."
                    </ApiRow>
                    <ApiRow name="on_drop_exit" ty="Option<Callback<DroppableCollectionExitEvent>>" default="None">
                        "The drag leaves that drop target again."
                    </ApiRow>
                    <ApiRow name="on_drop_activate" ty="Option<Callback<DroppableCollectionActivateEvent>>" default="None">
                        "A drag activates a target, as for "<AnchorLink href="#use-drop-input">"use_drop"</AnchorLink>"."
                    </ApiRow>
                </ApiTable>
                <p>
                    "Items can never be dropped on themselves or into their own children. When a drop changed the "
                    "collection, inserted items are selected and the first focused; after a drop on an item, the item is focused."
                </p>
            </Section>

            <Section title="Return" id="use-droppable-collection-state-return">
                <p>"Returns a "<Code inline=true>"DroppableCollectionState"</Code>" ("<Code inline=true>"Copy"</Code>")."</p>
                <ApiTable kind=ApiKind::Fields of="DroppableCollectionState">
                    <ApiRow name="list" ty="ListState">"The collection."</ApiRow>
                    <ApiRow name="target" ty="Signal<Option<DropTarget>>">"The current drop target."</ApiRow>
                    <ApiRow name="is_disabled" ty="Signal<bool>">"The input\u{2019}s "<Code inline=true>"is_disabled"</Code>"."</ApiRow>
                </ApiTable>

                <DocTable headers=&["Method", "Description"]>
                    <TableRow>
                        <TableCell><Code inline=true>"is_drop_target(Option<&DropTarget>) -> bool"</Code></TableCell>
                        <TableCell>
                            "Whether a target is the current one. \u{201c}After A\u{201d} and \u{201c}before B\u{201d} are the same "
                            "position when B follows A."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"set_target(Option<DropTarget>)"</Code></TableCell>
                        <TableCell>"Changes the drop target, firing exit and enter events."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>
        </Section>
    }
}

#[component]
fn DroppableCollectionHooks() -> impl IntoView {
    view! {
        <Section title="use_droppable_collection">
            <Section title="Input" id="use-droppable-collection-input">
                <p>"The input has no defaults: set every field."</p>
                <ApiTable kind=ApiKind::Input of="UseDroppableCollectionInput">
                    <ApiRow name="state" ty="DroppableCollectionState">"From "<Code inline=true>"use_droppable_collection_state"</Code>". Required."</ApiRow>
                    <ApiRow name="element" ty="CapturedElement">"The collection element, captured by the collection hook\u{2019}s props. Required."</ApiRow>
                    <ApiRow name="collection_id" ty="String">"The collection element\u{2019}s id; drop indicators on the collection itself reference it. Required."</ApiRow>
                    <ApiRow name="keyboard_delegate" ty="Signal<Arc<dyn KeyboardDelegate>>">
                        "Navigation between items during keyboard drags, e.g. "<Code inline=true>"use_list_keyboard_delegate"</Code>". Required."
                    </ApiRow>
                    <ApiRow name="drop_target_delegate" ty="Arc<dyn DropTargetDelegate>">
                        "The drop target under the pointer. "<Code inline=true>"ListDropTargetDelegate::new(collection, item_elements, element)"</Code>
                        " covers lists and grids ("<Code inline=true>"with_layout"</Code>", "<Code inline=true>"with_orientation"</Code>", "
                        <Code inline=true>"with_direction"</Code>"): before or after an item by the pointer\u{2019}s half, or on "
                        "it when the item accepts drops, with its edges still before and after. Required."
                    </ApiRow>
                    <ApiRow name="on_key_down" ty="Option<Callback<SendWrapper<KeyboardEvent>>>">
                        "Key presses during keyboard drags, after the collection handled them. Required ("<Code inline=true>"None"</Code>" for none)."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-droppable-collection-return">
                <ApiTable kind=ApiKind::Return of="UseDroppableCollectionReturn">
                    <ApiRow name="collection_props" ty="UseDropProps">
                        "For the collection element: native drops, and "<AnchorLink href="#use-auto-scroll">"auto-scrolling"</AnchorLink>
                        " when a drag nears the edges. The collection itself isn\u{2019}t described as a drop target; its items "
                        "and indicators are."
                    </ApiRow>
                    <ApiRow name="data" ty="DroppableCollectionData">"Hand this to "<Code inline=true>"use_droppable_item"</Code>" and "<Code inline=true>"use_drop_indicator"</Code>"."</ApiRow>
                </ApiTable>
                <p>
                    "A keyboard drag entering the collection starts at the focused item (after it, or around the selection) "
                    "and falls back to the next valid target."
                </p>
            </Section>
        </Section>

        <Section title="use_droppable_item">
            <Section title="Input" id="use-droppable-item-input">
                <p>"The input has no defaults: set every field."</p>
                <ApiTable kind=ApiKind::Input of="UseDroppableItemInput">
                    <ApiRow name="collection" ty="DroppableCollectionData">"From "<Code inline=true>"use_droppable_collection"</Code>". Required."</ApiRow>
                    <ApiRow name="target" ty="DropTarget">"Usually "<Code inline=true>"DropTarget::item(key, DropPosition::On)"</Code>". Required."</ApiRow>
                    <ApiRow name="element" ty="CapturedElement">"The item element; capture it with "<Code inline=true>"element.attr()"</Code>". Required."</ApiRow>
                    <ApiRow name="activate_button" ty="Option<CapturedElement>">
                        "A button activating the item (e.g. opening a folder) during keyboard drags. Required ("<Code inline=true>"None"</Code>" for none)."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-droppable-item-return">
                <ApiTable kind=ApiKind::Return of="UseDroppableItemReturn">
                    <ApiRow name="drop_props" ty="UseDroppableItemProps">
                        <Code inline=true>"aria-describedby"</Code>" (how to drop) and, during keyboard drags, "
                        <Code inline=true>"aria-hidden"</Code>" on items that can\u{2019}t take the drop."
                    </ApiRow>
                    <ApiRow name="is_drop_target" ty="Signal<bool>">"Whether the item is the current drop target. Keyboard drags focus it then."</ApiRow>
                </ApiTable>
            </Section>
        </Section>

        <Section title="use_drop_indicator">
            <Section title="Input" id="use-drop-indicator-input">
                <p>"The input has no defaults: set every field."</p>
                <ApiTable kind=ApiKind::Input of="UseDropIndicatorInput">
                    <ApiRow name="collection" ty="DroppableCollectionData">"From "<Code inline=true>"use_droppable_collection"</Code>". Required."</ApiRow>
                    <ApiRow name="target" ty="DropTarget">
                        "The position: "<Code inline=true>"DropTarget::item(key, DropPosition::Before)"</Code>" (or "
                        <Code inline=true>"After"</Code>" for the last item), or "<Code inline=true>"DropTarget::Root"</Code>". Required."
                    </ApiRow>
                    <ApiRow name="activate_button" ty="Option<CapturedElement>">
                        "A button activating the target during keyboard drags. Required ("<Code inline=true>"None"</Code>" for none)."
                    </ApiRow>
                </ApiTable>
            </Section>

            <Section title="Return" id="use-drop-indicator-return">
                <ApiTable kind=ApiKind::Return of="UseDropIndicatorReturn">
                    <ApiRow name="drop_indicator_props" ty="UseDropIndicatorProps">
                        "An id, "<Code inline=true>"aria-roledescription=\"drop indicator\""</Code>", an "
                        <Code inline=true>"aria-label"</Code>" (\u{201c}Insert between Plan and Design\u{201d}, \u{201c}Insert "
                        "before Plan\u{201d}, \u{201c}Insert after Release\u{201d}, \u{201c}Drop on Plan\u{201d}), "
                        <Code inline=true>"tabindex=-1"</Code>", "<Code inline=true>"aria-hidden"</Code>" outside keyboard "
                        "drags, and the element capture."
                    </ApiRow>
                    <ApiRow name="is_drop_target" ty="Signal<bool>">"Whether a drag is over this position: show the indicator."</ApiRow>
                    <ApiRow name="is_hidden" ty="Signal<bool>">
                        "Whether to hide the indicator. Keep its element rendered (e.g. with "<Code inline=true>"display: none"</Code>
                        "), as the hook registers it."
                    </ApiRow>
                </ApiTable>
            </Section>
        </Section>
    }
}

#[component]
fn UseAutoScrollSection() -> impl IntoView {
    view! {
        <Section title="use_auto_scroll">
            <ReactAriaSource path="dnd/useAutoScroll.ts"/>
            <p>
                <Code inline=true>"use_auto_scroll(element)"</Code>" scrolls a drop target (or its scroll parent) while a "
                "pointer drag nears its edges. Browsers do this during native drags themselves, except Safari on the "
                "desktop, so the hook only acts there. "<Code inline=true>"use_droppable_collection"</Code>" uses it "
                "already; call it for scrollable drop targets you build with "<Code inline=true>"use_drop"</Code>"."
            </p>
            <p>"It returns an "<Code inline=true>"AutoScroll"</Code>" ("<Code inline=true>"Copy"</Code>"), which stops scrolling when the component unmounts:"</p>
            <DocTable headers=&["Method", "Description"]>
                <TableRow>
                    <TableCell><Code inline=true>"move_to(x: f64, y: f64)"</Code></TableCell>
                    <TableCell>
                        "The drag is at "<Code inline=true>"x"</Code>", "<Code inline=true>"y"</Code>" (relative to the drop "
                        "target, as in "<Code inline=true>"DropMoveEvent"</Code>"): scroll while it is within 20 px of an edge."
                    </TableCell>
                </TableRow>
                <TableRow>
                    <TableCell><Code inline=true>"stop()"</Code></TableCell>
                    <TableCell>"Stop scrolling, e.g. when the drag leaves the target or drops."</TableCell>
                </TableRow>
            </DocTable>
            <Code language=Language::Rust>
                {indoc!(r"
                    use leptonic::{hooks::*, utils::CapturedElement};

                    let element = CapturedElement::new();
                    let auto_scroll = use_auto_scroll(element);
                    let UseDropReturn { drop_props, .. } = use_drop(UseDropInput {
                        element,
                        get_drop_operation: None,
                        get_drop_operation_for_point: None,
                        on_drop_enter: None,
                        on_drop_move: Some(Callback::new(move |e: DropMoveEvent| auto_scroll.move_to(e.x, e.y))),
                        on_drop_activate: None,
                        on_drop_exit: Some(Callback::new(move |_| auto_scroll.stop())),
                        on_drop: Some(Callback::new(move |_| auto_scroll.stop())),
                        has_drop_button: false,
                        is_disabled: Signal::stored(false),
                    });

                ")}
            </Code>
        </Section>
    }
}

#[component]
fn UseVirtualDropSection() -> impl IntoView {
    view! {
        <Section title="use_virtual_drop">
            <ReactAriaSource path="dnd/useVirtualDrop.ts"/>
            <p>
                <Code inline=true>"use_virtual_drop()"</Code>" returns the id of a hidden description telling how to drop "
                "(\u{201c}Press Enter to drop. Press Escape to cancel drag.\u{201d}), as a "
                <Code inline=true>"Signal<Option<String>>"</Code>" that is "<Code inline=true>"Some"</Code>" only during "
                "keyboard and screen reader drags. "<Code inline=true>"use_drop"</Code>" and "
                <Code inline=true>"use_droppable_item"</Code>" add it to their "<Code inline=true>"aria-describedby"</Code>
                " already; use it for drop buttons or other elements that receive keyboard drops."
            </p>
            <Code language=Language::Rust>
                {indoc!(r#"
                    use leptonic::hooks::use_virtual_drop;

                    let description = use_virtual_drop();
                    view! { <button aria-describedby=move || description.get()>"Drop here"</button> }
                "#)}
            </Code>
        </Section>
    }
}

#[component]
fn DataModel() -> impl IntoView {
    view! {
        <Section title="Data Model">
            <Section title="DragItem">
                <p>
                    "One dragged item, in one or more representations: (MIME type, data) pairs. Offer several to let "
                    "different targets (and other applications) pick the one they understand."
                </p>
                <Code language=Language::Rust>
                    {indoc!(r#"
                        DragItem::text("Hello");                         // text/plain
                        DragItem::new()
                            .with("text/uri-list", "https://leptos.dev")
                            .with("text/plain", "https://leptos.dev");
                        DragItem::new().with("application/x-task-id", "42"); // a custom type
                    "#)}
                </Code>
            </Section>

            <Section title="DropItem">
                <DocTable headers=&["Variant", "Data"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Text(TextDropItem)"</Code></TableCell>
                        <TableCell>
                            "The representations of a dragged item: "<Code inline=true>"types()"</Code>", "
                            <Code inline=true>"has_type(type)"</Code>", "<Code inline=true>"get_text(type)"</Code>" (synchronous)."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"File(FileDropItem)"</Code></TableCell>
                        <TableCell>
                            "A dropped file: "<Code inline=true>"kind"</Code>" (MIME type), "<Code inline=true>"name"</Code>", "
                            <Code inline=true>"file()"</Code>", "<Code inline=true>"get_text().await"</Code>"."
                        </TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Directory(DirectoryDropItem)"</Code></TableCell>
                        <TableCell>"A dropped folder: "<Code inline=true>"name"</Code>", "<Code inline=true>"get_entries().await"</Code>" (its files and folders)."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="DropOperation">
                <p>
                    <Code inline=true>"Move"</Code>", "<Code inline=true>"Copy"</Code>", "<Code inline=true>"Link"</Code>
                    " or "<Code inline=true>"Cancel"</Code>" (no drop). Drags list the operations they allow in order of "
                    "preference; drop targets choose one. During pointer drags, modifier keys restrict the allowed operations: "
                    "on macOS "<Keys keys="Option"/>" copies, "<Keys keys="Control"/>" links and "<Keys keys="Meta"/>" moves; elsewhere "
                    <Keys keys="Control"/>" copies, "<Keys keys="Alt"/>" links and "<Keys keys="Shift"/>" moves."
                </p>
            </Section>

            <Section title="DragTypes and AcceptedDragTypes">
                <p>
                    "Drop targets see the "<Code inline=true>"DragTypes"</Code>" of a drag before the drop (not its data). "
                    <Code inline=true>"has(&DragType)"</Code>" checks for a type: "<Code inline=true>"DragType::from(\"text/plain\")"</Code>
                    ", a wildcard like "<Code inline=true>"\"image/*\""</Code>" or "<Code inline=true>"\"*/*\""</Code>", or "
                    <Code inline=true>"DragType::Directory"</Code>". Browsers hide the types of dragged files until the drop, so "
                    "while files are dragged in, "<Code inline=true>"has"</Code>" accepts every type. "
                    <Code inline=true>"AcceptedDragTypes::All"</Code>" or "<Code inline=true>"AcceptedDragTypes::Types(vec![..])"</Code>
                    " declares what a collection accepts."
                </p>
            </Section>

            <Section title="DropTarget">
                <p>
                    <Code inline=true>"DropTarget::Root"</Code>" is the collection itself; "
                    <Code inline=true>"DropTarget::Item(ItemDropTarget { key, drop_position })"</Code>" a position at an item, "
                    "with "<Code inline=true>"DropPosition::Before"</Code>", "<Code inline=true>"On"</Code>" or "
                    <Code inline=true>"After"</Code>". "<Code inline=true>"DropTarget::item(key, position)"</Code>" builds one."
                </p>
            </Section>
        </Section>
    }
}

#[component]
fn KeyboardAndScreenReaders() -> impl IntoView {
    view! {
        <Section title="Keyboard and Screen Readers">
            <KeyboardTable>
                <KeyRow keys="Enter">
                    "On a draggable element or drag button: start a drag. Focus moves to the nearest drop target that "
                    "accepts the data (the one containing the element, if any). Collection items with an action start with "
                    <Keys keys="Alt + Enter"/>"."
                </KeyRow>
                <KeyRow keys="Tab / Shift + Tab">"Next or previous drop target; past the last one, back to the drag source."</KeyRow>
                <KeyRow keys="Arrow keys / Home / End / PageUp / PageDown">"Inside a droppable collection: move between its drop positions."</KeyRow>
                <KeyRow keys="Enter">"Drop on the focused target. On the drag source: cancel the drag."</KeyRow>
                <KeyRow keys="Alt + Enter">"Activate the focused target ("<Code inline=true>"on_drop_activate"</Code>")."</KeyRow>
                <KeyRow keys="Escape">"Cancel the drag; focus returns to the drag source."</KeyRow>
            </KeyboardTable>
            <p>
                "Screen reader users start drags by clicking the element (or drag button), and drop by clicking a target. "
                "During a drag, mouse and pointer events are blocked, and everything except the drag source, the valid "
                "drop targets and their activate buttons is hidden from assistive technology."
            </p>
            <DocTable headers=&["When", "Message"]>
                <TableRow><TableCell>"Describing a draggable element"</TableCell><TableCell>"Press Enter to start dragging."</TableCell></TableRow>
                <TableRow><TableCell>"Describing a dragged element"</TableCell><TableCell>"Dragging. Press Enter to cancel drag."</TableCell></TableRow>
                <TableRow><TableCell>"Announced when a drag starts"</TableCell><TableCell>"Started dragging. Press Tab to navigate to a drop target, then press Enter to drop, or press Escape to cancel."</TableCell></TableRow>
                <TableRow><TableCell>"Describing a drop target during a drag"</TableCell><TableCell>"Press Enter to drop. Press Escape to cancel drag."</TableCell></TableRow>
                <TableRow><TableCell>"Announced after a drop or cancel"</TableCell><TableCell>"Drop complete. / Drop canceled."</TableCell></TableRow>
            </DocTable>
            <p>
                "The messages follow the "<AnchorLink href="#use-drag-modality">"drag modality"</AnchorLink>": after a "
                "keyboard interaction they mention "<Keys keys="Enter"/>", on touch screens double taps or long presses "
                "(\u{201c}Double tap to start dragging.\u{201d}, \u{201c}Long press to drag 2 selected items.\u{201d}), "
                "otherwise clicks (\u{201c}Click to start dragging.\u{201d}). They are English for now."
            </p>

            <DragSessionSection/>
            <DragModalitySection/>
        </Section>
    }
}

#[component]
fn DragSessionSection() -> impl IntoView {
    view! {
        <Section title="use_drag_session">
            <ReactAriaSource path="dnd/DragManager.ts"/>
            <p>
                <Code inline=true>"use_drag_session()"</Code>" returns the running keyboard or screen reader drag as a "
                <Code inline=true>"Signal<Option<DragSessionInfo>>"</Code>": "<Code inline=true>"Some"</Code>" from the "
                "start of such a drag until its drop or cancel, "<Code inline=true>"None"</Code>" otherwise (also during "
                "native mouse and touch drags). Use it to render things only keyboard drags need, such as drop buttons."
            </p>

            <Section title="DragSessionInfo">
                <ApiTable kind=ApiKind::Fields of="DragSessionInfo">
                    <ApiRow name="items" ty="Vec<DragItem>">"The dragged data."</ApiRow>
                    <ApiRow name="allowed_drop_operations" ty="Vec<DropOperation>">"The operations the drag allows, in order of preference."</ApiRow>
                </ApiTable>
            </Section>

            <Section title="is_virtual_dragging">
                <p>
                    <Code inline=true>"is_virtual_dragging() -> bool"</Code>" tells whether a keyboard or screen reader drag "
                    "is running, without tracking: use it in event handlers, and "<Code inline=true>"use_drag_session"</Code>
                    " where the view has to update."
                </p>
            </Section>
        </Section>
    }
}

#[component]
fn DragModalitySection() -> impl IntoView {
    view! {
        <Section title="use_drag_modality">
            <ReactAriaSource path="dnd/utils.ts"/>
            <p>
                <Code inline=true>"use_drag_modality()"</Code>" returns a "<Code inline=true>"Signal<DragModality>"</Code>
                " that follows the "<Link href=routes::doc::focus::UseFocusVisible.materialize()>"interaction modality"</Link>
                ". The hooks use it to word their descriptions and announcements; use it for your own instructions."
            </p>

            <Section title="DragModality">
                <DocTable headers=&["Variant", "When"]>
                    <TableRow>
                        <TableCell><Code inline=true>"Keyboard"</Code></TableCell>
                        <TableCell>"The last interaction was with the keyboard."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Touch"</Code></TableCell>
                        <TableCell>"Otherwise, on devices whose primary pointer is coarse (touch screens)."</TableCell>
                    </TableRow>
                    <TableRow>
                        <TableCell><Code inline=true>"Virtual"</Code></TableCell>
                        <TableCell>"Otherwise: mouse, pen and screen reader clicks."</TableCell>
                    </TableRow>
                </DocTable>
            </Section>

            <Section title="get_drag_modality">
                <p>
                    <Code inline=true>"get_drag_modality() -> DragModality"</Code>" returns the current modality without "
                    "tracking, for event handlers."
                </p>
            </Section>
        </Section>
    }
}
