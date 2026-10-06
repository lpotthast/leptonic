use leptos::{
    context::Provider,
    prelude::*,
    tachys::html::{class::class, style::style},
};

use super::press::PressResponder;
use crate::{
    hooks::{
        DialogRole, IntoAttrs, OverlayTriggerState, PressResponderTrigger, UseDialogInput,
        UseDialogReturn, UseOverlayTriggerStateInput, use_dialog, use_overlay_trigger_state,
    },
    utils::{
        CapturedElement,
        aria::AriaExpanded,
        classes::Classes,
        dev_warn,
        heading_level::HeadingLevel,
        id::use_id,
        slot_id::{SlotProps, use_slot},
        styles::Styles,
    },
};

/// Context provided by [`Dialog`] for its [`DialogTitle`] and [`DialogDescription`].
#[derive(Debug, Clone)]
struct DialogContext {
    title: StoredValue<SlotProps>,
    content: StoredValue<SlotProps>,
}

/// A dialog: `role="dialog"` (or `alertdialog`), named by its [`DialogTitle`] (or `aria_label`),
/// an alert dialog described by its [`DialogDescription`], focused when it opens. Render it in a
/// [`ModalContent`](super::modal::ModalContent) for a modal dialog.
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
    let trigger = use_context::<DialogTriggerContext>();
    // Without a title, `aria_label` or `aria_labelledby`, the trigger names the dialog
    // (react-aria-components). Its id is ensured once the dialog is rendered, before the hook
    // checks that the dialog has a name.
    let trigger_id = RwSignal::new(None);
    if let Some(trigger) = trigger {
        Effect::new(move |_| trigger_id.set(trigger.ensure_trigger_id()));
    }
    let UseDialogReturn {
        dialog_props,
        title_props,
        content_props,
    } = use_dialog(UseDialogInput {
        role,
        aria_label,
        aria_labelledby,
        aria_describedby,
        fallback_aria_labelledby: trigger_id.into(),
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
#[component]
pub fn DialogTitle(
    #[prop(optional)] level: HeadingLevel,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
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
#[component]
pub fn DialogDescription(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let props = use_context::<DialogContext>().map_or_else(
        || {
            dev_warn!("A <DialogDescription> must be inside a <Dialog>.");
            use_slot("dialog-content").props
        },
        |ctx| ctx.content.get_value(),
    );
    view! { <div {..props.into_attrs()} class=classes style=styles>{children()}</div> }
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
    /// The id the trigger element gets when it has none (see [`Self::ensure_trigger_id`]).
    generated_trigger_id: StoredValue<String>,
}

impl DialogTriggerContext {
    /// The context for an overlay opened by `trigger` (a `DialogTrigger`'s, a `MenuTrigger`'s).
    pub(crate) fn new(state: OverlayTriggerState, trigger: CapturedElement) -> Self {
        Self {
            state,
            trigger,
            overlay_id: RwSignal::new(None),
            generated_trigger_id: StoredValue::new(use_id("dialog-trigger")),
        }
    }

    /// The trigger element's id, once rendered: its own (`attr:id`), else a generated one it gets
    /// now. An untitled dialog is named by it. Called on demand (not on render), so the server
    /// and the hydrated page agree on the trigger's attributes.
    pub fn ensure_trigger_id(&self) -> Option<String> {
        let el = self.trigger.get()?;
        if el.id().is_empty() {
            el.set_id(&self.generated_trigger_id.get_value());
        }
        Some(el.id())
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
    /// Whether the overlay starts open. Ignored when `state` is given.
    #[prop(optional)]
    default_open: bool,
    /// Called when the overlay opens or closes. Ignored when `state` is given.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// The open state as app state (`state=rw_signal`) or a shared [`OverlayTriggerState`],
    /// replacing `default_open`.
    #[prop(into, optional)]
    state: Option<OverlayTriggerState>,
    children: Children,
) -> impl IntoView {
    let state = state.unwrap_or_else(|| {
        use_overlay_trigger_state(UseOverlayTriggerStateInput {
            default_open,
            on_open_change,
            ..UseOverlayTriggerStateInput::default()
        })
    });
    // The trigger needs an id to name an untitled dialog. Its own (`attr:id`) wins, which only the
    // rendered element shows (react-aria-components merges both ids into one).
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
