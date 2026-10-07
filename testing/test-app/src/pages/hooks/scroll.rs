use leptonic::utils::scroll::{
    ScrollAlignment, ScrollIntoViewOpts, get_scroll_parents, scroll_into_view,
};
use leptos::{html, prelude::*, web_sys};
use wasm_bindgen::JsCast;

/// The scroll utilities (react-aria's `getScrollParents.test.ts`, `scrollIntoView.test.ts`):
/// `#test-scroll-parents` lists the scroll parents of a child of a scrolling box and of a child of
/// a plain box; the scroll buttons give the root element a 100px border and scroll
/// `#test-scroll-target` to the start or end of the root's scroll port, then report the target's
/// distance from the viewport's top or bottom edge in `#test-scroll-offset` (`top: 0` and
/// `bottom: 0` when the root's border and scrollbar aren't counted).
#[component]
pub fn PageHookScroll() -> impl IntoView {
    let box_child = NodeRef::<html::Div>::new();
    let plain_child = NodeRef::<html::Div>::new();
    let target = NodeRef::<html::Div>::new();
    let parents = RwSignal::new(String::new());
    let offset = RwSignal::new(String::new());

    let describe = |element: &web_sys::Element| {
        if element.id().is_empty() {
            element.tag_name()
        } else {
            element.id()
        }
    };
    let list_parents = move |_| {
        let (Some(box_child), Some(plain_child)) =
            (box_child.get_untracked(), plain_child.get_untracked())
        else {
            return;
        };
        let list = |child: &web_sys::Element| {
            get_scroll_parents(child, false)
                .iter()
                .map(describe)
                .collect::<Vec<_>>()
                .join(",")
        };
        parents.set(format!("{} | {}", list(&box_child), list(&plain_child)));
    };

    let scroll_to = move |alignment: ScrollAlignment| {
        move |_| {
            let Some(target) = target.get_untracked() else {
                return;
            };
            let Some(document) = target.owner_document() else {
                return;
            };
            let Some(root) = document
                .scrolling_element()
                .and_then(|root| root.dyn_into::<web_sys::HtmlElement>().ok())
            else {
                return;
            };
            let _ = root.style().set_property("border", "100px solid gray");
            scroll_into_view(
                &root,
                &target,
                ScrollIntoViewOpts {
                    block: alignment,
                    inline: ScrollAlignment::Nearest,
                },
            );
            let rect = target.get_bounding_client_rect();
            let (edge, distance) = match alignment {
                ScrollAlignment::End => ("bottom", f64::from(root.client_height()) - rect.bottom()),
                _ => ("top", rect.top()),
            };
            // `+ 0.0` turns a rounded `-0` into `0`.
            offset.set(format!("{edge}: {}", distance.round() + 0.0));
        }
    };

    view! {
        <div id="test-page-hook-scroll">
            <h1>"Scroll utilities"</h1>
            <button id="test-scroll-list-parents" on:click=list_parents>
                "List scroll parents"
            </button>
            <button id="test-scroll-to-start" on:click=scroll_to(ScrollAlignment::Start)>
                "Scroll target to start"
            </button>
            <button id="test-scroll-to-end" on:click=scroll_to(ScrollAlignment::End)>
                "Scroll target to end"
            </button>
            <div>
                "Scroll parents: " <span id="test-scroll-parents">{move || parents.get()}</span>
            </div>
            <div>"Offset: " <span id="test-scroll-offset">{move || offset.get()}</span></div>
            <div id="test-scroll-box" style="overflow: auto; height: 60px;">
                <div node_ref=box_child style="height: 120px;">
                    "In a scrolling box"
                </div>
            </div>
            <div id="test-scroll-plain">
                <div node_ref=plain_child>"In a plain box"</div>
            </div>
            <div style="height: 3000px;"></div>
            <div id="test-scroll-target" node_ref=target style="height: 50px; width: 50px;">
                "Target"
            </div>
            <div style="height: 3000px;"></div>
        </div>
    }
}
