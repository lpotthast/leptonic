use std::sync::Arc;

use leptos::*;
use tracing::info;
use uuid::Uuid;

use crate::tabs::TabsContext;
use crate::Mount;

#[derive(Clone)]
pub struct TabLabel {
    pub id: Uuid,
    pub name: &'static str,
    pub label: Arc<View>,
}

// TODO: We might want to take only `Children` and hide them when the tab is not active...
#[component]
pub fn Tab<L>(
    cx: Scope,
    #[prop(optional)] id: Option<Uuid>,
    /// Uniquely identifies this tab.
    name: &'static str,
    label: L,
    #[prop(optional)] mount: Option<Mount>,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional)] on_show: Option<fn()>,
    #[prop(optional)] on_hide: Option<fn()>,
) -> impl IntoView
where
    L: IntoView + 'static,
{
    let id = id.unwrap_or_else(|| Uuid::new_v4());
    let tabs = use_context::<TabsContext>(cx).unwrap();

    let mount = mount.or(tabs.mount).unwrap_or(Mount::Once);

    let tab_label = TabLabel {
        id,
        name,
        label: Arc::new(label.into_view(cx)),
    };

    tabs.set_tab_labels.update(|labels| labels.push(tab_label));

    if tabs.history.get().get_active() == None {
        tabs.set_history.update(|history| {
            history.push(name);
        });
    }

    if let Some(on_show) = on_show {
        create_effect(cx, move |_| {
            let history = tabs.history.get();
            if history.get_active() == Some(name) && history.get_previous() != Some(name) {
                on_show();
            }
        });
    }

    if let Some(on_hide) = on_hide {
        create_effect(cx, move |_| {
            let history = tabs.history.get();
            if history.get_active() != Some(name) && history.get_previous() == Some(name) {
                on_hide();
            }
        });
    }

    let is_active = move || tabs.history.get().get_active() == Some(name);

    on_cleanup(cx, move || {
        info!("cleanup tab");
    });

    match mount {
        Mount::Once | Mount::OnceShown => view! { cx,
            {
                view! { cx,
                    <leptonic-tab id=id.to_string() data-name=name aria-hidden=move || if is_active() { "false" } else { "true"} >
                        {
                            if let Some(children) = &children {
                                children(cx)
                            } else {
                                Fragment::new(vec![])
                            }
                        }
                    </leptonic-tab>
                }.into_view(cx)
            }
        },
        Mount::WhenShown => view! { cx,
            {
                view! { cx,
                    <Show when=is_active fallback=|_cx| view! { cx,  }>
                        <leptonic-tab id=id.to_string() data:name=name>
                            {
                                if let Some(children) = &children {
                                    children(cx)
                                } else {
                                    Fragment::new(vec![])
                                }
                            }
                        </leptonic-tab>
                    </Show>
                }.into_view(cx)
            }
        },
    }
}
