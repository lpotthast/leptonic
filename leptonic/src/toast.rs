use leptos::*;
use leptos_icons::BsIcon;
use uuid::Uuid;

use crate::icon::Icon;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum ToastVariant {
    Success,
    Info,
    Warn,
    Error,
}

impl ToastVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            ToastVariant::Success => "success",
            ToastVariant::Info => "info",
            ToastVariant::Warn => "warn",
            ToastVariant::Error => "error",
        }
    }
}

impl std::fmt::Display for ToastVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl Default for ToastVariant {
    fn default() -> Self {
        Self::Info
    }
}

#[derive(Debug, Clone)]
pub struct Toast {
    pub id: uuid::Uuid,
    pub created_at: time::OffsetDateTime,
    pub variant: ToastVariant,
    pub header: View,
    pub body: View,
    pub timeout: ToastTimeout,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(unused)]
pub enum ToastTimeout {
    None,
    DefaultDelay,
    CustomDelay(time::Duration),
}

impl std::fmt::Display for ToastTimeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToastTimeout::None => f.write_str("None"),
            ToastTimeout::DefaultDelay => f.write_str("Default delay"),
            ToastTimeout::CustomDelay(_) => f.write_str("Custom delay"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Toasts {
    pub toasts: ReadSignal<Vec<Toast>>,
    set_toasts: WriteSignal<Vec<Toast>>,
}

impl Toasts {
    /// Adds a toast and schedules its removal.
    pub fn push(&self, toast: Toast) {
        let setter = self.set_toasts;

        // Prepare cleanup. We do it before adding the toast so that we can save a clone.
        // Display durations for toasts are generally high (order of seconds), so this is not a problem.
        if toast.timeout != ToastTimeout::None {
            set_timeout(
                move || {
                    setter.update(|toasts| {
                        if let Some(idx) = toasts.iter().position(|it| it.id == toast.id) {
                            toasts.remove(idx);
                        }
                    });
                },
                match &toast.timeout {
                    ToastTimeout::None => panic!("unreachable"),
                    ToastTimeout::DefaultDelay => std::time::Duration::from_secs(3),
                    ToastTimeout::CustomDelay(delay) => std::time::Duration::from_nanos(
                        delay.whole_nanoseconds().try_into().unwrap_or(u64::MAX),
                    ),
                },
            );
        }

        setter.update(|toasts| toasts.push(toast));
    }

    pub fn try_remove(&self, id: Uuid) -> Option<Toast> {
        self.set_toasts.update_ret(|toasts| {
            toasts.iter().position(|it| it.id == id).map(|idx| toasts.remove(idx))
        })
    }

    /// Removes all toasts. Does not interfere with scheduled removals of pushed toasts.
    pub fn clear(&self) {
        self.set_toasts.update(|toasts| toasts.clear())
    }
}

pub trait SignalUpdateExt<T> {
    fn update_ret<O>(&self, f: impl FnOnce(&mut T) -> Option<O>) -> Option<O>;
}

impl<T> SignalUpdateExt<T> for WriteSignal<T> {
    fn update_ret<O>(&self, f: impl FnOnce(&mut T) -> Option<O>) -> Option<O> {
        match self.try_update(f) {
            Some(value) => value,
            None => {
                tracing::warn!("Attempted to update a signal after it was disposed.");
                None
            }
        }
    }
}

#[component]
pub fn ToastRoot(cx: Scope, children: Children) -> impl IntoView {
    let (toasts, set_toasts) = create_signal(cx, Vec::new());

    provide_context::<Toasts>(cx, Toasts { toasts, set_toasts });

    view! { cx,
        { children(cx) }

        <leptonic-toasts>
            <For
                each=move || toasts.get()
                key=|toast| toast.id
                view=move |_cx, toast| {
                    view! { cx,
                        <Toast toast/>
                    }
                }
            />
        </leptonic-toasts>
    }
}

// TODO: Incorporate
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ToastHorizontalPosition {
    Left,
    Right,
}

// TODO: Incorporate
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum ToastVerticalPosition {
    Top,
    Bottom,
}

#[component]
pub fn Toast(cx: Scope, toast: Toast) -> impl IntoView {
    let manually_closable = match toast.timeout {
        ToastTimeout::None => true,
        ToastTimeout::DefaultDelay => false,
        ToastTimeout::CustomDelay(duration) => duration.whole_seconds() > 10,
    };

    view! { cx,
        <leptonic-toast id=toast.id.to_string() variant=toast.variant.as_str()>
            <leptonic-toast-header>
                { toast.header }

                { match manually_closable {
                    true => view! {cx,
                        <div>
                            <Icon
                                class="dismiss"
                                icon=BsIcon::BsXCircleFill
                                on:click=move |_e| { expect_context::<Toasts>(cx).try_remove(toast.id); }
                            />
                        </div>
                    }.into_view(cx),
                    false => ().into_view(cx),
                } }
            </leptonic-toast-header>
            <leptonic-toast-message>
                { toast.body }
            </leptonic-toast-message>
        </leptonic-toast>
    }
}
