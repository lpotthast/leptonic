use std::{cell::RefCell, collections::HashMap, time::Duration};

use leptonic::utils::{AriaHideOutsideOptions, aria_hide_outside, keep_visible};
use leptos::{prelude::*, web_sys};

type Reverts = HashMap<&'static str, Box<dyn FnOnce()>>;

thread_local! {
    /// The revert functions of the active hides, by case (only touched by click handlers, in the
    /// browser).
    static REVERTS: RefCell<Reverts> = RefCell::new(HashMap::new());
}

fn element(id: &str) -> web_sys::Element {
    document()
        .get_element_by_id(id)
        .unwrap_or_else(|| panic!("no element #{id}"))
}

fn hide(case: &'static str, targets: &[&str], root: &str) {
    let targets: Vec<_> = targets.iter().map(|id| element(id)).collect();
    let revert = aria_hide_outside(
        &targets,
        AriaHideOutsideOptions {
            root: Some(element(root)),
            ..AriaHideOutsideOptions::default()
        },
    );
    REVERTS.with_borrow_mut(|reverts| reverts.insert(case, revert));
}

/// Appends an overlay (`#test-aho-late-<name>`) in a portal container to the late root, lets the
/// active hide's observer hide it, then registers it as an overlay opened from inside: with
/// `keep_visible` (non-modal) or a nested hide (modal).
fn open_late_overlay(name: &'static str, nested: bool) {
    let document = document();
    let Ok(portal) = document.create_element("div") else {
        return;
    };
    let Ok(overlay) = document.create_element("button") else {
        return;
    };
    overlay.set_id(&format!("test-aho-late-{name}"));
    overlay.set_text_content(Some(name));
    let _ = portal.append_child(&overlay);
    portal.set_id(&format!("test-aho-late-{name}-portal"));
    let _ = element("test-aho-late").append_child(&portal);
    // After the observer's callback (a microtask): Leptos effects may run that late.
    set_timeout(
        move || {
            let overlay = element(&format!("test-aho-late-{name}"));
            let revert = if nested {
                Some(aria_hide_outside(
                    &[overlay],
                    AriaHideOutsideOptions {
                        root: Some(element("test-aho-late")),
                        ..AriaHideOutsideOptions::default()
                    },
                ))
            } else {
                keep_visible(&overlay)
            };
            if let Some(revert) = revert {
                REVERTS.with_borrow_mut(|reverts| reverts.insert(name, revert));
            }
        },
        Duration::from_millis(50),
    );
}

/// Creates an element with an id (and optional text and attribute).
fn create(tag: &str, id: &str) -> web_sys::Element {
    let element = document()
        .create_element(tag)
        .unwrap_or_else(|_| panic!("can't create a <{tag}>"));
    if !id.is_empty() {
        element.set_id(id);
    }
    element
}

fn checkbox(id: &str) -> web_sys::Element {
    let input = create("input", id);
    let _ = input.set_attribute("type", "checkbox");
    input
}

