use leptos::*;

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
}

#[component]
pub fn Tabs(cx: Scope, children: Children) -> impl IntoView {
    let (history, set_history) = create_signal(cx, TabHistory::new());
    let (tab_labels, set_tab_labels) = create_signal(cx, Vec::new());
    provide_context::<TabsContext>(
        cx,
        TabsContext {
            history,
            set_history,
            tab_labels,
            set_tab_labels,
        },
    );
    view! { cx,
        <div class="leptonic-tabs">
            <TabSelectors tab_labels history set_history/>
            { children(cx) }
        </div>
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
        <div class="leptonic-tab-selectors">
            <For
                each=tab_labels
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
        </div>
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
        <button class="leptonic-tab-selector"
                data:for-name=name
                class:active=is_active
                on:click=move |_event| set_active()>
                { label }
        </button>
    }
}
