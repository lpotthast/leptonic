use leptos::prelude::*;

use crate::{
    Out,
    atoms::{
        dialog::{Dialog, DialogTrigger, DialogTriggerProps},
        popover::Popover as PopoverAtom,
    },
    hooks::{DialogRole, Placement, PopoverModality},
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
/// <Popover placement=Placement::Bottom>
///     <PopoverTrigger slot>
///         <Button>"Open"</Button>
///     </PopoverTrigger>
///     "Popover content"
/// </Popover>
/// ```
///
/// Bind the open state to app state with `is_open` and `set_open`.
#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Popover(
    /// Slot for the trigger: put a `Button` in it, it opens and closes the popover.
    popover_trigger: PopoverTrigger,

    /// Whether the popover is open (controlled): a value or any signal. Default: owned by the
    /// popover.
    #[prop(into, optional)]
    is_open: Option<Signal<bool>>,
    /// Receives the open state: an `RwSignal`, `WriteSignal`, closure, `Callback`, ...
    #[prop(into, optional)]
    set_open: Option<Out<bool>>,

    /// Where the popover goes relative to the trigger.
    #[prop(into, default = Signal::stored(Placement::Top))]
    placement: Signal<Placement>,

    /// Whether the popover takes over the page while open. Default: non-modal (the page stays
    /// usable).
    #[prop(default = PopoverModality::NonModal)]
    modality: PopoverModality,

    /// Whether Escape no longer closes the popover.
    #[prop(into, optional)]
    is_keyboard_dismiss_disabled: Signal<bool>,

    /// Which outside interactions close the popover: `true` closes.
    #[prop(optional)]
    should_close_on_interact_outside: Option<crate::hooks::InteractOutsideFilter>,

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
                    placement=placement
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
    trigger_props.is_open = is_open;
    trigger_props.set_open = set_open;
    DialogTrigger(trigger_props)
}
