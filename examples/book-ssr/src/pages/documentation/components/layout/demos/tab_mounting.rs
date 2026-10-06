use leptonic::{Mount, components::prelude::*};
use leptos::prelude::*;

#[component]
pub fn TabMountingDemo() -> impl IntoView {
    view! {
        <div class="demo-control-stack">
            <NestedTabs mount=Mount::Once label="Mount::Once"/>
            <NestedTabs mount=Mount::WhenShown label="Mount::WhenShown"/>
        </div>
    }
}

/// Tabs with nested tabs in their first tab, to show whether the inner selection survives switching.
#[component]
fn NestedTabs(mount: Mount, label: &'static str) -> impl IntoView {
    view! {
        <div>
            <strong>{label}</strong>
            <Tabs mount>
                <Tab name="outer-1" label=|| "Outer 1">
                    <Tabs>
                        <Tab name="inner-1" label=|| "Inner 1">"Select Inner 2, then switch to Outer 2 and back."</Tab>
                        <Tab name="inner-2" label=|| "Inner 2">"Inner 2 is selected."</Tab>
                    </Tabs>
                </Tab>
                <Tab name="outer-2" label=|| "Outer 2">"Now switch back to Outer 1."</Tab>
            </Tabs>
        </div>
    }
}
