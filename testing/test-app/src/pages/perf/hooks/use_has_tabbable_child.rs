//! `use_has_tabbable_child` on a group (`#perf-group`) of plain text (10 spans, or 1,000:
//! controls `use-10`, `use-1000`) and a tabbable button (`#perf-child`), for its performance test
//! (`leptonic/tests/perf_tests/hooks/use_has_tabbable_child.rs`): mounted at first, its result
//! as `data-has-tabbable-child`. Its controls ([`Control`]) mount and unmount it, add and remove
//! the button, disable and enable the button and the hook.

use leptonic::{
    IntoAttrs, flag,
    hooks::focus::{UseHasTabbableChildInput, use_has_tabbable_child},
};
use leptos::prelude::*;

use crate::pages::perf::{Control, PerfControls};

/// The group's content and the hook's input.
#[derive(Clone, Copy)]
struct Inputs {
    texts: RwSignal<usize>,
    child: RwSignal<bool>,
    child_disabled: RwSignal<bool>,
    disabled: RwSignal<bool>,
}

#[component]
pub fn PagePerf() -> impl IntoView {
    let mounted = RwSignal::new(true);
    let inputs = Inputs {
        texts: RwSignal::new(10),
        child: RwSignal::new(true),
        child_disabled: RwSignal::new(false),
        disabled: RwSignal::new(false),
    };
    let Inputs {
        texts,
        child,
        child_disabled,
        disabled,
    } = inputs;
    let controls = vec![
        Control::action("mount", move || mounted.set(true)),
        Control::action("unmount", move || mounted.set(false)),
        Control::action("use-10", move || texts.set(10)),
        Control::action("use-1000", move || texts.set(1000)),
        Control::action("add-child", move || child.set(true)),
        Control::action("remove-child", move || child.set(false)),
        Control::action("disable-child", move || child_disabled.set(true)),
        Control::action("enable-child", move || child_disabled.set(false)),
        Control::action("disable", move || disabled.set(true)),
        Control::action("enable", move || disabled.set(false)),
    ];
    view! {
        <h1>"use_has_tabbable_child performance"</h1>
        <PerfControls controls />
        <Show when=move || mounted.get()>
            <Group inputs />
        </Show>
    }
}

/// The group with `use_has_tabbable_child`.
#[component]
fn Group(inputs: Inputs) -> impl IntoView {
    let tabbable = use_has_tabbable_child(UseHasTabbableChildInput {
        is_disabled: inputs.disabled.into(),
    });
    view! {
        <div
            id="perf-group"
            {..tabbable.props.into_attrs()}
            data-has-tabbable-child=flag(tabbable.has_tabbable_child)
        >
            {move || {
                (0..inputs.texts.get())
                    .map(|i| view! { <span>{i}</span> })
                    .collect_view()
            }}
            <Show when=move || inputs.child.get()>
                <button id="perf-child" disabled=inputs.child_disabled>
                    "Tabbable"
                </button>
            </Show>
        </div>
    }
}
