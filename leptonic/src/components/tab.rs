use leptos::prelude::*;

use crate::{
    Mount,
    components::tabs::{TabSpec, TabsRegistry},
};

/// A tab of the [`Tabs`](super::tabs::Tabs) around it: its `label` goes into the tab list, its
/// children into its panel. Renders nothing itself (`Tabs` renders the tab and the panel).
#[component]
pub fn Tab(
    /// Uniquely identifies this tab (its key in `Tabs`' selection).
    #[prop(into)]
    name: String,
    /// The tab's label in the tab list.
    #[prop(into)]
    label: ViewFn,
    /// Whether the tab can't be selected (keyboard navigation skips it).
    #[prop(into, optional)]
    is_disabled: Signal<bool>,
    /// Whether the panel stays mounted while hidden. Default: the `mount` of `Tabs`.
    #[prop(optional)]
    mount: Option<Mount>,
    /// The panel's content.
    #[prop(optional, default = std::sync::Arc::new(|| ().into_any()))]
    children: ChildrenFn,
) -> impl IntoView {
    let TabsRegistry(tabs) = expect_context::<TabsRegistry>();
    let removed = name.clone();
    tabs.update(|tabs| {
        tabs.push(TabSpec {
            name,
            label,
            is_disabled,
            mount,
            content: children,
        });
    });
    on_cleanup(move || {
        tabs.try_update(|tabs| tabs.retain(|tab| tab.name != removed));
    });
}
