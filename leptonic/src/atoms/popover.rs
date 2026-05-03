use leptos::{context::Provider, portal::Portal, prelude::*};

use crate::{
    hooks::{DialogRole, UseOverlayAttrs, UseOverlayUnderlayAttrs, *},
    utils::{CapturedElement, classes::Classes, locale::WritingDirection, styles::Styles},
};

use super::focus_scope::FocusScope;

#[derive(Debug, Clone)]
pub struct PopoverContext {
    pub state: ReadSignal<bool>,
    pub set_state: WriteSignal<bool>,

    /// Whether the popover allows interaction with elements outside.
    pub is_non_modal: bool,

    pub(crate) id: Oco<'static, str>,
    pub(crate) overlay_attrs: UseOverlayAttrs,
    pub(crate) underlay_attrs: UseOverlayUnderlayAttrs,
    #[cfg_attr(feature = "ssr", allow(dead_code))]
    pub(crate) overlay_element: CapturedElement,
    pub(crate) trigger_element: CapturedElement,
    pub(crate) on_close: Callback<()>,
}

#[component]
#[allow(clippy::fn_params_excessive_bools)]
pub fn Popover(
    /// Whether the popover is non-modal (allows interaction with elements outside).
    /// Default: `true`.
    ///
    /// When `false`, focus is trapped, body scroll is prevented, and elements
    /// outside the popover are hidden from assistive technology.
    #[prop(default = true)]
    is_non_modal: bool,
    /// Whether clicking outside closes the popover. Default: `true`.
    #[prop(default = true)]
    is_dismissable: bool,
    /// Whether the popover closes when focus leaves. Default: `false`.
    #[prop(default = false)]
    should_close_on_blur: bool,
    /// Whether Escape key dismiss is disabled. Default: `false`.
    #[prop(default = false)]
    is_keyboard_dismiss_disabled: bool,
    /// Filter for which outside interactions should close the popover.
    #[prop(optional)]
    should_close_on_interact_outside: Option<Callback<web_sys::Element, bool>>,
    /// Called when the overlay dismisses (Escape, outside click, blur).
    /// The atom already sets its internal state to `false`; this callback
    /// is for notifying the parent.
    #[prop(into, optional)]
    on_close: Option<Callback<()>>,
    children: Children,
) -> impl IntoView {
    let (state, set_state) = signal(false);

    let close_callback = Callback::new(move |()| {
        set_state.set(false);
        if let Some(on_close) = on_close {
            on_close.run(());
        }
    });

    // react-aria: isDismissable = !isNonModal || isSubmenu (no submenu support).
    // When modal (!is_non_modal), always dismissable via outside click.
    let effective_is_dismissable = is_dismissable || !is_non_modal;

    let UseOverlayReturn {
        props: overlay_props,
        underlay_props,
        id,
        overlay_element,
    } = use_overlay(UseOverlayInput {
        is_open: state.into(),
        on_close: close_callback,
        is_dismissable: effective_is_dismissable,
        should_close_on_blur,
        is_keyboard_dismiss_disabled,
        should_close_on_interact_outside,
    });

    let trigger_element = CapturedElement::new();
    let overlay_attrs = overlay_props.into_attrs();
    let underlay_attrs = underlay_props.into_attrs();

    view! {
        <Provider value=PopoverContext {
            state,
            set_state,
            is_non_modal,
            id,
            overlay_attrs,
            underlay_attrs,
            overlay_element,
            trigger_element,
            on_close: close_callback,
        }>{children()}</Provider>
    }
}

#[component]
pub fn PopoverTrigger(
    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: Children,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let UseOverlayTriggerReturn {
        props: trigger_props,
    } = use_overlay_trigger(UseOverlayTriggerInput {
        show: ctx.state.into(),
        overlay_id: ctx.id,
        overlay_type: OverlayTriggerType::Dialog,
    });

    let trigger_capture = ctx.trigger_element.attr();

    view! {
        <div class=classes.add("leptonic-popover-trigger") style={styles} {..trigger_props.into_attrs()} {..trigger_capture}>
            {children()}
        </div>
    }
}

