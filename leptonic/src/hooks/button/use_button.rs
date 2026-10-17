// Upstream: react-aria/src/button/useButton.ts @ 99e6102368
// Upstream: react-aria/test/button/useButton.test.js @ 99e6102368
// Upstream: react-aria-components/test/Button.test.js @ 99e6102368
use leptos::{ev, oco::Oco, prelude::*};
use web_sys::{DragEvent, FocusEvent, KeyboardEvent, MouseEvent, PointerEvent};

use crate::{
    ElementCaptureAttr, EventHandler, Id, IdRefs, IntoAttrs, OnEvent, PropsWithStyles,
    hooks::{
        focus::{
            FocusHandle, FocusableContextAttr, FocusableContextAttrs, UseFocusRingReturn,
            UseFocusableReturn,
            use_focus_ring::{FocusRingTarget, UseFocusRingInput, use_focus_ring},
            use_focusable::{UseFocusableInput, use_focusable},
        },
        interactions::{
            ContextMenuEvent, HoverEndEvent, HoverStartEvent, LongPress, PressEvent,
            PressPropagation, PressResponderContext, UseContextMenuInput, UseHoverReturn,
            UsePressReturn, chain_optional_callbacks, use_context_menu,
            use_hover::{UseHoverInput, use_hover},
            use_keyboard::KeyboardEventWrapper,
            use_press::{UsePressInput, merge_long_press, use_press},
        },
        link::{LinkRel, LinkTarget, link_rel},
    },
    utils::{
        aria::{
            AriaChecked, AriaCurrent, AriaDisabled, AriaExpanded, AriaHasPopup, AriaPressed,
            AriaRole,
        },
        keyboard_shortcut::KeyboardShortcuts,
        part::{Part, Remount},
        probe::{either, probe_sources},
        reactions::sharing_one_effect,
        signal_attr::{FixedAttr, SignalAttr, fixed_attr, maybe_attr, signal_attr},
        spread::Spread,
    },
};

