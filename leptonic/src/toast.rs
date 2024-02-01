use leptos::*;
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
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::Info => "info",
            Self::Warn => "warn",
            Self::Error => "error",
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
    pub id: Uuid,
    pub created_at: time::OffsetDateTime,
    pub variant: ToastVariant,
    pub header: View,
    pub body: View,
    pub timeout: ToastTimeout,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
pub enum ToastTimeout {
    None,
    DefaultDelay,
    CustomDelay(time::Duration),
}

impl std::fmt::Display for ToastTimeout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::DefaultDelay => f.write_str("Default delay"),
            Self::CustomDelay(_) => f.write_str("Custom delay"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
                    ToastTimeout::None => unreachable!(),
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
            toasts
                .iter()
                .position(|it| it.id == id)
                .map(|idx| toasts.remove(idx))
        })
    }

    /// Removes all toasts. Does not interfere with scheduled removals of pushed toasts.
    pub fn clear(&self) {
        self.set_toasts.update(Vec::clear);
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
pub fn ToastRoot(children: Children) -> impl IntoView {
    let (toasts, set_toasts) = create_signal(Vec::new());

    provide_context::<Toasts>(Toasts { toasts, set_toasts });

    view! {
        { children() }

        <leptonic-toasts>
            <For
                each=move || toasts.get()
                key=|toast| toast.id
                children=move |toast| {
                    view! {
                        <Toast toast/>
                    }
                }
            />
        </leptonic-toasts>
    }
}

// TODO: Incorporate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToastHorizontalPosition {
    Left,
    Right,
}

// TODO: Incorporate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToastVerticalPosition {
    Top,
    Bottom,
}

#[component]
pub fn Toast(toast: Toast) -> impl IntoView {
    let manually_closable = match toast.timeout {
        ToastTimeout::None => true,
        ToastTimeout::DefaultDelay => false,
        ToastTimeout::CustomDelay(duration) => duration.whole_seconds() > 10,
    };

    view! {
        <leptonic-toast id=toast.id.to_string() data-variant=toast.variant.as_str()>
            <leptonic-toast-header>
                { toast.header }

                { match manually_closable {
                    true => view! {
                        <div>
                            <Icon
                                class="dismiss"
                                icon=icondata::BsXCircleFill
                                on:click=move |_e| { expect_context::<Toasts>().try_remove(toast.id); }
                            />
                        </div>
                    }.into_view(),
                    false => ().into_view(),
                } }
            </leptonic-toast-header>
            <leptonic-toast-message>
                { toast.body }
            </leptonic-toast-message>
        </leptonic-toast>
    }
}
