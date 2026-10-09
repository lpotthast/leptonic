// Upstream: react-aria-components/src/Dialog.tsx @ 99e6102368
// Upstream: react-aria-components/test/Dialog.test.js @ 99e6102368
// Upstream: react-aria-components/test/Dialog.browser.test.tsx @ 99e6102368
use leptos::{
    context::Provider,
    prelude::*,
    tachys::html::{class::class, style::style},
};
use leptos_classes::Classes;

use super::press::PressResponder;
use crate::{
    CapturedElement, IntoAttrs, Out,
    hooks::{
        dialog::{DialogRole, UseDialogInput, UseDialogReturn, use_dialog},
        interactions::PressResponderTrigger,
        overlay::{OverlayTriggerState, UseOverlayTriggerStateInput, use_overlay_trigger_state},
    },
    utils::{
        aria::AriaExpanded,
        default_class::with_default_class,
        dev_warn,
        heading_level::HeadingLevel,
        id::use_id,
        slot_id::{SlotProps, use_slot},
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `DialogTitle` and `DialogDescription` instead of `Heading slot="title"` and a described-by
//   slot; the title's level is the typed `HeadingLevel`.
// - `DialogTrigger`'s open state (C4): `default_open` + `on_open_change`, or `is_open` +
//   `set_open`.
// - Render props (`close`) are not offered: close through the trigger's or the overlay's open
//   state.
// - The trigger gets its props through a `PressResponder`; an untitled dialog is named by the
//   trigger's rendered id (its own `attr:id`, else a generated one), ensured once the dialog
//   renders (react-aria-components merges both ids into one).
//
// =============================================================================

/// Context provided by [`Dialog`] for its [`DialogTitle`] and [`DialogDescription`].
#[derive(Debug, Clone)]
struct DialogContext {
    title: StoredValue<SlotProps>,
    content: StoredValue<SlotProps>,
}

/// A dialog: `role="dialog"` (or `alertdialog`), named by its [`DialogTitle`] (or `aria_label`),
/// an alert dialog described by its [`DialogDescription`], focused when it opens. Render it in a
/// [`ModalContent`](super::modal::ModalContent) for a modal dialog.
///
/// Default class: `leptonic-Dialog`.
#[component]
pub fn Dialog(
    #[prop(optional)] role: DialogRole,
    /// Names the dialog when it has no `DialogTitle`.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    /// The ids of the elements naming the dialog, instead of its title.
    #[prop(into, optional)]
    aria_labelledby: Option<String>,
    /// The ids of the elements describing the dialog. Default for alert dialogs: its
    /// `DialogDescription`.
    #[prop(into, optional)]
    aria_describedby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Dialog", classes);
    let trigger = use_context::<DialogTriggerContext>();
    // Without a title, `aria_label` or `aria_labelledby`, the trigger names the dialog
    // (react-aria-components).
    let UseDialogReturn {
        dialog_props,
        title_props,
        content_props,
    } = use_dialog(UseDialogInput {
        role,
        aria_label,
        aria_labelledby,
        aria_describedby,
        fallback_aria_labelledby: trigger
            .and_then(|trigger| trigger.dialog_labelledby)
            .or_else(|| {
                trigger.map(|trigger| {
                    let trigger_id = trigger.trigger_id;
                    Signal::derive(move || Some(trigger_id.get()))
                })
            })
            .unwrap_or_default(),
        ..UseDialogInput::default()
    });
    // A dialog opened by a `DialogTrigger` is what its trigger controls (react-aria-components passes
    // the overlay id to the dialog).
    if let Some(trigger) = trigger {
        trigger.overlay_id.set(Some(dialog_props.id.clone()));
    }
    let context = DialogContext {
        title: StoredValue::new(title_props),
        content: StoredValue::new(content_props),
    };

    view! {
        <section {..dialog_props.into_attrs()} class=classes style=styles>
            <Provider value=context>{children()}</Provider>
        </section>
    }
}

/// The title of the [`Dialog`] around it: a heading (`level`, default `<h2>`) naming the dialog.
///
/// Default class: `leptonic-DialogTitle`.
#[component]
pub fn DialogTitle(
    #[prop(optional)] level: HeadingLevel,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DialogTitle", classes);
    let props = use_context::<DialogContext>().map_or_else(
        || {
            dev_warn!("A <DialogTitle> must be inside a <Dialog>.");
            use_slot("dialog-title").props
        },
        |ctx| ctx.title.get_value(),
    );
    level
        .render(children)
        .add_any_attr(props.into_attrs())
        .add_any_attr(class(classes))
        .add_any_attr(style(styles))
}

/// The description of the [`Dialog`] around it (an alert dialog's `aria-describedby`).
///
/// Default class: `leptonic-DialogDescription`.
#[component]
pub fn DialogDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let classes = with_default_class("leptonic-DialogDescription", classes);
    let props = use_context::<DialogContext>().map_or_else(
        || {
            dev_warn!("A <DialogDescription> must be inside a <Dialog>.");
            use_slot("dialog-content").props
        },
        |ctx| ctx.content.get_value(),
    );
    view! { <div {..props.into_attrs()} class=classes style=styles>{children()}</div> }
}

/// The open state of an overlay atom (`ModalBackdrop`, `Popover`) from its state props, as RAC's
/// overlays: its own state when it gets `is_open` or `default_open`, or isn't in a
/// [`DialogTrigger`]; the trigger's state otherwise (then `set_open` and `on_open_change` belong on
/// the trigger).
pub(crate) fn overlay_open_state(
    is_open: Option<Signal<bool>>,
    set_open: Option<crate::Out<bool>>,
    default_open: Option<bool>,
    on_open_change: Option<Callback<bool>>,
    context: Option<DialogTriggerContext>,
) -> OverlayTriggerState {
    match context {
        Some(context) if is_open.is_none() && default_open.is_none() => {
            if set_open.is_some() || on_open_change.is_some() {
                dev_warn!(
                    "An overlay inside a <DialogTrigger> ignores its own `set_open` and \
                     `on_open_change` (without `is_open` or `default_open`, the trigger owns the \
                     open state): set them on the trigger."
                );
            }
            context.state
        }
        _ => {
            let (value, on_open_change) =
                crate::ValueBinding::from_state_props(is_open, set_open, on_open_change);
            use_overlay_trigger_state(UseOverlayTriggerStateInput {
                default_open: default_open.unwrap_or(false),
                value,
                on_open_change,
            })
        }
    }
}

/// Context from a [`DialogTrigger`] to the overlay it opens ([`Popover`](super::popover::Popover),
/// [`ModalBackdrop`](super::modal::ModalBackdrop)).
#[derive(Debug, Clone, Copy)]
pub struct DialogTriggerContext {
    /// Whether the overlay is open.
    pub state: OverlayTriggerState,
    /// The element that opens the overlay (the trigger's pressable child), for positioning.
    pub trigger: CapturedElement,
    /// The id of the open overlay, set by it: the trigger's `aria-controls`.
    pub overlay_id: RwSignal<Option<String>>,
    /// The trigger element's id (see [`PressResponderTrigger::id`]): an untitled dialog or menu
    /// is named by it.
    pub trigger_id: RwSignal<String>,
    /// What names an untitled dialog instead of the trigger (e.g. a date picker's button and
    /// label).
    pub(crate) dialog_labelledby: Option<Signal<Option<String>>>,
}

impl DialogTriggerContext {
    /// The context for an overlay opened by `trigger` (a `DialogTrigger`'s, a `MenuTrigger`'s).
    pub(crate) fn new(state: OverlayTriggerState, trigger: CapturedElement) -> Self {
        Self {
            state,
            trigger,
            overlay_id: RwSignal::new(None),
            trigger_id: RwSignal::new(use_id("dialog-trigger")),
            dialog_labelledby: None,
        }
    }

    /// An untitled dialog is named by `labelledby` instead of the trigger.
    #[must_use]
    pub(crate) fn with_dialog_labelledby(mut self, labelledby: Signal<Option<String>>) -> Self {
        self.dialog_labelledby = Some(labelledby);
        self
    }
}

/// Opens an overlay (a [`Popover`](super::popover::Popover) or a
/// [`ModalBackdrop`](super::modal::ModalBackdrop) with a dialog) when its pressable child (a
/// `Button`) is pressed. The child gets `aria-expanded`/`aria-controls`; the overlay finds its state
/// and trigger through context.
///
/// ```ignore
/// <DialogTrigger>
///     <Button>"Settings"</Button>
///     <Popover>
///         <Dialog>
///             <DialogTitle>"Settings"</DialogTitle>
///         </Dialog>
///     </Popover>
/// </DialogTrigger>
/// ```
#[component]
pub fn DialogTrigger(
    /// Whether the overlay starts open. Ignored with `is_open`.
    #[prop(optional)]
    default_open: bool,
    /// Called when the overlay opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// Whether the overlay is open (controlled): a value or any signal.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the open state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,
    children: Children,
) -> impl IntoView {
    let (value, on_open_change) =
        crate::ValueBinding::from_state_props(is_open, set_open, on_open_change);
    let state = use_overlay_trigger_state(UseOverlayTriggerStateInput {
        default_open,
        value,
        on_open_change,
    });
    // The trigger needs an id to name an untitled dialog: the context's, or the trigger element's
    // own, which it writes there (react-aria-components merges both ids into one).
    let context = DialogTriggerContext::new(state, CapturedElement::new());
    let DialogTriggerContext {
        trigger,
        overlay_id,
        ..
    } = context;
    let is_open = state.is_open;
    // As react-aria's `useOverlayTrigger` for dialogs: no `aria-haspopup`.
    let press_trigger = PressResponderTrigger {
        aria_haspopup: Signal::stored(None),
        aria_expanded: Signal::derive(move || Some(AriaExpanded::from(is_open.get()))),
        aria_controls: Signal::derive(move || is_open.get().then(|| overlay_id.get()).flatten()),
        element: trigger,
        id: context.trigger_id,
    };

    view! {
        <Provider value=context>
            <PressResponder
                on_press=move |_| state.toggle()
                force_is_pressed=is_open
                trigger=press_trigger
            >
                {children()}
            </PressResponder>
        </Provider>
    }
}
