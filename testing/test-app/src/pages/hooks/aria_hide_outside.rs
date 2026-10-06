use std::{cell::RefCell, collections::HashMap};

use leptonic::utils::{AriaHideOutsideOptions, aria_hide_outside};
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
#[component]
pub fn PageHookAriaHideOutside() -> impl IntoView {
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
    }
}
