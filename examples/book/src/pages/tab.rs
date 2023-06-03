use leptonic::prelude::*;
use leptos::*;
use tracing::info;

#[component]
pub fn PageTab(cx: Scope) -> impl IntoView {
    let (count, _set_count) = create_signal_ls(cx, "count", 0u64);
    let (test_bool, set_test_bool) = create_signal(cx, false);
    view! { cx,
        <H2>"Tabs"</H2>

        <Tabs mount=Mount::WhenShown>
            <Tab
                name="outer-1"
                label=view! {cx, "Toasts; Count is" {move || count.get()}}
                on_show=Callback::new(cx, move |()| {info!("tab1 is now shown!")})
                on_hide=Callback::new(cx, move |()| {info!("tab1 is now hidden!")})
            >
                <Checkbox checked=test_bool on_toggle=move || set_test_bool.update(|it| *it = !*it) />
                <Checkbox checked=test_bool on_toggle=move || set_test_bool.update(|it| *it = !*it) />
                <Toggle state=test_bool on_toggle=move |s| set_test_bool.set(s) />
                <If sig=move || test_bool.get()>
                    "asd"
                </If>
            </Tab>
            <Tab name="outer-2" label="Tab2Label">
                <Tabs>
                    <Tab name="inner-1" label="Inner1">
                        "That is nested!"
                    </Tab>
                    <Tab name="inner-2" label="Inner2">
                        "That is nested as well!"
                    </Tab>
                </Tabs>
            </Tab>
            <Tab name="outer-3" label="Tab2Label">
                <ProgressBar progress=create_signal(cx, 34).0/>
            </Tab>
            <Tab name="outer-4" label="Tab4Label">
            </Tab>
        </Tabs>
    }
}