/// The mutation observer cases (react-aria's `ariaHideOutside.test.js` "should handle when a new
/// element is added ..."), all while `#test-aho-mo-target` is the hide's target under
/// `#test-aho-mo`.
fn mutate(case: &str) {
    let root = element("test-aho-mo");
    match case {
        // A new element outside the target.
        "outside" => {
            let _ = root.append_child(&checkbox("test-aho-mo-outside"));
        }
        // A new element in a container that is hidden already: the container stays hidden, the
        // new element isn't marked itself.
        "in-hidden" => {
            let _ =
                element("test-aho-mo-container").append_child(&checkbox("test-aho-mo-in-hidden"));
        }
        // A new element inside the target stays visible.
        "inside" => {
            let radio = create("input", "test-aho-mo-inside");
            let _ = radio.set_attribute("type", "radio");
            let _ = element("test-aho-mo-target").append_child(&radio);
        }
        // A top-layer element added with a checkbox next to it: the top-layer element stays
        // visible, the checkbox is hidden.
        "top-layer" => {
            let wrapper = create("div", "test-aho-mo-top-wrapper");
            let top = create("div", "test-aho-mo-top");
            let _ = top.set_attribute("role", "alert");
            let _ = top.set_attribute("data-leptonic-top-layer", "");
            top.set_text_content(Some("Top layer"));
            let _ = wrapper.append_child(&top);
            let _ = wrapper.append_child(&checkbox("test-aho-mo-top-checkbox"));
            let _ = root.append_child(&wrapper);
        }
        // An element added to a parent that is removed, then moved to a new parent in `into`: the
        // mutation record lists a disconnected parent with a connected child.
        "reparent-target" | "reparent-hidden" => {
            let (into, id) = if case == "reparent-target" {
                ("test-aho-mo-target", "test-aho-mo-li-target")
            } else {
                ("test-aho-mo-container", "test-aho-mo-li-hidden")
            };
            let into = element(into);
            let parent = create("ul", "");
            let child = create("li", id);
            child.set_text_content(Some("Item"));
            let _ = into.append_child(&parent);
            let _ = parent.append_child(&child);
            parent.remove();
            let new_parent = create("ul", &format!("{id}-list"));
            let _ = new_parent.append_child(&child);
            let _ = into.append_child(&new_parent);
        }
        _ => {}
    }
}

fn revert(case: &'static str) {
    if let Some(revert) = REVERTS.with_borrow_mut(|reverts| reverts.remove(case)) {
        revert();
    }
}

