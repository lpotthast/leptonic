use leptos::*;

use crate::Mount;

use super::tab::TabLabel;

#[derive(Debug, Clone)]
pub struct TabHistory {
    active: Option<Oco<'static, str>>,
    previous: Option<Oco<'static, str>>,
}

impl TabHistory {
    pub const fn new() -> Self {
        Self {
            active: None,
            previous: None,
        }
    }

    pub const fn get_active(&self) -> Option<&Oco<'static, str>> {
        self.active.as_ref()
    }

    pub const fn get_previous(&self) -> Option<&Oco<'static, str>> {
        self.previous.as_ref()
    }

    pub fn push(&mut self, active: Oco<'static, str>) {
        self.previous = self.active.clone();
        self.active = Some(active);
    }
}

impl Default for TabHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TabsContext {
    pub history: ReadSignal<TabHistory>,
    pub set_history: WriteSignal<TabHistory>,

    pub tab_labels: ReadSignal<Vec<TabLabel>>,
    pub set_tab_labels: WriteSignal<Vec<TabLabel>>,

    /// Default mount option when not otherwise specified for an individual tab.
    pub mount: Option<Mount>,
}

#[component]
pub fn Tabs(#[prop(optional)] mount: Option<Mount>, children: Children) -> impl IntoView {
    let (history, set_history) = create_signal(TabHistory::new());
    let (tab_labels, set_tab_labels) = create_signal(Vec::new());
    view! {
        <Provider value=TabsContext {
            history,
            set_history,
            tab_labels,
            set_tab_labels,
            mount,
        }>
            <leptonic-tabs>
                <TabSelectors tab_labels history set_history/>
                { children() }
            </leptonic-tabs>
        </Provider>
    }
}

#[component]
pub fn TabSelectors(
    tab_labels: ReadSignal<Vec<TabLabel>>,
    history: ReadSignal<TabHistory>,
    set_history: WriteSignal<TabHistory>,
) -> impl IntoView {
    view! {
        <leptonic-tab-selectors role="tablist">
            <For
                each=move || tab_labels.get()
                key=|label| label.id
                children=move |label| {
                    let n1 = label.name.clone();
                    let n2 = label.name.clone();
                    view! {
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
    is_active: A,
    set_active: S,
    name: Oco<'static, str>,
    label: View,
) -> impl IntoView
where
    A: Fn() -> bool + 'static,
    S: Fn() + 'static,
{
    view! {
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
