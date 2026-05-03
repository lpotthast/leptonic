use leptos::prelude::*;

pub use crate::atoms::popover::PopoverContext;
use crate::{
    atoms::{
        dismiss_button::DismissButton,
        popover::{
            Popover as PopoverAtom, PopoverContent as PopoverContentAtom,
            PopoverTrigger as PopoverTriggerAtom,
        },
        press::{ClearPressResponder, PressResponder},
    },
    hooks::{DialogRole, PlacementX, PlacementY, PressEvent},
    utils::{classes::Classes, locale::WritingDirection, styles::Styles},
};

/// Slot for the popover trigger element.
///
/// The trigger is the element the popover positions itself relative to.
/// Child [`Button`](crate::components::button::Button) elements automatically
/// inherit toggle behavior from the component's [`PressResponder`] context.
/// Non-interactive elements (text, icons) should be wrapped in
/// [`Pressable`](crate::atoms::press::Pressable) to receive press events.
#[slot]
pub struct PopoverTrigger {
    #[prop(into, optional)]
    pub classes: Classes,
    #[prop(into, optional)]
    pub styles: Styles,
    pub children: Children,
}

/// A themed popover component that bundles overlay behavior, positioning,
/// ARIA dialog semantics, and screen-reader dismiss buttons.
///
/// Uses [`PressResponder`] to automatically inject toggle behavior into child
/// [`Button`](crate::components::button::Button) elements — no manual wiring needed.
///
/// # Uncontrolled mode (simplest)
///
/// ```ignore
/// <Popover placement_y=PlacementY::Below>
///     <PopoverTrigger slot>
///         <Button>"Open"</Button> // Auto-toggles via PressResponder
///     </PopoverTrigger>
///     "Popover content"
/// </Popover>
/// ```
///
/// # Controlled mode
///
/// ```ignore
/// let (show, set_show) = signal(false);
///
/// <Popover show_when=show on_close=move |_| set_show.set(false)>
///     <PopoverTrigger slot>
///         <Button on_press=move |_| set_show.set(true)>"Open"</Button>
///     </PopoverTrigger>
///     "Content"
/// </Popover>
/// ```
#[component]
#[allow(clippy::needless_pass_by_value, clippy::fn_params_excessive_bools)]
pub fn Popover(
    /// Slot for the trigger element.
    popover_trigger: PopoverTrigger,

    /// Controlled open state. Omit for uncontrolled mode.
    #[prop(into, optional)]
    show_when: Option<Signal<bool>>,

    /// Called when the popover should close (Escape, outside click, dismiss button).
    #[prop(into, optional)]
    on_close: Option<Callback<()>>,

    /// Horizontal placement relative to trigger.
    #[prop(into, default = Signal::stored(PlacementX::Center))]
    placement_x: Signal<PlacementX>,

    /// Vertical placement relative to trigger.
    #[prop(into, default = Signal::stored(PlacementY::Above))]
    placement_y: Signal<PlacementY>,

    /// Writing direction for logical placement resolution.
    #[prop(into, default = Signal::stored(WritingDirection::Ltr))]
    writing_direction: Signal<WritingDirection>,

    /// Whether the popover is non-modal (allows interaction outside). Default: `true`.
    #[prop(default = true)]
    is_non_modal: bool,

    /// Whether clicking outside / pressing Escape dismisses the popover. Default: `true`.
    #[prop(default = true)]
    is_dismissable: bool,

    /// Whether Escape key dismiss is disabled. Default: `false`.
    #[prop(default = false)]
    is_keyboard_dismiss_disabled: bool,

    /// Filter for which outside interactions should close the popover.
    #[prop(optional)]
    should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,

    /// Dialog role for ARIA semantics.
    #[prop(default = DialogRole::Dialog)]
    role: DialogRole,

    /// Optional ARIA label for the dialog.
    #[prop(into, optional)]
    aria_label: Option<String>,

    /// Additional CSS classes on the popover panel.
    #[prop(into, optional)]
    classes: Classes,

    /// Additional CSS styles on the popover panel.
    #[prop(into, optional)]
    styles: Styles,

    children: ChildrenFn,
) -> impl IntoView {
    let aria_label = StoredValue::new(aria_label);
    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(styles);
    let children = StoredValue::new(children);

    view! {
        <PopoverAtom
            is_non_modal=is_non_modal
            is_dismissable=is_dismissable
            should_close_on_blur=!is_non_modal
            is_keyboard_dismiss_disabled=is_keyboard_dismiss_disabled
            nostrip:should_close_on_interact_outside=should_close_on_interact_outside
            nostrip:on_close=on_close
        >
            // Controlled state sync: external show_when → atom's internal state.
            {
                if let Some(show_when) = show_when {
                    let ctx = expect_context::<PopoverContext>();
                    Effect::new(move || {
                        ctx.set_state.set(show_when.get());
                    });
                }
            }

            // PressResponder injects toggle into child Button via PressResponderContext.
            // force_is_pressed makes the trigger button appear pressed while popover is open.
            {
                let ctx = expect_context::<PopoverContext>();
                let is_open = Signal::derive(move || ctx.state.get());
                view! {
                    <PressResponder
                        on_press=Callback::new(move |_: PressEvent| {
                            ctx.set_state.set(!ctx.state.get_untracked());
                        })
                        force_is_pressed=is_open
                    >
                        <PopoverTriggerAtom
                            classes=popover_trigger.classes
                            styles=popover_trigger.styles
                        >
                            {(popover_trigger.children)()}
                        </PopoverTriggerAtom>
                    </PressResponder>
                }
            }

            // Content: positioned overlay with DismissButton.
            <PopoverContentAtom
                placement_x=placement_x
                placement_y=placement_y
                writing_direction=writing_direction
                contain_focus=!is_non_modal
                role=role
                nostrip:aria_label=aria_label.get_value()
            >
                {
                    let ctx = expect_context::<PopoverContext>();
                    view! {
                        <ClearPressResponder>
                            <DismissButton on_dismiss=ctx.on_close />
                            <div class=classes.get_value().add("leptonic-popover") style=styles.get_value()>
                                {(children.get_value())()}
                            </div>
                            <DismissButton on_dismiss=ctx.on_close />
                        </ClearPressResponder>
                    }
                }
            </PopoverContentAtom>
        </PopoverAtom>
    }
}
