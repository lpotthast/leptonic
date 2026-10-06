// Upstream: react-aria/src/button/useButton.ts @ 99e6102368
use leptos::{
    attr,
    attr::{
        Attr,
        custom::{CustomAttr, custom_attribute},
    },
    ev,
    ev::{On, SharedEventCallback},
    oco::Oco,
    prelude::*,
};
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::{
    hooks::{
        FocusHandle, FocusableContextAttr, FocusableContextAttrs, HoverEndEvent, HoverStartEvent,
        IntoAttrs, LinkRel, LinkTarget, LongPressEvent, PressEvent, PropsWithStyles,
        UseFocusRingReturn, UseFocusableReturn, UseHoverReturn, UsePressReturn,
        focus::{
            use_focus_ring::{UseFocusRingInput, use_focus_ring},
            use_focusable::{UseFocusableInput, use_focusable},
        },
        interactions::{
            use_hover::{UseHoverInput, use_hover},
            use_keyboard::KeyboardEventWrapper,
            use_press::{UsePressInput, use_press},
        },
        link_rel_to_string,
    },
    utils::{
        ElementCaptureAttr, EventHandler,
        aria::{
            AriaChecked, AriaCurrent, AriaDisabled, AriaExpanded, AriaHasPopup, AriaPressed,
            AriaRole,
        },
        keyboard_shortcut::KeyboardShortcuts,
    },
};

// This is mostly based on work in: https://github.com/adobe/react-spectrum/blob/main/packages/react-aria/src/button/useButton.ts

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## DIFFERENT BEHAVIOR
// - Hover and focus tracking are built in (`is_hovered`, `is_focused`, `is_focus_visible`,
//   `data-focus-visible`). React-aria leaves this to the `Button` component of
//   react-aria-components, which combines `useButton`, `useHover` and `useFocusRing`. Every
//   leptonic button needs them, so the hook provides them.
//
// ## API DIFFERENCES
// - Flat input with `Default`. Hooks that configure a button (menu trigger, spin button, ...)
//   return a `UseButtonInput` instead of DOM props, mirroring how react-aria passes
//   `AriaButtonProps` around. Combine them with struct update syntax:
//   `use_button(UseButtonInput { on_press_end: .., ..trigger.button })`.
// - Long press callbacks are part of the input, because leptonic's `use_press` handles long
//   presses itself (react-aria uses a separate `useLongPress`).
// - Of the DOM props react-aria forwards via `filterDOMProps`, only `id`, `aria-label` and
//   `aria-labelledby` are part of the input (hooks configuring a button need them). Set others
//   directly on the element.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Press events on `ButtonElementType::Anchor` always propagate, so that client-side routers
//   listening on the document (leptos_router) still see link clicks. Leptonic stops event
//   propagation by default; react-aria has no such default.
//
// ## OMITTED FEATURES
// - `onClick` (deprecated in react-aria; use `on_press`).
//
// =============================================================================

/// The kind of element a button is rendered as. Determines which attributes are set.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ButtonElementType {
    /// A native `<button>`.
    #[default]
    Button,
    /// An `<a>`, e.g. for a link styled and announced as a button. Gets `role="button"`.
    Anchor,
    /// An `<input>` (`type="button"`, `"submit"` or `"reset"`).
    Input,
    /// Any other element, like `<div>` or `<span>`. Gets `role="button"`.
    Other,
}

/// The `type` of a `<button>` or `<input>` element.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ButtonType {
    /// A plain button. This is the default, unlike the HTML default (`submit`), so that buttons
    /// in forms don't submit them unintentionally.
    #[default]
    Button,
    /// Submits the form the button belongs to.
    Submit,
    /// Resets the form the button belongs to.
    Reset,
}

impl ButtonType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::Submit => "submit",
            Self::Reset => "reset",
        }
    }
}

