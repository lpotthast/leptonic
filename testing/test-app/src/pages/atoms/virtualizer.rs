use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItems},
        virtualizer::Virtualizer,
    },
    hooks::{
        collections::{Key, SelectionMode, UseListCollectionInput, use_list_collection},
        virtualizer::{EndAnchor, ItemSize, ListLayout, ListLayoutOptions},
    },
    leptos_styles::Styles,
};
use leptos::prelude::*;

/// Virtualized list boxes (react-aria-components' `ListBox.test.js` "should support virtualizer"):
/// - `#test-virt-list`: 50 items ("Item 0", ...), 25px rows, 100 × 100 px, single selection;
///   between the buttons `#test-virt-before` and `#test-virt-after`; `#test-virt-hide` toggles
///   `display: none` on it.
/// - `#test-virt-log`: a log of variable-height lines (every fifth is 60px, the others about 20px;
///   estimated 20px), 200px high, anchored to the end; `#test-virt-append` appends 10 lines.
/// - `#test-virt-plain`: a list box of 30 items after the `Virtualizer`s, not inside one: it
///   renders all of them (a `Virtualizer`'s context must not reach its siblings).
#[component]
pub fn PageAtomVirtualizer() -> impl IntoView {
    let items = use_list_collection(UseListCollectionInput {
        items: Signal::stored((0..50).collect::<Vec<usize>>()),
        key: |i| Key::from(format!("item-{i}")),
        text_value: |i| format!("Item {i}"),
    });
    let lines = RwSignal::new(100_usize);
    let log = use_list_collection(UseListCollectionInput {
        items: Signal::derive(move || (0..lines.get()).collect::<Vec<usize>>()),
        key: |i| Key::from(format!("line-{i}")),
        text_value: |i| format!("Line {i}"),
    });
    let plain = use_list_collection(UseListCollectionInput {
        items: Signal::stored((0..30).collect::<Vec<usize>>()),
        key: |i| Key::from(format!("plain-{i}")),
        text_value: |i| format!("Plain {i}"),
    });
    let plain_box = Styles::new()
        .add_unchecked("height", "100px")
        .add_unchecked("overflow", "auto");
    let square = Styles::new()
        .add_unchecked("width", "100px")
        .add_unchecked("height", "100px");
    let hidden = RwSignal::new(false);
    let log_box = Styles::new()
        .add_unchecked("width", "300px")
        .add_unchecked("height", "200px");
    view! {
        <h1>"Virtualizer"</h1>
        <button id="test-virt-before">"Before"</button>
        <div id="test-virt-list" style:display=move || if hidden.get() { "none" } else { "block" }>
            <Virtualizer layout=ListLayout::new(ListLayoutOptions {
                row_size: ItemSize::Fixed(25.0),
                ..ListLayoutOptions::default()
            })>
                <ListBox
                    collection=items
                    aria_label="Test"
                    selection_mode=SelectionMode::Single
                    styles=square
                >
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </Virtualizer>
        </div>
        <button id="test-virt-after">"After"</button>
        <button id="test-virt-hide" on:click=move |_| hidden.update(|hidden| *hidden = !*hidden)>
            "Hide or show"
        </button>
        <div id="test-virt-log">
            <Virtualizer
                layout=ListLayout::new(ListLayoutOptions {
                    row_size: ItemSize::Estimated(20.0),
                    anchor_to_end: Some(EndAnchor { threshold: 10.0 }),
                    ..ListLayoutOptions::default()
                })
                should_observe_item_size=true
            >
                <ListBox collection=log aria_label="Log" styles=log_box>
                    <ListBoxItems let:node>
                        {
                            let tall = node.key.to_string().trim_start_matches("line-").parse::<usize>().unwrap_or(0) % 5 == 0;
                            let height = if tall { "60px" } else { "20px" };
                            view! { <div style:height=height>{node.text_value.to_string()}</div> }
                        }
                    </ListBoxItems>
                </ListBox>
            </Virtualizer>
        </div>
        <button id="test-virt-append" on:click=move |_| lines.update(|lines| *lines += 10)>"Append"</button>
        <div id="test-virt-plain">
            <ListBox collection=plain aria_label="Plain" styles=plain_box>
                <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
            </ListBox>
        </div>
    }
}