#[component]
#[allow(clippy::needless_pass_by_value, clippy::too_many_lines)]
pub fn PopoverContent(
    #[prop(into)] placement_x: Signal<PlacementX>,
    #[prop(into)] placement_y: Signal<PlacementY>,
    #[prop(into)] writing_direction: Signal<WritingDirection>,

    /// Whether to trap focus within the popover. Default: `false`.
    /// Set to `true` for modal popovers.
    #[prop(default = false)]
    contain_focus: bool,

    /// Whether to restore focus to the previously focused element on close.
    /// Default: `true`.
    #[prop(default = true)]
    restore_focus: bool,

    /// Whether to auto-focus the first focusable element on open.
    /// Default: `true`.
    #[prop(default = true)]
    auto_focus: bool,

    /// Optional ARIA dialog role. When set, the content element gets
    /// `role` and `tabindex="-1"` for dialog semantics without requiring
    /// the full `<Dialog>` atom.
    #[prop(optional)]
    role: Option<DialogRole>,

    /// Optional ARIA label (used when `role` is set).
    #[prop(into, optional)]
    aria_label: Option<String>,

    #[prop(into, optional)] classes: Classes,
    #[prop(into, optional)] styles: Styles,
    children: ChildrenFn,
) -> impl IntoView {
    let ctx = expect_context::<PopoverContext>();

    let UseOverlayPositionReturn {
        props: overlay_pos_props,
        resolved_placement_x,
        resolved_placement_y,
    } = use_overlay_position(UseOverlayPositionInput {
        target: ctx.trigger_element,
        placement_x,
        placement_y,
        writing_direction,
        offset: 0.0.into(),
        cross_offset: 0.0.into(),
        container_padding: 12.0.into(),
        should_flip: true.into(),
        max_height: None,
        is_open: ctx.state.into(),
    });

    // Scroll prevention for modal popovers.
    let _prevent_scroll = use_prevent_scroll(UsePreventScrollInput {
        disabled: Signal::derive(move || ctx.is_non_modal || !ctx.state.get()),
    });

    // Close on scroll of trigger's scroll parents (non-modal only;
    // modal popovers prevent scroll, so no scroll events fire).
    use_close_on_scroll(UseCloseOnScrollInput {
        is_open: ctx.state.into(),
        trigger_element: ctx.trigger_element,
        on_close: ctx.on_close,
    });

    // aria-hide-outside (modal) / keep-visible (non-modal).
    #[cfg(not(feature = "ssr"))]
    {
        use crate::utils::aria_hide_outside::{
            AriaHideOutsideOptions, aria_hide_outside, keep_visible,
        };

        let overlay_element = ctx.overlay_element;
        let trigger_element = ctx.trigger_element;
        let is_non_modal = ctx.is_non_modal;

        let hide_cleanup: StoredValue<Option<Box<dyn FnOnce()>>, LocalStorage> =
            StoredValue::new_local(None);

        Effect::new(move |_| {
            // Clean up previous.
            hide_cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });

            if !ctx.state.get() {
                return;
            }

            if is_non_modal {
                // Non-modal: ensure this popover stays visible to AT even if
                // another modal's aria_hide_outside is active.
                if let Some(el) = overlay_element.get() {
                    if let Some(undo) = keep_visible(&el) {
                        hide_cleanup.set_value(Some(undo));
                    }
                }
            } else {
                // Modal: hide everything outside the popover and trigger from AT.
                let mut targets = Vec::new();
                if let Some(el) = overlay_element.get() {
                    targets.push((*el).clone());
                }
                if let Some(el) = trigger_element.get() {
                    targets.push((*el).clone());
                }
                if !targets.is_empty() {
                    let undo = aria_hide_outside(&targets, AriaHideOutsideOptions::default());
                    hide_cleanup.set_value(Some(undo));
                }
            }
        });

        on_cleanup(move || {
            hide_cleanup.update_value(|opt| {
                if let Some(f) = opt.take() {
                    f();
                }
            });
        });
    }

    let (overlay_pos_attrs, overlay_pos_styles) = overlay_pos_props.into_parts();

    // data-placement for CSS styling based on resolved placement.
    let resolved_placement = Memo::new(move |_| {
        let y = match resolved_placement_y.get() {
            PlacementY::Above | PlacementY::Top => "top",
            PlacementY::Center => "center",
            PlacementY::Bottom | PlacementY::Below => "bottom",
        };
        let x = match resolved_placement_x.get() {
            PhysicalPlacementX::OuterLeft | PhysicalPlacementX::Left => "left",
            PhysicalPlacementX::Center => "center",
            PhysicalPlacementX::Right | PhysicalPlacementX::OuterRight => "right",
        };
        format!("{y} {x}")
    });

    let role_str = role.map(|r| match r {
        DialogRole::Dialog => "dialog",
        DialogRole::AlertDialog => "alertdialog",
    });
    let tabindex_str = role.map(|_| "-1");

    let classes = StoredValue::new(classes);
    let styles = StoredValue::new(overlay_pos_styles.merge(styles));
    let aria_label = StoredValue::new(aria_label);
    let children = StoredValue::new(children);
    let is_non_modal = ctx.is_non_modal;

    let overlay_attrs = StoredValue::new(ctx.overlay_attrs.clone());
    let underlay_attrs = StoredValue::new(ctx.underlay_attrs.clone());
    let overlay_pos_attrs = StoredValue::new(overlay_pos_attrs);

    view! {
        <Portal>
            <Show when=move || ctx.state.get()>
                // Underlay for modal popovers: captures pointer events.
                {
                    if is_non_modal {
                        None
                    } else {
                        Some(view! {
                            <div
                                style="position: fixed; inset: 0; z-index: 99999;"
                                {..underlay_attrs.get_value()}
                            />
                        })
                    }
                }

                <FocusScope contain=contain_focus restore_focus=restore_focus auto_focus=auto_focus>
                    <div
                        class=classes.get_value().add("leptonic-popover-content")
                        style=styles.get_value()
                        role=role_str
                        tabindex=tabindex_str
                        aria-label=aria_label.get_value()
                        attr:data-placement=move || resolved_placement.get()
                        {..overlay_attrs.get_value()}
                        {..overlay_pos_attrs.get_value()}
                    >
                        {(children.get_value())()}
                    </div>
                </FocusScope>
            </Show>
        </Portal>
    }
}