/// Form-related attributes of a native `<button>`. Ignored for other element types.
///
/// See the [MDN reference](https://developer.mozilla.org/docs/Web/HTML/Element/button#attributes).
#[derive(Debug, Clone, Default)]
pub struct ButtonFormAttributes {
    /// The id of the `<form>` the button belongs to, if it is not its ancestor.
    pub form: Option<Oco<'static, str>>,
    /// Overrides the form's `action`.
    pub form_action: Option<Oco<'static, str>>,
    /// Overrides the form's `enctype`.
    pub form_enc_type: Option<FormEncType>,
    /// Overrides the form's `method`.
    pub form_method: Option<FormMethod>,
    /// Overrides the form's `novalidate`.
    pub form_no_validate: bool,
    /// Overrides the form's `target`.
    pub form_target: Option<LinkTarget>,
    /// Name submitted with the form data when this button submits the form.
    pub name: Option<Oco<'static, str>>,
    /// Value submitted with the form data when this button submits the form.
    pub value: Option<Oco<'static, str>>,
}

/// How a form's data is encoded when submitted (`enctype`, `formenctype`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormEncType {
    /// `application/x-www-form-urlencoded` (the browser's default).
    UrlEncoded,
    /// `multipart/form-data`, needed to upload files.
    Multipart,
    /// `text/plain`.
    TextPlain,
}

impl FormEncType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UrlEncoded => "application/x-www-form-urlencoded",
            Self::Multipart => "multipart/form-data",
            Self::TextPlain => "text/plain",
        }
    }
}

/// How a form is submitted (`method`, `formmethod`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormMethod {
    Get,
    Post,
    /// Closes the `<dialog>` the form is in.
    Dialog,
}

impl FormMethod {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Dialog => "dialog",
        }
    }
}

/// Input of [`use_button`]. Everything is optional; start from `UseButtonInput::default()`.
#[derive(Debug, Clone, Default)]
pub struct UseButtonInput {
    /// The kind of element the props are spread onto.
    pub element_type: ButtonElementType,

    /// The `type` of a `<button>` or `<input>` element. Defaults to `button`.
    pub button_type: ButtonType,

    /// The element's id.
    pub id: Option<Oco<'static, str>>,

    /// An accessible name, for buttons without visible text (e.g. icon buttons).
    pub aria_label: MaybeProp<String>,

    /// The id(s) of the element(s) naming the button. A signal: e.g. a field's label ids follow
    /// whether its label is rendered.
    pub aria_labelledby: Signal<Option<String>>,

    /// Whether the button is disabled.
    pub is_disabled: Signal<bool>,

    /// Keep the button focusable (but out of the tab order) while disabled.
    pub allow_focus_when_disabled: bool,

    /// Remove the button from the tab order. It stays focusable programmatically and by pointer.
    pub exclude_from_tab_order: Signal<bool>,

    /// Focus the button when it mounts.
    pub auto_focus: bool,

    /// Don't move focus to the button when it is pressed.
    pub prevent_focus_on_press: bool,

    /// For `ButtonElementType::Anchor`: the link target. Removed while disabled.
    pub href: Signal<Option<String>>,

    /// For `ButtonElementType::Anchor`: where to open the link.
    pub target: LinkTarget,

    /// For `ButtonElementType::Anchor`: the link relationship. `NoOpener` is added for
    /// `LinkTarget::Blank`.
    pub rel: Vec<LinkRel>,

    /// For `ButtonElementType::Button`: form attributes.
    pub form: ButtonFormAttributes,

    /// The kind of popup the button opens.
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    /// Whether the element the button controls is expanded.
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    /// The id(s) of the element(s) the button controls.
    pub aria_controls: Signal<Option<String>>,
    /// The id(s) of the element(s) describing the button.
    pub aria_describedby: Signal<Option<String>>,
    /// The pressed state of a toggle button.
    pub aria_pressed: Signal<Option<AriaPressed>>,
    /// `aria-checked`, for buttons acting as checkable items (e.g. `role="radio"`).
    pub aria_checked: Signal<Option<AriaChecked>>,
    /// Overrides the element's role (e.g. `radio` for the buttons of a single-selection toggle
    /// button group).
    pub role: Option<AriaRole>,
    /// Whether the button represents the current item of a set.
    pub aria_current: Signal<Option<AriaCurrent>>,

