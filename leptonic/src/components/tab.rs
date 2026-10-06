use std::{fmt::Debug, sync::Arc};

use leptos::prelude::*;

use crate::{
    Mount, Out,
    components::tabs::use_tabs,
    utils::{classes::Classes, styles::Styles},
};

#[derive(Clone)]
pub struct TabData {
    pub id: Oco<'static, str>,
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
    /// The tab panel's id. Default: a generated, hydration-stable one.
    #[prop(into, optional)]
    id: Option<Oco<'static, str>>,

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

    #[prop(into, optional)] classes: Classes,

    #[prop(into, optional)] styles: Styles,
) -> impl IntoView {
    let id = id.unwrap_or_else(|| crate::utils::id::use_id("tab").into());
    let tabs = use_tabs();

    let mount = mount.or(tabs.default_mount_type).unwrap_or(Mount::Once);

    let name = StoredValue::new(name);

    tabs.register(TabData {
        id: id.clone(),
        name: name.get_value(),
        label,
    });

    let id = StoredValue::new(id);
    on_cleanup(move || {
        if let Some(id) = id.try_get_value() {
            tabs.deregister(&id);
        }
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
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);

    match mount {
        Mount::Once => view! {
            <div
                class=classes.get_value().add("leptonic-tab")
                style=styles.get_value()
                id=id.get_value()
                data-name=name.get_value()
                role="tabpanel"
                aria-hidden=move || if is_active() { "false" } else { "true" }
            >
                {children()}
            </div>
        }
        .into_any(),
        Mount::WhenShown => view! {
            <Show when=is_active fallback=|| ()>
                <div
                    class=classes.get_value().add("leptonic-tab")
                    style=styles.get_value()
                    id=id.get_value()
                    data-name=name.get_value()
                    role="tabpanel"
                >
                    {children()}
                </div>
            </Show>
        }
        .into_any(),
    }
}
