use std::rc::Rc;

use leptos::*;
use uuid::Uuid;

use crate::components::tabs::use_tabs;
use crate::Mount;

#[derive(Debug, Clone)]
pub struct TabData {
    pub id: Uuid,
    pub name: Oco<'static, str>,
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

    /// Called whenever the tab comes into view.
    #[prop(into, optional)]
    on_show: Option<Callback<()>>,

    /// Called whenever the tab gets hidden.
    #[prop(into, optional)]
    on_hide: Option<Callback<()>>,
) -> impl IntoView {
    let id = id.unwrap_or_else(Uuid::new_v4);
    let tabs = use_tabs();

    let mount = mount.or(tabs.default_mount_type).unwrap_or(Mount::Once);

    let name = store_value(name);

    tabs.register(TabData {
        id,
        name: name.get_value(),
        label: Rc::new(label.into_view()),
    });

    on_cleanup(move || {
        tabs.deregister(id);
    });

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