    pub on_press: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press_change: Option<Callback<bool>>,

    pub on_long_press_start: Option<Callback<LongPressEvent>>,
    pub on_long_press: Option<Callback<LongPressEvent>>,
    pub on_long_press_end: Option<Callback<LongPressEvent>>,
    /// Describes the long press action to assistive technology, e.g. "Long press to open menu".
    pub long_press_accessibility_description: MaybeProp<String>,

    pub on_hover_start: Option<Callback<HoverStartEvent>>,
    pub on_hover_end: Option<Callback<HoverEndEvent>>,
    pub on_hover_change: Option<Callback<bool>>,

    pub on_focus: Option<Callback<FocusEvent>>,
    pub on_blur: Option<Callback<FocusEvent>>,
    pub on_focus_change: Option<Callback<bool>>,

    pub on_key_down: Option<Callback<KeyboardEventWrapper>>,
    pub on_key_up: Option<Callback<KeyboardEventWrapper>>,

    /// Keyboard shortcuts handled while the button has focus.
    pub shortcuts: Option<KeyboardShortcuts>,

    /// Called when a context menu is requested on the button (right click, Shift+F10, long press
    /// on iOS, ...; see `use_context_menu`).
    pub on_context_menu: Option<Callback<crate::hooks::ContextMenuEvent>>,
}

/// Return value of [`use_button`].
#[derive(Debug)]
pub struct UseButtonReturn {
    /// Props for the button element. Call `.into_parts()` for view spreading and styles.
    pub props: PropsWithStyles<UseButtonProps>,
    /// Whether the button is disabled: by its input, or by a surrounding `PressResponder`.
    pub is_disabled: Signal<bool>,
    pub is_pressed: Signal<bool>,
    pub is_hovered: Signal<bool>,
    /// Whether the button is focused.
    pub is_focused: Signal<bool>,
    /// Whether the focus ring should be visible (keyboard navigation only).
    pub is_focus_visible: Signal<bool>,
    /// Programmatic focus.
    pub focus_handle: FocusHandle,
}

/// Props from [`use_button`], to be spread onto the button element.
#[derive(Debug)]
pub struct UseButtonProps {
    pub id: Option<Oco<'static, str>>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<String>>,
    pub role: Option<AriaRole>,
    pub button_type: Option<&'static str>,
    pub disabled: Signal<bool>,
    pub tabindex: Signal<Option<i32>>,
    pub href: Signal<Option<String>>,
    pub target: Option<Oco<'static, str>>,
    pub rel: Option<String>,
    pub form: ButtonFormAttributes,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<String>>,
    pub aria_pressed: Signal<Option<AriaPressed>>,
    pub aria_checked: Signal<Option<AriaChecked>>,
    pub aria_current: Signal<Option<AriaCurrent>>,
    pub aria_describedby: Signal<Option<String>>,
    pub data_focus_visible: Signal<Option<&'static str>>,
    pub element_capture: ElementCaptureAttr,
    pub on_keydown: EventHandler<KeyboardEvent>,
    pub on_keyup: EventHandler<KeyboardEvent>,
    pub on_focus: EventHandler<FocusEvent>,
    pub on_blur: EventHandler<FocusEvent>,
    pub on_click: EventHandler<MouseEvent>,
    pub on_dblclick: EventHandler<MouseEvent>,
    pub on_pointerdown: EventHandler<PointerEvent>,
    pub on_contextmenu: EventHandler<MouseEvent>,
    pub on_pointerup: EventHandler<PointerEvent>,
    pub on_mousedown: EventHandler<MouseEvent>,
    pub on_dragstart: EventHandler<DragEvent>,
    pub on_pointerenter: EventHandler<PointerEvent>,
    pub on_pointerleave: EventHandler<PointerEvent>,
    /// A `FocusableContext`'s further attributes (e.g. a tooltip trigger's pointer handlers).
    pub context_attrs: Option<FocusableContextAttrs>,
}

