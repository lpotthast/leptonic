use leptos::{context::Provider, prelude::*};

use crate::{
    Mount, Out,
    atoms::tabs::{Tab as TabAtom, TabList, TabPanel, Tabs as TabsAtom},
    hooks::{
        KeyboardActivation, Orientation,
        collections::{Key, use_list_collection},
    },
    utils::{classes::Classes, styles::Styles},
};

/// A tab declared by a [`Tab`](super::tab::Tab).
#[derive(Clone)]
pub(crate) struct TabSpec {
    pub(crate) name: String,
    pub(crate) label: ViewFn,
    pub(crate) is_disabled: Signal<bool>,
    pub(crate) mount: Option<Mount>,
    pub(crate) content: ChildrenFn,
}

/// The tabs declared inside a [`Tabs`], in declaration order.
#[derive(Clone, Copy)]
pub(crate) struct TabsRegistry(pub(crate) RwSignal<Vec<TabSpec>>);

/// Tabs: a tab list and the selected tab's panel, built on the tabs atoms. Declare the tabs as
/// [`Tab`](super::tab::Tab) children.
///
/// ```ignore
/// <Tabs>
///     <Tab name="overview" label=|| "Overview">"A short summary."</Tab>
///     <Tab name="activity" label=|| "Activity">"The latest changes."</Tab>
/// </Tabs>
/// ```
#[component]
#[allow(clippy::too_many_arguments)]
pub fn Tabs(
    /// The initially selected tab's name. Default: the first enabled tab.
    #[prop(into, optional)]
    default_selected_key: Option<String>,
    /// The selected tab's name (controlled): a value or any signal.
    #[prop(into, optional)]
    selected_key: Option<Signal<String>>,
    /// Receives the selected tab's name: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_selected_key: Option<Out<String>>,
    /// Called with the name of the tab the user selects.
    #[prop(into, optional)]
    on_selection_change: Option<Callback<String>>,
    /// Whether hidden panels stay mounted (keeping their state). Default: [`Mount::Once`].
    #[prop(optional)]
    mount: Option<Mount>,
    #[prop(default = Orientation::Horizontal)] orientation: Orientation,
    /// Whether focusing a tab with the arrow keys selects it.
    #[prop(optional)]
    keyboard_activation: KeyboardActivation,
    /// Names the tab list.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let tabs = RwSignal::new(Vec::<TabSpec>::new());
    // The `Tab`s register while the children are created, before the tab list renders (so the
    // server renders every tab).
    let declarations = view! { <Provider value=TabsRegistry(tabs)>{children()}</Provider> };

    let specs = Signal::derive(move || tabs.get());
    let collection = use_list_collection(
        specs,
        |tab: &TabSpec| Key::from(tab.name.clone()),
        |tab: &TabSpec| tab.name.clone(),
    );
    let disabled_keys = Signal::derive(move || {
        tabs.with(|tabs| {
            tabs.iter()
                .filter(|tab| tab.is_disabled.get())
                .map(|tab| Key::from(tab.name.clone()))
                .collect()
        })
    });
    let mount = mount.unwrap_or_default();

    view! {
        <TabsAtom
            collection=collection
            nostrip:default_selected_key=default_selected_key.map(Key::from)
            nostrip:selected_key=selected_key.map(|name| Signal::derive(move || Key::from(name.get())))
            nostrip:set_selected_key=set_selected_key
                .map(|out| Out::new_callback(move |key: Key| out.set(key.to_string())))
            nostrip:on_selection_change=on_selection_change
                .map(|callback| Callback::new(move |key: Key| callback.run(key.to_string())))
            disabled_keys=disabled_keys
            orientation=orientation
            keyboard_activation=keyboard_activation
            classes=classes.add("leptonic-tabs")
            styles=styles
        >
            {declarations}
            <TabList classes="leptonic-tab-selectors" aria_label=aria_label>
                <For
                    each=move || tabs.get()
                    key=|tab| tab.name.clone()
                    children=|tab| {
                        view! {
                            <TabAtom key=tab.name.clone() classes="leptonic-tab-selector">
                                {tab.label.run()}
                            </TabAtom>
                        }
                    }
                />
            </TabList>
            <For
                each=move || tabs.get()
                key=|tab| tab.name.clone()
                children=move |tab| {
                    view! {
                        <TabPanel
                            key=tab.name.clone()
                            should_force_mount=tab.mount.unwrap_or(mount) == Mount::Once
                            classes="leptonic-tab"
                        >
                            {(tab.content)()}
                        </TabPanel>
                    }
                }
            />
        </TabsAtom>
    }
}
