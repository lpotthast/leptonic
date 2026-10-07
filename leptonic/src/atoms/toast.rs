//! Headless toast atoms.
// Upstream: react-aria-components/src/Toast.tsx @ 99e6102368
use std::sync::Arc;

use leptos::{context::Provider, portal::Portal, prelude::*};

use crate::{
    hooks::{
        IntoAttrs, QueuedToast, ToastQueue, UseButtonInput, UseButtonReturn, UseFocusRingInput,
        UseFocusRingReturn, UseToastContentProps, UseToastInput, UseToastRegionInput,
        UseToastRegionReturn, UseToastReturn, use_button, use_focus_ring, use_toast,
        use_toast_region,
    },
    utils::{
        CapturedElement, classes::Classes, data_attributes::flag,
        default_class::with_default_class, i18n::use_direction, locale::WritingDirection,
        slot_id::SlotProps, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The title, description and close button are atoms of their own (`ToastTitle`,
//   `ToastDescription`, `ToastCloseButton`; react-aria-components: `Text` and `Button` slots).
// - The region renders its toasts with `children` per toast (react-aria-components: a render
//   function, through a `ToastList`); the list is an `ol` of `li`s with `display: contents`.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - The region (with `use_toast_region`) is mounted only while there are toasts
//   (react-aria-components keeps the hook mounted and portals the region conditionally): the
//   focus returns when it unmounts, its hover and focus state start over with each batch.
//
// ## OMITTED FEATURES
// - A portal container (react-aria: `PortalProvider`); the region goes to the end of the body.
// - `data-hovered` on the region.
//
// =============================================================================

/// What the toasts of a region need from it.
struct ToastRegionContext<T: Clone + Send + Sync + 'static> {
    queue: ToastQueue<T>,
}

impl<T: Clone + Send + Sync + 'static> Clone for ToastRegionContext<T> {
    fn clone(&self) -> Self {
        Self { queue: self.queue }
    }
}

/// What a toast's parts need from it.
#[derive(Clone)]
struct ToastContext {
    content: UseToastContentProps,
    title_id: String,
    description: StoredValue<Option<SlotProps>>,
    close_button: StoredValue<UseButtonInput>,
}

/// The region showing the toasts of a queue (react-aria-components' `ToastRegion`): a landmark
/// (F6 reaches it), rendered at the end of the page while there are toasts. Renders `children`
/// per visible toast (e.g. a `Toast`).
///
/// Data attributes: `data-focused`, `data-focus-visible`.
///
/// Default class: `leptonic-ToastRegion`.
#[component]
pub fn ToastRegion<T, F, V>(
    queue: ToastQueue<T>,
    children: F,
    /// Default: "1 notification." / "2 notifications.".
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    #[prop(into, optional)] aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
    F: Fn(QueuedToast<T>) -> V + Send + Sync + 'static,
    V: IntoView + 'static,
{
    let classes = with_default_class("leptonic-ToastRegion", classes);
    // Mounted while there are toasts (see the deviations): the focus returns when the last one
    // closes.
    let props = StoredValue::new((aria_label, aria_labelledby, classes, styles));
    let children = StoredValue::new(Arc::new(children));
    let direction = use_direction();
    let render = move || {
        let (aria_label, aria_labelledby, classes, styles) = props.get_value();
        let element = CapturedElement::new();
        let UseToastRegionReturn { region_props } = use_toast_region(UseToastRegionInput {
            queue,
            element,
            aria_label,
            aria_labelledby,
        });
        let UseFocusRingReturn {
            props: focus_ring,
            is_focused,
            is_focus_visible,
        } = use_focus_ring(UseFocusRingInput::default());
        view! {
            <Provider value=ToastRegionContext { queue }>
                <div
                    {..region_props.into_attrs()}
                    {..focus_ring.into_attrs()}
                    {..element.attr()}
                    class=classes
                    style=styles
                    data-focused=flag(is_focused)
                    data-focus-visible=flag(is_focus_visible)
                    // Portaled out of the locale's subtree (react-aria-components sets it too).
                    dir=move || match direction.get() {
                        WritingDirection::Ltr => "ltr",
                        WritingDirection::Rtl => "rtl",
                    }
                >
                    <ol style="display: contents">
                        <For
                            each=move || queue.visible_toasts.get()
                            key=|toast| toast.key.clone()
                            children=move |toast| {
                                let children = children.get_value();
                                view! { <li style="display: contents">{children(toast)}</li> }
                            }
                        />
                    </ol>
                </div>
            </Provider>
        }
    };
    let has_toasts = move || queue.visible_toasts.with(|toasts| !toasts.is_empty());
    view! {
        <Show when=has_toasts>
            <Portal>{render()}</Portal>
        </Show>
    }
}

/// A toast of the region around it (react-aria-components' `Toast`): a non-modal alert dialog.
/// Put a `ToastContent` (with a `ToastTitle` and a `ToastDescription`) and a `ToastCloseButton`
/// inside.
///
/// Data attributes: `data-focused`, `data-focus-visible`.
///
/// Default class: `leptonic-Toast`.
#[component]
pub fn Toast<T: Clone + Send + Sync + 'static>(
    toast: QueuedToast<T>,
    #[prop(into, optional)] aria_label: MaybeProp<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Toast", classes);
    let Some(region) = use_context::<ToastRegionContext<T>>() else {
        crate::utils::dev_warn!("Toast: not inside a ToastRegion of its content type");
        return ().into_any();
    };
    let UseToastReturn {
        toast_props,
        content_props,
        title_id,
        description_props,
        close_button,
    } = use_toast(UseToastInput {
        queue: region.queue,
        aria_label,
        toast,
        aria_labelledby: None,
        aria_describedby: None,
    });
    let UseFocusRingReturn {
        props: focus_ring,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput::default());
    let context = ToastContext {
        content: content_props,
        title_id,
        description: StoredValue::new(Some(description_props)),
        close_button: StoredValue::new(close_button),
    };
    view! {
        <div
            {..toast_props.into_attrs()}
            {..focus_ring.into_attrs()}
            class=classes
            style=styles
            data-focused=flag(is_focused)
            data-focus-visible=flag(is_focus_visible)
        >
            <Provider value=context>{children()}</Provider>
        </div>
    }
    .into_any()
}

fn expect_toast(part: &str) -> Option<ToastContext> {
    let context = use_context::<ToastContext>();
    if context.is_none() {
        crate::utils::dev_warn!("{part}: not inside a Toast");
    }
    context
}

/// The content of the toast around it: an alert, announced when the toast appears.
///
/// Default class: `leptonic-ToastContent`.
#[component]
pub fn ToastContent(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ToastContent", classes);
    let Some(context) = expect_toast("ToastContent") else {
        return ().into_any();
    };
    view! {
        <div {..context.content.into_attrs()} class=classes style=styles>
            {children()}
        </div>
    }
    .into_any()
}

/// The title of the toast around it, naming it.
///
/// Default class: `leptonic-ToastTitle`.
#[component]
pub fn ToastTitle(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ToastTitle", classes);
    let Some(context) = expect_toast("ToastTitle") else {
        return ().into_any();
    };
    view! { <div id=context.title_id class=classes style=styles>{children()}</div> }.into_any()
}

/// The description of the toast around it, describing it.
///
/// Default class: `leptonic-ToastDescription`.
#[component]
pub fn ToastDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ToastDescription", classes);
    let Some(props) = expect_toast("ToastDescription")
        .and_then(|context| context.description.try_update_value(Option::take).flatten())
    else {
        return ().into_any();
    };
    view! { <div {..props.into_attrs()} class=classes style=styles>{children()}</div> }.into_any()
}

/// The button closing the toast around it ("Close").
///
/// Data attributes: `data-pressed`, `data-hovered`, `data-focused`, `data-focus-visible`,
/// `data-disabled`.
///
/// Default class: `leptonic-ToastCloseButton`.
#[component]
pub fn ToastCloseButton(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-ToastCloseButton", classes);
    let Some(context) = expect_toast("ToastCloseButton") else {
        return ().into_any();
    };
    let UseButtonReturn {
        props,
        is_disabled,
        is_pressed,
        is_hovered,
        is_focused,
        ..
    } = use_button(context.close_button.get_value());
    let (attrs, button_styles) = props.into_parts();
    view! {
        <button
            {..attrs}
            class=classes
            style=button_styles.merge(styles)
            data-pressed=flag(is_pressed)
            data-hovered=flag(is_hovered)
            data-focused=flag(is_focused)
            data-disabled=flag(is_disabled)
        >
            {children()}
        </button>
    }
    .into_any()
}
