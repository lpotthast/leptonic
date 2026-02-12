use crate::hooks::*;
use crate::utils::classes::Classes;
use crate::utils::styles::Styles;
use leptos::context::Provider;
use leptos::prelude::*;

// TODO: add documentation page
#[component]
pub fn FocusManager<C, V>(
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
