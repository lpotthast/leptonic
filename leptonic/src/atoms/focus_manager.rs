use leptos::{context::Provider, prelude::*};

use crate::{
    hooks::*,
    utils::{classes::Classes, styles::Styles},
};

/// A container providing a [`FocusManager`] for the elements inside it (as a context, and to its
/// children function): moving focus with `focus_next`, `focus_first`, ... without containing it.
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
    let UseFocusManagerReturn {
        focus_manager,
        props,
    } = use_focus_manager(UseFocusManagerInput {});

    view! {
        <Provider value=focus_manager.clone()>
            <div {..props.into_attrs()} class=classes style=styles>
                {children(focus_manager)}
            </div>
        </Provider>
    }
}
