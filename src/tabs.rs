use leptos::*;

use super::tab::TabLabel;

#[derive(Copy, Clone)]
pub struct TabsContext {
    pub active_tab: ReadSignal<Option<&'static str>>,
    pub set_active_tab: WriteSignal<Option<&'static str>>,

    pub tab_labels: ReadSignal<Vec<TabLabel>>,
    pub set_tab_labels: WriteSignal<Vec<TabLabel>>,
}

#[component]
pub fn Tabs(cx: Scope, children: Children) -> impl IntoView {
    let (active_tab, set_active_tab) = create_signal(cx, None);
    let (tab_labels, set_tab_labels) = create_signal(cx, Vec::new());
    provide_context::<TabsContext>(
        cx,
        TabsContext {
            active_tab,
            set_active_tab,
            tab_labels,
            set_tab_labels,
        },
    );
    view! { cx,
        <div class="crud-tabs">
            <TabSelectors tab_labels active_tab set_active_tab/>
            { children(cx) }
        </div>
    }
}

#[component]
pub fn TabSelectors(
    cx: Scope,
    tab_labels: ReadSignal<Vec<TabLabel>>,
    active_tab: ReadSignal<Option<&'static str>>,
    set_active_tab: WriteSignal<Option<&'static str>>,
) -> impl IntoView {
    view! { cx,
        <div class="crud-tab-selectors">
            <For
                each=tab_labels
                key=|label| label.id
                view=move |cx, label| {
                    view! { cx,
                        <TabSelector
                            is_active=Signal::derive(cx, move || active_tab.get() == Some(label.name))
                            set_active=move || set_active_tab.update(|c| *c = Some(label.name))
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
        <button class="crud-tab-selector"
                data:for-name=name
                class:active=is_active
                on:click=move |_event| set_active()>
                { label }
        </button>
    }
}
