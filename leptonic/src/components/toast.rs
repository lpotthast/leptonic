//! Styled toasts on the toast atoms: a `ToastRoot` (part of `Root`) shows the toasts pushed to the
//! `Toasts` context.
use std::time::Duration;

use leptos::prelude::*;

use crate::{
    atoms::toast::{
        Toast as ToastAtom, ToastCloseButton, ToastContent, ToastDescription, ToastRegion,
        ToastTitle,
    },
    components::icon::Icon,
    hooks::{ToastOptions, ToastQueue},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter, Default)]
pub enum ToastVariant {
    Success,
    #[default]
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

/// When a toast closes by itself. Hovering or focusing the toasts pauses the time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ToastTimeout {
    /// Only when closed (toasts with actions shouldn't time out).
    None,
    /// After 5 seconds (the least react-aria recommends).
    #[default]
    DefaultDelay,
    CustomDelay(Duration),
}

impl ToastTimeout {
    /// The default delay.
    pub const DEFAULT_DELAY: Duration = Duration::from_secs(5);

    pub const fn duration(self) -> Option<Duration> {
        match self {
            Self::None => None,
            Self::DefaultDelay => Some(Self::DEFAULT_DELAY),
            Self::CustomDelay(delay) => Some(delay),
        }
    }
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

/// A toast: its header names it, its body describes it.
#[derive(Clone)]
pub struct Toast {
    pub variant: ToastVariant,
    pub header: ViewFn,
    pub body: ViewFn,
    pub timeout: ToastTimeout,
}

impl std::fmt::Debug for Toast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Toast")
            .field("variant", &self.variant)
            .field("timeout", &self.timeout)
            .finish_non_exhaustive()
    }
}

/// The toasts shown by the `ToastRoot` around (a context: `expect_context::<Toasts>()`).
#[derive(Clone, Copy)]
pub struct Toasts {
    queue: ToastQueue<Toast>,
}

impl std::fmt::Debug for Toasts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Toasts").finish_non_exhaustive()
    }
}

impl Toasts {
    /// Shows a toast (above the others); returns its key, to close it with.
    pub fn push(&self, toast: Toast) -> String {
        let timeout = toast.timeout.duration();
        self.queue.add(
            toast,
            ToastOptions {
                timeout,
                on_close: None,
            },
        )
    }

    /// Closes a toast.
    pub fn close(&self, key: &str) {
        self.queue.close(key);
    }

    /// Closes all toasts.
    pub fn clear(&self) {
        self.queue.clear();
    }

    /// The queue (e.g. for `on_close` callbacks: `queue().add(..)`).
    pub const fn queue(&self) -> ToastQueue<Toast> {
        self.queue
    }
}

/// Provides `Toasts` and shows them: a region at the bottom of the page (a landmark F6 reaches),
/// each toast an alert dialog with a close button. Part of `Root`.
#[component]
pub fn ToastRoot(
    /// How many toasts show at once (default: all).
    #[prop(optional)]
    max_visible_toasts: Option<usize>,
    children: Children,
) -> impl IntoView {
    let queue = ToastQueue::<Toast>::new(max_visible_toasts);
    provide_context(Toasts { queue });

    view! {
        {children()}
        <ToastRegion queue=queue classes="leptonic-toasts" let:toast>
            {
                let Toast { variant, header, body, .. } = toast.content.clone();
                view! {
                    <ToastAtom toast=toast classes="leptonic-toast" attr:data-variant=variant.as_str()>
                        <ToastContent classes="leptonic-toast-content">
                            <ToastTitle classes="leptonic-toast-header">{header.run()}</ToastTitle>
                            <ToastDescription classes="leptonic-toast-message">
                                {body.run()}
                            </ToastDescription>
                        </ToastContent>
                        <ToastCloseButton classes="leptonic-toast-close">
                            <Icon icon=icondata::BsXCircleFill />
                        </ToastCloseButton>
                    </ToastAtom>
                }
            }
        </ToastRegion>
    }
}
