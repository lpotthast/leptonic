use leptonic::prelude::*;
use leptos::*;
use tracing::info;
use uuid::Uuid;

#[component]
pub fn PageTab(cx: Scope) -> impl IntoView {
    let (count, _set_count) = create_signal_ls(cx, "count", 0u64);
    let (test_bool, set_test_bool) = create_signal(cx, false);
    let toasts = use_context::<Toasts>(cx).unwrap();
    view! { cx,
        <h2>"Tabs"</h2>

        <Tabs mount=Mount::WhenShown>
            <Tab
                name="outer-1"
                label=view! {cx, "Toasts; Count is" {move || count.get()}}
                on_show=move || {info!("tab1 is now shown!")}
                on_hide=move || {info!("tab1 is now hidden!")}
            >
                <Checkbox checked=(test_bool, set_test_bool) />
                <Checkbox checked=(test_bool, set_test_bool) />
                <Toggle on=test_bool set_on=set_test_bool />
                <Button on_click=move |_| toasts.push(op_success_toast(cx))>"Create Toast"</Button>
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

fn op_success_toast(cx: Scope) -> Toast {
    Toast {
        id: Uuid::new_v4(),
        created_at: time::OffsetDateTime::now_utc(),
        variant: ToastVariant::Success,
        header: view! { cx, "Header" }.into_view(cx),
        body: view! { cx, "Body" }.into_view(cx),
        timeout: ToastTimeout::DefaultDelay,
    }
}