// =============================================================================
// REACT-ARIA DEVIATIONS
// =============================================================================
//
// ## API DIFFERENCES
// - Flat input with `Default`. Hooks that configure a button (menu trigger, spin button, ...)
//   return a `UseButtonInput` instead of DOM props, mirroring how react-aria passes
//   `AriaButtonProps` around. Combine them with struct update syntax:
//   `use_button(UseButtonInput { on_press_end: .., ..trigger.button })`.
// - Long presses are part of the input (`long_press`), because leptonic's `use_press` handles
//   long presses itself (react-aria uses a separate `useLongPress`).
// - Of the DOM props react-aria forwards via `filterDOMProps`, only `id`, `aria-label` and
//   `aria-labelledby` are part of the input (hooks configuring a button need them). Set others
//   directly on the element.
//
// ## DIFFERENT BEHAVIOR
// - Hover and focus tracking are built in (`is_hovered`, `is_focused`, `is_focus_visible`,
//   `data-focus-visible`). React-aria leaves this to the `Button` component of
//   react-aria-components, which combines `useButton`, `useHover` and `useFocusRing`. Every
//   leptonic button needs them, so the hook provides them.
//
// ## LEPTOS-SPECIFIC ADAPTATIONS
// - Press events on `ButtonElementType::Anchor` always propagate, so that client-side routers
//   listening on the document (leptos_router) still see link clicks. Leptonic stops event
//   propagation by default; react-aria has no such default.
//
// ## ADDITIONS
// - For `ButtonElementType::Anchor`, `rel="noopener"` is added for `LinkTarget::Blank` (unless
//   `rel` has `Opener`), so that the new browsing context gets no access to this one.
//   React-aria: `rel` as given.
// - `is_pending` (react-aria-components' `Button` handles it, with `useDisableInteractions`):
//   pending, the button keeps its focus but ignores presses, hover, keyboard handlers and
//   context menu requests, is `aria-disabled`, a submit button becomes a plain one and an
//   anchor drops its `href` and `target`.
// - `role` and `aria_checked` override the element's role (e.g. the radios of a single-selection
//   toggle button group; react-aria passes them as DOM props).
// - `shortcuts` (keyboard shortcuts while focused) and `on_context_menu` (`use_context_menu`),
//   both also from a surrounding `PressResponder`.
// - The hook merges a surrounding `PressResponder`'s props, as react-aria's `usePress` does for
//   any pressable: its press handlers, disabled state, shortcuts, context menu handler and
//   trigger props (`aria-haspopup`, `aria-expanded`, `aria-controls`, the element capture and
//   the trigger's id, which the button takes or replaces with its own; react-aria: `mergeIds`).
// - `allow_focus_when_disabled` (undocumented in react-aria) is a signal; so are
//   `prevent_focus_on_press` and `target`.
// - `aria_disabled` overrides the computed `aria-disabled` (react-aria passes `aria-disabled`
//   through).
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
    pub form: Option<String>,
    /// Overrides the form's `action`.
    pub form_action: Option<String>,
    /// Overrides the form's `enctype`.
    pub form_enc_type: Option<FormEncType>,
    /// Overrides the form's `method`.
    pub form_method: Option<FormMethod>,
    /// Overrides the form's `novalidate`.
    pub form_no_validate: bool,
    /// Overrides the form's `target`.
    pub form_target: Option<LinkTarget>,
    /// Name submitted with the form data when this button submits the form.
    pub name: Option<String>,
    /// Value submitted with the form data when this button submits the form.
    pub value: Option<String>,
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

    /// The element's id. Inside an overlay trigger (`PressResponder` with a trigger) the button
    /// gets the trigger's id without one, and its own becomes the trigger's.
    pub id: Option<Id>,

    /// An accessible name, for buttons without visible text (e.g. icon buttons).
    pub aria_label: MaybeProp<String>,

    /// The id(s) of the element(s) naming the button. A signal: e.g. a field's label ids follow
    /// whether its label is rendered.
    pub aria_labelledby: Signal<Option<Id>>,

    /// Whether the button is disabled.
    pub is_disabled: Signal<bool>,

    /// Whether the button is pending (an action it started is in progress): it stays focusable,
    /// but ignores presses, hover, keyboard handlers and context menu requests, is
    /// `aria-disabled`, a submit button turns into a plain button (so that the form can't be
    /// submitted again, also not implicitly) and an anchor loses its `href`.
    pub is_pending: Signal<bool>,

    /// Keep the button focusable (but out of the tab order) while disabled.
    pub allow_focus_when_disabled: Signal<bool>,

    /// Remove the button from the tab order. It stays focusable programmatically and by pointer.
    pub exclude_from_tab_order: Signal<bool>,

    /// Focus the button when it mounts.
    pub auto_focus: bool,

    /// Don't move focus to the button when it is pressed.
    pub prevent_focus_on_press: Signal<bool>,

    /// For `ButtonElementType::Anchor`: the link target. Removed while disabled or pending.
    pub href: Signal<Option<String>>,

    /// For `ButtonElementType::Anchor`: where to open the link. Rendered with the `href`.
    pub target: Signal<LinkTarget>,

    /// For `ButtonElementType::Anchor`: the link relationship. `NoOpener` is added for
    /// `LinkTarget::Blank` (unless it has `Opener`).
    pub rel: Vec<LinkRel>,

    /// For `ButtonElementType::Button`: form attributes.
    pub form: ButtonFormAttributes,

    /// The kind of popup the button opens.
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    /// Whether the element the button controls is expanded.
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    /// The id(s) of the element(s) the button controls.
    pub aria_controls: Signal<Option<Id>>,
    /// The id(s) of the element(s) describing the button.
    pub aria_describedby: Signal<Option<Id>>,
    /// The pressed state of a toggle button.
    pub aria_pressed: Signal<Option<AriaPressed>>,
    /// `aria-checked`, for buttons acting as checkable items (e.g. `role="radio"`).
    pub aria_checked: Signal<Option<AriaChecked>>,
    /// Overrides the element's role (e.g. `radio` for the buttons of a single-selection toggle
    /// button group).
    pub role: Option<AriaRole>,
    /// Whether the button represents the current item of a set.
    pub aria_current: Signal<Option<AriaCurrent>>,
    /// Overrides the computed `aria-disabled` (e.g. a button that announces itself disabled but
    /// stays interactive). `None`: `true` while disabled (non-native buttons) or pending.
    pub aria_disabled: Signal<Option<AriaDisabled>>,

    pub on_press: Option<Callback<PressEvent>>,
    pub on_press_start: Option<Callback<PressEvent>>,
    pub on_press_end: Option<Callback<PressEvent>>,
    pub on_press_up: Option<Callback<PressEvent>>,
    pub on_press_change: Option<Callback<bool>>,

    /// Long press handling (see `use_press`). `None`: no long presses.
    pub long_press: Option<LongPress>,

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
    pub on_context_menu: Option<Callback<ContextMenuEvent>>,
}

