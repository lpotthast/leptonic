use leptos::prelude::*;

use crate::{
    atoms::{
        dialog::{Dialog, DialogTrigger, DialogTriggerProps},
        popover::Popover as PopoverAtom,
    },
    hooks::{DialogRole, OverlayTriggerState, PlacementX, PlacementY, PopoverModality},
    utils::{classes::Classes, styles::Styles},
};

#[slot]
pub struct PopoverTrigger {
    #[prop(into, optional)]
    pub classes: Classes,
    #[prop(into, optional)]
    pub styles: Styles,
    pub children: Children,
}

/// A themed popover: opened by its trigger's `Button`, positioned next to it, a dialog with
/// dismiss buttons for screen reader users. Built from the `DialogTrigger`, `Popover` and `Dialog`
/// atoms.
///
/// ```ignore
/// <Popover placement_y=PlacementY::Below>
///     <PopoverTrigger slot>
///         <Button>"Open"</Button>
///     </PopoverTrigger>
///     "Popover content"
/// </Popover>
/// ```
///
/// Bind the open state to app state with `state=rw_signal`.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Popover(
    /// Slot for the trigger: put a `Button` in it, it opens and closes the popover.
    popover_trigger: PopoverTrigger,

    /// The open state as app state (`state=rw_signal`) or a shared `OverlayTriggerState`.
    /// Default: owned by the popover.
    #[prop(into, optional)]
    state: Option<OverlayTriggerState>,

    /// Horizontal placement relative to trigger.
    #[prop(into, default = Signal::stored(PlacementX::Center))]
    placement_x: Signal<PlacementX>,

    /// Vertical placement relative to trigger.
    #[prop(into, default = Signal::stored(PlacementY::Above))]
    placement_y: Signal<PlacementY>,

    /// Whether the popover takes over the page while open. Default: non-modal (the page stays
    /// usable).
    #[prop(default = PopoverModality::NonModal)]
    modality: PopoverModality,

    /// Whether Escape no longer closes the popover.
    #[prop(optional)]
    is_keyboard_dismiss_disabled: bool,

    /// Which outside interactions close the popover: `true` closes.
    #[prop(optional)]
    should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,

    #[prop(optional)] role: DialogRole,

    /// Names the popover's dialog.
    #[prop(into, optional)]
    aria_label: MaybeProp<String>,

    /// Additional CSS classes on the popover panel.
    #[prop(into, optional)]
    classes: Classes,

    /// Additional CSS styles on the popover panel.
    #[prop(into, optional)]
    styles: Styles,

    children: ChildrenFn,
) -> impl IntoView {
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);
    let children = StoredValue::new(children);
    let mut trigger_props = DialogTriggerProps::builder()
        .children(Box::new(move || {
            view! {
                <div
                    class=popover_trigger.classes.add("leptonic-popover-trigger")
                    style=popover_trigger.styles
                >
                    {(popover_trigger.children)()}
                </div>
                <PopoverAtom
                    placement_x=placement_x
                    placement_y=placement_y
                    modality=modality
                    is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
                    nostrip:should_close_on_interact_outside=should_close_on_interact_outside
                    classes="leptonic-popover-content"
                >
                    <Dialog
                        role=role
                        aria_label=aria_label
                        classes=classes.get_value().add("leptonic-popover")
                        styles=styles.get_value()
                    >
                        {(children.get_value())()}
                    </Dialog>
                </PopoverAtom>
            }
            .into_any()
        }))
        .build();
    trigger_props.state = state;
    DialogTrigger(trigger_props)
}
