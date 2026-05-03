use leptonic::Mount;
use leptonic::components::prelude::*;
use leptos::prelude::*;

#[component]
pub fn TabDemo() -> impl IntoView {
    let (test_reactive_label_bool, set_test_reactive_label_bool) = signal(false);

    view! {
        <Tabs mount=Mount::Once>
            <Tab name="tab-1" label=|| "Tab 1">"Content of tab 1"</Tab>
            <Tab name="tab-2" label=|| "Tab 2">"Content of tab 2"</Tab>
            <Tab name="tab-3" label=|| "Tab 3">"Content of tab 3"</Tab>
        </Tabs>

        <Tabs mount=Mount::Once>
            <Tab name="tab-1" label=move || format!("State: {}", test_reactive_label_bool.get())>
                <Toggle state=test_reactive_label_bool set_state=set_test_reactive_label_bool/>
            </Tab>
            <Tab name="tab-2" label=|| "Tab 2">
                "Content of tab 2"
            </Tab>
        </Tabs>

        <Tabs mount=Mount::Once>
            <Tab name="outer-1" label=|| "Outer 1">
                <Tabs>
                    <Tab name="inner-1" label=|| "Inner 1">
                        "This is a nested tab."
                    </Tab>
                    <Tab name="inner-2" label=|| "Inner 2">
                        "This tab is nested as well."
                    </Tab>
                </Tabs>
            </Tab>
            <Tab name="outer-2" label=|| "Outer 2"></Tab>
        </Tabs>

        <Tabs mount=Mount::WhenShown>
            <Tab name="outer-1" label=|| "Outer 1">
                <Tabs>
                    <Tab name="inner-1" label=|| "Inner 1">
                        "This is a nested tab."
                    </Tab>
                    <Tab name="inner-2" label=|| "Inner 2">
                        "This tab is nested as well."
                    </Tab>
                </Tabs>
            </Tab>
            <Tab name="outer-2" label=|| "Outer 2"></Tab>
        </Tabs>
    }
}
