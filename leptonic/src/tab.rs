use std::rc::Rc;

use leptos::leptos_dom::{Callable, Callback};
use leptos::*;
use uuid::Uuid;

use crate::tabs::TabsContext;
use crate::Mount;

#[derive(Clone)]
pub struct TabLabel {
    pub id: Uuid,
    pub name: leptos::Oco<'static, str>,
    pub label: Rc<View>,
}

// TODO: We might want to take only `Children` and hide them when the tab is not active...
#[component]
pub fn Tab(
    // TODO: Can / should we accept a String instead?
    #[prop(optional)] id: Option<Uuid>,
    /// Uniquely identifies this tab.
    #[prop(into)]
    name: Oco<'static, str>,
    #[prop(into)] label: View,
    #[prop(optional)] mount: Option<Mount>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(into, optional)] on_show: Option<Callback<()>>,
    #[prop(into, optional)] on_hide: Option<Callback<()>>,
) -> impl IntoView {
    let id = id.unwrap_or_else(Uuid::new_v4);
    let tabs = use_context::<TabsContext>().unwrap();

    let mount = mount.or(tabs.mount).unwrap_or(Mount::Once);

    let name = store_value(name);

    tabs.set_tab_labels.update(|labels| {
        labels.push(TabLabel {
            id,
            name: name.get_value(),
            label: Rc::new(label.into_view()),
        })
    });

    if tabs.history.get_untracked().get_active().is_none() {
        tabs.set_history.update(|history| {
            history.push(name.get_value());
        });
    }

    if let Some(on_show) = on_show {
        create_effect(move |_| {
            let history = tabs.history.get();
            let this = name.get_value();
            if history.get_active() == Some(&this) && history.get_previous() != Some(&this) {
                on_show.call(());
            }
        });
    }

    if let Some(on_hide) = on_hide {
        create_effect(move |_| {
            let history = tabs.history.get();
            let this = name.get_value();
            if history.get_active() != Some(&this) && history.get_previous() == Some(&this) {
                on_hide.call(());
            }
        });
    }

    let is_active = move || tabs.history.get().get_active() == Some(&name.get_value());

    match mount {
        Mount::Once => view! {
            {
                view! {
                    <leptonic-tab id=id.to_string() data-name=name.get_value() role="tabpanel" aria-hidden=move || if is_active() { "false" } else { "true"} >
                        {
                            if let Some(children) = &children {
                                children()
                            } else {
                                Fragment::new(vec![])
                            }
                        }
                    </leptonic-tab>
                }.into_view()
            }
        },
        Mount::WhenShown => view! {
            {
                view! {
                    <Show when=is_active fallback=|| ()>
                        <leptonic-tab id=id.to_string() data:name=name.get_value() role="tabpanel">
                            {
                                if let Some(children) = &children {
                                    children()
                                } else {
                                    Fragment::new(vec![])
                                }
                            }
                        </leptonic-tab>
                    </Show>
                }.into_view()
            }
        },
    }
}
