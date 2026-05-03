use indoc::indoc;
use leptonic::components::prelude::*;
use leptos::prelude::*;

use crate::pages::documentation::{article::Article, demo_shell::DemoShell, toc::Toc};

use super::demos::{dnd_drag_to_drop, dnd_draggable, dnd_droppable, dnd_reorder};

#[component]
pub fn PageUseDnd() -> impl IntoView {
    view! {
        <Article>
            <h1 id="dnd" class="anchor">
                "Drag and Drop Hooks"
                <AnchorLink href="#dnd" description="Direct link to article header"/>
            </h1>

            <p>
                "Hooks for creating accessible drag and drop interactions with full keyboard support and ARIA announcements. "
                "See the "<Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link>" for domain guidance."
            </p>

            <p>
                "Based on react-aria\u{2019}s "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useDrag.html" target=LinkTarget::_Blank>
                    "useDrag"
                </LinkExt>
                " and "
                <LinkExt href="https://react-spectrum.adobe.com/react-aria/useDrop.html" target=LinkTarget::_Blank>
                    "useDrop"
                </LinkExt>
                "."
            </p>

            <h2 id="use_draggable" class="anchor">
                "use_draggable"
                <AnchorLink href="#use_draggable" description="Direct link to use_draggable"/>
            </h2>

            <p>"Makes an element draggable with proper accessibility attributes."</p>

            <DemoShell source=include_str!("demos/dnd_draggable.rs")>
                <dnd_draggable::DraggableDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let draggable = use_draggable(UseDraggableInput {
                        get_items: Callback::new(|_| vec![DragItem::text("Hello")]),
                        on_drag_start: Some(Callback::new(|e: DragStartEvent| {
                            // Handle drag start
                        })),
                        on_drag_end: Some(Callback::new(|e: DragEndEvent| {
                            // Handle drag end
                        })),
                        ..Default::default()
                    });

                    view! {
                        <div {..draggable.drag_props.into_attrs()}>"Drag me"</div>
                    }
                "#)}
            </Code>

            <h3>"Return Value"</h3>
            <ul>
                <li><code>"drag_props"</code>" - Props to spread on the draggable element"</li>
                <li><code>"draggable_id"</code>" - Unique ID for the draggable element"</li>
                <li><code>"is_dragging"</code>" - Signal indicating if this element is being dragged"</li>
            </ul>

            <h2 id="use_droppable" class="anchor">
                "use_droppable"
                <AnchorLink href="#use_droppable" description="Direct link to use_droppable"/>
            </h2>

            <p>"Makes an element a drop target that can accept dragged items."</p>

            <DemoShell source=include_str!("demos/dnd_droppable.rs")>
                <dnd_droppable::DroppableDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    let droppable = use_droppable(UseDroppableInput {
                        accepted_types: vec!["text/plain".to_string()],
                        on_drop: Some(Callback::new(|e: DropEvent| {
                            for item in e.items {
                                // Handle dropped item
                            }
                        })),
                        ..Default::default()
                    });

                    view! {
                        <div
                            {..droppable.drop_props.into_attrs()}
                            class:drop-target=move || droppable.is_drop_target.get()
                        >
                            "Drop here"
                        </div>
                    }
                "#)}
            </Code>

            <h3>"Return Value"</h3>
            <ul>
                <li><code>"drop_props"</code>" - Props to spread on the drop zone element"</li>
                <li><code>"droppable_id"</code>" - Unique ID for the drop zone"</li>
                <li><code>"is_drop_target"</code>" - Signal indicating if something is being dragged over this zone"</li>
            </ul>

            <h2 id="drag-to-drop" class="anchor">
                "Drag to Drop"
                <AnchorLink href="#drag-to-drop" description="Direct link to drag to drop"/>
            </h2>

            <p>"Combine use_draggable and use_droppable to create drag-and-drop interactions."</p>

            <DemoShell source=include_str!("demos/dnd_drag_to_drop.rs")>
                <dnd_drag_to_drop::DragToDropDemo />
            </DemoShell>

            <h2 id="collection-reorder" class="anchor">
                "Collection Reordering"
                <AnchorLink href="#collection-reorder" description="Direct link to collection reordering"/>
            </h2>

            <p>
                "For sortable lists, use the collection API: "
                <code>"use_draggable_collection_state"</code>", "
                <code>"use_droppable_collection_state"</code>", "
                <code>"use_droppable_collection"</code>", "
                <code>"use_draggable_collection_item"</code>", "
                <code>"use_collection_droppable_item"</code>", and "
                <code>"use_drop_indicator"</code>
                ". This provides cursor-position-aware drop indicators and keyboard navigation."
            </p>

            <DemoShell source=include_str!("demos/dnd_reorder.rs")>
                <dnd_reorder::ReorderDemo />
            </DemoShell>

            <Code language=Language::Rust>
                {indoc!(r#"
                    // 1. Create collection state
                    let keys = Signal::derive(move || items.get().iter().map(|(k, _)| k.clone()).collect());
                    let drag_state = use_draggable_collection_state(DraggableCollectionStateInput {
                        collection_keys: keys,
                        get_items: Callback::new(|keys: Vec<String>| keys.into_iter().map(DragItem::text).collect()),
                        ..  // dummy selection for simple reorder
                    });
                    let drop_state = use_droppable_collection_state(DroppableCollectionStateInput {
                        collection_keys: keys,
                        on_reorder: Some(Callback::new(|e: CollectionReorderEvent| { /* reorder items */ })),
                        ..Default::default()
                    });

                    // 2. Set up the collection container
                    let collection_ref = NodeRef::<html::Div>::new();
                    let collection = use_droppable_collection(UseDroppableCollectionInput {
                        state: drop_state.clone(),
                        keyboard_delegate: Box::new(ListKeyboardDelegate::new(keys)),
                        drop_target_delegate: Box::new(/* DropTargetDelegate impl */),
                        is_disabled: Signal::derive(|| false),
                        accepted_types: vec!["text/plain".to_string()],
                    });

                    // 3. Per item: drag + drop + data-key + drop indicators
                    let drag = use_draggable_collection_item(UseDraggableCollectionItemInput { key, state: drag_state, .. });
                    let drop_item = use_collection_droppable_item(UseCollectionDroppableItemInput { target, state: drop_state, .. });
                    view! { <div {..drag.drag_props.into_attrs()} {..drop_item.drop_item_props.into_attrs()} data-key=key>...</div> }
                "#)}
            </Code>

            <h3>"Collection Callbacks"</h3>
            <ul>
                <li><code>"on_reorder"</code>" - Called when items are reordered within the same collection"</li>
                <li><code>"on_insert"</code>" - Called when external items are inserted"</li>
                <li><code>"on_root_drop"</code>" - Called when items are dropped on the collection root"</li>
                <li><code>"on_item_drop"</code>" - Called when items are dropped on a specific item"</li>
            </ul>

            <h2 id="drop-positions" class="anchor">
                "Drop Positions"
                <AnchorLink href="#drop-positions" description="Direct link to drop positions"/>
            </h2>

            <p>"When reordering, the drop target includes position information:"</p>

            <ul>
                <li><code>"DropPosition::Before"</code>" - Insert before the target item"</li>
                <li><code>"DropPosition::After"</code>" - Insert after the target item"</li>
                <li><code>"DropPosition::On"</code>" - Drop onto the target item (for nested structures like trees)"</li>
            </ul>

            <h2 id="drop-effects" class="anchor">
                "Drop Effects"
                <AnchorLink href="#drop-effects" description="Direct link to drop effects"/>
            </h2>

            <ul>
                <li><code>"DropEffect::Copy"</code>" - Create a copy of the dragged item"</li>
                <li><code>"DropEffect::Move"</code>" - Move the item (remove from source)"</li>
                <li><code>"DropEffect::Link"</code>" - Create a link/reference"</li>
                <li><code>"DropEffect::All"</code>" - Allow all effects"</li>
                <li><code>"DropEffect::None"</code>" - Disallow dropping"</li>
            </ul>

            <h2 id="drag-item" class="anchor">
                "DragItem"
                <AnchorLink href="#drag-item" description="Direct link to DragItem"/>
            </h2>

            <p>"Data that can be transferred during a drag operation:"</p>

            <Code language=Language::Rust>
                {indoc!(r#"
                    // Plain text
                    DragItem::text("Hello, World!")

                    // JSON data
                    DragItem::json("{\"id\": 1, \"name\": \"Item\"}")

                    // Custom MIME type
                    DragItem::custom("application/x-my-type", "custom data")
                "#)}
            </Code>

            <h2 id="features" class="anchor">
                "Features"
                <AnchorLink href="#features" description="Direct link to features"/>
            </h2>

            <ul>
                <li>"Draggable items with custom data (text, JSON, or custom MIME types)"</li>
                <li>"Drop zones with type filtering"</li>
                <li>"Collection reordering support"</li>
                <li>"Insert/remove between collections"</li>
                <li>"Visual feedback via is_dragging and is_drop_target signals"</li>
                <li>"ARIA attributes for accessibility (aria-grabbed, aria-dropeffect)"</li>
            </ul>

            <h2 id="see-also" class="anchor">
                "See Also"
                <AnchorLink href="#see-also" description="Direct link to section: See Also"/>
            </h2>

            <ul>
                <li><Link href=crate::routes::doc::Interactions.materialize()>"Interactions overview"</Link></li>
                <li><Link href=crate::routes::doc::interactions::UseMove.materialize()>"use_move"</Link>" \u{2014} for pointer-based movement without data transfer"</li>
            </ul>
        </Article>

        <Toc toc=Toc::List {
            inner: vec![
                Toc::Leaf { title: "Drag and Drop", link: "#dnd" },
                Toc::Leaf { title: "use_draggable", link: "#use_draggable" },
                Toc::Leaf { title: "use_droppable", link: "#use_droppable" },
                Toc::Leaf { title: "Drag to Drop", link: "#drag-to-drop" },
                Toc::Leaf { title: "Collection Reordering", link: "#collection-reorder" },
                Toc::Leaf { title: "Drop Positions", link: "#drop-positions" },
                Toc::Leaf { title: "Drop Effects", link: "#drop-effects" },
                Toc::Leaf { title: "DragItem", link: "#drag-item" },
                Toc::Leaf { title: "Features", link: "#features" },
                Toc::Leaf { title: "See Also", link: "#see-also" },
            ]
        }/>
    }
}