/// `aria_hide_outside` (react-aria's `ariaHideOutside.test.js` setups), each case under its own
/// root, driven by the buttons above them (`#test-aho-<action>`):
/// - basic: a checkbox, a wrapped checkbox, the target button and an author-hidden checkbox.
/// - row: a grid; the target is the second row.
/// - nested: two hides (`[button, radios]`, then `[button]`), reverted in either order.
/// - outer root: a root not containing the target is hidden itself.
/// - late: overlays inserted while a hide is active and registered only after its observer hid
///   them (`#test-aho-late-open-popover`: `keep_visible`, `#test-aho-late-open-modal`: nested).
/// - mutations (`#test-aho-mo`): elements added while the hide is active, by
///   `#test-aho-mo-<case>` (see `mutate`).
/// - reorder (`#test-aho-reorder`): keyed rows (`#test-aho-row-a`..) reordered by
///   `#test-aho-reorder-button` (the hide's target) while hidden.
#[component]
pub fn PageHookAriaHideOutside() -> impl IntoView {
    let order = RwSignal::new(vec!["a", "b", "c", "d"]);
    let reorder = move |_| {
        order.update(|order| {
            let first = order.remove(0);
            order.insert(1, first);
        });
    };
    view! {
        <div>
            <button id="test-aho-hide-basic" on:click=|_| hide("basic", &["test-aho-target"], "test-aho-basic")>
                "Hide basic"
            </button>
            <button id="test-aho-revert-basic" on:click=|_| revert("basic")>"Revert basic"</button>
            <button id="test-aho-hide-row" on:click=|_| hide("row", &["test-aho-row-2"], "test-aho-grid")>
                "Hide row"
            </button>
            <button id="test-aho-revert-row" on:click=|_| revert("row")>"Revert row"</button>
            <button
                id="test-aho-hide-nested-1"
                on:click=|_| hide("nested-1", &["test-aho-n-button", "test-aho-n-r1", "test-aho-n-r2"], "test-aho-nested")
            >
                "Hide nested 1"
            </button>
            <button id="test-aho-hide-nested-2" on:click=|_| hide("nested-2", &["test-aho-n-button"], "test-aho-nested")>
                "Hide nested 2"
            </button>
            <button id="test-aho-revert-nested-1" on:click=|_| revert("nested-1")>"Revert nested 1"</button>
            <button id="test-aho-revert-nested-2" on:click=|_| revert("nested-2")>"Revert nested 2"</button>
            <button id="test-aho-hide-outer" on:click=|_| hide("outer", &["test-aho-target"], "test-aho-outer-root")>
                "Hide outer"
            </button>
            <button id="test-aho-revert-outer" on:click=|_| revert("outer")>"Revert outer"</button>
            <button id="test-aho-hide-late" on:click=|_| hide("late", &["test-aho-late-dialog"], "test-aho-late")>
                "Hide late"
            </button>
            <button id="test-aho-late-open-popover" on:click=|_| open_late_overlay("popover", false)>
                "Open popover"
            </button>
            <button id="test-aho-late-open-modal" on:click=|_| open_late_overlay("modal", true)>
                "Open modal"
            </button>
            <button id="test-aho-revert-late" on:click=|_| { revert("modal"); revert("popover"); revert("late"); }>
                "Revert late"
            </button>
            <button id="test-aho-hide-mo" on:click=|_| hide("mo", &["test-aho-mo-target"], "test-aho-mo")>
                "Hide mutations"
            </button>
            <button id="test-aho-revert-mo" on:click=|_| revert("mo")>"Revert mutations"</button>
            <button id="test-aho-mo-outside-button" on:click=|_| mutate("outside")>"Add outside"</button>
            <button id="test-aho-mo-in-hidden-button" on:click=|_| mutate("in-hidden")>
                "Add to hidden container"
            </button>
            <button id="test-aho-mo-inside-button" on:click=|_| mutate("inside")>"Add inside"</button>
            <button id="test-aho-mo-top-layer-button" on:click=|_| mutate("top-layer")>
                "Add top layer"
            </button>
            <button id="test-aho-mo-reparent-target-button" on:click=|_| mutate("reparent-target")>
                "Reparent into target"
            </button>
            <button id="test-aho-mo-reparent-hidden-button" on:click=|_| mutate("reparent-hidden")>
                "Reparent into hidden"
            </button>
            <button
                id="test-aho-hide-reorder"
                on:click=|_| hide("reorder", &["test-aho-reorder-button"], "test-aho-reorder")
            >
                "Hide reorder"
            </button>
            <button id="test-aho-revert-reorder" on:click=|_| revert("reorder")>"Revert reorder"</button>
        </div>

        <div id="test-aho-basic">
            <input type="checkbox" id="test-aho-c1" />
            <div id="test-aho-wrap">
                <input type="checkbox" id="test-aho-c2" />
            </div>
            <button id="test-aho-target">"Target"</button>
            <input type="checkbox" id="test-aho-author" aria-hidden="true" />
        </div>

        <div id="test-aho-grid" role="grid">
            <div id="test-aho-row-1" role="row">
                <div id="test-aho-cell-1" role="gridcell">
                    <span id="test-aho-span">"Cell 1"</span>
                </div>
            </div>
            <div id="test-aho-row-2" role="row">
                <div id="test-aho-cell-2" role="gridcell">"Cell 2"</div>
            </div>
        </div>

        <div id="test-aho-nested">
            <input type="checkbox" id="test-aho-n-c1" />
            <input type="radio" id="test-aho-n-r1" />
            <button id="test-aho-n-button">"Button"</button>
            <input type="radio" id="test-aho-n-r2" />
            <input type="checkbox" id="test-aho-n-c2" />
        </div>

        <div id="test-aho-outer-root">
            <span>"Outside the target"</span>
        </div>

        <div id="test-aho-mo">
            <div id="test-aho-mo-container"></div>
            <div id="test-aho-mo-target">
                <button>"Target"</button>
            </div>
        </div>

        <div id="test-aho-reorder">
            <button id="test-aho-reorder-button" on:click=reorder>"Reorder"</button>
            <For each=move || order.get() key=|row| *row let:row>
                <div role="presentation">
                    <div id=format!("test-aho-row-{row}") role="row">
                        <div role="gridcell"></div>
                    </div>
                </div>
            </For>
        </div>

        <div id="test-aho-late">
            <span id="test-aho-late-outside">"Outside the dialog"</span>
            <div id="test-aho-late-dialog">"Dialog"</div>
        </div>
    }
}
