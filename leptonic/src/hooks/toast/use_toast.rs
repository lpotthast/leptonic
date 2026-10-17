// Upstream: react-aria/src/toast/useToast.ts @ 99e6102368
// Upstream: react-aria/test/toast/useToast.test.js @ 99e6102368
use leptos::prelude::*;

use super::use_toast_state::{QueuedToast, ToastQueue};
use crate::{
    Id, IntoAttrs, Part,
    hooks::button::UseButtonInput,
    utils::{
        aria::{AriaAtomic, AriaHidden, AriaModal, AriaRole},
        id::use_id,
        intl_strings::{ToastStrings, use_localized_strings},
        part::Remount,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        slot_id::{SlotProps, use_slot},
    },
};

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
    /// The toast (from the queue's `visible_toasts`).
    pub toast: QueuedToast<T>,
    pub aria_label: MaybeProp<String>,
    /// Default: the toast's title.
    pub aria_labelledby: Signal<Option<Id>>,
    /// Default: the toast's description.
    pub aria_describedby: Signal<Option<Id>>,
}

/// The return value of [`use_toast`].
#[derive(Debug)]
pub struct UseToastReturn {
    /// The toast element.
    pub toast: Part<UseToastProps>,
    /// Its content (an alert, announced when it appears).
    pub content: Part<UseToastContentProps>,
    /// Its title, which labels it.
    pub title: Part<UseToastTitleProps>,
    /// For the description (referenced while rendered).
    pub description: Part<SlotProps>,
    /// For the button closing it.
    pub close_button: UseButtonInput,
}

/// Props of a toast.
#[derive(Debug)]
pub struct UseToastProps {
    pub role: AriaRole,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Id>,
    pub aria_describedby: Signal<Option<Id>>,
}

impl Remount for UseToastProps {
    fn remount(&self) -> Self {
        Self {
            role: self.role,
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            aria_describedby: self.aria_describedby,
        }
    }
}

impl IntoAttrs for UseToastProps {
    type Attrs = (
        FixedAttr,
        FixedAttr,
        SignalAttr<Option<String>>,
        SignalAttr<Id>,
        SignalAttr<Option<Id>>,
        FixedAttr,
    );

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", self.role),
            fixed_attr("aria-modal", AriaModal::False),
            maybe_attr("aria-label", self.aria_label),
            signal_attr("aria-labelledby", self.aria_labelledby),
            signal_attr("aria-describedby", self.aria_describedby),
            fixed_attr("tabindex", 0),
        )
    }
}

/// Props of a toast's content.
#[derive(Debug)]
pub struct UseToastContentProps {
    /// Hidden until mounted: NVDA announces the alert only when it becomes visible.
    pub aria_hidden: Signal<Option<AriaHidden>>,
}

impl IntoAttrs for UseToastContentProps {
    type Attrs = (FixedAttr, FixedAttr, SignalAttr<Option<AriaHidden>>);

    fn into_attrs(self) -> Self::Attrs {
        (
            fixed_attr("role", AriaRole::Alert),
            fixed_attr("aria-atomic", AriaAtomic::True),
            signal_attr("aria-hidden", self.aria_hidden),
        )
    }
}

/// Props of a toast's title.
#[derive(Debug)]
pub struct UseToastTitleProps {
    pub id: Id,
}

impl IntoAttrs for UseToastTitleProps {
    type Attrs = FixedAttr;

    fn into_attrs(self) -> Self::Attrs {
        fixed_attr("id", self.id)
    }
}

/// Behavior and accessibility of a toast (react-aria's `useToast`): a non-modal alert dialog
/// labelled by its title, whose content is announced; its timeout runs while it is shown.
pub fn use_toast<T: Clone + Send + Sync + 'static>(input: UseToastInput<T>) -> UseToastReturn {
    crate::metrics::subject!();
    let UseToastInput {
        queue,
        toast,
        aria_label,
        aria_labelledby,
        aria_describedby,
    } = input;

    // The timeout runs while the toast is shown.
    if let (Some(timer), Some(timeout)) = (toast.timer.clone(), toast.timeout) {
        let paused_timer = timer.clone();
        Effect::new(move |_| timer.reset(timeout));
        on_cleanup(move || paused_timer.pause());
    }

    let title_id = use_id("toast-title");
    let description = use_slot("toast-description");
    let is_visible = RwSignal::new(false);
    Effect::new(move |_| is_visible.set(true));

    let key = toast.key;
    let referenced_description = description.referenced_id;
    UseToastReturn {
        toast: Part::stored(UseToastProps {
            role: AriaRole::Alertdialog,
            aria_label,
            aria_labelledby: {
                let title_id = title_id.clone();
                Signal::derive(move || aria_labelledby.get().unwrap_or_else(|| title_id.clone()))
            },
            aria_describedby: Signal::derive(move || {
                aria_describedby
                    .get()
                    .or_else(|| referenced_description.get())
            }),
        }),
        content: {
            let aria_hidden =
                Signal::derive(move || (!is_visible.get()).then_some(AriaHidden::True));
            Part::new(move || UseToastContentProps { aria_hidden })
        },
        title: Part::new(move || UseToastTitleProps {
            id: title_id.clone(),
        }),
        description: description.part,
        close_button: UseButtonInput {
            aria_label: {
                let strings = use_localized_strings::<ToastStrings>();
                Signal::derive(move || Some(strings.read().close())).into()
            },
            on_press: Some(Callback::new(move |_| queue.close(key))),
            ..UseButtonInput::default()
        },
    }
}
