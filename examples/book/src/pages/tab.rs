use leptonic::prelude::*;
use leptos::*;
use tracing::info;

#[component]
pub fn PageTab(cx: Scope) -> impl IntoView {
    let (count, _set_count) = create_signal_ls(cx, "count", 0u64);
    let (test_bool, set_test_bool) = create_signal(cx, false);
    view! { cx,
        <Typography variant=TypographyVariant::H2>"Tabs"</Typography>

        <Tabs mount=Mount::WhenShown>
            <Tab
                name="outer-1"
                label=view! {cx, "Toasts; Count is" {move || count.get()}}
                on_show=move || {info!("tab1 is now shown!")}
                on_hide=move || {info!("tab1 is now hidden!")}
            >
                <Checkbox checked=(test_bool, set_test_bool) />
                <Checkbox checked=(test_bool, set_test_bool) />
                <Toggle state=test_bool on_toggle=set_test_bool />
                <If sig=test_bool>
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
