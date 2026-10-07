use std::time::Duration;

use leptonic::{
    components::prelude::{Button, ButtonVariant, Icon},
    hooks::{
        IntoAttrs, QueuedToast, ToastOptions, ToastQueue, UseToastInput, UseToastRegionInput,
        UseToastRegionReturn, UseToastReturn, use_button, use_toast, use_toast_region,
        use_toast_state,
    },
    prelude::icondata,
    utils::CapturedElement,
};
use leptos::prelude::*;

/// What a toast of this demo shows.
#[derive(Clone)]
struct Notice {
    title: String,
    description: String,
}

/// One toast: an alert dialog named by its title, with a close button.
#[component]
fn NoticeToast(toast: QueuedToast<Notice>, queue: ToastQueue<Notice>) -> impl IntoView {
    let Notice { title, description } = toast.content.clone();
    let UseToastReturn {
        toast_props,
        content_props,
        title_id,
        description_props,
        close_button,
    } = use_toast(
        UseToastInput {
            toast,
            aria_label: MaybeProp::default(),
            aria_labelledby: None,
            aria_describedby: None,
        },
        queue,
    );
    let (close_attrs, close_styles) = use_button(close_button).props.into_parts();

    view! {
        <div {..toast_props.into_attrs()} class="demo-hook-toast">
            <div {..content_props.into_attrs()}>
                <div id=title_id class="demo-hook-toast-title">{title}</div>
                <div {..description_props.into_attrs()}>{description}</div>
            </div>
            <button {..close_attrs} style=close_styles class="demo-hook-toast-close">
                <Icon icon=icondata::BsX/>
            </button>
        </div>
    }
}

/// The region showing the visible toasts, newest first.
#[component]
fn NoticeRegion(queue: ToastQueue<Notice>) -> impl IntoView {
    let element = CapturedElement::new();
    let UseToastRegionReturn { region_props } =
        use_toast_region(UseToastRegionInput::default(), queue, element);

    view! {
        <div {..region_props.into_attrs()} {..element.attr()} class="demo-hook-toast-region">
            <For
                each=move || queue.visible_toasts.get()
                key=|toast| toast.key.clone()
                children=move |toast| view! { <NoticeToast toast queue/> }
            />
        </div>
    }
}

#[component]
pub fn ToastHooksDemo() -> impl IntoView {
    // Shows up to two toasts; further ones wait until one closes.
    let queue = use_toast_state::<Notice>(Some(2));
    let added = StoredValue::new(0);
    let closed = RwSignal::new(0);

    let add = move |timeout: Option<Duration>| {
        added.update_value(|added| *added += 1);
        queue.add(
            Notice {
                title: format!("Message {}", added.get_value()),
                description: "Your message was sent.".to_owned(),
            },
            ToastOptions {
                timeout,
                on_close: Some(Callback::new(move |()| {
                    closed.update(|closed| *closed += 1);
                })),
            },
        );
    };

    view! {
        <div class="demo-inline-controls">
            <Button on_press=move |_| add(Some(Duration::from_secs(5)))>"Send"</Button>
            <Button variant=ButtonVariant::Outlined on_press=move |_| add(None)>"Send (toast stays)"</Button>
        </div>

        // The region is rendered only while there are toasts.
        <Show when=move || queue.visible_toasts.with(|toasts| !toasts.is_empty())>
            <NoticeRegion queue/>
        </Show>

        <p class="demo-status">
            {move || match closed.get() {
                1 => "1 toast closed.".to_owned(),
                closed => format!("{closed} toasts closed."),
            }}
        </p>
    }
}
