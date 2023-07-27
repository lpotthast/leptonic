use std::borrow::Cow;

use leptos::*;

use crate::Mount;

use super::tab::TabLabel;

#[derive(Debug, Clone)]
pub struct TabHistory {
    active: Option<Cow<'static, str>>,
    previous: Option<Cow<'static, str>>,
}

impl TabHistory {
    pub fn new() -> Self {
        Self {
            active: None,
            previous: None,
        }
    }

    pub fn get_active(&self) -> Option<&Cow<'static, str>> {
        self.active.as_ref()
    }

    pub fn get_previous(&self) -> Option<&Cow<'static, str>> {
        self.previous.as_ref()
    }

    pub fn push(&mut self, active: Cow<'static, str>) {
        self.previous = self.active.clone();
        self.active = Some(active);
    }
}

impl Default for TabHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Copy, Clone)]
pub struct TabsContext {
    pub history: ReadSignal<TabHistory>,
    pub set_history: WriteSignal<TabHistory>,

    pub tab_labels: ReadSignal<Vec<TabLabel>>,
    pub set_tab_labels: WriteSignal<Vec<TabLabel>>,

    /// Default mount option when not otherwise specified for an individual tab.
    pub mount: Option<Mount>,
}

#[component]
pub fn Tabs(
    cx: Scope,
    #[prop(optional)] mount: Option<Mount>,
    children: Children,
) -> impl IntoView {
    let (history, set_history) = create_signal(cx, TabHistory::new());
    let (tab_labels, set_tab_labels) = create_signal(cx, Vec::new());
    provide_context::<TabsContext>(
        cx,
        TabsContext {
            history,
            set_history,
            tab_labels,
            set_tab_labels,
            mount,
        },
    );
    view! { cx,
        <leptonic-tabs>
            <TabSelectors tab_labels history set_history/>
            { children(cx) }
        </leptonic-tabs>
    }
}

#[component]
pub fn TabSelectors(
    cx: Scope,
    tab_labels: ReadSignal<Vec<TabLabel>>,
    history: ReadSignal<TabHistory>,
    set_history: WriteSignal<TabHistory>,
) -> impl IntoView {
    view! { cx,
        <leptonic-tab-selectors role="tablist">
            <For
                each=move || tab_labels.get()
                key=|label| label.id
                view=move |cx, label| {
                    let n1 = label.name.clone();
                    let n2 = label.name.clone();
                    view! { cx,
                        <TabSelector
                            is_active=move || history.get().get_active() == Some(&n1.clone())
                            set_active=move || set_history.update(|history| history.push(n2.clone()))
                            name=label.name.clone()
                            label=(*label.label).clone() />
                    }
                }
            />
        </leptonic-tab-selectors>
    }
}

#[component]
fn TabSelector<A, S>(
    cx: Scope,
    is_active: A,
    set_active: S,
    name: Cow<'static, str>,
    label: View,
) -> impl IntoView
where
    A: Fn() -> bool + 'static,
    S: Fn() + 'static,
{
    view! { cx,
        <leptonic-tab-selector
            data:for-name=name
            class:active=is_active
            on:click=move |_event| set_active()
            role="tab"
        >
            { label }
        </leptonic-tab-selector>
    }
}
