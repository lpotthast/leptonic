// Upstream: react-aria-components/src/Popover.tsx @ 99e6102368
use leptos::{context::Provider, either::Either, portal::Portal, prelude::*};

use super::{
    dialog::DialogTriggerContext, dismiss_button::DismissButton, focus_scope::FocusScope,
    overlay_arrow::OverlayArrowContext, press::ClearTriggerContexts,
};
use crate::{
    Out,
    hooks::{
        OverlayFocusContain, OverlayState, Placement, PlacementAxis, PopoverModality,
        PropsWithStyles, UseEnterAnimationInput, UseExitAnimationInput, UseOverlayArrowProps,
        UsePopoverInput, UsePopoverProps, UsePopoverReturn, use_enter_animation,
        use_exit_animation, use_popover,
    },
    utils::{
        CapturedElement, classes::Classes, data_attributes::flag,
        default_class::with_default_class, focus::focus_safely, point::Point,
        shadow_dom::get_active_element, styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - The open state and `trigger` come from a surrounding `DialogTrigger` (react-aria-components:
//   `OverlayTriggerStateContext` and `triggerRef`), or are passed directly: `is_open` +
//   `set_open` (C4), `default_open`, `on_open_change`.
// - `placement` is the typed `Placement` enum.
// - Render props become the `data-placement` attribute plus plain children.
// - Whether a modal popover is a dialog (no `[role=dialog]` inside) is decided once it is rendered;
//   its label (an own `aria_labelledby`, else the `DialogTrigger`'s trigger) is set then.
//
// ## OMITTED FEATURES
// - `isEntering`/`isExiting` animations, `UNSTABLE_portalContainer`, submenu triggers.
//
// =============================================================================

/// A popover: an overlay positioned at its trigger. Inside a [`DialogTrigger`](super::dialog::DialogTrigger)
/// it opens when the trigger is pressed. Put a [`Dialog`](super::dialog::Dialog) in it, or let a
/// modal popover be the dialog itself: without a dialog inside it gets `role="dialog"`, is focused
/// when it opens (unless focus already moved into it) and is named by `aria_label`,
/// `aria_labelledby` or its trigger.
///
/// Modal by default (react-aria-components): focus stays inside, the rest of the page is hidden
/// from assistive technology and doesn't scroll, and screen reader users get dismiss buttons. A
/// non-modal popover leaves the page usable and closes when it scrolls. Escape, interacting outside
/// and moving focus out close it.
///
/// Put an [`OverlayArrow`](super::overlay_arrow::OverlayArrow) in it for an arrow pointing at the
/// trigger.
///
/// Data attributes: `data-placement` (`top`, `bottom`, `left` or `right`, after flipping). CSS
/// variables: `--trigger-width` (the trigger's width, e.g. for a popover as wide as its trigger)
/// and `--trigger-anchor-point` (the point closest to the trigger, e.g. as `transform-origin`).
///
/// Default class: `leptonic-Popover`.
#[component]
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn Popover(
    /// Whether the popover is open (controlled): a value or any signal. Default: the surrounding
    /// `DialogTrigger`'s state.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the open state (closing): an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,
    /// Whether it starts open, with its own state. Ignored with `is_open`.
    #[prop(optional, into)]
    default_open: Option<bool>,
    /// Called when it opens or closes.
    #[prop(into, optional)]
    on_open_change: Option<Callback<bool>>,
    /// The element the popover is positioned at. Default: the surrounding `DialogTrigger`'s trigger.
    #[prop(optional)]
    trigger: Option<CapturedElement>,
    /// Where the popover goes relative to the trigger. Default: below it; next to the trigger
    /// item (`EndTop`) for a submenu.
    #[prop(into, optional)]
    placement: Option<Signal<Placement>>,
    /// The distance from the trigger, in pixels.
    #[prop(into, optional)]
    offset: Option<Signal<f64>>,
    #[prop(into, optional)] cross_offset: Signal<f64>,
    /// The minimum distance from the viewport edges, in pixels.
    #[prop(into, default = Signal::stored(12.0))]
    container_padding: Signal<f64>,
    /// Whether the popover flips to the other side when there is no room.
    #[prop(into, default = Signal::stored(true))]
    should_flip: Signal<bool>,
    /// The popover's maximum height. Default: the room available.
    #[prop(into, optional)]
    max_height: Signal<Option<f64>>,
    /// The minimum distance between an `OverlayArrow` and the popover's edges.
    #[prop(into, optional)]
    arrow_boundary_offset: Signal<f64>,
    /// Whether the popover takes over the page while open. Default: modal; non-modal for a
    /// submenu.
    #[prop(optional, into)]
    modality: Option<PopoverModality>,
    #[prop(into, optional)] is_keyboard_dismiss_disabled: Signal<bool>,
    /// Which outside interactions close the popover: `true` closes.
    #[prop(optional)]
    should_close_on_interact_outside: Option<crate::hooks::InteractOutsideFilter>,
    /// Names the popover when it is a dialog itself.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,
    /// The ids of the elements naming the popover when it is a dialog itself. Default: its
    /// `DialogTrigger`'s trigger.
    #[prop(into, optional)]
    aria_labelledby: Option<String>,
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let classes = with_default_class("leptonic-Popover", classes);
    let context = use_context::<DialogTriggerContext>();
    let state =
        super::dialog::overlay_open_state(is_open, set_open, default_open, on_open_change, context);
    let trigger = trigger
        .or(context.map(|ctx| ctx.trigger))
        .expect("a <Popover> needs a `trigger` or a surrounding <DialogTrigger>");
    // A submenu's popover belongs to the group of its root popover.
    let submenu = use_context::<Option<SubmenuPopoverContext>>().flatten();
    let group = match (submenu, use_context::<PopoverGroupContext>()) {
        (Some(_), Some(root)) => PopoverGroup::Sub(root.0),
        _ => PopoverGroup::Root(CapturedElement::new()),
    };
    let defaults = use_context::<Option<PopoverDefaults>>().flatten();
    let placement = placement.unwrap_or_else(|| {
        Signal::stored(match (submenu, defaults) {
            (Some(_), _) => Placement::EndTop,
            (None, Some(defaults)) => defaults.placement,
            (None, None) => Placement::Bottom,
        })
    });
    let offset =
        offset.unwrap_or_else(|| Signal::stored(defaults.map_or(8.0, |defaults| defaults.offset)));
    let modality = modality.unwrap_or(if submenu.is_some() {
        PopoverModality::NonModal
    } else {
        PopoverModality::Modal
    });
    let should_close_on_interact_outside = should_close_on_interact_outside
        .or_else(|| submenu.map(|submenu| submenu.should_close_on_interact_outside.get_value()));
    let aria_labelledby =
        aria_labelledby.or_else(|| submenu.map(|submenu| submenu.aria_labelledby.get_value()));

    let UsePopoverReturn {
        props,
        id,
        arrow_props,
        placement: resolved_placement,
        trigger_anchor_point,
        ..
    } = use_popover(UsePopoverInput {
        trigger,
        placement,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        max_height,
        arrow_boundary_offset,
        modality,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        group: Some(group.element()),
        is_submenu: submenu.is_some(),
        state,
        arrow_size: Signal::stored(None),
        boundary: None,
        target_rect: Signal::stored(None),
    });
    // The trigger's `aria-controls`.
    if let Some(context) = context {
        context.overlay_id.set(Some(id.to_string()));
    }

    render_popover(
        state,
        PopoverParts {
            props,
            arrow_props,
            placement: resolved_placement,
            trigger_anchor_point,
            trigger,
            trigger_name: context.is_some().then_some(match (submenu, defaults) {
                (Some(_), _) => "SubmenuTrigger",
                (None, Some(_)) => "MenuTrigger",
                (None, None) => "DialogTrigger",
            }),
        },
        modality,
        CapturedElement::new(),
        group,
        PopoverDialogLabel {
            aria_label,
            aria_labelledby: move || {
                aria_labelledby
                    .clone()
                    .or_else(|| context.and_then(|ctx| ctx.ensure_trigger_id()))
            },
        },
        classes,
        styles,
        children,
    )
}

/// What a `SubmenuTrigger` tells the `Popover` of its submenu (react-aria-components' popover
/// context values for submenus).
#[derive(Debug, Clone, Copy)]
pub(crate) struct SubmenuPopoverContext {
    pub should_close_on_interact_outside: StoredValue<crate::hooks::InteractOutsideFilter>,
    /// The trigger item, naming the popover if it is a dialog.
    pub aria_labelledby: StoredValue<String>,
}

/// What a trigger (a `MenuTrigger`) sets as default for its popover (react-aria-components'
/// popover context values).
#[derive(Debug, Clone, Copy)]
pub(crate) struct PopoverDefaults {
    pub placement: Placement,
    pub offset: f64,
}

/// The container of a root popover, which also holds the popovers of its submenus
/// (react-aria-components' `PopoverGroupContext`).
#[derive(Debug, Clone, Copy)]
pub(crate) struct PopoverGroupContext(CapturedElement);

/// Where a popover sits in a group of popovers.
#[derive(Debug, Clone, Copy)]
pub(crate) enum PopoverGroup {
    /// A root popover, rendering the group's container around itself.
    Root(CapturedElement),
    /// A submenu's popover, mounted into its root popover's container.
    Sub(CapturedElement),
}

impl PopoverGroup {
    /// The group's container.
    pub(crate) fn element(self) -> CapturedElement {
        match self {
            Self::Root(element) | Self::Sub(element) => element,
        }
    }
}

/// What names a popover that is a dialog itself.
pub(crate) struct PopoverDialogLabel<L> {
    pub aria_label: MaybeProp<String>,
    /// The ids of the naming elements, read once the popover turned out to be a dialog.
    pub aria_labelledby: L,
}

/// The parts of a [`UsePopoverReturn`] the rendering needs, and the trigger.
pub(crate) struct PopoverParts {
    pub props: PropsWithStyles<UsePopoverProps>,
    pub arrow_props: PropsWithStyles<UseOverlayArrowProps>,
    pub placement: Signal<Option<PlacementAxis>>,
    pub trigger_anchor_point: Signal<Option<Point>>,
    pub trigger: CapturedElement,
    /// What opened the popover, as `data-trigger` (react-aria-components' component names:
    /// `MenuTrigger`, `SubmenuTrigger`, `DialogTrigger`, `Select`, `ComboBox`).
    pub trigger_name: Option<&'static str>,
}

/// Renders a popover (the `Popover` atom, and the select's and combo box's popovers): in a portal
/// while open, a modal one with an underlay, focus kept inside and a leading dismiss button,
/// every one with a trailing dismiss button, `data-placement`, `--trigger-width`,
/// `--trigger-anchor-point`, an `OverlayArrow` context and the trigger's press responder cleared
/// for its content. `capture` receives the popover element as well. A modal popover
/// without a dialog inside is the dialog (react-aria-components): `role="dialog"`, named by
/// `label`, focused once rendered unless focus is already inside.
#[allow(clippy::too_many_arguments)]
pub(crate) fn render_popover<S: OverlayState, L: Fn() -> Option<String> + 'static>(
    state: S,
    parts: PopoverParts,
    modality: PopoverModality,
    capture: CapturedElement,
    group: PopoverGroup,
    label: PopoverDialogLabel<L>,
    classes: Classes,
    styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let PopoverParts {
        props,
        arrow_props,
        placement,
        trigger_anchor_point,
        trigger,
        trigger_name,
    } = parts;
    let trigger_width =
        leptos_use::use_element_bounding(Signal::derive(move || trigger.get())).width;

    let (attrs, popover_styles) = props.into_parts();
    let attrs = StoredValue::new(attrs);
    let classes = StoredValue::new(classes);
    // The trigger's width and the anchor point as CSS variables (react-aria-components).
    let variables = Styles::builder()
        .with_optional_unchecked("--trigger-width", move || {
            let width = trigger_width.get();
            (width > 0.0).then(|| format!("{width}px"))
        })
        .with_optional_unchecked("--trigger-anchor-point", move || {
            trigger_anchor_point
                .get()
                .map(|point| format!("{}px {}px", point.x, point.y))
        })
        .build();
    let styles = StoredValue::new(popover_styles.merge(variables).merge(styles));
    let arrow_context = StoredValue::new(OverlayArrowContext::new(arrow_props, placement));
    let children = StoredValue::new(children);
    let close = Callback::new(move |()| state.close());

    let PopoverDialogLabel {
        aria_label,
        aria_labelledby,
    } = label;
    let is_dialog = RwSignal::new(false);
    let dialog_labelledby = RwSignal::new(None::<String>);
    let is_submenu = matches!(group, PopoverGroup::Sub(_));
    // Once rendered: a modal popover or a submenu's (subdialog's) without a dialog inside is the
    // dialog.
    Effect::new(move |_| {
        let Some(el) = capture.get() else {
            return;
        };
        let has_dialog = el.query_selector("[role=dialog]").ok().flatten().is_some();
        let dialog = (modality.is_modal() || is_submenu) && !has_dialog;
        if dialog {
            dialog_labelledby.set(aria_labelledby());
        }
        is_dialog.set(dialog);
    });
    // Focus the popover itself once it is a dialog (and focusable), unless focus already moved
    // into it. Hovering a submenu trigger keeps focus on the trigger.
    Effect::new(move |_| {
        if !is_dialog.get()
            || (is_submenu && crate::hooks::get_modality() == crate::hooks::Modality::Pointer)
        {
            return;
        }
        let Some(el) = capture.get_untracked() else {
            return;
        };
        let focus_within = el
            .owner_document()
            .as_ref()
            .and_then(get_active_element)
            .is_some_and(|active| el.contains(Some(&active)));
        if !focus_within {
            focus_safely(&el);
        }
    });

    // While the popover's exit animations run, it stays rendered (`data-exiting`).
    let is_open = Signal::derive(move || state.is_open());
    let is_exiting = use_exit_animation(UseExitAnimationInput {
        element: capture,
        is_open,
        on_exit: None,
    })
    .is_exiting;

    // The popover, in a focus scope (containing focus while `contain`). `entering` tracks this
    // opening's element.
    let popover =
        move |contain: Signal<bool>, entering: CapturedElement, is_entering: Signal<bool>| {
            view! {
                <FocusScope contain=contain restore_focus=true>
                    <div
                        {..attrs.get_value()}
                        {..capture.attr().chain(entering.attr())}
                        data-entering=flag(is_entering)
                        data-exiting=flag(is_exiting)
                        class=classes.get_value()
                        style=styles.get_value()
                        data-placement=move || placement.get().map(PlacementAxis::as_str)
                        data-trigger=trigger_name
                        role=move || is_dialog.get().then_some("dialog")
                        tabindex=move || is_dialog.get().then_some(-1)
                        aria-label=move || is_dialog.get().then(|| aria_label.get()).flatten()
                        aria-labelledby=move || {
                            is_dialog.get().then(|| dialog_labelledby.get()).flatten()
                        }
                    >
                        // Pressing in the popover must not toggle it through the trigger's responder.
                        <ClearTriggerContexts>
                            <Provider value=arrow_context.get_value()>
                                {modality
                                    .is_modal()
                                    .then(|| view! { <DismissButton on_dismiss=close /> })}
                                {(children.get_value())()}
                                <DismissButton on_dismiss=close />
                            </Provider>
                        </ClearTriggerContexts>
                    </div>
                </FocusScope>
            }
        };
    // A modal popover's underlay catches interaction with the page.
    let underlay = move || {
        move || {
            (modality.is_modal() && is_open.get())
                .then(|| view! { <div style="position: fixed; inset: 0;" /> })
        }
    };

    view! {
            // No portal container while closed: a modal would make it inert.
            <Show when=move || is_open.get() || is_exiting.get()>
                {
                    // Entering once the placement is known (react-aria-components).
                    let entering = CapturedElement::new();
                    let is_entering = use_enter_animation(UseEnterAnimationInput {
                        is_ready: Signal::derive(move || placement.get().is_some() && is_open.get()),
                        element: entering,
    on_enter: None
                    })
                    .is_entering;
                    // A non-modal popover contains focus once a dialog is inside (per opening).
                    let overlay = OverlayFocusContain::new();
                    let contain = Signal::derive(move || {
                        // Not while exiting: the page is usable again.
                        (modality.is_modal() || is_dialog.get() || overlay.contain().get())
                            && !is_exiting.get()
                    });
                    match group {
                        // A root popover renders the container its submenus' popovers mount into.
                        PopoverGroup::Root(container) => Either::Left(view! {
                            <Portal>
                                <Provider value=overlay>
                                    {underlay()}
                                    <div style="display: contents" {..container.attr()}>
                                        <Provider value=PopoverGroupContext(container)>
                                            {popover(contain, entering, is_entering)}
                                        </Provider>
                                    </div>
                                </Provider>
                            </Portal>
                        }),
                        PopoverGroup::Sub(root) => Either::Right(view! {
                            <Portal nostrip:mount=root.get_untracked().map(|root| (*root).clone())>
                                <Provider value=overlay>
                                    {underlay()}
                                    {popover(contain, entering, is_entering)}
                                </Provider>
                            </Portal>
                        }),
                    }
                }
            </Show>
        }
}
