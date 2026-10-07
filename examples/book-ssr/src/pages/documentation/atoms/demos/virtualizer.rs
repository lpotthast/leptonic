use leptonic::{
    atoms::{
        listbox::{ListBox, ListBoxItemCtx, ListBoxItems},
        virtualizer::Virtualizer,
    },
    hooks::{
        SelectionMode,
        collections::{Key, Node, use_list_collection},
        virtualizer::{ListLayout, ListLayoutOptions},
    },
};
use leptos::prelude::*;

/// How many orders the list holds.
const ORDERS: usize = 10_000;

#[component]
pub fn VirtualizerDemo() -> impl IntoView {
    let orders = use_list_collection(
        Signal::stored((1..=ORDERS).collect::<Vec<usize>>()),
        |number| Key::from(*number),
        |number| format!("Order {number}"),
    );
    // What the demo shows below the list: how many options are rendered right now, and the option
    // that had focus last.
    let rendered = RwSignal::new(0_usize);
    let focused = RwSignal::new(None::<String>);

    view! {
        // Every row is 36px high: the layout knows where each of the 10,000 options goes without
        // rendering them.
        <Virtualizer layout=ListLayout::new(ListLayoutOptions {
            row_size: Some(36.0),
            ..ListLayoutOptions::default()
        })>
            // The listbox scrolls: its stylesheet gives it a fixed height.
            <ListBox
                collection=orders
                selection_mode=SelectionMode::Single
                aria_label="Orders"
                classes="demo-virt-listbox"
            >
                <ListBoxItems classes="demo-virt-option" let:node>
                    <OrderRow node=node rendered=rendered focused=focused/>
                </ListBoxItems>
            </ListBox>
        </Virtualizer>

        <p class="demo-status">
            {move || {
                let count = rendered.get();
                let options = if count == 1 { "option" } else { "options" };
                let focused = focused.get().unwrap_or_else(|| "none".to_owned());
                format!("{count} of {ORDERS} {options} rendered. Focused: {focused}.")
            }}
        </p>
    }
}

/// The content of an option: counts itself while it is rendered and reports when it has focus.
#[component]
fn OrderRow(
    node: Node,
    rendered: RwSignal<usize>,
    focused: RwSignal<Option<String>>,
) -> impl IntoView {
    rendered.update(|count| *count += 1);
    on_cleanup(move || {
        rendered.try_update(|count| *count -= 1);
    });

    let text = node.text_value.to_string();
    let item = expect_context::<ListBoxItemCtx>();
    let name = text.clone();
    Effect::new(move |_| {
        if item.is_focused.get() {
            focused.set(Some(name.clone()));
        }
    });

    text
}
