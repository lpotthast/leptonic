// No upstream: a container providing a `FocusManager` without a `FocusScope` (react-aria:
// `createFocusManager(ref)` on an element of the app's own).
use leptos::{context::Provider, prelude::*};
use leptos_classes::Classes;

use crate::{
    IntoAttrs,
    hooks::focus::{CreateFocusManagerReturn, FocusManager, create_focus_manager},
    utils::{default_class::with_default_class, styles::Styles},
};

/// A container providing a [`FocusManager`] for the elements inside it (as a context, and to its
/// children function): moving focus with `focus_next`, `focus_first`, ... without containing it.
///
/// Default class: `leptonic-FocusManagerProvider`.
#[component]
pub fn FocusManagerProvider<C, V>(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: C,
) -> impl IntoView
where
    C: Fn(FocusManager) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let classes = with_default_class("leptonic-FocusManagerProvider", classes);
    let CreateFocusManagerReturn {
        focus_manager,
        props,
    } = create_focus_manager();

    view! {
        <Provider value=focus_manager>
            <div {..props.into_attrs()} class=classes style=styles>
                {children(focus_manager)}
            </div>
        </Provider>
    }
}