/// The return value of [`use_button`].
#[derive(Debug)]
pub struct UseButtonReturn {
    /// The button element: `let (attrs, styles) = button.attrs();`, the styles merged with the
    /// element's own.
    pub button: Part<PropsWithStyles<UseButtonProps>>,
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
    pub id: Option<Id>,
    pub aria_label: MaybeProp<String>,
    pub aria_labelledby: Signal<Option<Id>>,
    pub role: Option<AriaRole>,
    pub button_type: Signal<Option<&'static str>>,
    pub disabled: Signal<bool>,
    pub tabindex: Signal<Option<i32>>,
    pub href: Signal<Option<String>>,
    pub target: Signal<Option<Oco<'static, str>>>,
    pub rel: Signal<Option<String>>,
    pub form: ButtonFormAttributes,
    pub aria_disabled: Signal<Option<AriaDisabled>>,
    pub aria_haspopup: Signal<Option<AriaHasPopup>>,
    pub aria_expanded: Signal<Option<AriaExpanded>>,
    pub aria_controls: Signal<Option<Id>>,
    pub aria_pressed: Signal<Option<AriaPressed>>,
    pub aria_checked: Signal<Option<AriaChecked>>,
    pub aria_current: Signal<Option<AriaCurrent>>,
    pub aria_describedby: Signal<Option<Id>>,
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

impl Remount for UseButtonProps {
    fn remount(&self) -> Self {
        Self {
            id: self.id.clone(),
            aria_label: self.aria_label,
            aria_labelledby: self.aria_labelledby,
            role: self.role,
            button_type: self.button_type,
            disabled: self.disabled,
            tabindex: self.tabindex,
            href: self.href,
            target: self.target,
            rel: self.rel,
            form: self.form.clone(),
            aria_disabled: self.aria_disabled,
            aria_haspopup: self.aria_haspopup,
            aria_expanded: self.aria_expanded,
            aria_controls: self.aria_controls,
            aria_pressed: self.aria_pressed,
            aria_checked: self.aria_checked,
            aria_current: self.aria_current,
            aria_describedby: self.aria_describedby,
            data_focus_visible: self.data_focus_visible,
            element_capture: self.element_capture.clone(),
            on_keydown: self.on_keydown.clone(),
            on_keyup: self.on_keyup.clone(),
            on_focus: self.on_focus.clone(),
            on_blur: self.on_blur.clone(),
            on_click: self.on_click.clone(),
            on_dblclick: self.on_dblclick.clone(),
            on_pointerdown: self.on_pointerdown.clone(),
            on_contextmenu: self.on_contextmenu.clone(),
            on_pointerup: self.on_pointerup.clone(),
            on_mousedown: self.on_mousedown.clone(),
            on_dragstart: self.on_dragstart.clone(),
            on_pointerenter: self.on_pointerenter.clone(),
            on_pointerleave: self.on_pointerleave.clone(),
            context_attrs: self.context_attrs.clone(),
        }
    }
}

/// Attributes of [`UseButtonProps`], spreadable with `{..attrs}`.
pub type UseButtonAttrs = Spread<(
    (
        FixedAttr,
        FixedAttr,
        SignalAttr<Option<&'static str>>,
        SignalAttr<bool>,
        SignalAttr<Option<i32>>,
        SignalAttr<Option<String>>,
        SignalAttr<Option<Oco<'static, str>>>,
        SignalAttr<Option<String>>,
    ),
    (
        FixedAttr,
        FixedAttr,
        FixedAttr,
        FixedAttr,
        FixedAttr,
        FixedAttr,
        FixedAttr,
        FixedAttr,
    ),
    (
        SignalAttr<Option<String>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<AriaDisabled>>,
        SignalAttr<Option<AriaHasPopup>>,
        SignalAttr<Option<AriaExpanded>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<AriaPressed>>,
        SignalAttr<Option<AriaChecked>>,
        SignalAttr<Option<AriaCurrent>>,
        SignalAttr<Option<Id>>,
        SignalAttr<Option<&'static str>>,
        ElementCaptureAttr,
    ),
    (
        OnEvent<ev::keydown>,
        OnEvent<ev::keyup>,
        OnEvent<ev::focus>,
        OnEvent<ev::blur>,
        OnEvent<ev::click>,
        OnEvent<ev::dblclick>,
        OnEvent<ev::pointerdown>,
        OnEvent<ev::contextmenu>,
        OnEvent<ev::pointerup>,
        OnEvent<ev::mousedown>,
        OnEvent<ev::dragstart>,
        OnEvent<ev::pointerenter>,
        OnEvent<ev::pointerleave>,
    ),
    FocusableContextAttr,
)>;

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
        Spread((
            (
                fixed_attr("id", self.id),
                fixed_attr("role", self.role),
                signal_attr("type", self.button_type),
                signal_attr("disabled", self.disabled),
                signal_attr("tabindex", self.tabindex),
                signal_attr("href", self.href),
                signal_attr("target", self.target),
                signal_attr("rel", self.rel),
            ),
            (
                fixed_attr("form", form),
                fixed_attr("formaction", form_action),
                fixed_attr("formenctype", form_enc_type.map(FormEncType::as_str)),
                fixed_attr("formmethod", form_method.map(FormMethod::as_str)),
                fixed_attr("formnovalidate", form_no_validate),
                fixed_attr("formtarget", form_target.map(|target| target.to_oco())),
                fixed_attr("name", name),
                fixed_attr("value", value),
            ),
            (
                maybe_attr("aria-label", self.aria_label),
                signal_attr("aria-labelledby", self.aria_labelledby),
                signal_attr("aria-disabled", self.aria_disabled),
                signal_attr("aria-haspopup", self.aria_haspopup),
                signal_attr("aria-expanded", self.aria_expanded),
                signal_attr("aria-controls", self.aria_controls),
                signal_attr("aria-pressed", self.aria_pressed),
                signal_attr("aria-checked", self.aria_checked),
                signal_attr("aria-current", self.aria_current),
                signal_attr("aria-describedby", self.aria_describedby),
                signal_attr("data-focus-visible", self.data_focus_visible),
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
        ))
    }
}