/// Attributes of [`UseButtonProps`], spreadable with `{..attrs}`.
pub type UseButtonAttrs = (
    (
        Attr<attr::Id, Option<Oco<'static, str>>>,
        Attr<attr::Role, Option<AriaRole>>,
        Attr<attr::Type, Option<&'static str>>,
        Attr<attr::Disabled, Signal<bool>>,
        Attr<attr::Tabindex, Signal<Option<i32>>>,
        Attr<attr::Href, Signal<Option<String>>>,
        Attr<attr::Target, Option<Oco<'static, str>>>,
        Attr<attr::Rel, Option<String>>,
    ),
    (
        Attr<attr::Form, Option<Oco<'static, str>>>,
        Attr<attr::Formaction, Option<Oco<'static, str>>>,
        Attr<attr::Formenctype, Option<&'static str>>,
        Attr<attr::Formmethod, Option<&'static str>>,
        Attr<attr::Formnovalidate, bool>,
        Attr<attr::Formtarget, Option<Oco<'static, str>>>,
        Attr<attr::Name, Option<Oco<'static, str>>>,
        Attr<attr::Value, Option<Oco<'static, str>>>,
    ),
    (
        Attr<attr::AriaLabel, MaybeProp<String>>,
        Attr<attr::AriaLabelledby, Signal<Option<String>>>,
        Attr<attr::AriaDisabled, Signal<Option<AriaDisabled>>>,
        Attr<attr::AriaHaspopup, Signal<Option<AriaHasPopup>>>,
        Attr<attr::AriaExpanded, Signal<Option<AriaExpanded>>>,
        Attr<attr::AriaControls, Signal<Option<String>>>,
        Attr<attr::AriaPressed, Signal<Option<AriaPressed>>>,
        Attr<attr::AriaChecked, Signal<Option<AriaChecked>>>,
        Attr<attr::AriaCurrent, Signal<Option<AriaCurrent>>>,
        Attr<attr::AriaDescribedby, Signal<Option<String>>>,
        CustomAttr<&'static str, Signal<Option<&'static str>>>,
        ElementCaptureAttr,
    ),
    (
        On<ev::keydown, SharedEventCallback<KeyboardEvent>>,
        On<ev::keyup, SharedEventCallback<KeyboardEvent>>,
        On<ev::focus, SharedEventCallback<FocusEvent>>,
        On<ev::blur, SharedEventCallback<FocusEvent>>,
        On<ev::click, SharedEventCallback<MouseEvent>>,
        On<ev::dblclick, SharedEventCallback<MouseEvent>>,
        On<ev::pointerdown, SharedEventCallback<PointerEvent>>,
        On<ev::contextmenu, SharedEventCallback<MouseEvent>>,
        On<ev::pointerup, SharedEventCallback<PointerEvent>>,
        On<ev::mousedown, SharedEventCallback<MouseEvent>>,
        On<ev::dragstart, SharedEventCallback<DragEvent>>,
        On<ev::pointerenter, SharedEventCallback<PointerEvent>>,
        On<ev::pointerleave, SharedEventCallback<PointerEvent>>,
    ),
    FocusableContextAttr,
);

impl IntoAttrs for UseButtonProps {
    type Attrs = UseButtonAttrs;

