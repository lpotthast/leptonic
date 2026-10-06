// Upstream: react-aria-components/src/Popover.tsx @ 99e6102368
use leptos::{portal::Portal, prelude::*};

use super::{
    dialog::DialogTriggerContext, dismiss_button::DismissButton, focus_scope::FocusScope,
    press::ClearPressResponder,
};
use crate::{
    hooks::{
        OverlayState, OverlayTriggerState, PhysicalPlacementX, PlacementX, PlacementY,
        PopoverModality, PropsWithStyles, UsePopoverInput, UsePopoverProps, UsePopoverReturn,
        use_popover,
    },
    utils::{
        CapturedElement, classes::Classes, focus::focus_safely, shadow_dom::get_active_element,
        styles::Styles,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - `state` and `trigger` come from a surrounding `DialogTrigger` (react-aria-components:
//   `OverlayTriggerStateContext` and `triggerRef`), or are passed directly; no
//   `isOpen`/`defaultOpen`/`onOpenChange` props (pass app state as `state`).
// - Placement is two typed axes (`placement_x`, `placement_y`).
// - Render props become the `data-placement` attribute plus plain children.
// - Whether a modal popover is a dialog (no `[role=dialog]` inside) is decided once it is rendered;
//   its label (an own `aria_labelledby`, else the `DialogTrigger`'s trigger) is set then.
//
// ## OMITTED FEATURES
// - `OverlayArrow`, `isEntering`/`isExiting` animations, `UNSTABLE_portalContainer`,
//   submenu triggers.
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
/// Data attributes: `data-placement` (`top`, `bottom`, `left` or `right`, after flipping).
#[component]
#[allow(clippy::needless_pass_by_value, clippy::too_many_arguments)]
pub fn Popover(
    /// Whether the popover is open. Default: the surrounding `DialogTrigger`'s state.
    #[prop(into, optional)]
    state: Option<OverlayTriggerState>,
    /// The element the popover is positioned at. Default: the surrounding `DialogTrigger`'s trigger.
    #[prop(optional)]
    trigger: Option<CapturedElement>,
    #[prop(into, default = Signal::stored(PlacementX::Center))] placement_x: Signal<PlacementX>,
    #[prop(into, default = Signal::stored(PlacementY::Below))] placement_y: Signal<PlacementY>,
    /// The distance from the trigger, in pixels.
    #[prop(into, default = Signal::stored(8.0))]
    offset: Signal<f64>,
    #[prop(into, optional)] cross_offset: Signal<f64>,
    /// The minimum distance from the viewport edges, in pixels.
    #[prop(into, default = Signal::stored(12.0))]
    container_padding: Signal<f64>,
    /// Whether the popover flips to the other side when there is no room.
    #[prop(into, default = Signal::stored(true))]
    should_flip: Signal<bool>,
    /// Whether the popover takes over the page while open. Default: modal.
    #[prop(optional)]
    modality: PopoverModality,
    #[prop(optional)] is_keyboard_dismiss_disabled: bool,
    /// Which outside interactions close the popover: `true` closes.
    #[prop(optional)]
    should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,
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
    let context = use_context::<DialogTriggerContext>();
    let state = state
        .or(context.map(|ctx| ctx.state))
        .expect("a <Popover> needs a `state` or a surrounding <DialogTrigger>");
    let trigger = trigger
        .or(context.map(|ctx| ctx.trigger))
        .expect("a <Popover> needs a `trigger` or a surrounding <DialogTrigger>");

    let UsePopoverReturn {
        props,
        id,
        resolved_placement_x,
        resolved_placement_y,
        ..
    } = use_popover(UsePopoverInput {
        trigger,
        placement_x,
        placement_y,
        offset,
        cross_offset,
        container_padding,
        should_flip,
        modality,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
        ..UsePopoverInput::new(state)
    });
    // The trigger's `aria-controls`.
    if let Some(context) = context {
        context.overlay_id.set(Some(id.to_string()));
    }

    render_popover(
        state,
        PopoverParts {
            props,
            resolved_placement_x,
            resolved_placement_y,
        },
        modality,
        CapturedElement::new(),
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

/// What names a popover that is a dialog itself.
pub(crate) struct PopoverDialogLabel<L> {
    pub aria_label: MaybeProp<String>,
    /// The ids of the naming elements, read once the popover turned out to be a dialog.
    pub aria_labelledby: L,
}

/// The parts of a [`UsePopoverReturn`] the rendering needs.
pub(crate) struct PopoverParts {
    pub props: PropsWithStyles<UsePopoverProps>,
    pub resolved_placement_x: Memo<PhysicalPlacementX>,
    pub resolved_placement_y: Memo<PlacementY>,
}

/// Renders a popover (the `Popover` atom, and the select's and combo box's popovers): in a portal
/// while open, a modal one with an underlay, focus kept inside and a leading dismiss button,
/// every one with a trailing dismiss button, `data-placement` and the trigger's press responder
/// cleared for its content. `capture` receives the popover element as well. A modal popover
/// without a dialog inside is the dialog (react-aria-components): `role="dialog"`, named by
/// `label`, focused once rendered unless focus is already inside.
#[allow(clippy::too_many_arguments)]
pub(crate) fn render_popover<S: OverlayState, L: Fn() -> Option<String> + 'static>(
    state: S,
    parts: PopoverParts,
    modality: PopoverModality,
    capture: CapturedElement,
    label: PopoverDialogLabel<L>,
    classes: Classes,
    styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let PopoverParts {
        props,
        resolved_placement_x,
        resolved_placement_y,
    } = parts;
    let placement = Memo::new(move |_| match resolved_placement_y.get() {
        PlacementY::Above | PlacementY::Top => "top",
        PlacementY::Bottom | PlacementY::Below => "bottom",
        PlacementY::Center => match resolved_placement_x.get() {
            PhysicalPlacementX::OuterLeft | PhysicalPlacementX::Left => "left",
            PhysicalPlacementX::Center
            | PhysicalPlacementX::Right
            | PhysicalPlacementX::OuterRight => "right",
        },
    });

    let (attrs, popover_styles) = props.into_parts();
    let attrs = StoredValue::new(attrs);
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(popover_styles.merge(styles));
    let children = StoredValue::new(children);
    let close = Callback::new(move |()| state.close());

    let PopoverDialogLabel {
        aria_label,
        aria_labelledby,
    } = label;
    let is_dialog = RwSignal::new(false);
    let dialog_labelledby = RwSignal::new(None::<String>);
    // Once rendered: a modal popover without a dialog inside is the dialog.
    Effect::new(move |_| {
        let Some(el) = capture.get() else {
            return;
        };
        let has_dialog = el.query_selector("[role=dialog]").ok().flatten().is_some();
        let dialog = modality.is_modal() && !has_dialog;
        if dialog {
            dialog_labelledby.set(aria_labelledby());
        }
        is_dialog.set(dialog);
    });
    // Focus the popover itself once it is a dialog (and focusable), unless focus already moved
    // into it.
    Effect::new(move |_| {
        if !is_dialog.get() {
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

    view! {
        <Portal>
            <Show when=move || state.is_open()>
                // A modal popover's underlay catches interaction with the page.
                {modality
                    .is_modal()
                    .then(|| {
                        view! {
                            <div style="position: fixed; inset: 0;" />
                        }
                    })}
                <FocusScope contain=modality.is_modal() restore_focus=true>
                    <div
                        {..attrs.get_value()}
                        {..capture.attr()}
                        class=classes.get_value()
                        style=styles.get_value()
                        data-placement=move || placement.get()
                        role=move || is_dialog.get().then_some("dialog")
                        tabindex=move || is_dialog.get().then_some(-1)
                        aria-label=move || is_dialog.get().then(|| aria_label.get()).flatten()
                        aria-labelledby=move || is_dialog.get().then(|| dialog_labelledby.get()).flatten()
                    >
                        // Pressing in the popover must not toggle it through the trigger's responder.
                        <ClearPressResponder>
                            {modality.is_modal().then(|| view! { <DismissButton on_dismiss=close /> })}
                            {(children.get_value())()}
                            <DismissButton on_dismiss=close />
                        </ClearPressResponder>
                    </div>
                </FocusScope>
            </Show>
        </Portal>
    }
}
