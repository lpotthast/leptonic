use leptonic::{
    ScrollAlignment, ScrollIntoViewOpts, atoms::button::Button, is_scrollable, scroll_into_view,
};
use leptos::{html, prelude::*, wasm_bindgen::JsCast};

const ITEMS: usize = 30;

#[component]
pub fn ScrollDemo() -> impl IntoView {
    let list = NodeRef::<html::Ul>::new();
    let target = RwSignal::new(None::<usize>);

    // Scrolls the list (and only the list, not the page) so that the item is visible, aligned to the list's center.
    let scroll_to = move |index: usize| {
        let Some(list) = list.get_untracked() else {
            return;
        };
        let Some(item) = list.children().item(u32::try_from(index).unwrap_or(0)) else {
            return;
        };
        let Ok(item) = item.dyn_into::<web_sys::HtmlElement>() else {
            return;
        };
        scroll_into_view(
            &list,
            &item,
            ScrollIntoViewOpts {
                block: ScrollAlignment::Center,
                inline: ScrollAlignment::Nearest,
            },
        );
        target.set(Some(index));
    };
    let scrollable =
        Signal::derive(move || list.get().is_some_and(|list| is_scrollable(&list, true)));

    view! {
        <ul node_ref=list class="demo-scroll-list" tabindex="0" aria-label="Items">
            {(1..=ITEMS).map(|n| view! { <li>{format!("Item {n}")}</li> }).collect_view()}
        </ul>
        <div class="demo-controls">
            <Button on_press=move |_| scroll_to(0) classes="demo-btn">"First"</Button>
            <Button on_press=move |_| scroll_to(ITEMS / 2) classes="demo-btn">"Middle"</Button>
            <Button on_press=move |_| scroll_to(ITEMS - 1) classes="demo-btn">"Last"</Button>
        </div>
        <p class="demo-status">
            {move || {
                let shown = target.get().map_or_else(|| "none yet".to_owned(), |index| format!("item {}", index + 1));
                format!("Scrolled to {shown}. The list is scrollable: {}.", scrollable.get())
            }}
        </p>
    }
}
