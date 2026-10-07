use leptonic::{
    atoms::virtualizer::VirtualList,
    hooks::{
        IntoAttrs, UseFocusRingInput, collections::Key, use_focus_ring,
        virtualizer::ListLayoutOptions,
    },
    utils::styles::Styles,
};
use leptos::prelude::*;

/// A virtualized log (`VirtualList`):
/// - `#test-vl-log` (role log): 2,000 lines "Line 0", ..., every seventh wrapping over several
///   rows (estimated 20px), 200px high, following its end (`#test-vl-follow` shows and toggles
///   it).
/// - `#test-vl-append` appends 50 lines.
/// - `#test-vl-source` switches the source of `#test-vl-rebuilt`, a view rebuilt in place (same
///   type): a focusable `VirtualList` (`#test-vl-rebuilt-list`), an element with `use_focus_ring`'s
///   props (`#test-vl-rebuilt-plain`), and a component with them spread onto it
///   (`#test-vl-rebuilt-wrapper`, a known Leptos bug: the old listeners stay).
#[component]
pub fn PageAtomVirtualList() -> impl IntoView {
    let lines = RwSignal::new((0..2000).collect::<Vec<usize>>());
    let follow = RwSignal::new(true);
    let source = RwSignal::new(0_usize);
    let line_text = |i: usize| {
        if i.is_multiple_of(7) {
            format!("Line {i} {}", "long text that wraps ".repeat(12))
        } else {
            format!("Line {i}")
        }
    };
    view! {
        <h1>"VirtualList"</h1>
        <button id="test-vl-follow" on:click=move |_| follow.update(|f| *f = !*f)>
            {move || if follow.get() { "following" } else { "not following" }}
        </button>
        <button
            id="test-vl-append"
            on:click=move |_| {
                lines
                    .update(|lines| {
                        let start = lines.len();
                        lines.extend(start..start + 50);
                    });
            }
        >
            "Append"
        </button>
        <VirtualList
            items=lines
            key=|i: &usize| Key::from(format!("line-{i}"))
            layout_options=ListLayoutOptions {
                estimated_row_size: Some(20.0),
                scroll_end_threshold: 10.0,
                ..ListLayoutOptions::default()
            }
            should_observe_item_size=true
            is_anchored_to_end=follow
            set_anchored_to_end=follow
            styles=Styles::new().add_unchecked("width", "300px").add_unchecked("height", "200px")
            attr:id="test-vl-log"
            attr:role="log"
            attr:aria-label="Log"
            attr:tabindex="0"
            let:i
        >
            <div class="line" style="font-family: monospace; white-space: pre-wrap; overflow-wrap: anywhere">
                {line_text(i)}
            </div>
        </VirtualList>
        <button id="test-vl-source" on:click=move |_| source.update(|s| *s += 1)>"Switch source"</button>
        <div id="test-vl-rebuilt">
            {move || {
                let source = source.get();
                view! { <RebuiltLogs source=source /> }.into_any()
            }}
        </div>
    }
}

/// A view of a source, rebuilt in place when the source changes.
#[component]
fn RebuiltLogs(source: usize) -> impl IntoView {
    let lines = Signal::stored(
        (0..20)
            .map(|i| format!("Source {source} line {i}"))
            .collect::<Vec<_>>(),
    );
    let plain_ring = use_focus_ring(UseFocusRingInput::default());
    let wrapper_ring = use_focus_ring(UseFocusRingInput::default());
    view! {
        <VirtualList
            items=lines
            key=|line: &String| Key::from(line.clone())
            layout_options=ListLayoutOptions {
                row_size: Some(20.0),
                ..ListLayoutOptions::default()
            }
            styles=Styles::new().add_unchecked("width", "200px").add_unchecked("height", "100px")
            attr:id="test-vl-rebuilt-list"
            is_focusable=true
            let:line
        >
            <div class="line">{line}</div>
        </VirtualList>
        <PlainWrapper attr:id="test-vl-rebuilt-wrapper" attr:tabindex="0" {..wrapper_ring.props.into_attrs()} />
        <div id="test-vl-rebuilt-plain" tabindex="0" {..plain_ring.props.into_attrs()}>
            {format!("Plain {source}")}
        </div>
    }
}

/// A component rendering one element, for attributes spread onto a component.
#[component]
fn PlainWrapper() -> impl IntoView {
    view! { <div>"Wrapper"</div> }
}
