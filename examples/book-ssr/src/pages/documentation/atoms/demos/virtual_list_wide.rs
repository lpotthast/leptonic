use leptonic::{
    atoms::virtualizer::{VirtualList, VirtualListOptions},
    hooks::{collections::Key, virtualizer::ItemSize},
};
use leptos::prelude::*;

/// A line of the access log.
#[derive(Clone)]
struct Request {
    number: usize,
    text: String,
}

/// The made-up request `number`: long lines that don't wrap.
fn request(number: usize) -> Request {
    let text = format!(
        "10.0.{}.{} - - [10/Oct/2026:13:{:02}:{:02} +0000] \"GET /api/v1/projects/{}/builds/{number}/artifacts?format=json&include=logs,metrics HTTP/1.1\" 200 {}",
        number % 8,
        number % 250,
        number / 60 % 60,
        number % 60,
        number % 12,
        1_024 + number * 37 % 9_000,
    );
    Request { number, text }
}

#[component]
pub fn VirtualListWideDemo() -> impl IntoView {
    let requests = Signal::stored((1..=500).map(request).collect::<Vec<_>>());

    view! {
        // The rows are measured once rendered (estimated size); the widest one sets how far the list scrolls across.
        <VirtualList
            items=requests
            key=|request: &Request| Key::from(request.number)
            layout_options=VirtualListOptions { row_size: ItemSize::Estimated(24.0), ..VirtualListOptions::default() }
            is_focusable=true
            classes="demo-virt-log"
            attr:role="region"
            attr:aria-label="Access log"
            let:request
        >
            // `width: max-content; min-width: 100%`: as wide as its text, at least as wide as the list.
            <div class="demo-virt-wide-line">{request.text}</div>
        </VirtualList>
    }
}
