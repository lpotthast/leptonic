use leptos::*;

use crate::Mount;

use super::tab::TabLabel;

#[derive(Debug, Clone)]
pub struct TabHistory {
    active: Option<&'static str>,
    previous: Option<&'static str>,
}

impl TabHistory {
    pub fn new() -> Self {
        Self {
            active: None,
            previous: None,
        }
    }

    pub fn get_active(&self) -> Option<&'static str> {
        self.active
    }

    pub fn get_previous(&self) -> Option<&'static str> {
        self.previous
    }

    pub fn push(&mut self, active: &'static str) {
        self.previous = self.active;
        self.active = Some(active);
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
        <leptonic-tab-selectors>
            <For
                each=move || tab_labels.get()
                key=|label| label.id
                view=move |cx, label| {
                    view! { cx,
                        <TabSelector
                            is_active=Signal::derive(cx, move || history.get().get_active() == Some(label.name))
                            set_active=move || set_history.update(|history| history.push(label.name))
                            name=label.name
                            label=(*label.label).clone() />
                    }
                }
            />
        </leptonic-tab-selectors>
    }
}

#[component]
pub fn TabSelector<S>(
    cx: Scope,
    #[prop(into)] is_active: Signal<bool>,
    set_active: S,
    name: &'static str,
    label: View,
) -> impl IntoView
where
    S: Fn() -> () + 'static,
{
    view! { cx,
        <leptonic-tab-selector
            data:for-name=name
            class:active=move || is_active.get()
            on:click=move |_event| set_active()
        >
            { label }
        </leptonic-tab-selector>
    }
}
