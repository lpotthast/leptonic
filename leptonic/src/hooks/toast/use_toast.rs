// Upstream: react-aria/src/toast/useToast.ts @ 99e6102368
use leptos::{
    attr::{self, Attr},
    prelude::*,
};

use super::use_toast_state::{QueuedToast, ToastQueue};
use crate::{
    hooks::{IntoAttrs, UseButtonInput},
    utils::{
        aria::{AriaHidden, AriaModal, AriaRole},
        id::use_id,
        slot_id::{SlotProps, use_slot},
    },
};
use crate::utils::intl_strings::{ToastStrings, use_localized_strings};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The queue goes into the input (C8; react-aria: `state`).
// - Returns the close button's `UseButtonInput` and the title's id (react-aria: props objects).
//
// =============================================================================

/// Input of [`use_toast`].
#[derive(Clone)]
pub struct UseToastInput<T: Clone + Send + Sync + 'static> {
    /// The queue the toast is in (closing removes it from there).
    pub queue: ToastQueue<T>,
    pub toast: QueuedToast<T>,
    pub aria_label: MaybeProp<String>,
    /// Default: the toast's title.
    pub aria_labelledby: Option<String>,
    /// Default: the toast's description.
    pub aria_describedby: Option<String>,
}

/// Return value of [`use_toast`].
pub struct UseToastReturn {
    /// For the toast (a non-modal alert dialog).
    pub toast_props: UseToastProps,
    /// For its content (an alert, announced when it appears).
    pub content_props: UseToastContentProps,
    /// The id of the title.
    pub title_id: String,
    /// For the description (referenced while rendered).
    pub description_props: SlotProps,
    /// For the button closing it.
    pub close_button: UseButtonInput,
}

/// Props of a toast.
#[derive(Debug, Clone)]
pub struct UseToastProps {
    pub role: AriaRole,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: String,
    pub aria_describedby: Signal<Option<String>>,
}

impl IntoAttrs for UseToastProps {
    type Attrs = (
        Attr<attr::Role, AriaRole>,
        Attr<attr::AriaModal, AriaModal>,
        Attr<attr::AriaLabel, MaybeProp<String>>,
        Attr<attr::AriaLabelledby, String>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        Attr<attr::Tabindex, i32>,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, self.role),
            Attr(attr::AriaModal, AriaModal::False),
            Attr(attr::AriaLabel, self.aria_label),
            Attr(attr::AriaLabelledby, self.aria_labelledby),
            Attr(attr::AriaDescribedby, self.aria_describedby),
            Attr(attr::Tabindex, 0),
        )
    }
}

/// Props of a toast's content.
#[derive(Debug, Clone)]
pub struct UseToastContentProps {
    /// Hidden until mounted: NVDA announces the alert only when it becomes visible.
    pub aria_hidden: Signal<Option<AriaHidden>>,
}

impl IntoAttrs for UseToastContentProps {
    type Attrs = (
        Attr<attr::Role, AriaRole>,
        Attr<attr::AriaAtomic, &'static str>,
        Attr<attr::AriaHidden, Signal<Option<AriaHidden>>>,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            Attr(attr::Role, AriaRole::Alert),
            Attr(attr::AriaAtomic, "true"),
            Attr(attr::AriaHidden, self.aria_hidden),
        )
    }
}

/// Behavior and accessibility of a toast (react-aria's `useToast`): a non-modal alert dialog
/// labelled by its title, whose content is announced; its timeout runs while it is shown.
pub fn use_toast<T: Clone + Send + Sync + 'static>(input: UseToastInput<T>) -> UseToastReturn {
    let UseToastInput {
        queue,
        toast,
        aria_label,
        aria_labelledby,
        aria_describedby,
    } = input;

    // The timeout runs while the toast is shown.
    if let (Some(timer), Some(timeout)) = (toast.timer.clone(), toast.timeout) {
        Effect::new(move |_| timer.reset(timeout));
        let timer = toast.timer.clone();
        on_cleanup(move || {
            if let Some(timer) = timer {
                timer.pause();
            }
        });
    }

    let title_id = use_id("toast-title");
    let description = use_slot("toast-description");
    let is_visible = RwSignal::new(false);
    Effect::new(move |_| is_visible.set(true));

    let key = toast.key;
    let referenced_description = description.referenced_id;
    UseToastReturn {
        toast_props: UseToastProps {
            role: AriaRole::Alertdialog,
            aria_label,
            aria_labelledby: aria_labelledby.unwrap_or_else(|| title_id.clone()),
            aria_describedby: match aria_describedby {
                Some(ids) => Signal::stored(Some(ids)),
                None => referenced_description,
            },
        },
        content_props: UseToastContentProps {
            aria_hidden: Signal::derive(move || (!is_visible.get()).then_some(AriaHidden::True)),
        },
        title_id,
        description_props: description.props,
        close_button: UseButtonInput {
            aria_label: {
                let strings = use_localized_strings::<ToastStrings>();
                Signal::derive(move || Some(strings.read().close())).into()
            },
            on_press: Some(Callback::new(move |_| queue.close(&key))),
            ..UseButtonInput::default()
        },
    }
}