    fn into_attrs(self) -> Self::Attrs {
        let ButtonFormAttributes {
            form,
            form_action,
            form_enc_type,
            form_method,
            form_no_validate,
            form_target,
            name,
            value,
        } = self.form;
        (
            (
                Attr(attr::Id, self.id),
                Attr(attr::Role, self.role),
                Attr(attr::Type, self.button_type),
                Attr(attr::Disabled, self.disabled),
                Attr(attr::Tabindex, self.tabindex),
                Attr(attr::Href, self.href),
                Attr(attr::Target, self.target),
                Attr(attr::Rel, self.rel),
            ),
            (
                Attr(attr::Form, form),
                Attr(attr::Formaction, form_action),
                Attr(attr::Formenctype, form_enc_type.map(FormEncType::as_str)),
                Attr(attr::Formmethod, form_method.map(FormMethod::as_str)),
                Attr(attr::Formnovalidate, form_no_validate),
                Attr(attr::Formtarget, form_target.map(|target| target.to_oco())),
                Attr(attr::Name, name),
                Attr(attr::Value, value),
            ),
            (
                Attr(attr::AriaLabel, self.aria_label),
                Attr(attr::AriaLabelledby, self.aria_labelledby),
                Attr(attr::AriaDisabled, self.aria_disabled),
                Attr(attr::AriaHaspopup, self.aria_haspopup),
                Attr(attr::AriaExpanded, self.aria_expanded),
                Attr(attr::AriaControls, self.aria_controls),
                Attr(attr::AriaPressed, self.aria_pressed),
                Attr(attr::AriaChecked, self.aria_checked),
                Attr(attr::AriaCurrent, self.aria_current),
                Attr(attr::AriaDescribedby, self.aria_describedby),
                custom_attribute("data-focus-visible", self.data_focus_visible),
                self.element_capture,
            ),
            (
                self.on_keydown.into_on(ev::keydown),
                self.on_keyup.into_on(ev::keyup),
                self.on_focus.into_on(ev::focus),
                self.on_blur.into_on(ev::blur),
                self.on_click.into_on(ev::click),
                self.on_dblclick.into_on(ev::dblclick),
                self.on_pointerdown.into_on(ev::pointerdown),
                self.on_contextmenu.into_on(ev::contextmenu),
                self.on_pointerup.into_on(ev::pointerup),
                self.on_mousedown.into_on(ev::mousedown),
                self.on_dragstart.into_on(ev::dragstart),
                self.on_pointerenter.into_on(ev::pointerenter),
                self.on_pointerleave.into_on(ev::pointerleave),
            ),
            FocusableContextAttr(self.context_attrs),
        )
    }
}

