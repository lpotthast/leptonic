use leptos::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(unused)]
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

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum ToastTimeout {
    None,
    DefaultDelay,
    CustomDelay(time::Duration),
}

#[derive(Copy, Clone)]
pub struct Toasts {
    pub toasts: ReadSignal<Vec<Toast>>,
    set_toasts: WriteSignal<Vec<Toast>>,
}

impl Toasts {
    /// Adds a toast and schedules its removal.
    pub fn push(&self, toast: Toast) {
        let t_id = toast.id.clone();
        let setter = self.set_toasts;
        setter.update(|toasts| toasts.push(toast));
        set_timeout(
            move || {
                setter.update(|toasts| {
                    if let Some(idx) = toasts.iter().position(|it| it.id == t_id) {
                        toasts.remove(idx);
                    }
                });
            },
            std::time::Duration::from_secs(3),
        );
    }

    /// Removes all toasts. Does not interfere with scheduled removals of pushed toasts.
    pub fn clear(&self) {
        self.set_toasts.update(|toasts| toasts.clear())
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

#[component]
pub fn Toast(cx: Scope, toast: Toast) -> impl IntoView {
    view! { cx,
        <leptonic-toast id=toast.id.to_string() variant=toast.variant.as_str()>
            <leptonic-toast-heading>
                { toast.header }
            </leptonic-toast-heading>
            <leptonic-toast-message>
                { toast.body }
            </leptonic-toast-message>
        </leptonic-toast>
    }
}
