use std::sync::Arc;

use leptos::*;
use tracing::info;
use uuid::Uuid;

use crate::tabs::TabsContext;

#[derive(Clone)]
pub struct TabLabel {
    pub id: Uuid,
    pub name: &'static str,
    pub label: Arc<View>,
}

// TODO: We might want to take only Children and hide them when the tab is not active...
#[component]
pub fn Tab<L>(
    cx: Scope,
    #[prop(optional)] id: Option<Uuid>,
    /// Uniquely identifies this tab.
    name: &'static str,
    label: L,
    #[prop(optional)] children: Option<ChildrenFn>,
    #[prop(optional_no_strip)] on_hide: Option<WriteSignal<()>>,
) -> impl IntoView
where
    L: IntoView + 'static,
{
    let id = id.unwrap_or_else(|| Uuid::new_v4());
    let tabs = use_context::<TabsContext>(cx).unwrap();

    let tab_label = TabLabel {
        id,
        name,
        label: Arc::new(label.into_view(cx)),
    };

    tabs.set_tab_labels.update(|labels| labels.push(tab_label));
    tabs.set_active_tab.update(|active| {
        if (*active).is_none() {
            *active = Some(name);
        }
    });

    let is_active = move || tabs.active_tab.get() == Some(name);

    on_cleanup(cx, move || {
        info!("cleanup tab");
    });

    view! { cx,
        {
            move || is_active().then(|| view! { cx,
                <div id=id.to_string() class={"crud-tab"} data:name=name>
                    {
                        if let Some(children) = &children {
                            children(cx)
                        } else {
                            Fragment::new(vec![])
                        }
                    }
                </div>
            })
        }
    }
}