/// Makes an element behave and announce itself as a button: press handling for pointer,
/// keyboard and assistive technology, focus management, hover and focus-visible state, and the
/// right attributes for the chosen [`ButtonElementType`].
///
/// ```ignore
/// let UseButtonReturn { props, .. } = use_button(UseButtonInput {
///     on_press: Some(Callback::new(|_| log!("pressed"))),
///     ..Default::default()
/// });
/// let (attrs, styles) = props.into_parts();
/// view! { <button {..attrs} style=styles>"Press me"</button> }
/// ```
pub fn use_button(input: UseButtonInput) -> UseButtonReturn {
    let UseButtonInput {
        element_type,
        button_type,
        id,
        aria_label,
        aria_labelledby,
        is_disabled: disabled,
        allow_focus_when_disabled,
        exclude_from_tab_order,
        auto_focus,
        prevent_focus_on_press,
        href,
        target,
        rel,
        form,
        aria_haspopup,
        aria_expanded,
        aria_controls,
        aria_describedby,
        aria_pressed,
        aria_checked,
        role,
        aria_current,
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        on_long_press_start,
        on_long_press,
        on_long_press_end,
        long_press_accessibility_description,
        on_hover_start,
        on_hover_end,
        on_hover_change,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        shortcuts,
        on_context_menu,
    } = input;

    // An overlay trigger's props from a `PressResponder` (`DialogTrigger`); the button's own win
    // (react-aria-components merges `triggerProps` into the pressable child).
    let responder = use_context::<crate::hooks::PressResponderContext>();
    let trigger = responder.as_ref().and_then(|ctx| ctx.trigger);
    // A disabled responder (a disabled `MenuTrigger` or `Disclosure`) disables the button itself:
    // its `disabled` attribute, focus, hover and shortcuts, not only its presses.
    let disabled = match responder.as_ref().and_then(|ctx| ctx.is_disabled) {
        Some(responder_disabled) => {
            Signal::derive(move || disabled.get() || responder_disabled.get())
        }
        None => disabled,
    };
    // The responder's shortcuts (a menu trigger's) after the button's own.
    let shortcuts = match (
        shortcuts,
        responder
            .and_then(|ctx| ctx.shortcuts)
            .map(|shortcuts| shortcuts.get_value()),
    ) {
        (Some(own), Some(responder)) => Some(own.with(responder)),
        (own, responder) => own.or(responder),
    };
    // Context menu requests: the responder's (a `MenuTrigger`'s) and the button's own. On iOS a
    // long press requests it.
    let context_menu = crate::hooks::use_context_menu(crate::hooks::UseContextMenuInput {
        on_context_menu: crate::hooks::chain_optional_callbacks(
            responder.as_ref().and_then(|ctx| ctx.on_context_menu),
            on_context_menu,
        ),
    });
    let on_long_press_start = crate::hooks::chain_optional_callbacks(
        context_menu.on_long_press_start,
        on_long_press_start,
    );
    let on_long_press =
        crate::hooks::chain_optional_callbacks(context_menu.on_long_press, on_long_press);
    let (aria_haspopup, aria_expanded, aria_controls) = match trigger {
        Some(trigger) => (
            Signal::derive(move || aria_haspopup.get().or_else(|| trigger.aria_haspopup.get())),
            Signal::derive(move || aria_expanded.get().or_else(|| trigger.aria_expanded.get())),
            Signal::derive(move || aria_controls.get().or_else(|| trigger.aria_controls.get())),
        ),
        None => (aria_haspopup, aria_expanded, aria_controls),
    };

    let UsePressReturn {
        props: press_props,
        is_pressed,
    } = use_press(UsePressInput {
        is_disabled: disabled,
        // Client-side routers (like leptos_router) handle link clicks in a document-level
        // listener, so clicks on anchors must bubble.
        propagation: if element_type == ButtonElementType::Anchor {
            crate::hooks::PressPropagation::Continue
        } else {
            crate::hooks::PressPropagation::Stop
        },
        prevent_focus_on_press: Signal::stored(prevent_focus_on_press),
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        on_long_press_start,
        on_long_press,
        on_long_press_end,
        long_press_accessibility_description,
        ..UsePressInput::default()
    });

    let UseFocusableReturn {
        props: focusable_props,
        focus_handle,
        ..
    } = use_focusable(UseFocusableInput {
        is_disabled: disabled,
        auto_focus,
        exclude_from_tab_order,
        on_focus,
        on_blur,
        on_focus_change,
        on_key_down,
        on_key_up,
        shortcuts,
        allow_shortcut_repeats: false,
    });
    let context_attrs = focusable_props.context_attrs.clone();
    let context_describedby = focusable_props.context_aria_describedby;

    let UseHoverReturn {
        props: hover_props,
        is_hovered,
    } = use_hover(UseHoverInput {
        is_disabled: disabled,
        on_hover_start,
        on_hover_end,
        on_hover_change,
    });

    let UseFocusRingReturn {
        props: focus_ring_props,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        is_disabled: disabled,
        ..UseFocusRingInput::default()
    });

    let (press_props, styles) = press_props.into_inner();

    // `disabled` only exists on form controls. Other elements announce it via `aria-disabled`.
    let has_disabled_attr = matches!(
        element_type,
        ButtonElementType::Button | ButtonElementType::Input
    );
    let tabindex = if allow_focus_when_disabled {
        let focusable_tabindex = focusable_props.tabindex;
        Signal::derive(move || {
            if disabled.get() {
                Some(-1)
            } else {
                focusable_tabindex.get()
            }
        })
    } else {
        focusable_props.tabindex
    };

    let is_anchor = element_type == ButtonElementType::Anchor;
    let props = UseButtonProps {
        id,
        aria_label,
        aria_labelledby,
        role: role.or(match element_type {
            ButtonElementType::Button => None,
            _ => Some(AriaRole::Button),
        }),
        button_type: match element_type {
            ButtonElementType::Button | ButtonElementType::Input => Some(button_type.as_str()),
            _ => None,
        },
        disabled: Signal::derive(move || has_disabled_attr && disabled.get()),
        tabindex,
        href: Signal::derive(move || href.get().filter(|_| is_anchor && !disabled.get())),
        target: (is_anchor && target != LinkTarget::Same).then(|| target.to_oco()),
        rel: {
            let mut rel = rel;
            if target == LinkTarget::Blank && !rel.contains(&LinkRel::NoOpener) {
                rel.push(LinkRel::NoOpener);
            }
            link_rel_to_string(&rel).filter(|_| is_anchor)
        },
        form: if element_type == ButtonElementType::Button {
            form
        } else {
            ButtonFormAttributes::default()
        },
        aria_disabled: Signal::derive(move || {
            (!has_disabled_attr && disabled.get()).then_some(AriaDisabled::True)
        }),
        aria_haspopup,
        aria_expanded,
        aria_controls,
        aria_pressed,
        aria_checked,
        aria_current,
        aria_describedby: {
            // The long press description (from `use_press`) adds to the caller's.
            let press_describedby = press_props.aria_describedby;
            Signal::derive(move || {
                let press_ids: Vec<String> = press_describedby
                    .with(|d| {
                        d.as_ref()
                            .map(|d| d.ids().map(str::to_owned).collect::<Vec<_>>())
                    })
                    .unwrap_or_default();
                let ids: Vec<String> = aria_describedby
                    .get()
                    .into_iter()
                    .chain(press_ids)
                    .chain(context_describedby.get())
                    .collect();
                (!ids.is_empty()).then(|| ids.join(" "))
            })
        },
        data_focus_visible: focus_ring_props.data_focus_visible,
        element_capture: match trigger {
            Some(trigger) => focusable_props
                .element_capture
                .chain(trigger.element.attr()),
            None => focusable_props.element_capture,
        },
        // Keyboard handlers (and shortcuts) run before press handling, as in react-aria's
        // `mergeProps(focusableProps, pressProps)`. Press handling prevents the default action of
        // Enter/Space, which shortcuts check to see whether something else handled the key.
        on_keydown: focusable_props
            .on_keydown
            .chain(press_props.on_keydown)
            .chain(context_menu.props.on_keydown),
        on_keyup: focusable_props.on_keyup,
        on_focus: focusable_props.on_focus.chain(focus_ring_props.on_focus),
        on_blur: focusable_props.on_blur.chain(focus_ring_props.on_blur),
        on_click: press_props.on_click,
        on_dblclick: press_props.on_dblclick,
        on_pointerdown: press_props.on_pointerdown,
        on_contextmenu: context_menu.props.on_contextmenu,
        on_pointerup: press_props.on_pointerup,
        on_mousedown: press_props.on_mousedown,
        on_dragstart: press_props.on_dragstart,
        on_pointerenter: hover_props.on_pointerenter,
        on_pointerleave: hover_props.on_pointerleave,
        context_attrs,
    };

    UseButtonReturn {
        props: PropsWithStyles::new(props, styles),
        is_disabled: disabled,
        is_pressed,
        is_hovered,
        is_focused,
        is_focus_visible,
        focus_handle,
    }
}