/// Makes an element behave and announce itself as a button: press handling for pointer,
/// keyboard and assistive technology, focus management, hover and focus-visible state, and the
/// right attributes for the chosen [`ButtonElementType`].
///
/// ```ignore
/// let UseButtonReturn { button: props, .. } = use_button(UseButtonInput {
///     on_press: Some(Callback::new(|_| log!("pressed"))),
///     ..Default::default()
/// });
/// let (attrs, styles) = props.attrs();
/// view! { <button {..attrs} style=styles>"Press me"</button> }
/// ```
pub fn use_button(input: UseButtonInput) -> UseButtonReturn {
    crate::metrics::subject!();
    // `use_press`, `use_hover` and `use_focus_ring` react to the disabled state in one effect.
    sharing_one_effect(|| button(input))
}

/// The body of [`use_button`].
fn button(input: UseButtonInput) -> UseButtonReturn {
    let UseButtonInput {
        element_type,
        button_type,
        id,
        aria_label,
        aria_labelledby,
        is_disabled: disabled,
        is_pending,
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
        aria_disabled,
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        long_press,
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
    let responder = use_context::<PressResponderContext>();
    let trigger = responder.as_ref().and_then(|ctx| ctx.trigger);
    // A disabled responder (a disabled `MenuTrigger` or `Disclosure`) disables the button itself:
    // its `disabled` attribute, focus, hover and shortcuts, not only its presses.
    let disabled = match responder.as_ref().and_then(|ctx| ctx.is_disabled) {
        Some(responder_disabled) => either(disabled, responder_disabled),
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
    let context_menu = use_context_menu(UseContextMenuInput {
        on_context_menu: chain_optional_callbacks(
            responder.as_ref().and_then(|ctx| ctx.on_context_menu),
            on_context_menu,
        ),
    });
    let context_menu_props = context_menu.target.into_props();
    let long_press = merge_long_press(context_menu.long_press, long_press);
    // Inside a trigger, the button has an id while rendering: its own, else the trigger's.
    let id = match trigger {
        Some(trigger) => Some(trigger.element_id(id)),
        None => id,
    };
    let (aria_haspopup, aria_expanded, aria_controls) = match trigger {
        Some(trigger) => (
            Signal::derive(move || aria_haspopup.get().or_else(|| trigger.aria_haspopup.get())),
            Signal::derive(move || aria_expanded.get().or_else(|| trigger.aria_expanded.get())),
            Signal::derive(move || aria_controls.get().or_else(|| trigger.aria_controls.get())),
        ),
        None => (aria_haspopup, aria_expanded, aria_controls),
    };

    // Pending, the button keeps its focus but takes no interactions (react-aria-components'
    // `useDisableInteractions`).
    let interactions_disabled = either(disabled, is_pending);

    let UsePressReturn {
        target: press_props,
        is_pressed,
    } = use_press(UsePressInput {
        is_disabled: interactions_disabled,
        // Client-side routers (like leptos_router) handle link clicks in a document-level
        // listener, so clicks on anchors must bubble.
        propagation: if element_type == ButtonElementType::Anchor {
            PressPropagation::Continue
        } else {
            PressPropagation::Stop
        },
        prevent_focus_on_press,
        on_press,
        on_press_start,
        on_press_end,
        on_press_up,
        on_press_change,
        on_double_press: None,
        long_press,
        // Named, not `..UsePressInput::default()`, which would create a stored signal for every
        // `Signal` field, also the ones set above (`performance.md` R3).
        allow_text_selection_on_press: Signal::stored(false),
        should_cancel_on_pointer_exit: Signal::stored(false),
        force_is_pressed: Signal::stored(false),
    });
    let press_props = press_props.into_props();

    let UseFocusableReturn {
        target: focusable_props,
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
    let focusable_props = focusable_props.into_props();
    let context_attrs = focusable_props.context_attrs.clone();

    let UseHoverReturn {
        target: hover_props,
        is_hovered,
    } = use_hover(UseHoverInput {
        is_disabled: interactions_disabled,
        on_hover_start,
        on_hover_end,
        on_hover_change,
    });
    let hover_props = hover_props.into_props();

    let UseFocusRingReturn {
        target: focus_ring_props,
        is_focused,
        is_focus_visible,
    } = use_focus_ring(UseFocusRingInput {
        is_disabled: disabled,
        target: FocusRingTarget::Element,
        is_text_input: false,
        on_focus: None,
        on_blur: None,
        on_focus_change: None,
    });
    let focus_ring_props = focus_ring_props.into_props();

    let (press_props, styles) = press_props.into_inner();

    // `disabled` only exists on form controls. Other elements announce it via `aria-disabled`.
    let has_disabled_attr = matches!(
        element_type,
        ButtonElementType::Button | ButtonElementType::Input
    );
    let focusable_tabindex = focusable_props.tabindex;
    let tabindex = match probe_sources(|| allow_focus_when_disabled.get()) {
        (false, false) => focusable_tabindex,
        _ => Signal::derive(move || {
            if allow_focus_when_disabled.get() && disabled.get() {
                Some(-1)
            } else {
                focusable_tabindex.get()
            }
        }),
    };

    // `None` if the button is never pending: its handlers need no pending check then.
    let may_be_pending = match probe_sources(|| is_pending.get()) {
        (false, false) => None,
        _ => Some(is_pending),
    };

    let is_anchor = element_type == ButtonElementType::Anchor;
    // The anchor's link, while it can be followed.
    let href = if is_anchor {
        Signal::derive(move || href.get().filter(|_| !disabled.get() && !is_pending.get()))
    } else {
        Signal::stored(None)
    };
    let props = UseButtonProps {
        id,
        aria_label,
        aria_labelledby,
        role: role.or(match element_type {
            ButtonElementType::Button => None,
            _ => Some(AriaRole::Button),
        }),
        button_type: match (element_type, button_type) {
            // Pending, a submit button must not submit the form, also not implicitly (Enter in a
            // text field).
            (ButtonElementType::Button | ButtonElementType::Input, ButtonType::Submit) => {
                Signal::derive(move || {
                    Some(if is_pending.get() {
                        ButtonType::Button.as_str()
                    } else {
                        ButtonType::Submit.as_str()
                    })
                })
            }
            (ButtonElementType::Button | ButtonElementType::Input, button_type) => {
                Signal::stored(Some(button_type.as_str()))
            }
            _ => Signal::stored(None),
        },
        disabled: if has_disabled_attr {
            disabled
        } else {
            Signal::stored(false)
        },
        tabindex,
        href,
        target: Signal::derive(move || {
            href.with(Option::is_some)
                .then(|| {
                    target.with(|target| (*target != LinkTarget::Same).then(|| target.to_oco()))
                })
                .flatten()
        }),
        rel: if is_anchor {
            Signal::derive(move || target.with(|target| link_rel(target, rel.clone())))
        } else {
            Signal::stored(None)
        },
        form: if element_type == ButtonElementType::Button {
            form
        } else {
            ButtonFormAttributes::default()
        },
        aria_disabled: Signal::derive(move || {
            aria_disabled.get().or_else(|| {
                ((!has_disabled_attr && disabled.get()) || is_pending.get())
                    .then_some(AriaDisabled::True)
            })
        }),
        aria_haspopup,
        aria_expanded,
        aria_controls,
        aria_pressed,
        aria_checked,
        aria_current,
        // The long press description (from `use_press`) and a `FocusableContext`'s (a
        // tooltip's) add to the caller's.
        aria_describedby: IdRefs::derive([
            aria_describedby,
            press_props.aria_describedby,
            focusable_props.context_aria_describedby,
        ]),
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
        on_keydown: unless_pending(
            may_be_pending,
            focusable_props
                .on_keydown
                .chain(press_props.on_keydown)
                .chain(context_menu_props.on_keydown),
        ),
        on_keyup: unless_pending(may_be_pending, focusable_props.on_keyup),
        on_focus: focusable_props.on_focus.chain(focus_ring_props.on_focus),
        on_blur: focusable_props.on_blur.chain(focus_ring_props.on_blur),
        on_click: press_props.on_click,
        on_dblclick: press_props.on_dblclick,
        on_pointerdown: press_props.on_pointerdown,
        on_contextmenu: unless_pending(may_be_pending, context_menu_props.on_contextmenu),
        on_pointerup: press_props.on_pointerup,
        on_mousedown: press_props.on_mousedown,
        on_dragstart: press_props.on_dragstart,
        on_pointerenter: hover_props.on_pointerenter,
        on_pointerleave: hover_props.on_pointerleave,
        context_attrs,
    };

    UseButtonReturn {
        button: Part::stored(PropsWithStyles::new(props, styles)),
        is_disabled: disabled,
        is_pressed,
        is_hovered,
        is_focused,
        is_focus_visible,
        focus_handle,
    }
}

/// `handler`, skipped while the button is pending (`is_pending` is `None` when it never is).
fn unless_pending<E: Clone + 'static>(
    is_pending: Option<Signal<bool>>,
    handler: EventHandler<E>,
) -> EventHandler<E> {
    let Some(is_pending) = is_pending.filter(|_| !handler.is_empty()) else {
        return handler;
    };
    EventHandler::new(move |e: E| {
        if !is_pending.get_untracked() {
            handler.call(e);
        }
    })
}
