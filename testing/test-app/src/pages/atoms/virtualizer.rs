use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItems},
        virtualizer::Virtualizer,
    },
    hooks::{
        collections::{Key, use_list_collection},
        virtualizer::{ListLayout, ListLayoutOptions, ScrollAnchorEdge},
    },
    utils::styles::Styles,
};
use leptos::prelude::*;

/// Virtualized list boxes (react-aria-components' `ListBox.test.js` "should support virtualizer"):
/// - `#test-virt-list`: 50 items ("Item 0", ...), 25px rows, 100 × 100 px.
/// - `#test-virt-log`: a log of variable-height lines (every fifth is 60px, the others about 20px;
///   estimated 20px), 200px high, anchored to the end; `#test-virt-append` appends 10 lines.
/// - `#test-virt-plain`: a list box of 30 items after the `Virtualizer`s, not inside one: it
///   renders all of them (a `Virtualizer`'s context must not reach its siblings).
#[component]
pub fn PageAtomVirtualizer() -> impl IntoView {
    let items = use_list_collection(
        Signal::stored((0..50).collect::<Vec<usize>>()),
        |i| Key::from(format!("item-{i}")),
        |i| format!("Item {i}"),
    );
    let lines = RwSignal::new(100_usize);
    let log = use_list_collection(
        Signal::derive(move || (0..lines.get()).collect::<Vec<usize>>()),
        |i| Key::from(format!("line-{i}")),
        |i| format!("Line {i}"),
    );
    let plain = use_list_collection(
        Signal::stored((0..30).collect::<Vec<usize>>()),
        |i| Key::from(format!("plain-{i}")),
        |i| format!("Plain {i}"),
    );
    let plain_box = Styles::new()
        .add_unchecked("height", "100px")
        .add_unchecked("overflow", "auto");
    let square = Styles::new()
        .add_unchecked("width", "100px")
        .add_unchecked("height", "100px");
    let log_box = Styles::new()
        .add_unchecked("width", "300px")
        .add_unchecked("height", "200px");
    view! {
        <h1>"Virtualizer"</h1>
        <button id="test-virt-before">"Before"</button>
        <div id="test-virt-list">
            <Virtualizer layout=ListLayout::new(ListLayoutOptions {
                row_size: Some(25.0),
                ..ListLayoutOptions::default()
            })>
                <ListBox collection=items aria_label="Test" styles=square>
                    <ListBoxItems let:node>{node.text_value.to_string()}</ListBoxItems>
                </ListBox>
            </Virtualizer>
        </div>
        <div id="test-virt-log">
            <Virtualizer
                layout=ListLayout::new(ListLayoutOptions {
                    estimated_row_size: Some(20.0),
                    anchor_to: Some(ScrollAnchorEdge::End),
                    scroll_end_threshold: 10.0,
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
