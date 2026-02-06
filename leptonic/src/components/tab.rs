use std::fmt::Debug;
use std::sync::Arc;

use leptos::prelude::*;
use uuid::Uuid;

use crate::components::tabs::use_tabs;
use crate::{Mount, Out};

#[derive(Clone)]
pub struct TabData {
    pub id: Uuid,
    pub name: Oco<'static, str>,
    pub label: ViewFn,
}

impl Debug for TabData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TabData")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("label", &"... (ViewFn)")
            .finish()
    }
}

// TODO: We might want to take only `Children` and hide them when the tab is not active...
#[component]
pub fn Tab(
    // TODO: Can / should we accept a String instead?
    #[prop(optional)] id: Option<Uuid>,

    /// Uniquely identifies this tab.
    #[prop(into)]
    name: Oco<'static, str>,

    #[prop(into)] label: ViewFn,

    #[prop(optional)] mount: Option<Mount>,

    #[prop(optional, default = Arc::new(|| ().into_any()))] children: ChildrenFn,

    /// Called whenever the tab comes into view.
    #[prop(into, optional)]
    on_show: Option<Out<()>>,

    /// Called whenever the tab gets hidden.
    #[prop(into, optional)]
    on_hide: Option<Out<()>>,
) -> impl IntoView {
    let id = id.unwrap_or_else(Uuid::new_v4);
    let tabs = use_tabs();

    let mount = mount.or(tabs.default_mount_type).unwrap_or(Mount::Once);

    let name = StoredValue::new(name);

    tabs.register(TabData {
        id,
        name: name.get_value(),
        label,
    });

    on_cleanup(move || {
        tabs.deregister(id);
    });

    if let Some(on_show) = on_show {
        Effect::new(move |_| {
            let history = tabs.history.get();
            let this = name.get_value();
            if history.get_active() == Some(&this) && history.get_previous() != Some(&this) {
                on_show.set(());
            }
        });
    }

    if let Some(on_hide) = on_hide {
        Effect::new(move |_| {
            let history = tabs.history.get();
            let this = name.get_value();
            if history.get_active() != Some(&this) && history.get_previous() == Some(&this) {
                on_hide.set(());
            }
        });
    }

    let is_active = move || tabs.history.get().get_active() == Some(&name.get_value());

    match mount {
        Mount::Once => view! {
            <leptonic-tab id=id.to_string() data-name=name.get_value() role="tabpanel" aria-hidden=move || if is_active() { "false" } else { "true"} >
                { children() }
            </leptonic-tab>
        }.into_any(),
        Mount::WhenShown => view! {
            <Show when=is_active fallback=|| ()>
                <leptonic-tab id=id.to_string() data:name=name.get_value() role="tabpanel">
                    { children() }
                </leptonic-tab>
            </Show>
        }.into_any(),
    }
}
